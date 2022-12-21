use serde_json::Value;

use crate::commands;
use crate::get_env_var;

pub async fn handle_message_event(body: &Value) {
    // Destructure everything needed
    let is_bot: bool = body["event"]["subtype"] == "bot_message";
    if is_bot {
        log::info!("This is a bot");
        return;
    }
    let channel: &str = body["event"]["channel"]
        .as_str()
        .unwrap_or("invalid_channel");
    let text: String = body["event"]["text"]
        .as_str()
        .unwrap_or("invalid_text")
        .to_lowercase();
    let timestamp: &str = body["event"]["event_ts"]
        .as_str()
        .unwrap_or("invalid_timestamp");
    let enterprise_user_id: &str = body["enterprise_id"]
        .as_str()
        .unwrap_or("invalid_enterprise_user_id");
    let _user: &str = body["event"]["user"].as_str().unwrap_or("invalid_user");
    let is_development = get_env_var("IS_DEVELOPMENT").parse().unwrap();
    let test_channel_id = get_env_var("TEST_CHANNEL_ID");
    // Stop the function if this is a development environment and outside the test channel
    if channel != test_channel_id && is_development {
        log::info!("This is a development environment {}", is_development);
        return;
    }

    // Match appropriate function
    match text.as_str() {
        // Add new commands below and create new async functions for them.
        "ping" => commands::ping::run(channel).await,
        _ => log::info!("Invalid command: {:?}", ..),
    }
    if text.contains("buns") {
        commands::buns::run(channel, enterprise_user_id, timestamp)
            .await
            .unwrap_or_else(|err| log::info!("Error running buns command: {}", err));
    }
}
