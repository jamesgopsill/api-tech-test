use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{game::Game, statics::EUROPEAN_BETS};

/// Represents a bet on a game handled by the
/// service.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Bet {
    /// An player identifier issued from the service
    /// that is making a GameRequest.
    pub player_id: Box<str>,
    /// The bet the have placed as a csv
    /// of numbers two character 0 padded.
    /// E.g., `00,01,02`.
    pub bet: Box<str>,
    /// The number of chips the player has placed.
    pub chips_in: u64,
    /// The number of chips returned following a successful
    /// game being played. This should be omitted on a
    /// GameRequest submission.
    pub chips_out: Option<u64>,
}

impl Bet {
    /// Checks if the bet is valid for the game and the size
    /// of the bet is small enough that a result can be computed.
    pub fn is_valid(&self, game: &Game) -> Result<(), &'static str> {
        match game {
            Game::EuropeanRoulette => {
                if !EUROPEAN_BETS.contains_key(&self.bet) {
                    return Err("Invalid bet string");
                }
                let odds = EUROPEAN_BETS.get(&self.bet).unwrap();
                if self.chips_in.checked_mul(*odds).is_none() {
                    return Err("Too many chips");
                }
            }
        }
        Ok(())
    }

    /// Checks if the bet was a winning bet and computes the winnings.
    pub fn did_win(&mut self, game: &Game, winning_number: &str) {
        if !self.bet.contains(winning_number) {
            self.chips_out = Some(0);
            return;
        }
        match game {
            Game::EuropeanRoulette => {
                let odds = EUROPEAN_BETS.get(&self.bet).unwrap();
                self.chips_out = self.chips_in.checked_mul(*odds);
            }
        }
    }
}
