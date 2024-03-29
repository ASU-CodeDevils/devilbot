use slack_morphism::api::{
    SlackApiConversationsOpenRequest, SlackApiConversationsOpenResponse,
    SlackApiConversationsRepliesRequest, SlackApiConversationsRepliesResponse,
};
use slack_morphism::hyper_tokio::SlackClientHyperConnector;
use slack_morphism::{
    SlackApiToken, SlackBasicChannelInfo, SlackChannelId, SlackClient, SlackHistoryMessage, SlackTs,
};

use crate::slack::client::{build_bot_token, build_user_token};

/**
 * Opens a conversation.
 * See https://api.slack.com/methods/conversations.open for request JSON examples.
 */
pub async fn open(
    users: Vec<&str>,
    is_bot_token: bool,
) -> Result<
    SlackApiConversationsOpenResponse<SlackBasicChannelInfo>,
    Box<dyn std::error::Error + Send + Sync>,
> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let slack_token = match is_bot_token {
        true => build_bot_token().await,
        false => build_user_token().await,
    };
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

pub async fn get_replies(
    channel: &str,
    timestamp: &str,
) -> Result<Vec<SlackHistoryMessage>, Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let slack_token: SlackApiToken = build_bot_token().await;
    let session = client.open_session(&slack_token);
    let slack_channel: SlackChannelId = channel.into();
    let slack_timestamp: SlackTs = timestamp.into();

    let conversations_replies_request =
        SlackApiConversationsRepliesRequest::new(slack_channel, slack_timestamp);

    let conversations_replies_response: SlackApiConversationsRepliesResponse = session
        .conversations_replies(&conversations_replies_request)
        .await?;
    Ok(conversations_replies_response.messages)
}
