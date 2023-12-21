use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::sync::{
    broadcast::{self, Sender},
    RwLock,
};
use tracing::{debug, info, warn};

use crate::router::{self, Chat, ResponseError};

pub async fn handle_socket(mut socket: WebSocket) {
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

pub async fn task_01(ws: WebSocketUpgrade) -> Result<impl IntoResponse, ResponseError> {
    Ok(ws.on_upgrade(handle_socket))
}

pub async fn task_02_reset(
    State(state): State<Arc<router::State>>,
) -> Result<impl IntoResponse, ResponseError> {
    let mut views = state.views.write().await;
    *views = 0;
    Ok(())
}

pub async fn task_02_views(
    State(state): State<Arc<router::State>>,
) -> Result<impl IntoResponse, ResponseError> {
    let views = state.views.read().await;
    info!(?views);
    Ok(views.to_string())
}

async fn task_02_handler(
    socket: WebSocket,
    tx: Arc<Sender<Chat>>,
    name: String,
    views: Arc<RwLock<usize>>,
) {
    let (sender, receiver) = socket.split();

    tokio::spawn(write(sender, tx.clone(), views));
    tokio::spawn(read(receiver, tx, name));
}

const RETRIES: usize = 5;

async fn write(
    mut sender: SplitSink<WebSocket, Message>,
    tx: Arc<Sender<Chat>>,
    views: Arc<RwLock<usize>>,
) {
    let mut rx = tx.subscribe();
    while let Ok(msg) = rx.recv().await {
        for attempt in 1..=RETRIES {
            if let Err(e) = sender
                .send(Message::Text(serde_json::to_string(&msg).unwrap()))
                .await
            {
                debug!(
                    "Failed to send message to websocket attempt {}: {:?}",
                    attempt, e
                );
            } else {
                let mut views = views.write().await;
                *views += 1;
                break;
            }
        }
    }
}

async fn read(mut reciever: SplitStream<WebSocket>, tx: Arc<Sender<Chat>>, name: String) {
    while let Some(Ok(Message::Text(text))) = reciever.next().await {
        if let Ok(mut chat) = serde_json::from_str::<Chat>(&text) {
            if chat.message.len() <= 128 {
                chat.user = Some(name.clone());
                debug!(?chat);
                for attempt in 1..=RETRIES {
                    if let Err(e) = tx.send(chat.clone()) {
                        warn!(
                            "Failed to transmit message to sender attempt {}: {:?}",
                            attempt, e
                        );
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

pub async fn task_02_room(
    Path((number, name)): Path<(usize, String)>,
    State(state): State<Arc<router::State>>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, ResponseError> {
    debug!(?name);
    let mut rooms = state.rooms.write().await;
    let room = if let Some(tx) = rooms.get(&number) {
        tx.clone()
    } else {
        let (tx, _rx) = broadcast::channel(100);
        rooms.insert(number, Arc::new(tx));
        rooms.get(&number).unwrap().clone()
    };
    drop(rooms);
    Ok(ws.on_upgrade(move |ws| task_02_handler(ws, room, name, state.views.clone())))
}
