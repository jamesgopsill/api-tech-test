use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::bet::Bet;

/// An enum of games the application supports.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum Game {
    EuropeanRoulette,
}

/// The data required to submit a game from an
/// authorised service.
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct GameRequest {
    pub game: Game,
    pub bets: Vec<Bet>,
}

/// A game that has been completed by the application,
/// which is stored in a database of historical games
/// run by the application and returned to the service that
/// requested the game to be run.
#[derive(Debug, Serialize, Deserialize, Validate, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlayedGame {
    pub uuid: Uuid,
    pub game: Game,
    pub bets: Vec<Bet>,
    pub service_id: Box<str>,
    pub occurred: u64,
    pub result: Box<str>,
}

impl GameRequest {
    /// Checks if the game request is valid by checking
    /// all the bets against the acceptable bets for the game.
    pub fn is_valid(&self) -> Result<(), &'static str> {
        for b in self.bets.iter() {
            b.is_valid(&self.game)?
        }
        Ok(())
    }

    /// Checks and plays the requested game. Cannot play a game
    /// without checking its validity.
    pub fn check_and_play(&mut self, service_id: Box<str>) -> Result<PlayedGame, &'static str> {
        self.is_valid()?;

        // Play the game
        let uuid = Uuid::new_v4();
        let mut rng = rand::rng();
        let result: u8 = match self.game {
            Game::EuropeanRoulette => rng.random_range(0..=36),
        };
        let result = format!("{:0>2}", result).into_boxed_str();
        let occurred = Utc::now().timestamp() as u64;

        for b in self.bets.iter_mut() {
            b.did_win(&self.game, &result);
        }

        Ok(PlayedGame {
            uuid,
            game: self.game.clone(),
            bets: self.bets.clone(),
            service_id,
            occurred,
            result,
        })
    }
}
