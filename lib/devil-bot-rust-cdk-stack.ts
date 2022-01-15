import * as cdk from "@aws-cdk/core";
import * as lambda from "@aws-cdk/aws-lambda";
import * as apigw from '@aws-cdk/aws-apigateway';
import { RetentionDays } from "@aws-cdk/aws-logs";

export class DevilBotRustCdkStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // Function that calls Rust
    const rustHello = new lambda.Function(this, "rust-hello", {
      description:
        "Deploying a Rust function on Lambda using the custom runtime",
      code: lambda.Code.fromAsset(
        "resources/target/x86_64-unknown-linux-musl/release/lambda"
      ),
      runtime: lambda.Runtime.PROVIDED_AL2,
      architectures: [lambda.Architecture.X86_64],
      handler: "not.required",
      environment: {
        RUST_BACKTRACE: "1",
      },
      logRetention: RetentionDays.ONE_WEEK,
    });

    // defines an API Gateway REST API resource backed by the "rust-hello" function.
    new apigw.LambdaRestApi(this, 'Endpoint', {
      handler: rustHello
    });
  }
}
