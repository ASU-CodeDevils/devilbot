# Devil Bot Rust
## Overview
* A Rust implementation of a Slack bot that will be used by the CodeDevils Slack workspace.
* All resources are managed using AWS CDK.
* The main driver is AWS API Gateway to provide static endpoints and AWS Lambda for serverless compute/request handling.

## Getting Started with Git
1. Create a new local directory for this project.
1. Run `git clone https://github.com/ASU-CodeDevils/devil_bot_rust.git` in that new directory.

## Useful commands
 * `npm run build`   compile typescript to js
 * `npm run watch`   watch for changes and compile
 * `npm run test`    perform the jest unit tests
 * `cdk deploy`      deploy this stack to your default AWS account/region
 * `cdk diff`        compare deployed stack with current state
 * `cdk synth`       emits the synthesized CloudFormation template
