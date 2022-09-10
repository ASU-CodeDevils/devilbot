use serde_json::Value;
use slack_hook::{PayloadBuilder, Slack};

// TODO: write docs
pub async fn run(body: &Value) {
    let slack_webhook_url: String = crate::get_env_var(crate::DEVIL_BOT_TEST_CHANNEL_URL);
    let slack = Slack::new(&*slack_webhook_url).unwrap();
    // Parse the body for what I need
    let username: &str = body["event"]["user"]["name"]
        .as_str()
        .unwrap_or("invalid_user_name");
    let first_name: &str = body["event"]["user"]["profile"]["first_name"]
        .as_str()
        .unwrap_or("");
    // let channel: &str = body[event][user][channel]
    //     .as_str()
    //     .unwrap_or("invalid_channel");
    let text: String = format!(":codedevils_flash: Hey welcome to CodeDevils {} :codedevilsflash:. I am DevilBot and I don't know much yet, but here's what I do know.\
        This Slack workspace serves as the main communication platform for all things CodeDevils :partywizard:. All our announcements can be found in the #announcemnets channel.\
        This includes all meetings and meeting recordings! I'd like you to go to the #intros channel and introduce yourself. After that, come on over to\
        #hangout. Most of my creators are there all day.", first_name);
    let p = PayloadBuilder::new()
        .text(text)
        .channel("WGJRL9NJD")
        .build()
        .unwrap();

    let res: Result<(), slack_hook::Error> = slack.send(&p);
    match res {
        Ok(()) => log::info!("Slack message sent successfully."),
        Err(x) => log::info!("ERR: {}", x),
    }
}
