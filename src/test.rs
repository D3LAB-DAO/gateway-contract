use cosmwasm_std::{Addr, attr};
use cosmwasm_std::from_binary;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

use QueryMsg::ProjectInfo;

use crate::contract::{instantiate, query};
use crate::execute::{create_project, save_exec_result};
use crate::msg::{Config, InstantiateMsg, ProjectResponse, QueryMsg, RequestIDResponse};
use crate::state::{ExecResult, RequestID};

#[test]
fn test_query_config() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("test", &[]);
    let msg = InstantiateMsg {
        admin: Addr::unchecked("test"),
        count: 0,
    };
    instantiate(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        msg,
    )
        .unwrap();

    let result = query(deps.as_ref(), env.clone(), QueryMsg::Config {}).unwrap();
    let res: Config = from_binary(&result).unwrap();
    assert_eq!(res.owner, "test");
    assert_eq!(res.count, 0);

    let req_result = query(deps.as_ref(), env.clone(), QueryMsg::RequestIDInfo { id: 1 }).unwrap();
    let req_info: RequestID = from_binary(&req_result).unwrap();
    assert_eq!(req_info.request_id[0], 0)
}

#[test]
fn test_query_request_id() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("test", &[]);
    let msg = InstantiateMsg {
        admin: Addr::unchecked("test"),
        count: 0,
    };

    instantiate(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        msg,
    )
        .unwrap();

    let res = create_project(deps.as_mut(), env.clone(), info.clone(), Addr::unchecked("test"), "github.com/test".to_string(), "test project".to_string()).unwrap();
    assert_eq!(res.attributes, vec![
        attr("id", 1.to_string())
    ]);

    let result = query(deps.as_ref(), env.clone(), QueryMsg::RequestIDInfo { id: 1 }).unwrap();
    let query_res: RequestIDResponse = from_binary(&result).unwrap();
    assert_eq!(query_res.req_id, 0);
    assert_eq!(query_res.project_id, 1);
}

#[test]
fn test_execute_create_project() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("test", &[]);
    let msg = InstantiateMsg {
        admin: Addr::unchecked("test"),
        count: 0,
    };

    instantiate(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        msg,
    )
        .unwrap();

    let res = create_project(deps.as_mut(), env.clone(), info.clone(), Addr::unchecked("test"), "github.com/test".to_string(), "test project".to_string()).unwrap();
    assert_eq!(res.attributes, vec![
        attr("id", 1.to_string())
    ]);

    let result = query(deps.as_ref(), env.clone(), ProjectInfo { id: 1 }).unwrap();
    let query_res: ProjectResponse = from_binary(&result).unwrap();
    assert_eq!(query_res.id, 1);
    assert_eq!(query_res.owner, Addr::unchecked("test"));
    assert_eq!(query_res.github_addr, "github.com/test".to_string());
    assert_eq!(query_res.description, "test project".to_string());
    assert_eq!(query_res.request, []);
    assert_eq!(query_res.result, []);
}

#[test]
fn test_exec_save_result() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("owner", &[]);
    let msg = InstantiateMsg {
        admin: Addr::unchecked("owner"),
        count: 0,
    };

    instantiate(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        msg,
    )
        .unwrap();
    create_project(deps.as_mut(), env.clone(), info.clone(), Addr::unchecked("owner"), "github.com/test".to_string(), "test project".to_string()).unwrap();
    save_exec_result(deps.as_mut(), env.clone(), info.clone(), Addr::unchecked("test"), 1, 1, "request".to_string(), "result".to_string()).unwrap();

    let result = query(deps.as_ref(), env.clone(), ProjectInfo { id: 1 }).unwrap();
    let query_res: ProjectResponse = from_binary(&result).unwrap();
    assert_eq!(query_res.id, 1);
    assert_eq!(query_res.owner, Addr::unchecked("owner"));
    assert_eq!(query_res.github_addr, "github.com/test".to_string());
    assert_eq!(query_res.description, "test project".to_string());
    assert_eq!(query_res.result, [ExecResult { req_id: 1, user: Addr::unchecked("test"), request: "request".to_string(), result: "result".to_string() }]);
}
