use slack_morphism::hyper_tokio::SlackClientHyperConnector;
use slack_morphism::prelude::{
    SlackApiChatPostEphemeralRequest, SlackApiChatPostEphemeralResponse,
};
use slack_morphism::{
    SlackApiToken, SlackChannelId, SlackClient, SlackMessageContent, SlackUserId,
};

use crate::slack::client::build_token;

pub async fn _post_ephemeral_message(
    channel: &str,
    user: &str,
    text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let slack_token: SlackApiToken = build_token().await;
    let session = client.open_session(&slack_token);

    // Create params
    let message_content = SlackMessageContent::new().with_text(text.into());
    let slack_channel_id = SlackChannelId(channel.to_string());
    let slack_user_id = SlackUserId(user.to_string());

    log::info!(
        "Chat post ephemeral message params: message_content {:?},\
         slack_channel_id {:?} slack_user_id {:?}",
        message_content,
        slack_channel_id,
        slack_user_id
    );

    let chat_post_ephemeral_message_request: SlackApiChatPostEphemeralRequest =
        SlackApiChatPostEphemeralRequest::new(slack_channel_id, slack_user_id, message_content);

    let _chat_post_ephemeral_message_response: SlackApiChatPostEphemeralResponse = session
        .chat_post_ephemeral(&chat_post_ephemeral_message_request)
        .await?;

    Ok(())
}
