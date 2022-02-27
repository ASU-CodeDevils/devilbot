import * as cdk from "@aws-cdk/core";
import * as lambda from "@aws-cdk/aws-lambda";
import * as apigw from '@aws-cdk/aws-apigateway';
import { RetentionDays } from "@aws-cdk/aws-logs";

export class DevilBotRustCdkStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // Function that calls Rust
    const rustSlackLambda = new lambda.Function(this, "rust-slack-lambda", {
      description:
        "Deploying a Rust function on Lambda using the custom runtime to back our Slack bot endpoint.",
      code: lambda.Code.fromAsset(
        "resources/target/x86_64-unknown-linux-musl/release/lambda"
      ),
      runtime: lambda.Runtime.PROVIDED_AL2,
      architecture: lambda.Architecture.X86_64,
      handler: "not.required",
      environment: { // Fill in your personal app's webhook URLs below when testing (remove them when creating a PR)
        RUST_BACKTRACE: "1",
        SLACK_API_TOKEN: "",
        DEVIL_BOT_TEST_CHANNEL_URL: "",
        DEVIL_BOT_DEV_CHANNEL_URL: ""
      },
      logRetention: RetentionDays.ONE_DAY, // There will be a lot of event logs, this will make sure to cut down on costs
    });

    // defines an API Gateway REST API resource backed by the "rust-slack-lambda" function.
    new apigw.LambdaRestApi(this, 'RustSlackEndpoint', {
      handler: rustSlackLambda
    });
  }
}
