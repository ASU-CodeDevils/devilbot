use slack_morphism::SlackTextFormat;

use crate::commands::conversations_open;
use crate::slack::chat_post_message;

// TODO: write docs
pub async fn run(
    username: &str,
    first_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let text: String = format!(":codedevils_flash: Hey welcome to CodeDevils {} :codedevils_flash: I am DevilBot and I don't know much yet, but here's what I do know: \
        This Slack workspace serves as the main communication platform for all things CodeDevils :partywizard: All our announcements can be found in the <#C30L07P18> channel. \
        This includes all meetings and meeting recordings! I'd like you to go to the <#CMGU8033K> channel and introduce yourself. After that, come on over to\
        <#C2N5P84BD>. Most of my creators are there all day.", &first_name);

    let users_vec = vec![username];
    let conversations_open_response = conversations_open::open_conversation(users_vec).await?;
    let channel_id = conversations_open_response.channel.id.to_slack_format();
    log::info!("Channel Id {:?}", channel_id);

    chat_post_message::post_message(&text, &channel_id)
        .await
        .unwrap_or_else(|err| log::info!("Chat post error: {}", err));
    Ok(())
}
