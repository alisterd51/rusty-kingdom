use serde_json::json;
use tracing::info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let rauthy_url = std::env::var("RAUTHY_URL")
        .unwrap_or_else(|_| "https://auth.rusty.anclarma.fr".to_string());
    let api_key_name =
        std::env::var("BOOTSTRAP_API_KEY_NAME").unwrap_or_else(|_| "bootstrap".to_string());
    let api_key_secret = std::env::var("BOOTSTRAP_API_KEY_SECRET")
        .map_err(|_| "BOOTSTRAP_API_KEY_SECRET must be set")?;
    let client_id = std::env::var("CLIENT_ID").unwrap_or_else(|_| "rusty-client".to_string());
    let client_name = std::env::var("CLIENT_NAME").unwrap_or_else(|_| "Rusty Client".to_string());
    let public_url =
        std::env::var("PUBLIC_URL").unwrap_or_else(|_| "https://rusty.anclarma.fr".to_string());

    let client = reqwest::blocking::Client::new();
    let auth_header = format!("API-Key {api_key_name}${api_key_secret}");

    info!(url = %rauthy_url, client_id = %client_id, "Starting Rauthy client initialization");

    let post_payload = json!({
        "id": client_id,
        "name": client_name,
        "confidential": false,
        "redirect_uris": [
            public_url
        ],
        "post_logout_redirect_uris": [
            public_url
        ]
    });

    let _res = client
        .post(format!("{rauthy_url}/auth/v1/clients"))
        .header("Authorization", &auth_header)
        .json(&post_payload)
        .send()?;

    info!("Injecting advanced configuration (CORS, etc.)...");

    let put_payload = json!({
        "id": client_id,
        "name": client_name,
        "enabled": true,
        "confidential": false,
        "redirect_uris": [
            public_url
        ],
        "post_logout_redirect_uris": [
            format!("{public_url}?destroy_session=true")
        ],
        "allowed_origins": [
            public_url
        ],
        "flows_enabled": [
            "authorization_code",
            "refresh_token",
            "urn:ietf:params:oauth:grant-type:device_code"
        ],
        "access_token_alg": "RS256",
        "id_token_alg": "RS256",
        "auth_code_lifetime": 60,
        "access_token_lifetime": 1800,
        "scopes": [
            "openid",
            "email",
            "profile",
            "groups"
        ],
        "default_scopes": [
            "openid"
        ],
        "challenges": [
            "S256"
        ],
        "force_mfa": false
    });

    client
        .put(format!("{rauthy_url}/auth/v1/clients/{client_id}"))
        .header("Authorization", auth_header)
        .json(&put_payload)
        .send()?
        .error_for_status()?;

    info!("Client configuration fully updated.");
    Ok(())
}
