use crate::slack;

// This function parses the text event from the slack event subscription
// if the text contains any form of "ping", it will respond with "pong".
// This can be used as an example command when creating new commands for
// the Slack bot.
pub async fn run(channel: &str) {
    let text = "pong";
    let _status_with_bot_token = slack::chat::post_message(&text, &channel, true).await;
    let _status_with_user_token = slack::chat::post_message(text, channel, false).await;
}
