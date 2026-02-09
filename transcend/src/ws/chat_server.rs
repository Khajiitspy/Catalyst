use actix::{Actor, Context, Handler, Message, Recipient};
use std::collections::{HashMap, HashSet};

use super::messages::WsMessage;
pub use crate::ws::messages::Leave;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub chat_id: i64,
    pub addr: Recipient<WsMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Broadcast {
    pub chat_id: i64,
    pub message: String,
}

pub struct ChatServer {
    rooms: HashMap<i64, HashSet<Recipient<WsMessage>>>,
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        self.rooms
            .entry(msg.chat_id)
            .or_default()
            .insert(msg.addr);
    }
}

impl Handler<Leave> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Leave, _: &mut Context<Self>) {
        if let Some(room) = self.rooms.get_mut(&msg.chat_id) {
            room.remove(&msg.addr);

            // optional cleanup
            if room.is_empty() {
                self.rooms.remove(&msg.chat_id);
            }
        }
    }
}

impl Handler<Broadcast> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Broadcast, _: &mut Context<Self>) {
        if let Some(room) = self.rooms.get(&msg.chat_id) {
            for client in room {
                let _ = client.do_send(WsMessage(msg.message.clone()));
            }
        }
    }
}
