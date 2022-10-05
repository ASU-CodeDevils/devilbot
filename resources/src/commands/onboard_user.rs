use crate::commands::{chat_post_message, conversations_open};
use serde_json::{json, Value};

// TODO: write docs
pub async fn run(body: &Value) {
    let username: &str = body["event"]["user"]["name"]
        .as_str()
        .unwrap_or("invalid_user_name");
    let first_name: &str = body["event"]["user"]["profile"]["first_name"]
        .as_str()
        .unwrap_or("");
    let text: String = format!(":codedevils_flash: Hey welcome to CodeDevils {} :codedevils_flash:. I am DevilBot and I don't know much yet, but here's what I do know: \
        This Slack workspace serves as the main communication platform for all things CodeDevils :partywizard:. All our announcements can be found in the <#C30L07P18> channel. \
        This includes all meetings and meeting recordings! I'd like you to go to the <#CMGU8033K> channel and introduce yourself. After that, come on over to\
        <#C2N5P84BD>. Most of my creators are there all day.", first_name);

    // After acceptance testing change this to the username
    let conversations_open_json = json!({ "users": "WGJRL9NJD" });
    let conversations_open_res =
        conversations_open::open_conversation(&conversations_open_json).await;

    let channel_id: &str = conversations_open_res["channel"]["id"]
        .as_str()
        .unwrap_or("invalid_id");
    log::info!("Channel Id {:?}", channel_id);
    let onboard_message_json = json!({
        "channel": channel_id,
        "text": &text,
    });
    let status = chat_post_message::post_message(&onboard_message_json).await;
}
