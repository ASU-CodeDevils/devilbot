use serde_json::Value;

use crate::commands;
use crate::get_env_var;

pub async fn handle_team_join_event(body: &Value) {
    // Deconstruct the body
    let username: &str = body["event"]["user"]["id"]
        .as_str()
        .unwrap_or("invalid_user_name");
    let mut first_name: &str = body["event"]["user"]["profile"]["first_name"]
        .as_str()
        .unwrap_or("invalid_first_name");

    if first_name == "invalid_first_name" {
        log::info!("Invalid First Name");
        first_name = "";
    }
    let is_development: bool = get_env_var("IS_DEVELOPMENT").parse().unwrap();
    // Stop the function if this is a development environment and outside the test channel
   