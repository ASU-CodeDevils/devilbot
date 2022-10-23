use crate::commands::emoji_reaction;
use crate::increment_buns;
use serde_json::json;

// This function parses the text event from the slack event subscription
// if the text contains any form of "buns", it will respond with the buns reaction.
pub async fn run(channel: &str, enterprise_user_id: &str, timestamp: &str) {
    increment_buns(enterprise_user_id).await;
    let buns_reaction_payload = json!({"channel": channel, "timestamp": timestamp, "name": "buns"});
    let emoji_reaction_response = emoji_reaction::reactions_add(&buns_reaction_payload).await;
    log::info!("Emoji Reaction Response {:?}", emoji_reaction_response);
}
