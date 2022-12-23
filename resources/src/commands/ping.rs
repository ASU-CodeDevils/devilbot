use crate::slack::chat_post_message;

// This function parses the text event from the slack event subscription
// if the text contains any form of "ping", it will respond with "pong".
// This can be used as an example command when creating new commands for
// the Slack bot.
pub async fn run(channel: &str) {
    let text = "pong";
    let _status = chat_post_message::post_message(text, channel, Option::None).await;
}
