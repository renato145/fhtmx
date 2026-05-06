//! # Server sent events

use actix_web::{Responder, web};
use actix_web_lab::sse::{Data, Event};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;
use uuid::Uuid;

/// Setups Server sent event state and routes
///
/// # Example
///
/// ```rust,ignore
/// use actix_web::App;
/// use fhtmx_actix::sse::SseSetup;
///
/// let sse_data = SseSetup::state_data();
/// App::new()
///     .configure(|cfg| SseSetup::setup_route("/sse", cfg))
///     .app_data(sse_data);
/// ```
#[derive(Clone, Copy)]
pub struct SseSetup;

impl SseSetup {
    /// Gets a `SseState` instance for you to add it to your app
    #[must_use]
    pub fn state_data() -> web::Data<SseState> {
        web::Data::new(SseState::default())
    }

    /// Setups the sse route
    pub fn setup_route(path: &str, cfg: &mut web::ServiceConfig) {
        cfg.route(path, web::get().to(sse_handler));
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
    pub fn send_message(&self, id: Uuid, data: Data) -> Option<()> {
        let sender = self.sessions.get(&id)?.clone();
        if sender.try_send(Event::Data(data)).is_err() {
            // Channel is closed so we remove the session
            self.remove_session(id);
        }
        Some(())
    }

    /// Broadcast a message to all sessions and returns the number of sent messages
    pub fn broadcast(&self, data: Data) -> usize {
        let senders = self
            .sessions
            .iter()
            .map(|o| o.value().clone())
            .collect::<Vec<_>>();
        sse_broadcast(senders, data)
    }

    /// Broadcast a message to all sessions but one id and returns the number of sent messages
    pub fn broadcast_all_but(&self, id: Uuid, data: Data) -> usize {
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

pub fn sse_broadcast(senders: Vec<mpsc::Sender<Event>>, data: Data) -> usize {
    senders
        .into_iter()
        .filter_map(|o| o.try_send(Event::Data(data.clone())).ok())
        .count()
}

/// Route to handle web sockets
#[tracing::instrument(skip_all)]
pub async fn sse_handler(state: web::Data<SseState>) -> impl Responder {
    let (tx, rx) = mpsc::channel(8);
    let id = Uuid::new_v4();
    state.add_session(id, tx.clone());

    let _ = tx
        .send(Event::Data(Data::new(id.to_string()).event("sse_id")))
        .await;

    let sessions = state.sessions.clone();
    tokio::spawn(async move {
        tx.closed().await;
        sessions.remove(&id);
    });

    actix_web_lab::sse::Sse::from_infallible_receiver(rx).with_keep_alive(Duration::from_secs(3))
}

/// Identifier for the session
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SseHandlerQuery {
    pub sse_id: Uuid,
}
