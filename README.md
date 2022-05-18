# Readme
This example is designed to help developers deploy a REST API written in Rust and deployed to AWS ApiGateway as a [LambdaRestApi](https://docs.aws.amazon.com/cdk/api/v2/docs/aws-cdk-lib.aws_apigateway.LambdaRestApi.html)

### Technologies used:
- [poem, poem_lambda, poem_openapi](https://github.com/poem-web/poem)
- [AWS CDK](https://docs.aws.amazon.com/cdk/api/v2/)

## Build lambda zip 
```bash
cd lambda/api
docker run --rm     \
    -v ${PWD}:/code     \
    -v ${HOME}/.cargo/registry:/root/.cargo/registry     \
    -v ${HOME}/.cargo/git:/root/.cargo/git     \
    rustserverless/lambda-rust
```

## Run main locally
```bash
cd lambda/api
cargo run --bin main
```

## Run lambda locally
```bash
cd lambda/api
unzip -o \
    target/lambda/release/bootstrap.zip \
    -d /tmp/lambda && \
  docker run \
    -i -e DOCKER_LAMBDA_USE_STDIN=1 \
    --rm \
    -v /tmp/lambda:/var/task \
    lambci/lambda:provided.al2
```

### Example Payload: 
```json
{"body":"eyJ0ZXN0IjoiYm9keSJ9","resource":"/{proxy+}","path":"/api/hello","httpMethod":"GET","isBase64Encoded":true,"queryStringParameters":{"foo":"bar"},"multiValueQueryStringParameters":{"foo":["bar"]},"pathParameters":{"proxy":"/api/hello"},"stageVariables":{"baz":"qux"},"headers":{"Accept":"text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8","Accept-Encoding":"gzip, deflate, sdch","Accept-Language":"en-US,en;q=0.8","Cache-Control":"max-age=0","CloudFront-Forwarded-Proto":"https","CloudFront-Is-Desktop-Viewer":"true","CloudFront-Is-Mobile-Viewer":"false","CloudFront-Is-SmartTV-Viewer":"false","CloudFront-Is-Tablet-Viewer":"false","CloudFront-Viewer-Country":"US","Host":"1234567890.execute-api.us-east-1.amazonaws.com","Upgrade-Insecure-Requests":"1","User-Agent":"Custom User Agent String","Via":"1.1 08f323deadbeefa7af34d5feb414ce27.cloudfront.net (CloudFront)","X-Amz-Cf-Id":"cDehVQoZnx43VYQb9j2-nvCh-9z396Uhbp027Y2JvkCPNLmGJHqlaA==","X-Forwarded-For":"127.0.0.1, 127.0.0.2","X-Forwarded-Port":"443","X-Forwarded-Proto":"https"},"multiValueHeaders":{"Accept":["text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"],"Accept-Encoding":["gzip, deflate, sdch"],"Accept-Language":["en-US,en;q=0.8"],"Cache-Control":["max-age=0"],"CloudFront-Forwarded-Proto":["https"],"CloudFront-Is-Desktop-Viewer":["true"],"CloudFront-Is-Mobile-Viewer":["false"],"CloudFront-Is-SmartTV-Viewer":["false"],"CloudFront-Is-Tablet-Viewer":["false"],"CloudFront-Viewer-Country":["US"],"Host":["0123456789.execute-api.us-east-1.amazonaws.com"],"Upgrade-Insecure-Requests":["1"],"User-Agent":["Custom User Agent String"],"Via":["1.1 08f323deadbeefa7af34d5feb414ce27.cloudfront.net (CloudFront)"],"X-Amz-Cf-Id":["cDehVQoZnx43VYQb9j2-nvCh-9z396Uhbp027Y2JvkCPNLmGJHqlaA=="],"X-Forwarded-For":["127.0.0.1, 127.0.0.2"],"X-Forwarded-Port":["443"],"X-Forwarded-Proto":["https"]},"requestContext":{"accountId":"123456789012","resourceId":"123456","stage":"prod","requestId":"c6af9ac6-7b61-11e6-9a41-93e8deadbeef","requestTime":"09/Apr/2015:12:34:56 +0000","requestTimeEpoch":1428582896000,"identity":{"cognitoIdentityPoolId":null,"accountId":null,"cognitoIdentityId":null,"caller":null,"accessKey":null,"sourceIp":"127.0.0.1","cognitoAuthenticationType":null,"cognitoAuthenticationProvider":null,"userArn":null,"userAgent":"Custom User Agent String","user":null},"path":"/api/hello","resourcePath":"/{proxy+}","httpMethod":"GET","apiId":"1234567890","protocol":"HTTP/1.1"}}
```


### References:
- https://hevodata.com/learn/rust-lambda/
- https://github.com/poem-web/poem
- https://github.com/awslabs/aws-lambda-rust-runtime



## Welcome to your CDK TypeScript project

This is a blank project for CDK development with TypeScript.

The `cdk.json` file tells the CDK Toolkit how to execute your app.

### Useful commands
* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
* `cdk deploy`      deploy this stack to your default AWS account/region
* `cdk diff`        compare deployed stack with current state
* `cdk synth`       emits the synthesized CloudFormation template


[![Tutorial](https://img.youtube.com/vi/LiJNS3hAAbo/0.jpg)](https://www.youtube.com/watch?v=LiJNS3hAAbo)