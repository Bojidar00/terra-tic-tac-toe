#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{GameResponse, ExecuteMsg, InstantiateMsg, QueryMsg, Tile};
use crate::state::{State, STATE, Game,GAMES,GameState};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
       )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::StartGame {player2,tile} => start_game(deps, info, player2, tile),
        ExecuteMsg::Play{owner,tile} =>play(deps, info, owner, tile),
    }
}

pub fn start_game(deps:DepsMut,info:MessageInfo,player2:String,tile:Tile)->Result<Response,ContractError>{
    let player_adr = deps.api.addr_validate(&player2)?;
    let mut board:[[u8; 3]; 3]=[[0;3];3];
    board[tile.col as usize][tile.row as usize]=1;
    let sender = info.sender.clone();
        let game_ = Game{
            owner:sender,
            player2:player_adr,
            turn:1,
            board:board,
            state:GameState::Active,
        };
    let start = |game:Option<Game>| ->StdResult<Game>{
       
        match game {
            Some(game) =>{
                if game.state!=GameState::Active{
                    Ok(game_)
                }else{
                    Ok(game)
                }

            },
            None =>{Ok(game_)},
        }
       
    };

    
    GAMES.update(deps.storage,&info.sender, start)?;


    Ok(Response::new().add_attribute("method", "start game"))
}

pub fn play(deps:DepsMut,info:MessageInfo,owner:String,tile:Tile)->Result<Response,ContractError>{

    let owner_adr = deps.api.addr_validate(&owner)?;
    let  game = GAMES.load(deps.storage,&owner_adr)?;

    if game.state!=GameState::Active{
        return Err(ContractError::GameOver)
    }
    

    match &tile {
        tile @ Tile {
            row: 0..=2,
            col: 0..=2,
        } => {if game.board[tile.col as usize][tile.row as usize]!=0{
            return Err (ContractError::AllreadySet)
        }},
        _ => return Err(ContractError::WrongTile ),
    }
    if info.sender!=game.owner && info.sender!=game.player2{
        return Err(ContractError::WrongGame)
    }
    if info.sender==game.owner && game.turn==1{
        return Err(ContractError::WrongPlayer)
    }
    if info.sender==game.player2 && game.turn==0{
        return Err(ContractError::WrongPlayer)
    }

    let play_ = |game:Option<Game>| ->StdResult<Game>{
        let mut g =game.unwrap();
        if info.sender==g.owner{
        g.board[tile.col as usize][tile.row as usize]=1; g.turn =1;
    }else{g.board[tile.col as usize][tile.row as usize]=2; g.turn =0;}
      Ok(g)
    };
    let win = |game:Option<Game>| ->StdResult<Game>{
        let winner = info.sender.clone();
        let mut g =game.unwrap();
        g.state=GameState::Won{winner:winner};
       Ok(g)
    };
    let tie = |game:Option<Game>| ->StdResult<Game>{
        let mut g =game.unwrap();
        g.state=GameState::Tie;
       Ok(g)
    };

    GAMES.update(deps.storage, &owner_adr, play_)?;
   if check_game(game.board)==1{
    GAMES.update(deps.storage, &owner_adr, win)?;
   }
   else if check_game(game.board)==1 {
    GAMES.update(deps.storage, &owner_adr, tie)?;
   }

    Ok(Response::new().add_attribute("method", "play"))
}
fn is_winning_trio(board: [[u8; 3]; 3],trio: [(usize, usize); 3]) -> bool {
    let [first, second, third] = trio;
      board[first.0][first.1]!=0
            && board[first.0][first.1] == board[second.0][second.1]
            && board[first.0][first.1] == board[third.0][third.1]
}
fn check_game(board: [[u8; 3]; 3]) ->u8{
    for i in 0..=2 {
        // three of the same in one row
        if is_winning_trio(board,[(i, 0), (i, 1), (i, 2)]) {
          return 1;
        }
        // three of the same in one column
        if is_winning_trio(board,[(0, i), (1, i), (2, i)]) {
            return 1;
        }
    }

    // three of the same in one diagonal
    if is_winning_trio(board,[(0, 0), (1, 1), (2, 2)])
        ||is_winning_trio(board,[(0, 2), (1, 1), (2, 0)])
    {
        return 1;
    }

    // reaching this code means the game has not been won,
        // so if there are unfilled tiles left, it's still active
        for row in 0..=2 {
            for column in 0..=2 {
                if board[row][column]==0 {
                    return 0;
                }
            }
        }

        // game has not been won
        // game has no more free tiles
        // -> game ends in a tie
       return 2
    
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetGame {owner}=>to_binary(&query_game(deps, owner)?),
    }
}
fn query_game(deps:Deps,owner:String)->StdResult<GameResponse>{
    let owner_adr = deps.api.addr_validate(&owner)?;
    let game = GAMES.load(deps.storage,&owner_adr)?;
    Ok(GameResponse { board: game.board,turn:game.turn })
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { count: 0 };
        let info = mock_info("creator", &coins(1000, "earth"));
        let info2 = mock_info("anyone", &coins(2, "token"));
        let info_=info.clone();
        let info__=info.clone();
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        let t = Tile{
            row:1,
            col:1,
        };
         let msg = ExecuteMsg::StartGame{player2:info2.sender.to_string(),tile:t};
         let _res = execute(deps.as_mut(), mock_env(), info_, msg).unwrap();
         let res2 = query(deps.as_ref(), mock_env(), QueryMsg::GetGame {owner:info__.sender.to_string()}).unwrap();
        let value: GameResponse = from_binary(&res2).unwrap();
        assert_eq!(1, value.turn); 
    }

   
}
