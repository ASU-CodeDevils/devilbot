use crate::increment_buns;
use crate::slack::add_reaction;

// This function parses the text event from the slack event subscription
// if the text contains any form of "buns", it will respond with a buns reaction.
// This can be used as an example command when creating new commands for
// the Slack bot.
pub async fn run(
    channel: &str,
    enterprise_user_id: &str,
    timestamp: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    increment_buns(enterprise_user_id).await;

    add_reaction::add_reaction(channel, timestamp, "buns")
        .await
        .unwrap_or_else(|err| log::info!("Reaction add error: {}", err));
    Ok(())
}
