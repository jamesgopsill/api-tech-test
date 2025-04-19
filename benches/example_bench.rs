use chrono::{Days, Utc};
use criterion::{Criterion, criterion_group, criterion_main};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::json;
use sky_bet_roulette::{middleware::ServiceClaims, stubs::JWT_SECRET};

/// An example of running a benchmark. Needs the server
/// to be run in a separate process.
/// Benchmarks the speed of repeated single requests.
fn criterion_benchmark(c: &mut Criterion) {
    // Check it is running
    let client = reqwest::blocking::Client::new();
    let r = client.get("http://localhost:3000/teapot").send().unwrap();
    println!("{}", r.status());

    // Create a JWT token
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
    let token = format!("Bearer {}", encoded);

    let json = json!({
        "game": "EuropeanRoulette",
        "bets": [{
            "playerId": "player_one",
            "bet": "00",
            "chipsIn": 10
        }]
    });

    let r = client
        .post("http://localhost:3000/game/new")
        .json(&json)
        .header("Authorization", &token)
        .send()
        .unwrap();
    println!("{}", r.status());

    // Setup the benchmark function.
    c.bench_function("new_game", |b| {
        b.iter(|| {
            client
                .post("http://localhost:3000/game/new")
                .json(&json)
                .header("Authorization", &token)
                .send()
                .unwrap()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
