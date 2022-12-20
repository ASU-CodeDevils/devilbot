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
    let is_development = get_env_var("IS_DEVELOPMENT");
    // Stop the function if this is a development environment and outside the test channel
    if is_development == "true" {
        log::info!("This is a development environment {}", is_development);
        return;
    }

    // Call onboard user
    // Add anything else here that should happen when a user joins the workspace
    commands::onboard_user::run(username, first_name)
        .await
        .unwrap();
}
