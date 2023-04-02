use cosmwasm_std::{
    DepsMut, entry_point, Env, MessageInfo, Response, StdResult,
};

use cw2::set_contract_version;

use crate::msg::InstantiateMsg;

const CONTRACT_NAME: &str = "gateway";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    crate::execute::init(deps, msg)
}