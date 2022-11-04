use crate::slack::client::build_token;
use slack_morphism::prelude::*;

// This function will add the specified reaction to a message identifed by its timestamp.
pub async fn run(
    channel: &str,
    timestamp: &str,
    reaction: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let slack_token: SlackApiToken = build_token().await;
    let session = client.open_session(&slack_token);

    let add_reaction_request =
        SlackApiReactionsAddRequest::new(channel.into(), reaction.into(), timestamp.into());

    let _add_reaction_response = session.reactions_add(&add_reaction_request).await?;

    Ok(())
}
