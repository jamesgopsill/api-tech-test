use axum::{
    extract::Request,
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use tokio::time::Instant;
use tracing::info;

use crate::stubs::JWT_SECRET;

/// The structure of the expected JsonWebToken from
/// an authorised service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceClaims {
    // Service ID
    pub service_id: Box<str>,
    pub iat: u64,
    pub exp: u64,
    pub nbf: u64,
}

/// Provides a response time for all endpoints.
pub async fn response_time(req: Request, next: Next) -> Result<impl IntoResponse, Response> {
    let uri = req.uri().clone();
    info!("{} Response Time [v]", uri);
    let now = Instant::now();
    let mut res = next.run(req).await;
    let headers = res.headers_mut();
    let mut elapsed = now.elapsed().as_micros().to_string();
    elapsed.push_str(" us");
    let val = HeaderValue::from_str(&elapsed).unwrap();
    headers.insert("x-response-time", val);
    info!("{} Response Time [^]", uri);
    Ok(res)
}

/// Authentication middleware stub that authenticates a JWT. This
/// would be replaced with the mechanism used by SB&G.
pub async fn authenticate(mut req: Request, next: Next) -> Result<impl IntoResponse, Response> {
    let uri = req.uri().clone();
    info!("{} Authenticate [v]", uri);
    let auth_header = req.headers().get("Authorization");
    if auth_header.is_none() {
        return Ok((StatusCode::FORBIDDEN, "No Authorization Header").into_response());
    }
    let auth_header = auth_header.unwrap();
    let header = auth_header.to_str().unwrap().to_string();
    if !header.contains("Bearer") {
        let res = (StatusCode::FORBIDDEN, "Authorization Wrong Format").into_response();
        return Ok(res);
    }
    let token = header.split_whitespace().last();
    if token.is_none() {
        let res = (StatusCode::FORBIDDEN, "Authorization Wrong Format").into_response();
        return Ok(res);
    }
    let token = token.unwrap();
    let key = DecodingKey::from_secret(JWT_SECRET.as_ref());
    let validator = Validation::default();
    let claims = decode::<ServiceClaims>(token, &key, &validator);
    match claims {
        Err(_) => {
            let res = (StatusCode::FORBIDDEN, "Token Validation Error").into_response();
            return Ok(res);
        }
        Ok(c) => {
            req.extensions_mut().insert(c.claims);
        }
    }
    let res = next.run(req).await;
    info!("{} Authenticate [^]", uri);
    Ok(res)
}
