use cosmwasm_std::{Addr};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Config {
    pub admin: Addr,
    pub cnt: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Project {
    pub id: i32,
    pub owner: Addr,
    pub github_addr: String,
    pub description: String,
    pub result: Vec<ExecResult>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ExecResult {
    pub user: Addr,
    pub result: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PROJECT: Map<i32, Project> = Map::new("project");