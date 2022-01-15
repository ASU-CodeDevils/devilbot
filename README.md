# Devil Bot Rust
## Overview
* A Rust implementation of a Slack bot that will be used by the CodeDevils Slack workspace.
* All resources are managed using AWS CDK.
* The main driver is AWS API Gateway to provide static endpoints and AWS Lambda for serverless compute/request handling.

## Prereqs
* Install Rust: https://rustup.rs/
* Install NodeJS v16 (latest LTS version): https://nodejs.org/en/download/
* Install AWS CLI: https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html
* Install AWS CDK Toolkit: `npm install -g aws-cdk`

## Getting Started with Git
1. Create a new local directory for this project.
1. Run `git clone https://github.com/ASU-CodeDevils/devil_bot_rust.git` in that new directory.

## Set up AWS account
1. Create a new AWS account for free: https://aws.amazon.com
1. Go to the IAM console (type IAM in search bar on AWS website after logging in).
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


## Connecting to your AWS Account
1. `aws configure`
1. For "access key" use your "Access key ID" from the "Set up AWS account" instructions above.
1. For "secret access key" use your "Secret access key" from the "Set up AWS account" instructions above.
1. For "default region name" use: `us-east-1`
1. For "defaut output format" use: `None` (just leave blank and press enter)

## Set up project for AL2 target Ubuntu
1. TBD
1. `sh build-function.sh`
1. TBD

## Set up project for AL2 target MacOS (Intel)
1. `rustup target add x86_64-unknown-linux-musl`
1. `mkdir .cargo`
1. Create a local Cargo config file (don't commit this):
    ```
    echo '[target.x86_64-unknown-linux-musl]
    linker = "x86_64-linux-musl-gcc"' > .cargo/config
    ```
1. `brew install filosottile/musl-cross/musl-cross`
1. `ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc`
1. `sh build-function.sh`
1. `npm install`
1. `npm run build`
1. `cdk deploy`

## Set up project for AL2 target MacOS (Apple Silicon)
1. `rustup target add x86_64-unknown-linux-musl`
1. `mkdir .cargo`
1. Create a local Cargo config file (don't commit this):
    ```
    echo '[target.x86_64-unknown-linux-musl]
    linker = "x86_64-linux-gnu-gcc"' > .cargo/config
    ```
1. `sudo apt install gcc-x86-64-linux-gnu`
1. `sh build-function.sh`
1. `npm install`
1. `npm run build`
1. `cdk deploy`

## After your project is set up use the following to build your code and deploy it to AWS test.
1. `npm run build`
1. `sh build-function.sh`
1. `cdk diff` (optional, but useful command)
1. `cdk deploy`

## Useful CDK commands and their descriptions
 * `npm run build`   compile typescript to js
 * `npm run watch`   watch for changes and compile
 * `npm run test`    perform the jest unit tests
 * `cdk deploy`      deploy this stack to your default AWS account/region
 * `cdk diff`        compare deployed stack with current state
 * `cdk synth`       emits the synthesized CloudFormation template
