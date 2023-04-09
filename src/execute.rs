use cosmwasm_std::{Addr, DepsMut, Response, StdResult};

use msg::{InstantiateMsg};
use state::{Config, CONFIG, ExecResult, Project, PROJECT};

use crate::{
    msg, state,
};

pub fn init(deps: DepsMut, msg: InstantiateMsg) -> StdResult<Response> {
    let config = Config {
        admin: msg.admin,
        cnt: msg.count,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

pub fn create_project(deps: DepsMut, owner: Addr, github_addr: String, des: String) -> StdResult<Response> {
    let mut config = CONFIG.load(deps.storage)?;

    let project = Project {
        id: config.cnt + 1,
        owner,
        github_addr,
        description: des,
        result: vec![],
    };
    PROJECT.save(deps.storage, project.id, &project)?;

    config.cnt = config.cnt + 1;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("id", project.id.to_string()))
}

pub fn save_exec_result(deps: DepsMut, user: Addr, id: i32, result: String) -> StdResult<Response> {
    let mut project = PROJECT.load(deps.storage, id)?;

    let result = ExecResult {
        user,
        result,
    };
    project.result.push(result);

    PROJECT.save(deps.storage, id, &project)?;
    Ok(Response::default())
}