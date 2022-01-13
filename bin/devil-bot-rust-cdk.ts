#!/usr/bin/env node
import 'source-map-support/register'
import * as cdk from '@aws-cdk/core'
import { DevilBotRustCdkStack } from '../lib/devil-bot-rust-cdk-stack';

const app = new cdk.App();
new DevilBotRustCdkStack(app, 'DevilBotRustCdkStack', {
    env: { region: app.node.tryGetContext('region') || 'us-east-1' },
});
