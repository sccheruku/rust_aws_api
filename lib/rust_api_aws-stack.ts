import {
  Stack, StackProps,
  aws_lambda as lambda,
  aws_apigateway as apigw,
  DockerImage
} from 'aws-cdk-lib';

import { Construct } from 'constructs';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class RustApiAwsStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const target = 'x86_64-unknown-linux-musl';
    const functionName = "example_in_rust";
    const lambdaHandler = new lambda.Function(this, `${functionName}Handler`, {
      code: lambda.Code.fromAsset('.', {
        bundling: {
          command: [
            'bash', '-c',
            `rustup target add ${target} && cd lambda/api && cargo build --release --target ${target} && cp target/${target}/release/bootstrap /asset-output/bootstrap`
          ],
          image: DockerImage.fromRegistry('rust:latest')
        }
      }),
      functionName,
      handler: 'main',
      runtime: lambda.Runtime.PROVIDED_AL2
    });

    const gw = new apigw.LambdaRestApi(this, "exampleApi", {
      handler: lambdaHandler
    });
  }
}
