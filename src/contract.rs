use cosmwasm_std::{Binary, Deps, DepsMut, entry_point, Env, MessageInfo, Response, StdResult, to_binary};
use cw2::set_contract_version;

use crate::{execute, query};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

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
    execute::init(deps, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateProjectMsg { owner, github_addr, description } => execute::create_project(deps, owner, github_addr, description),
        ExecuteMsg::SaveResultMsg { user, id, result } => execute::save_exec_result(deps, user, id, result),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config { } => to_binary(&query::config(deps)?),
        QueryMsg::ProjectQueryMsg { id } => to_binary(&query::project_info(
            deps,
            id,
        )?),
    }
}