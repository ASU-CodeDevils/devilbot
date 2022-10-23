use crate::commands::chat_post_message;
use serde_json::json;

// This function parses the text event from the slack event subscription
// if the text contains any form of "ping", it will respond with "pong".
// This can be used as an example command when creating new commands for
// the Slack bot.
pub async fn run(channel: &str) {
    let text = "pong";
    let chat_request_json = json!({
        "channel": channel,
        "text": text,
    });
    let _status = chat_post_message::post_message(&chat_request_json).await;
}
