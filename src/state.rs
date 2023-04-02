use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Project {
    pub id: i32,
    pub owner: Addr,
    pub github_addr: String,
    pub description: String,
}

pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
pub const PROJECT: Item<Project> = Item::new("project");
