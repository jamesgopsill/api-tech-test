use axum::{
    Router,
    routing::{get, post},
};

pub mod bet;
pub mod game;
pub mod handlers;
pub mod middleware;
pub mod statics;
pub mod stubs;

/// Creates the Axum application that handles the
/// TCP traffic and provides API documentation.
pub fn create_app() -> Router {
    let mut app = Router::new();

    app = app.route("/game/new", post(handlers::post_game));

    app = app.route("/game/check", post(handlers::game_check));

    app = app.route("/game/{id}", get(handlers::get_game));

    let auth_stub = axum::middleware::from_fn(middleware::authenticate);
    app = app.layer(auth_stub);

    app = app.route("/teapot", get(handlers::teapot));

    app = app.fallback(handlers::handler_404);

    let response_time = axum::middleware::from_fn(middleware::response_time);
    app = app.layer(response_time);

    app
}
