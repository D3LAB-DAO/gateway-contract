use cosmwasm_std::{Addr};
use serde::{Deserialize, Serialize};

use crate::state::ExecResult;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub count: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ExecuteMsg {
    CreateProjectMsg {
        owner: Addr,
        github_addr: String,
        description: String,
    },

    SaveResultMsg {
        id: i32,
        user: Addr,
        result: String,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    Config {},
    ProjectQueryMsg {
        id: i32
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Config {
    pub owner: Addr,
    pub count: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectResponse {
    pub id: i32,
    pub github_addr: String,
    pub description: String,
    pub owner: Addr,
    pub result: Vec<ExecResult>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SaveResultMsg {
    pub id: i32,
    pub result: String,
    pub user_addr: Addr,
}