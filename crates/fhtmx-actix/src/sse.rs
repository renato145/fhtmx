//! # Server sent events

use actix_web::{Responder, web};
use actix_web_lab::sse::{Data, Event};
use dashmap::DashMap;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{marker::PhantomData, sync::Arc, time::Duration};
use tokio::{sync::mpsc, task::JoinHandle};
use uuid::Uuid;

/// Setups Server sent event state and routes
///
/// # Example
///
/// ```rust,ignore
/// use actix_web::App;
/// use fhtmx_actix::sse::SseSetup;
///
/// let sse_setup = SseSetup::new();
/// let sse_data = sse_setup.state();
/// App::new()
///     .configure(|cfg| sse_setup.setup_route("/sse", cfg))
///     .app_data(sse_data);
/// ```
#[derive(Clone, Copy)]
pub struct SseSetup<T> {
    session_data: PhantomData<T>,
}

/// Setup sse without session data
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct FhtmxUiNoSessionData;

impl SseSetup<()> {
    pub fn new_with_data<T>() -> SseSetup<T> {
        SseSetup {
            session_data: PhantomData,
        }
    }
}

impl Default for SseSetup<FhtmxUiNoSessionData> {
    fn default() -> Self {
        Self::new()
    }
}

impl SseSetup<FhtmxUiNoSessionData> {
    pub fn new() -> Self {
        SseSetup {
            session_data: PhantomData,
        }
    }
}

impl<T> SseSetup<T>
where
    T: DeserializeOwned + 'static,
{
    /// Gets a `SseState` instance for you to add it to your app
    #[must_use]
    pub fn state_data(&self) -> web::Data<SseState<T>> {
        web::Data::new(SseState::default())
    }

    /// Setups the sse route
    pub fn setup_route(&self, path: &str, cfg: &mut web::ServiceConfig) {
        cfg.route(path, web::get().to(sse_handler::<T>));
    }
}

impl<T> SseSetup<T>
where
    T: Send + Sync + 'static,
{
    /// Gets a `SseState` instance for you to add it to your app and launches a task to execute
    /// `SseState::remove_stale_sessions`.
    #[must_use]
    pub fn state_data_and_spawn_cleaner(&self, period: Duration) -> web::Data<SseState<T>> {
        let state = web::Data::new(SseState::default());
        spawn_remove_stale_sessions_task(state.clone(), period);
        state
    }
}

/// SSE state
pub struct SseState<T> {
    pub sessions: Arc<DashMap<Uuid, SseSession<T>>>,
}

impl<T> Default for SseState<T> {
    fn default() -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
        }
    }
}

impl<T: Clone> SseState<T> {
    pub fn get_session_data(&self, id: Uuid) -> Option<T> {
        self.sessions.get(&id).and_then(|x| x.data.clone())
    }
}

impl<T> SseState<T> {
    pub fn add_session(
        &self,
        id: Uuid,
        data: Option<T>,
        sender: mpsc::Sender<Event>,
    ) -> Option<SseSession<T>> {
        let session = SseSession { data, sender };
        self.sessions.insert(id, session)
    }

    pub fn remove_session(&self, id: Uuid) -> Option<(Uuid, SseSession<T>)> {
        self.sessions.remove(&id)
    }

    pub fn remove_stale_sessions(&self) -> usize {
        let mut removed = 0;
        self.sessions.retain(
            |_, o| match o.sender.try_send(Event::Comment("ping".into())) {
                Ok(()) => true,
                Err(_) => {
                    removed += 1;
                    false
                }
            },
        );
        removed
    }

    /// Sends a message to session id
    pub fn send_message(&self, id: Uuid, data: Data) -> Option<()> {
        let sender = self.sessions.get(&id)?.sender.clone();
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
            .map(|o| o.value().sender.clone())
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
                    Some(o.value().sender.clone())
                }
            })
            .collect::<Vec<_>>();
        sse_broadcast(senders, data)
    }
}

/// Launches task to clean disconnected sessions
pub fn spawn_remove_stale_sessions_task<T>(
    state: web::Data<SseState<T>>,
    period: Duration,
) -> JoinHandle<()>
where
    T: Send + Sync + 'static,
{
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(period);
        loop {
            interval.tick().await;
            let removed = state.remove_stale_sessions();
            if removed > 0 {
                tracing::info!("Removed {removed} staled sessions.");
            }
        }
    })
}

pub fn sse_broadcast(senders: Vec<mpsc::Sender<Event>>, data: Data) -> usize {
    senders
        .into_iter()
        .filter_map(|o| o.try_send(Event::Data(data.clone())).ok())
        .count()
}

pub struct SseSession<T> {
    pub data: Option<T>,
    pub sender: mpsc::Sender<Event>,
}

/// Identifier for the session
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SseHandlerQuery {
    pub id: Uuid,
}

/// Route to handle web sockets
#[tracing::instrument(skip_all)]
pub async fn sse_handler<T>(
    web::Query(query): web::Query<SseHandlerQuery>,
    state: web::Data<SseState<T>>,
) -> impl Responder {
    let (tx, rx) = mpsc::channel(8);
    state.add_session(query.id, None, tx);
    // TODO: spawn task to automatically clean sessions here
    actix_web_lab::sse::Sse::from_infallible_receiver(rx).with_keep_alive(Duration::from_secs(3))
}
