use serde::{Deserialize, Serialize};
use uuid::Uuid;
use actix::Message;
use actix::Recipient;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClientWsMessage {
    JoinChat { chat_id: Uuid },
    LeaveChat { chat_id: Uuid },
    SendMessage { chat_id: Uuid, message: String },
}

#[derive(Serialize)]
pub struct ServerWsMessage<T> {
    pub event: &'static str,
    pub data: T,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Leave {
    pub chat_id: Uuid,
    pub addr: Recipient<WsMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);
