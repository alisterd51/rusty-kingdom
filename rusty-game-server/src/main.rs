mod handler;
mod router;

use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

#[derive(Clone)]
struct ApiState {
    api_url: String,
    client: reqwest::Client,
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let router = router::router();
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    #[cfg(feature = "tracing")]
    tracing::info!("listening on {}", addr);
    axum::serve(tcp_listener, router).await.unwrap();
}
