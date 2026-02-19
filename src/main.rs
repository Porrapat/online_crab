use actix::{Actor, AsyncContext, StreamHandler};
use actix_web::{
    get,
    web::{self, Data, Query},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use serde::Deserialize;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::broadcast;

#[derive(Clone)]
struct AppState {
    client_count: Arc<AtomicUsize>,
    tx: broadcast::Sender<usize>,
}

#[derive(Deserialize)]
struct WsQuery {
    role: Option<String>, // admin | client
}

/* =========================
   WebSocket Session Actor
========================= */

struct WsSession {
    is_admin: bool,
    state: AppState,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        if self.is_admin {
            // 🔥 send current now
            let current = self.state.client_count.load(Ordering::SeqCst);
            ctx.text(current.to_string());

            // subscribe broadcast
            let mut rx = self.state.tx.subscribe();
            let addr = ctx.address();

            actix_rt::spawn(async move {
                while let Ok(count) = rx.recv().await {
                    addr.do_send(AdminCount(count));
                }
            });
        } else {
            // client connect
            let new = self.state.client_count.fetch_add(1, Ordering::SeqCst) + 1;
            let _ = self.state.tx.send(new);
            println!("client connected -> {}", new);
        }
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        if !self.is_admin {
            let new = self.state.client_count.fetch_sub(1, Ordering::SeqCst) - 1;
            let _ = self.state.tx.send(new);
            println!("client disconnected -> {}", new);
        }
    }
}

/* =========================
   Admin receive count msg
========================= */

struct AdminCount(usize);

impl actix::Message for AdminCount {
    type Result = ();
}

impl actix::Handler<AdminCount> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: AdminCount, ctx: &mut Self::Context) {
        ctx.text(msg.0.to_string());
    }
}

/* =========================
   Handle incoming WS msgs
========================= */

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, _msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        // ignore incoming messages (MVP)
    }
}

/* =========================
   Route handler
========================= */

#[get("/ws")]
async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    query: Query<WsQuery>,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    let role = query.role.clone().unwrap_or("client".into());
    let is_admin = role == "admin";

    let session = WsSession {
        is_admin,
        state: state.get_ref().clone(),
    };

    ws::start(session, &req, stream)
}

/* =========================
   Main
========================= */

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (tx, _) = broadcast::channel(100);

    let state = AppState {
        client_count: Arc::new(AtomicUsize::new(0)),
        tx,
    };

    println!("Server running http://127.0.0.1:3000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(ws_route)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
