use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::{SinkExt, StreamExt};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};
use tokio::{spawn, sync::broadcast::Sender};

#[derive(Clone, serde::Serialize)]
struct Message {
    user: String,
    message: String,
}

#[derive(Clone)]
struct Day19State {
    view_count: Arc<AtomicU64>,
    sockets: Arc<Mutex<HashMap<u64, Sender<Message>>>>,
}

pub(crate) fn router() -> Router {
    Router::new()
        .route("/19/ws/ping", get(ping_handler))
        .route("/19/reset", post(reset))
        .route("/19/views", get(views))
        .route("/19/ws/room/:id/user/:user", get(room_handler))
        .with_state(Day19State {
            view_count: Arc::new(AtomicU64::new(0)),
            sockets: Arc::new(Mutex::new(HashMap::new())),
        })
}

async fn ping_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(ping)
}

async fn ping(mut ws: WebSocket) {
    let mut served = false;
    while let Some(Ok(msg)) = ws.recv().await {
        if let Ok(msg) = msg.to_text() {
            match msg {
                "serve" => served = true,
                "ping" if served => {
                    let _ = ws.send("pong".into()).await;
                }
                _ => {}
            }
        }
    }
}

async fn reset(State(state): State<Day19State>) {
    state.view_count.store(0, Ordering::Relaxed);
}

async fn views(State(state): State<Day19State>) -> impl IntoResponse {
    state.view_count.load(Ordering::Relaxed).to_string()
}

async fn room_handler(
    ws: WebSocketUpgrade,
    Path((id, user)): Path<(u64, String)>,
    State(state): State<Day19State>,
) -> impl IntoResponse {
    ws.on_upgrade(move |s| room(s, id, user, state))
}

#[derive(serde::Deserialize)]
struct WsMsg {
    message: String,
}

async fn room(ws: WebSocket, id: u64, user: String, state: Day19State) {
    let send = {
        let Ok(mut map) = state.sockets.lock() else {
            return;
        };

        if let Some(ch) = map.get(&id) {
            ch.clone()
        } else {
            let ch = Sender::new(128);
            map.insert(id, ch.clone());
            ch
        }
    };

    let (mut ws_send, mut ws_recv) = ws.split();

    let mut recv = send.subscribe();
    let recv_task = spawn(async move {
        while let Ok(message) = recv.recv().await {
            if let Ok(message) = serde_json::to_string(&message) {
                if ws_send.send(message.into()).await.is_ok() {
                    state.view_count.fetch_add(1, Ordering::Relaxed);
                } else {
                    return;
                }
            }
        }
    });

    while let Some(Ok(msg)) = ws_recv.next().await {
        if let Ok(msg) = msg.into_text() {
            if let Ok(WsMsg { message }) = serde_json::from_str::<WsMsg>(&msg) {
                if message.len() <= 128 {
                    let _ = send.send(Message {
                        user: user.clone(),
                        message,
                    });
                }
            }
        }
    }

    recv_task.abort();
}
