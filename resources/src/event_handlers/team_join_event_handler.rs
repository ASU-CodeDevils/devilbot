use crate::commands;
use serde_json::Value;
pub async fn handle_team_join_event(body: &Value) {
    // Deconstruct the body
    let username: &str = body["event"]["user"]["id"]
        .as_str()
        .unwrap_or("invalid_user_name");
    let first_name: &str = body["event"]["user"]["profile"]["first_name"]
        .as_str()
        .unwrap_or("");

    // Call onboard user
    // Add anything else here that should happen when a user joins the workspace
    commands::onboard_user::run(username, first_name).await
}
