use cosmwasm_std::{Deps, StdResult};

use state::CONFIG;

use crate::state;
use crate::msg::{Config, ProjectResponse};
use crate::state::PROJECT;

pub fn config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    let resp = Config {
        owner: (config.admin),
        count: config.cnt,
    };
    Ok(resp)
}

pub fn project_info(deps: Deps, id: i32) -> StdResult<ProjectResponse> {
    let project = PROJECT.load(deps.storage, id)?;

    let resp = ProjectResponse {
        id: project.id,
        github_addr: project.github_addr,
        description: project.description,
        owner: project.owner,
        result: project.result,
    };
    Ok(resp)
}