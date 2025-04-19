use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::game::PlayedGame;

/// A stub for a JWT secret that could come from an env var
/// stored on the service that the app is being run from.
pub static JWT_SECRET: &str = "some_secret";

/// Simplyifying the type definition of the DatabaseStub
pub type DatabaseStub = LazyLock<Arc<RwLock<HashMap<Uuid, PlayedGame>>>>;

/// Instantiates a DatabaseStub for the example.
pub(crate) static DB: DatabaseStub = LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));
