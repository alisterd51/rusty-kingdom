mod handler;
mod router;

use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let router = router::router().await;
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    #[cfg(feature = "tracing")]
    tracing::info!("listening on {}", addr);
    axum::serve(tcp_listener, router).await.unwrap();
}
