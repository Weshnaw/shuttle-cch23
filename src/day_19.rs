use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    u64,
};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    Mutex,
};
use tracing::{info, warn};

use crate::router::Error;

async fn task_01_ping(ws: WebSocketUpgrade) -> Result<impl IntoResponse, Error> {
    info!("Created ping socket");
    Ok(ws.on_upgrade(handle_socket))
}

async fn handle_socket(mut socket: WebSocket) {
    let mut playing = false;
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        let msg_text = msg.to_text().unwrap_or_default();

        if msg_text == "serve" {
            playing = true;
        } else if playing
            && msg_text == "ping"
            && socket
                .send(Message::Text("pong".to_string()))
                .await
                .is_err()
        {
            // client disconnected
            return;
        }
    }
}

#[derive(Clone)]
struct BirdState {
    views: Arc<AtomicU64>,
    rooms: Arc<Mutex<HashMap<usize, Sender<ChatTx>>>>,
}

async fn task_02_reset(State(state): State<BirdState>) -> Result<impl IntoResponse, Error> {
    state.views.store(0, Ordering::SeqCst);
    info!("Reset views");
    Ok(())
}

async fn task_02_views(State(state): State<BirdState>) -> Result<impl IntoResponse, Error> {
    let views = state.views.load(Ordering::Relaxed);
    info!(?views);
    Ok(views.to_string())
}

const BROADCAST_CAPACITY: usize = 1024;
async fn task_02_room(
    Path((number, name)): Path<(usize, Arc<str>)>,
    State(state): State<BirdState>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, Error> {
    info!(?name, ?number);

    let (tx, rx) = {
        let mut rooms = state.rooms.lock().await;

        match rooms.entry(number) {
            Entry::Occupied(o) => {
                let tx = o.into_mut();
                (tx.clone(), tx.subscribe())
            }
            Entry::Vacant(v) => {
                let (tx, rx) = broadcast::channel(BROADCAST_CAPACITY);
                v.insert(tx.clone());
                (tx, rx)
            }
        }
    };

    Ok(ws.on_upgrade(move |ws| room_handler(ws, tx, rx, name, state.views.clone())))
}

async fn room_handler(
    ws: WebSocket,
    tx: Sender<ChatTx>,
    rx: Receiver<ChatTx>,
    name: Arc<str>,
    views: Arc<AtomicU64>,
) {
    let (sender, receiver) = ws.split();

    tokio::spawn(tx_handler(receiver, tx, name));
    tokio::spawn(rx_handler(sender, rx, views));
}

#[derive(Deserialize)]
struct ChatRx {
    message: Arc<str>,
}

#[derive(Serialize, Clone, Debug)]
struct ChatTx {
    user: Arc<str>,
    message: Arc<str>,
}

async fn tx_handler(mut ws_receiver: SplitStream<WebSocket>, tx: Sender<ChatTx>, name: Arc<str>) {
    while let Some(Ok(Message::Text(text))) = ws_receiver.next().await {
        if let Ok(chat) = serde_json::from_str::<ChatRx>(&text) {
            if chat.message.len() <= 128 {
                let tx_message = ChatTx {
                    user: name.clone(),
                    message: chat.message.clone(),
                };
                info!(?tx_message);

                if let Err(e) = tx.send(tx_message) {
                    warn!("Failed to transmit message: {:?}", e);
                }
            }
        }
    }

    // cleanup socket somehow
}
async fn rx_handler(
    mut ws_sender: SplitSink<WebSocket, Message>,
    mut rx: Receiver<ChatTx>,
    views: Arc<AtomicU64>,
) {
    while let Ok(msg) = rx.recv().await {
        if let Err(e) = ws_sender
            .send(Message::Text(serde_json::to_string(&msg).unwrap()))
            .await
        {
            warn!("Failed to send message to ws: {:?}", e);
            break;
        } else {
            views.fetch_add(1, Ordering::SeqCst);
        }
    }
}

pub fn router() -> Router {
    let state = BirdState {
        views: Arc::new(AtomicU64::new(0)),
        rooms: Arc::new(Mutex::new(HashMap::new())),
    };

    Router::new()
        .route("/ws/ping", get(task_01_ping))
        .route("/reset", post(task_02_reset))
        .route("/views", get(task_02_views))
        .route("/ws/room/:number/user/:name", get(task_02_room))
        .with_state(state)
}

