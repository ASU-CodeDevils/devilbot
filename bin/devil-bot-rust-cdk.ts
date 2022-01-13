#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { DevilBotRustCdkStack } from '../lib/devil-bot-rust-cdk-stack';

const app = new cdk.App();
new DevilBotRustCdkStack(app, 'DevilBotRustCdkStack');
