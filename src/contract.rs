#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:bloated_contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const FILLER_SIZE: usize = 2_000_000; // Approx 2 MB of data
static FILLER: [u8; FILLER_SIZE] = [42; FILLER_SIZE]; // Array filled with 42s

const DUMMY_DATA: &[u8] = include_bytes!("../dummy.bin"); // Adjust path if needed, e.g., "../dummy.bin"


const TABLE_SIZE: usize = 2_000_000; // 2 MB of bytes
// static LOOKUP_TABLE: [u8; TABLE_SIZE] = {
//     let mut arr = [0; TABLE_SIZE];
//     let mut i = 0;
//     while i < TABLE_SIZE {
//         arr[i] = (i % 256) as u8; // Fill with a pattern
//         i += 1;
//     }
//     arr
// };

const BIG_BLOB: &str = include_str!("../filler.txt"); // Create a filler.txt with 2 MB of text


macro_rules! generate_functions {
    ($($name:ident),*) => {
        $(
            fn $name(x: u64) -> u64 {
                x.wrapping_add(42) // Simple computation
            }
        )*
    };
}

// Generate 5000 dummy functions (adjust number for size)
generate_functions!(
func_0001,
func_0002,
func_0003,
func_0004,
func_0005,
func_0006,
func_0007,
func_0008,
func_0009,
func_0010,
func_0011,
func_0012,
func_0013,
func_0014,
func_0015,
func_0016,
func_0017,
func_0018,
func_0019,
func_0020,
func_0021,
func_0022,
func_0023,
func_0024,
func_0025,
func_0026,
func_0027,
func_0028,
func_0029,
func_0030,
func_0031,
func_0032,
func_0033,
func_0034,
func_0035,
func_0036,
func_0037,
func_0038,
func_0039,
func_0040,
func_0041,
func_0042,
func_0043,
func_0044,
func_0045,
func_0046,
func_0047,
func_0048,
func_0049,
func_0050
);

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    //do some regex stuff
    let re = regex::Regex::new(r"^\d{5}-\d{4}$").unwrap();
    let _ = re.is_match("12345-6789");

    let result = func_0001(100);

    let _ = FILLER[0];

    let x = BIG_BLOB.len();

    generate_functions!();

    let block_height = env.block.height as usize;
    //let value = LOOKUP_TABLE[block_height % TABLE_SIZE]; // Dynamic access


    let y = BIG_BLOB.as_bytes();

    let _ = y.binary_search(&42);

    let mut checksum: u64 = 0;
    for &byte in DUMMY_DATA {
        checksum = checksum.wrapping_add(byte as u64);
    }

    // Include the block height to make it dynamic
    let height = env.block.height;
    let final_value = checksum.wrapping_add(height);

    let mut _x = 0;
    for _i in 0..1_000_000 {
        _x += 1; // Some trivial computation
    }

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => execute::increment(deps),
        ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
        ExecuteMsg::Reseta { count }=> execute::reset(deps, info, count),
        ExecuteMsg::Resetw { count}=> execute::reset(deps, info, count),
        ExecuteMsg::Reseatw { count }=> execute::reset(deps, info, count),
        ExecuteMsg::Reseagatw { count }=> execute::reset(deps, info, count),
        ExecuteMsg::Reseasagatw { count }=> execute::reset(deps, info, count),
        ExecuteMsg::Reseaadfsagatw { count }=> execute::reset(deps, info, count),

    }
}

pub mod execute {
    use super::*;

    pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count += 1;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }

    pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.count = count;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_json_binary(&query::count(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
