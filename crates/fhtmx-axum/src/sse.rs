//! # Server sent events

use axum::{
    Router,
    extract::State,
    response::{
        Sse,
        sse::{Event, KeepAlive},
    },
    routing::get,
};
use dashmap::DashMap;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, sync::Arc};
use tokio::sync::mpsc;
use tokio_stream::{StreamExt, wrappers::ReceiverStream};
use uuid::Uuid;

/// Setups Server sent event state and routes
///
/// # Example
///
/// ```rust,ignore
/// use actix_web::App;
/// use fhtmx_axum::sse::SseSetup;
///
/// let sse_setup = SseSetup::new();
/// let sse_data = sse_setup.state();
/// App::new()
///     .configure(|cfg| sse_setup.setup_route("/sse", cfg))
///     .app_data(sse_data);
/// ```
#[derive(Clone, Copy)]
pub struct SseSetup;

impl SseSetup {
    /// Gets a `SseState` instance for you to add it to your app
    #[must_use]
    pub fn state_data() -> SseState {
        SseState::default()
    }

    /// Setups the sse route
    pub fn see_route(path: &str) -> Router<SseState> {
        Router::new().route(path, get(sse_handler))
    }
}

/// SSE state
#[derive(Clone, Default)]
pub struct SseState {
    pub sessions: Arc<DashMap<Uuid, mpsc::Sender<Event>>>,
}

impl SseState {
    pub fn add_session(
        &self,
        id: Uuid,
        sender: mpsc::Sender<Event>,
    ) -> Option<mpsc::Sender<Event>> {
        self.sessions.insert(id, sender)
    }

    pub fn remove_session(&self, id: Uuid) -> Option<(Uuid, mpsc::Sender<Event>)> {
        self.sessions.remove(&id)
    }

    /// Sends a message to session id
    pub fn send_message<D: AsRef<str>>(&self, id: Uuid, data: D) -> Option<()> {
        let sender = self.sessions.get(&id)?.clone();
        if sender.try_send(Event::default().data(data)).is_err() {
            // Channel is closed so we remove the session
            self.remove_session(id);
        }
        Some(())
    }

    /// Broadcast a message to all sessions and returns the number of sent messages
    pub fn broadcast<D: AsRef<str>>(&self, data: D) -> usize {
        let senders = self
            .sessions
            .iter()
            .map(|o| o.value().clone())
            .collect::<Vec<_>>();
        sse_broadcast(senders, data)
    }

    /// Broadcast a message to all sessions but one id and returns the number of sent messages
    pub fn broadcast_all_but<D: AsRef<str>>(&self, id: Uuid, data: D) -> usize {
        let senders = self
            .sessions
            .iter()
            .filter_map(|o| {
                if *o.key() == id {
                    None
                } else {
                    Some(o.value().clone())
                }
            })
            .collect::<Vec<_>>();
        sse_broadcast(senders, data)
    }
}

pub fn sse_broadcast<D: AsRef<str>>(senders: Vec<mpsc::Sender<Event>>, data: D) -> usize {
    let data = data.as_ref();
    senders
        .into_iter()
        .filter_map(|o| o.try_send(Event::default().data(data)).ok())
        .count()
}

/// Route to handle web sockets
#[tracing::instrument(skip_all)]
pub async fn sse_handler(
    State(state): State<SseState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let (tx, rx) = mpsc::channel(8);
    let id = Uuid::new_v4();
    state.add_session(id, tx.clone());

    let _ = tx
        .send(Event::default().data(id.to_string()).event("sse_id"))
        .await;

    let sessions = state.sessions.clone();
    tokio::spawn(async move {
        tx.closed().await;
        sessions.remove(&id);
    });

    let stream = ReceiverStream::new(rx).map(Ok);
    Sse::new(stream).keep_alive(KeepAlive::default())
}

/// Identifier for the session
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SseHandlerQuery {
    pub id: Uuid,
}
