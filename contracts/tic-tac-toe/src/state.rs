use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Addr },
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Game{
    pub owner: Addr,
    pub turn: u8,
    pub board: [[u8; 3]; 3], 
    pub player2: Addr,
    pub state: GameState,
}

pub const STATE: Item<State> = Item::new("state");
pub const GAMES: Map<&Addr,Game> = Map::new("game");
