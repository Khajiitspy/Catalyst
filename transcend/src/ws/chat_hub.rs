use actix::{Actor, StreamHandler, Addr, Handler};
use actix_web_actors::ws;
use serde_json::json;
use uuid::Uuid;
use actix::AsyncContext;
use sqlx::PgPool;

use super::chat_server::{ChatServer, Join, Broadcast};
use super::messages::{ClientWsMessage, WsMessage};
use crate::db::chat_repository::ChatRepository;

pub struct ChatSocket {
    pub user_id: Uuid,
    pub server: Addr<ChatServer>,
    pub pool: PgPool,
}


impl Actor for ChatSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<WsMessage> for ChatSocket {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            if let Ok(cmd) = serde_json::from_str::<ClientWsMessage>(&text) {
                match cmd {
                    ClientWsMessage::JoinChat { chat_id } => {
                        self.server.do_send(Join {
                            chat_id,
                            addr: ctx.address().recipient(),
                        });
                    }

                    ClientWsMessage::SendMessage { chat_id, message } => {
                        let pool = self.pool.clone();
                        let user_id = self.user_id;
                        let server = self.server.clone();

                        actix::spawn(async move {
                            let repo = ChatRepository::new(pool);

                            match repo.send_message(chat_id, user_id, &message).await {
                                Ok(saved) => {
                                    let payload = json!({
                                        "id": saved.id,
                                        "chatId": saved.chat_id,
                                        "userId": saved.user_id,
                                        "message": saved.message
                                    });

                                    server.do_send(Broadcast {
                                        chat_id,
                                        message: payload.to_string(),
                                    });
                                }

                                Err(err) => {
                                    eprintln!("Failed to send message: {err}");
                                }
                            }
                        });
                    }

                    ClientWsMessage::LeaveChat { chat_id } => {
                    }
                }
            }
        }
    }
}
