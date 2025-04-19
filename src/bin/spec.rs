use std::fs;

use sky_bet_roulette::handlers::Docs;
use utoipa::OpenApi;

fn main() {
    let spec = Docs::openapi().to_pretty_json().unwrap();
    fs::write("spec.json", spec.as_bytes()).unwrap();
}
