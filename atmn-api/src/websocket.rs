use actix::{Actor, StreamHandler, Handler, Message as ActixMessage, Context as ActixContext, ActorContext, AsyncContext};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebSocketSession {
    id: usize,
    hb: Instant,
    subscriptions: Vec<String>,
}

impl WebSocketSession {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            hb: Instant::now(),
            subscriptions: Vec::new(),
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                // Handle subscriptions
                if text.starts_with("subscribe:") {
                    let topic = text.strip_prefix("subscribe:").unwrap().to_string();
                    self.subscriptions.push(topic.clone());
                    ctx.text(format!("{{\"status\":\"subscribed\",\"topic\":\"{}\"}}", topic));
                } else if text.starts_with("unsubscribe:") {
                    let topic = text.strip_prefix("unsubscribe:").unwrap();
                    self.subscriptions.retain(|s| s != topic);
                    ctx.text(format!("{{\"status\":\"unsubscribed\",\"topic\":\"{}\"}}", topic));
                }
            }
            Ok(ws::Message::Binary(_)) => {}
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub topic: String,
    pub data: String,
}

impl Handler<BroadcastMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        if self.subscriptions.contains(&msg.topic) || self.subscriptions.contains(&"all".to_string()) {
            ctx.text(msg.data);
        }
    }
}

pub async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let session = WebSocketSession::new(0);
    ws::start(session, &req, stream)
}

// Broadcast manager for sending messages to all connected clients
lazy_static::lazy_static! {
    pub static ref WS_SESSIONS: Arc<Mutex<HashMap<usize, actix::Addr<WebSocketSession>>>> = 
        Arc::new(Mutex::new(HashMap::new()));
}

pub async fn broadcast_new_block(block_data: String) {
    let sessions = WS_SESSIONS.lock().await;
    for (_id, addr) in sessions.iter() {
        addr.do_send(BroadcastMessage {
            topic: "blocks".to_string(),
            data: format!("{{\"type\":\"new_block\",\"data\":{}}}", block_data),
        });
    }
}

pub async fn broadcast_new_transaction(address: &str, tx_data: String) {
    let sessions = WS_SESSIONS.lock().await;
    for (_id, addr) in sessions.iter() {
        addr.do_send(BroadcastMessage {
            topic: format!("address:{}", address),
            data: format!("{{\"type\":\"new_transaction\",\"data\":{}}}", tx_data),
        });
    }
}
