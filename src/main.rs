use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_ws::{Message};
use futures_util::StreamExt;
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
    role: Option<String>,
}

#[get("/ws")]
async fn ws_route(
    req: HttpRequest,
    body: web::Payload,
    state: web::Data<AppState>,
    query: web::Query<WsQuery>,
) -> HttpResponse {
    let (response, mut session, mut msg_stream) =
        actix_ws::handle(&req, body).unwrap();

    let role = query.role.clone().unwrap_or("client".into());
    let is_admin = role == "admin";

    let state_clone = state.get_ref().clone();

    actix_web::rt::spawn(async move {
        if is_admin {
            // 🔥 ส่ง snapshot ทันที
            let current = state_clone.client_count.load(Ordering::SeqCst);
            let _ = session.text(current.to_string()).await;

            let mut rx = state_clone.tx.subscribe();

            while let Ok(count) = rx.recv().await {
                if session.text(count.to_string()).await.is_err() {
                    break;
                }
            }
        } else {
            // client connect
            let new = state_clone.client_count.fetch_add(1, Ordering::SeqCst) + 1;
            let _ = state_clone.tx.send(new);

            println!("client connected -> {}", new);

            // รอจนกว่าจะ disconnect
            while let Some(Ok(msg)) = msg_stream.next().await {
                match msg {
                    Message::Close(_) => break,
                    _ => {}
                }
            }

            let new = state_clone.client_count.fetch_sub(1, Ordering::SeqCst) - 1;
            let _ = state_clone.tx.send(new);

            println!("client disconnected -> {}", new);
        }
    });

    response
}

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
            .app_data(web::Data::new(state.clone()))
            .service(ws_route)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
