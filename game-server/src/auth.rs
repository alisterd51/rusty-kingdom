use jsonwebtoken::{DecodingKey, Validation, decode, decode_header, jwk::JwkSet};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tonic::{Request, Status, metadata::MetadataMap, service::Interceptor};

const CLIENT_ID: &str = "rusty-client";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
    #[serde(default)]
    pub roles: Vec<String>,
}

impl Claims {
    #[must_use]
    pub fn is_admin(&self) -> bool {
        self.roles.contains(&"admin".to_string())
            || self.roles.contains(&"rauthy_admin".to_string())
    }
}

#[derive(Clone)]
pub struct AuthInterceptor {
    pub jwks: Arc<JwkSet>,
    pub issuer: String,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let Ok(token) = extract_token(request.metadata()) else {
            return Ok(request);
        };

        let header =
            decode_header(&token).map_err(|_| Status::unauthenticated("Invalid token header"))?;

        let kid = header
            .kid
            .ok_or_else(|| Status::unauthenticated("Token without key identifier (KID)"))?;

        let jwk = self
            .jwks
            .find(&kid)
            .ok_or_else(|| Status::unauthenticated("Public key not found for this token"))?;

        let decoding_key = DecodingKey::from_jwk(jwk)
            .map_err(|_| Status::internal("Internal error while reading the key"))?;

        let mut validation = Validation::new(header.alg);
        validation.set_issuer(std::slice::from_ref(&self.issuer));
        validation.set_audience(&[CLIENT_ID]);

        let token_data = decode::<Claims>(&token, &decoding_key, &validation)
            .map_err(|e| Status::unauthenticated(format!("Invalid or expired token: {e}")))?;

        request.extensions_mut().insert(token_data.claims);

        Ok(request)
    }
}

fn extract_token(metadata: &MetadataMap) -> Result<String, Status> {
    let auth_header = metadata
        .get("authorization")
        .ok_or_else(|| Status::unauthenticated("Missing 'Authorization' header"))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| Status::unauthenticated("Malformed 'Authorization' header"))?;

    let token = auth_str
        .strip_prefix("Bearer ")
        .ok_or_else(|| Status::unauthenticated("Incorrect 'Bearer' format"))?;

    Ok(token.to_string())
}
