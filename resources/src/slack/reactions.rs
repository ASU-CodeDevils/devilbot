use slack_morphism::api::{SlackApiReactionsAddRequest, SlackApiReactionsAddResponse};
use slack_morphism::hyper_tokio::SlackClientHyperConnector;
use slack_morphism::{SlackApiToken, SlackClient};

use crate::slack::client::build_bot_token;

/**
 * Adds the specified reaction to a message identified by its timestamp.
 * See https://api.slack.com/methods/reactions.add for request JSON examples.
 */
pub async fn add(
    channel: &str,
    timestamp: &str,
    reaction: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let slack_token: SlackApiToken = build_bot_token().await;
    let session = client.open_session(&slack_token);

    let add_reaction_request =
        SlackApiReactionsAddRequest::new(channel.into(), reaction.into(), timestamp.into());

    let _add_reaction_response: SlackApiReactionsAddResponse =
        session.reactions_add(&add_reaction_request).await?;

    Ok(())
}
