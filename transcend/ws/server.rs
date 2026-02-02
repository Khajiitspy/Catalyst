use actix::{Actor, Context, Handler, Message};
use std::collections::HashMap;
use uuid::Uuid;

pub struct WsServer {
    sessions: HashMap<Uuid, actix::Addr<WsSession>>,
}

impl WsServer {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
}

impl Actor for WsServer {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub user_id: Uuid,
    pub addr: actix::Addr<WsSession>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub user_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct IncomingMessage {
    pub user_id: Uuid,
    pub payload: String,
}

impl Handler<Connect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.user_id, msg.addr);
    }
}

impl Handler<Disconnect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.user_id);
    }
}
