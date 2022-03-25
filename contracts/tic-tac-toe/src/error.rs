use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized,
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Tile allready set!")]
    AllreadySet,
    #[error("It is not your turn yet!")]
    WrongPlayer,
    #[error("Wrong address!")]
    WrongGame ,
    #[error("Wrong tile!")]
    WrongTile,
    #[error("Game over!")]
    GameOver,
}


