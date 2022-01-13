import { expect as expectCDK, haveResourceLike } from '@aws-cdk/assert'
import * as cdk from '@aws-cdk/core'
import * as RustLambda from '../lib/devil-bot-rust-cdk-stack'
import { Code, CodeConfig } from '@aws-cdk/aws-lambda'

let fromAssetMock: jest.SpyInstance

beforeAll(() => {
  fromAssetMock = jest.spyOn(Code, 'fromAsset').mockReturnValue({
    isInline: false,
    bind: (): CodeConfig => {
      return {
        s3Location: {
          bucketName: 'my-bucket',
          objectKey: 'my-key',
        },
      }
    },
    bindToResource: () => {
      return
    },
  } as any)
})

afterAll(() => {
  fromAssetMock?.mockRestore()
})

test('Lambda function and corresponding IAM role is created', () => {
  const app = new cdk.App()
  const stack = new RustLambda.DevilBotRustCdkStack(app, 'RustLambda')

  expectCDK(stack).to(
    haveResourceLike('AWS::IAM::Role', {
      AssumeRolePolicyDocument: {
        Statement: [
          {
            Action: 'sts:AssumeRole',
            Effect: 'Allow',
            Principal: {
              Service: 'lambda.amazonaws.com',
            },
          },
        ],
        Version: '2012-10-17',
      },
      ManagedPolicyArns: [],
    })
  )

  expectCDK(stack).to(
    haveResourceLike('AWS::Lambda::Function', {
      Role: {},
      Architectures: ['arm64'],
      Description:
        'Deploying a Rust function on Lambda using the custom runtime',
      Environment: {
        Variables: {
          RUST_BACKTRACE: '1',
        },
      },
      Handler: 'not.required',
      Runtime: 'provided.al2',
    })
  )
})
