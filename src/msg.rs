use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use serde::{Deserialize, Serialize};

use crate::state::{ExecResult, ResultRequest};

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

    ResultRequestMsg {
        user: Addr,
        id: i32,
        input: String,
    },

    SaveResultMsg {
        project_id: i32,
        user: Addr,
        request: String,
        req_id: i32,
        result: String,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    Config {},
    RequestIDInfo {
        id: i32
    },
    ProjectInfo {
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
    pub request: Vec<ResultRequest>,
    pub result: Vec<ExecResult>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ResultRequestMsg {
    pub id: i32,
    pub user_addr: Addr,
    pub input: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SaveResultMsg {
    pub id: i32,
    pub result: String,
    pub user_addr: Addr,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RequestIDResponse {
    pub project_id: i32,
    pub req_id: i32,
}