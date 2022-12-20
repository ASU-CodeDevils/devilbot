use serde_json::Value;

use crate::get_env_var;
use crate::slack::add_reaction;

pub async fn handle_reaction_added_event(body: &Value) {
    // deconstruct the information needed from the event
    let reaction = body["event"]["reaction"]
        .as_str()
        .unwrap_or("invalid_reaction");
    let channel = body["event"]["item"]["channel"]
        .as_str()
        .unwrap_or("invalid_channel");
    let timestamp = body["event"]["item"]["ts"]
        .as_str()
        .unwrap_or("invalid_timestamp");

    log::info!(
        "Information parse from handle_reaction_added_event reaction: \
    {}, channel {}, timestamp {}",
        reaction,
        channel,
        timestamp
    );
    let is_development = get_env_var("IS_DEVELOPMENT");
    // Stop the function if this is a development environment and outside the test channel
    if channel != "C0351GJ62Q0" && is_development == "true" {
        log::info!("This is a development environment {}", is_development);
        return;
    }
    // Add any other emojis to copy against here
    // Please keep alphabetized
    let emojis_to_copy = vec![
        "beer",
        "buns",
        "catjam",
        "codedevils_flash",
        "codedevils_rainbow_fast",
        "distressed_salamander",
        "fax",
        "ferris_happy",
        "fishjam",
        "heart",
        "kek",
        "kekhands",
        "kekleave",
        "keks",
        "kekthar",
        "kekw",
        "kekwoo",
        "kekzos",
        "party-ferris",
        "party-pogfish-pogging",
        "partywizard",
        "party_blob",
        "pepehands",
        "pepehands-6102",
        "pepelol",
        "peperun",
        "pesfuckyou",
        "pog",
        "pogfish-fast",
        "pogfish-pogging",
        "rust_logo",
        "sadkek",
        "temple-of-rust",
    ];
    if emojis_to_copy.contains(&reaction) {
        add_reaction::run(channel, timestamp, reaction)
            .await
            .unwrap_or_else(|err| log::info!("Error adding reaction: {}", err))
    }
}
