import { Construct, Stack, StackProps } from '@aws-cdk/core'
import { Architecture, Code, Function, Runtime } from '@aws-cdk/aws-lambda'
import { Policy, PolicyStatement } from '@aws-cdk/aws-iam'
import { LambdaRestApi } from '@aws-cdk/aws-apigateway'
import { AttributeType, Table } from '@aws-cdk/aws-dynamodb'
import { RetentionDays } from '@aws-cdk/aws-logs'

export class DevilBotRustCdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props)

    // Dynamo DB Table for holding persistent data that the lambda will call into.
    const bunsTable = new Table(this, 'buns-table', {
      partitionKey: {
        name: 'user_id',
        type: AttributeType.STRING,
      },
    })

    // Lambda function that wraps the Rust binary.
    const rustSlackLambda = new Function(this, 'rust-slack-lambda', {
      description:
        'Deploying a Rust function on Lambda using the custom runtime to back our Slack bot endpoint.',
      code: Code.fromAsset(
        'resources/target/x86_64-unknown-linux-musl/release/lambda'
      ),
      runtime: Runtime.PROVIDED_AL2,
      architecture: Architecture.X86_64,
      handler: 'not.required',
      environment: {
        // Fill in your personal app's token below when testing (remove them when creating a PR)
        RUST_BACKTRACE: '1',
        SLACK_API_BOT_TOKEN: '',
        SLACK_API_USER_TOKEN: '',
        BUNS_TABLE_NAME: bunsTable.tableName,
        IS_DEVELOPMENT: 'true',
        TEST_CHANNEL_ID: 'C0351GJ62Q0',
        INTROS_CHANNEL_ID: 'CMGU8033K',
      },
      logRetention: RetentionDays.ONE_DAY, // There will be a lot of event logs, this will make sure to cut down on costs
    })

    // Add Dynamo read/write access to the buns table.
    rustSlackLambda.role?.attachInlinePolicy(
      new Policy(this, 'read-write-buns-table-policy', {
        statements: [
          new PolicyStatement({
            actions: [
              'dynamodb:DescribeTable',
              'dynamodb:Query',
              'dynamodb:Scan',
              'dynamodb:PutItem',
              'dynamodb:UpdateItem',
            ],
            resources: [bunsTable.tableArn],
          }),
        ],
      })
    )

    // Defines an API Gateway REST API resource backed by the "rust-slack-lambda" function.
    new LambdaRestApi(this, 'RustSlackEndpoint', {
      handler: rustSlackLambda,
    })
  }
}
