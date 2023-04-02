use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};

use msg::InstantiateMsg;
use state::ADMINS;

use crate::{
    msg, state,
};

pub fn init(deps: DepsMut, msg: InstantiateMsg) -> StdResult<Response> {
    let admins: StdResult<Vec<_>> = msg
        .admins
        .into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();
    ADMINS.save(deps.storage, &admins?)?;

    Ok(Response::new())
}