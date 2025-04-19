use sky_bet_roulette::create_app;
use tracing::info;

/// Runtime for the application. Can be configured
/// for single and multi-threaded environments.
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Creating app.");
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Serving app at localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod test {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use chrono::{Days, Utc};
    use jsonwebtoken::{EncodingKey, Header, encode};
    use serde_json::json;

    use sky_bet_roulette::{
        create_app, game::PlayedGame, middleware::ServiceClaims, stubs::JWT_SECRET,
    };

    /// Creates a test server.
    fn test_server() -> TestServer {
        let app = create_app();
        TestServer::new(app).unwrap()
    }

    // Creates a valid JWT for testing.
    fn valid_token() -> String {
        // Create a json web token that represents an
        // example authentication between services.
        let now = Utc::now();
        let exp = now.checked_add_days(Days::new(1)).unwrap();
        let now = now.timestamp() as u64;
        let exp = exp.timestamp() as u64;
        let claims = ServiceClaims {
            service_id: "test_service_id".to_string().into_boxed_str(),
            iat: now,
            nbf: now,
            exp,
        };
        let header = Header::default();
        let key = EncodingKey::from_secret(JWT_SECRET.as_ref());
        let encoded = encode(&header, &claims, &key).unwrap();
        format!("Bearer {}", encoded)
    }

    #[tokio::test]
    async fn test_teapot() {
        let server = test_server();
        let response = server.get("/teapot").await;
        response.assert_status(StatusCode::IM_A_TEAPOT);
    }

    #[tokio::test]
    async fn test_404() {
        let server = test_server();
        let response = server.get("/dead_end").await;
        response.assert_status_not_found()
    }

    #[tokio::test]
    async fn test_play_roulette() {
        let mut server = test_server();
        let token = valid_token();

        let json = json!({
            "game": "EuropeanRoulette",
            "bets": [{
                "playerId": "player_one",
                "bet": "00",
                "chipsIn": 10
            }]
        });

        server.add_header("Authorization", token);

        let response = server.post("/game/new").json(&json).await;

        response.assert_status_ok();
        println!("{:?}", response);
        let json = response.json::<PlayedGame>();
        println!("{:?}", json);

        let url = format!("/game/{}", json.uuid);
        let response = server.get(&url).await;
        println!("{:?}", response);
        response.assert_status_ok();

        let response = server.get("/game/wrong-uuid").await;
        println!("{:?}", response);
        response.assert_status_not_ok();
    }

    #[tokio::test]
    async fn test_bet_check() {
        let mut server = test_server();
        let token = valid_token();

        let json = json!({
            "game": "EuropeanRoulette",
            "bets": [{
                "playerId": "player_one",
                "bet": "00",
                "chipsIn": 10
            }]
        });
        server.add_header("Authorization", token);

        let response = server.post("/game/check").json(&json).await;

        response.assert_status(StatusCode::NO_CONTENT);
        println!("{:?}", response);
    }

    // TODO: fuzzy test and benchmarks.
}
