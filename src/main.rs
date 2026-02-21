use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_ws::Message;
use futures_util::StreamExt;
use serde::Deserialize;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::watch;
use std::env;
use dotenvy::dotenv;
use tracing_subscriber::{EnvFilter};
use tracing::info;

#[derive(Clone)]
struct AppState {
    client_count: Arc<AtomicUsize>,
    tx: watch::Sender<usize>,
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
            let mut rx = state_clone.tx.subscribe();

            // 🔥 ส่ง snapshot ล่าสุดทันที
            let current = *rx.borrow();
            let _ = session.text(current.to_string()).await;

            loop {
                tokio::select! {
                    _ = rx.changed() => {
                        let count = *rx.borrow();
                        if session.text(count.to_string()).await.is_err() {
                            break;
                        }
                    }

                    Some(Ok(msg)) = msg_stream.next() => {
                        if let Message::Close(_) = msg {
                            break;
                        }
                    }

                    else => break,
                }
            }
        } else {
            let mut rx = state_clone.tx.subscribe();

            // ====================
            // CLIENT CONNECT
            // ====================
            let new = state_clone
                .client_count
                .fetch_add(1, Ordering::SeqCst) + 1;

            let _ = state_clone.tx.send(new);
            info!("client connected -> {}", new);

            // ====================
            // 🔥 DISCONNECT GUARD
            // ====================
            struct DisconnectGuard {
                state: AppState,
            }

            impl Drop for DisconnectGuard {
                fn drop(&mut self) {
                    let new = self.state
                        .client_count
                        .fetch_sub(1, Ordering::SeqCst) - 1;

                    let _ = self.state.tx.send(new);
                    info!("client disconnected -> {}", new);
                }
            }

            let _guard = DisconnectGuard {
                state: state_clone.clone(),
            };

            // ====================
            // SEND SNAPSHOT ทันที
            // ====================
            let current = *rx.borrow();
            let _ = session.text(current.to_string()).await;

            // ====================
            // LOOP รอ event
            // ====================
            loop {
                tokio::select! {

                    // broadcast update
                    _ = rx.changed() => {
                        let count = *rx.borrow();
                        if session.text(count.to_string()).await.is_err() {
                            break;
                        }
                    }

                    // ถ้า stream จบ = disconnect
                    msg = msg_stream.next() => {
                        match msg {
                            Some(Ok(Message::Close(_))) => break,
                            Some(Ok(_)) => {}
                            Some(Err(_)) => break,
                            None => break,
                        }
                    }
                }
            }

            // 🔥 ไม่ต้องเขียน decrement ตรงนี้แล้ว
            // เพราะ Drop guard จะทำงานอัตโนมัติ
        }
    });

    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse()
        .expect("PORT must be a number");

    let (tx, _) = watch::channel(0usize);

    let state = AppState {
        client_count: Arc::new(AtomicUsize::new(0)),
        tx,
    };

    info!("Server running http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(ws_route)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}