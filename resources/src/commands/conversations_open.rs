use slack_morphism::api::{SlackApiConversationsOpenRequest, SlackApiConversationsOpenResponse};
use slack_morphism::hyper_tokio::SlackClientHyperConnector;
use slack_morphism::{SlackApiToken, SlackBasicChannelInfo, SlackClient};

use crate::slack::client::build_token;

// Opens a conversation. See https://api.slack.com/methods/conversations.open for request json examples
pub async fn open_conversation(
    users: Vec<&str>,
) -> Result<
    SlackApiConversationsOpenResponse<SlackBasicChannelInfo>,
    Box<dyn std::error::Error + Send + Sync>,
> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let slack_token: SlackApiToken = build_token().await;
    let session = client.open_session(&slack_token);

    let conversation_open_request: SlackApiConversationsOpenRequest =
        SlackApiConversationsOpenRequest::new().with_users(
            users
                .into_iter()
                .map(|user_string| user_string.into())
                .collect(),
        );

    let open_conversation_response = session
        .conversations_open(&conversation_open_request)
        .await?;
    Ok(open_conversation_response)
}
