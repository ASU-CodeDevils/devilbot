use crate::event_handlers::message_event_handler;
use lambda_http::{service_fn, Error, IntoResponse, Request};
use log::LevelFilter;
use serde_json::{json, Value};
use simple_logger::SimpleLogger;
use std::{env, process};

mod aws;
mod commands;
mod event_handlers;

const BUNS_TABLE_NAME: &str = "BUNS_TABLE_NAME";
const DEVIL_BOT_TEST_CHANNEL_URL: &str = "DEVIL_BOT_TEST_CHANNEL_URL";

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let func = service_fn(handler);
    lambda_http::run(func).await?;
    Ok(())
}

// This is the main event handler in the AWS Lambda. It parses the
// requests that were sent to the static endpoint behind our AWS
// API Gateway.
async fn handler(request: Request) -> Result<impl IntoResponse, Error> {
    let (_parts, body) = request.into_parts();
    let body: Value = serde_json::from_slice(&body)?;
    log::info!("{}", body);
    let challenge: String = intercept_challenge_request(&body).await;
    intercept_command(&body).await;

    Ok(json!({ "challenge": challenge }))
}

// When you create a Slack event subscription, your endpoint needs
// to respond to a challenge request with the challenge ID for
// the subscription to be successfully created.
// Read more here: https://api.slack.com/apis/connections/events-api
async fn intercept_challenge_request(body: &Value) -> String {
    let token: &str = body["token"].as_str().unwrap_or("invalid_token");
    let challenge: &str = body["challenge"].as_str().unwrap_or("invalid_challenge");
    let message_type: &str = body["type"].as_str().unwrap_or("invalid_type");

    if challenge == "invalid_challenge" {
        log::info!("Not a challenge request.");
    } else {
        let challenge_info: String = format!(
            "token: {}\nchallenge: {}\ntype: {}",
            token, challenge, message_type
        );
        log::info!("{}", challenge_info);
    }

    challenge.to_string()
}

// This function parses the event body received in the request
// and pulls out the Slack message text if there is any.
async fn intercept_command(body: &Value) {
    // Deconstruct the event type
    let event_type: &str = body["event"]["type"]
        .as_str()
        .unwrap_or("invalid event type");
    // Call the correct event handler
    match event_type {
        "team_join" => event_handlers::team_join_event_handler::handle_team_join_event(body).await,
        "reaction_added" => {
            event_handlers::reaction_added_event_handler::handle_reaction_added_event(body).await
        }
        "message" => message_event_handler::handle_message_event(body).await,
        _ => log::info!("invalid event type {}", event_type),
    }
}

// Helper function for getting Lambda environment variables. If
// you want to add new env vars, you can add them to the
// environment list in the devil-bot-rust-cdk-stack.ts file.
pub fn get_env_var(env_var: &str) -> String {
    match env::var(env_var) {
        Ok(val) => val,
        Err(_) => {
            log::info!("Required the {} environment variable", env_var);
            process::exit(1);
        }
    }
}

pub async fn increment_buns(enterprise_user_id: &str) {
    let buns_table_name: String = get_env_var(BUNS_TABLE_NAME);
    aws::dynamo::increment_item(&*buns_table_name, "user_id", enterprise_user_id, "buns")
        .await
        .unwrap_or_else(|err| log::info!("DynamoDB increment buns error: {}", err));
}
