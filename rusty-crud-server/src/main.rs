mod handler;
mod router;
#[cfg(feature = "tracing")]
mod trace;

use std::net::{Ipv4Addr, SocketAddr};
use tokio::{
    net::TcpListener,
    signal::unix::{SignalKind, signal},
};
#[cfg(feature = "tracing")]
use trace::init_tracing_subscriber;

#[tokio::main]
async fn main() {
    #[cfg(feature = "tracing")]
    let _guard = init_tracing_subscriber();
    let router = router::router().await;
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    #[cfg(feature = "tracing")]
    tracing::info!("listening on {}", addr);
    axum::serve(tcp_listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let mut stream = signal(SignalKind::terminate()).unwrap();
    stream.recv().await;
}
