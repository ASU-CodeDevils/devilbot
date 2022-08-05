# Devil Bot Rust
![devil_bot_rust](https://github.com/ASU-CodeDevils/devil_bot_rust/actions/workflows/check-and-lint.yaml/badge.svg) ![devil_bot_rust](https://github.com/ASU-CodeDevils/devil_bot_rust/actions/workflows/build-and-test.yaml/badge.svg) [![codecov](https://codecov.io/gh/ASU-CodeDevils/devil_bot_rust/branch/main/graph/badge.svg?token=6NS5GOSXZ2)](https://codecov.io/gh/ASU-CodeDevils/devil_bot_rust) [![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Table of Contents
1. [Overview](#overview)
1. [Prereqs](#prereqs)
1. [Getting Started with Git](#getting-started-with-git)
1. [Set up AWS account](#set-up-aws-account)
1. [Connecting to your AWS Account to your personal device](#connecting-to-your-aws-account-to-your-personal-device)
1. [Set up project for AL2 target Mac, Ubuntu (aarch64/x86_64), and Windows (WSL 2 Ubuntu 20.04 LTS)](#set-up-project-for-al2-target-mac-ubuntu-aarch64x86_64-and-windows-wsl-2-ubuntu-2004-lts)
1. [After your project is set up use the following to build your code and deploy it to AWS test](#after-your-project-is-set-up-use-the-following-to-build-your-code-and-deploy-it-to-aws-test)
1. [Setting up to test against a personal Slack bot](#setting-up-to-test-against-a-personal-slack-bot)
1. [Useful CDK commands and their descriptions](#useful-cdk-commands-and-their-descriptions)
1. [How to Enable API Throttling](#how-to-enable-api-throttling)

## Overview
* A Rust implementation of a Slack bot that will be used by the CodeDevils Slack workspace.
* All resources are managed using AWS CDK.
* The main driver is AWS API Gateway to provide static endpoints and AWS Lambda for serverless compute/request handling.

## Prereqs
* Install Rust: https://rustup.rs/
* Install NodeJS v16 (latest LTS version): https://nodejs.org/en/download/
* Install AWS CLI: https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html
* Install AWS CDK Toolkit: `npm install -g aws-cdk`
* You may get a warning that -g is deprecated and to use --location=global instead

## Getting Started with Git
1. Create a new local directory for this project.
1. Run `git clone https://github.com/ASU-CodeDevils/devil_bot_rust.git` in that new directory.

## Set up AWS account
1. Create a new AWS account for free: https://aws.amazon.com
1. Go to the IAM console (type "IAM" in search bar on AWS website after logging in).
1. Click on "Users" on the left-hand side bar under "Access Management".
1. Click "Add Users" to the right.
1. For username choose something like "devil-bot-test-user-${your_asu_alias}" (e.g. "devil-bot-test-user-jtmichel").
1. Make sure "Access key - Programmatic access" check box is checked.
1. Click "Next: Permissions".
1. Click "Attach existing policies directly".
1. Check "AdministratorAccess" (you can use less permissions if you know what you're doing, but this should work fine as long as you don't give away your credentials).
1. Click "Next: Tags".
1. Click "Next: Review".
1. Click "Create user".
1. Copy both your "Access key ID" and your "Secret access key" somewhere locally (only store this temporarily then delete).
1. Continue to "Connecting to your AWS Account" steps below.

## Connecting to your AWS Account to your personal device
1. Run the following on your personal device's terminal.
1. `aws configure`
1. For "access key" use your "Access key ID" from the "Set up AWS account" instructions above.
1. For "secret access key" use your "Secret access key" from the "Set up AWS account" instructions above.
1. For "default region name" use: `us-east-1`
1. For "defaut output format" use: `None` (just leave blank and press enter)

## Set up project for AL2 target Mac, Ubuntu (aarch64/x86_64), and Windows (WSL 2 Ubuntu 20.04 LTS)
1. If on Windows install and configure Ubuntu 20.04 LTS using WSL2 the following step are to be done wihtin that VM
https://docs.microsoft.com/en-us/windows/wsl/install
1. Ensure you've installed Rust, NPM, AWS-CDK Toolkit, and AWS-CLI
1. Confirm you've setup your AWS account and Connected it!
1. `chmod +x build-function.sh`
1. `sh build-function.sh`
1. `cdk bootstrap`
1. `cdk deploy`

## After your project is set up use the following to build your code and deploy it to AWS test
1. `sh build-function.sh`
1. `cdk diff` (optional, but useful command)
1. `cdk deploy`

## Setting up to test against a personal Slack bot
1. Do the following after creating the above infrastructure (after successfully running `cdk deploy` to your personal AWS account).
1. Go to the API Gateway console (type "API Gateway" in search bar on AWS website after logging in).
1. Click on "APIs" on the left-hand side bar under "API Gateway".
1. Check to make sure you are signed in under us-east-1
1. Click on "RustSlackEndpoint".
1. Click on "Stages" on the left-hand side bar under "API: RustSlackEndpoint".
1. Click on "Prod" under "Stages".
1. Copy the "Invoke URL" provided.
1. Contact one of our officers and tell them you have your Slack bot API Gateway endpoint ready for a personal Slack bot app.
1. Provide the officer with the URL and they will give you a Slack webhook URL which you can then plug in to the `environment` list in `devil-bot-rust-cdk-stack.ts`.
1. This will allow you to test in the `#devil-bot-test Slack` channel while you are developing.
1. When you have your code ready for review, remove the environment variable before creating your PR. Follow the instructions found in `CONTRIBUTING.md` for more info on creating your PR.

## Useful CDK commands and their descriptions
 * `npm run build`   compile typescript to js
 * `npm run watch`   watch for changes and compile
 * `npm run test`    perform the jest unit tests
 * `cdk deploy`      deploy this stack to your default AWS account/region
 * `cdk diff`        compare deployed stack with current state
 * `cdk synth`       emits the synthesized CloudFormation template

## Testing with POST requests
Sometimes you may not want to spam messages into the Slack channels when you want to test. In this case you can POST messages directly to your API Gateway endpoint and view CloudWatch logs to troubleshoot problems with your code.

### Postman
Postman is a UI alternative to using [`curl`](https://everything.curl.dev).
1. Download postman for free from https://www.postman.com/downloads/
1. Ignore the make an account messages and "workspaces" and just use the "scratchpad" offline feature. This used to be the only way Postman operated, but they are trying to make money so we will forgive them for begging us to use cloud storage on a platform that definitely doesn't need it.
1. Under the scratchpad menu "Overview" click on the "Create a request" button.
1. Click on the "GET" dropdown and swap to "POST".
1. In the "Enter request URL" box insert your API Gateway URL obtained in the [Setting up to test against a personal Slack bot](#setting-up-to-test-against-a-personal-slack-bot) section.
1. Click on the "Body" tab and enter a modified version of one of the message body from below.
1. Click on the "raw" radio button, and select "JSON" from the dropdown to the right of the radio buttons.
1. You can now send requests directly to your endpoint.

### Example Message Event Body JSON
```javascript
{
    "api_app_id": "XXXXXXXXXXX",
    "authorizations": [
        {
            "enterprise_id": "XXXXXXXXX",
            "is_bot": false,
            "is_enterprise_install": false,
            "team_id": "XXXXXXXXX",
            "user_id": "XXXXXXXXX"
        }
    ],
    "enterprise_id": "XXXXXXXXX",
    "event": {
        "blocks": [
            {
                "block_id": "xXxx",
                "elements": [
                    {
                        "elements": [
                            {
                                "text": "Test message here.",
                                "type": "text"
                            }
                        ],
                        "type": "rich_text_section"
                    }
                ],
                "type": "rich_text"
            }
        ],
        "channel": "XXXXXXXXXXX",
        "channel_type": "group",
        "client_msg_id": "9173c749-d7b3-4330-9576-590740901793",
        "event_ts": "1645903860.916719",
        "team": "XXXXXXXXX",
        "text": "test",
        "ts": "1645903860.916719",
        "type": "message",
        "user": "XXXXXXXXX"
    },
    "event_context": "4-eyJldCI6Im1lc3NhZ2UiLCJ0aWQiOiJUMk43NkZaM1EiLCJhaWQiOiJBMDJVOUc4NUI2WiIsImNpZCI6IkMwMzUxR0o2MlEwIn0",
    "event_id": "Ev0356A5S917",
    "event_time": 1645903860,
    "is_ext_shared_channel": false,
    "team_id": "XXXXXXXXX",
    "token": "oooooooooooooooooo",
    "type": "event_callback"
}
```

### Example Challenge Body JSON
* https://api.slack.com/events/url_verification
```javascript
{
    "token": "Jhj5dZrVaK7ZwHHjRyZWjbDl",
    "challenge": "3eZbrw1aBm2rZgRNFdxV2595E9CY3gmdALWMmHkvFXO7tYXAYM8P",
    "type": "url_verification"
}
```

### How to Enable API Throttling
1. Sign into AWS
1. In the search bar, search for API Gateway
1. Click on RustSlackEndpint
1. In the left menu, click on Usage Plans
1. In the Usage Plans menu, create a new usage plan
** This is pretty customizable. Recommended to cap your requests per month at 900,000
