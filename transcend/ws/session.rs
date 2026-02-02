use actix::{Actor, StreamHandler, Addr};
use actix_web_actors::ws;
use uuid::Uuid;

pub struct WsSession {
    pub user_id: Uuid,
    pub server: Addr<WsServer>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.server.do_send(Connect {
            user_id: self.user_id,
            addr: ctx.address(),
        });
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        self.server.do_send(Disconnect {
            user_id: self.user_id,
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                self.server.do_send(IncomingMessage {
                    user_id: self.user_id,
                    payload: text,
                });
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(_)) => ctx.stop(),
            _ => {}
        }
    }
}
