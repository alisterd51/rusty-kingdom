pub const GAME_URL: &str = "http://localhost:8080";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let _client = reqwest::Client::new();
    Ok(())
}
