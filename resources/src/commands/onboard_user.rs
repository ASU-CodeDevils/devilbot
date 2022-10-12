use crate::commands::{chat_post_message, conversations_open};
use serde_json::json;

// TODO: write docs
pub async fn run(username: &str, first_name: &str) {
    let text: String = format!(":codedevils_flash: Hey welcome to CodeDevils {} :codedevils_flash: I am DevilBot and I don't know much yet, but here's what I do know: \
        This Slack workspace serves as the main communication platform for all things CodeDevils :partywizard: All our announcements can be found in the <#C30L07P18> channel. \
        This includes all meetings and meeting recordings! I'd like you to go to the <#CMGU8033K> channel and introduce yourself. After that, come on over to\
        <#C2N5P84BD>. Most of my creators are there all day.", &first_name);

    // After acceptance testing change this to the username
    let conversation_with_user_and_devilbot_payload = json!({ "users": "WGJRL9NJD" });
    let conversations_open_response =
        conversations_open::open_conversation(&conversation_with_user_and_devilbot_payload).await;
    log::info!(
        "Conversations Open Response {:?}",
        conversations_open_response
    );

    let channel_id: &str = conversations_open_response["channel"]["id"]
        .as_str()
        .unwrap_or("invalid_id");
    log::info!("Channel Id {:?}", channel_id);
    let onboard_message_request_json = json!({
        "channel": channel_id,
        "text": &text,
    });
    chat_post_message::post_message(&onboard_message_request_json).await
}
