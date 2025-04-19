use axum::{Extension, Json, extract::Path, http::StatusCode, response::IntoResponse};
use tracing::info;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    game::{GameRequest, PlayedGame},
    middleware::ServiceClaims,
    stubs::DB,
};

#[derive(OpenApi)]
#[openapi(paths(teapot, game_check, post_game, get_game))]
pub struct Docs;

/// An alive service test.
#[utoipa::path(
    get,
    path = "/teapot",
    responses(
        (status = IM_A_TEAPOT, description = "Teapot found"),
        (status = NOT_FOUND, description = "Teapot was not found")
    ),
)]
pub(crate) async fn teapot() -> impl IntoResponse {
    info!("teapot()");
    StatusCode::IM_A_TEAPOT
}

/// Handle any invalid route requests.
pub(crate) async fn handler_404() -> impl IntoResponse {
    info!("handler_404()");
    (StatusCode::NOT_FOUND, "")
}

/// Confirms whether a game is valid or not.
#[utoipa::path(
    post,
    path = "/game/check",
    responses(
        (status = OK, description = "The game is valid and can be played by the service."),
        (status = BAD_REQUEST, description = "The is an issue with the game setup that needs to be addressed.")
    ),
)]
pub(crate) async fn game_check(Json(gr): Json<GameRequest>) -> impl IntoResponse {
    info!("game_check()");
    match gr.is_valid() {
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
    }
}

/// Play a game of roulette.
#[utoipa::path(
    get,
    path = "/game/new",
    params(GameRequest),
    responses(
        (status = OK, description = "The game is valid and can be played by the service.", body = PlayedGame),
        (status = BAD_REQUEST, description = "The was an issue with the game setup that needs to be addressed.")
    ),
    security(
        ("authorization" = ["Bearer Token Required"])
    )
)]
pub(crate) async fn post_game(
    Extension(sc): Extension<ServiceClaims>,
    Json(mut gr): Json<GameRequest>,
) -> impl IntoResponse {
    info!("post_game()");
    match gr.check_and_play(sc.service_id) {
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        Ok(p) => {
            let mut db = DB.write().await;
            db.insert(p.uuid, p.clone());
            (StatusCode::OK, Json(p)).into_response()
        }
    }
}

/// Get a past Roulette game.
#[utoipa::path(
    get,
    path = "/game/{id}",
    params(
    	("id" = Uuid, Path, description = "Game id")
    ),
    responses(
        (status = OK, description = "Game Details.", body = PlayedGame),
        (status = BAD_REQUEST, description = "The was an issue getting the game.")
    ),
    security(
        ("authorization" = ["Bearer Token Required"])
    )
)]
pub(crate) async fn get_game(
    Extension(service_claims): Extension<ServiceClaims>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    info!("get_game()");
    let db = DB.read().await;
    match db.get(&id) {
        Some(r) => {
            if service_claims.service_id != r.service_id {
                return (StatusCode::NOT_FOUND).into_response();
            }
            (StatusCode::OK, Json(r)).into_response()
        }
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}
