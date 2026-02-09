use serde::{Deserialize, Serialize};
use actix::Message;
use actix::Recipient;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClientWsMessage {
    JoinChat { chat_id: i64 },
    LeaveChat { chat_id: i64 },
    SendMessage { chat_id: i64, message: String },
}

#[derive(Serialize)]
pub struct ServerWsMessage<T> {
    pub event: &'static str,
    pub data: T,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Leave {
    pub chat_id: i64,
    pub addr: Recipient<WsMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);
