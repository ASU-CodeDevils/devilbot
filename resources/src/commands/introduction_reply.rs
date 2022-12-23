use crate::slack::add_reaction;
use crate::slack::chat_post_message;

pub async fn reply_to_introduction(
    base_message_timestamp: &str,
    intros_channel_id: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // add the reactions to the base message
    let emoji_names = vec![
        "codedevils_rainbow_fast",
        "partywizard",
        "pogfish-pogging",
        "meow_code",
        "fork",
        "forks",
    ];
    for emoji_name in emoji_names {
        add_reaction::add_reaction(
            intros_channel_id.as_str(),
            base_message_timestamp,
            emoji_name,
        )
        .await
        .unwrap_or_else(|err| log::info!("Add reaction error: {}", err));
    }
    let text: String = format!(
        "Hey welcome to CodeDevils I am DevilBot! :partywizard: \
    If you are looking to learn more about programming, I am an official CodeDevils project \
    that can be worked on!"
    );
    chat_post_message::post_message(&text, &intros_channel_id, Some(base_message_timestamp))
        .await
        .unwrap_or_else(|err| log::info!("Chat post error: {}", err));
    Ok(())
}
