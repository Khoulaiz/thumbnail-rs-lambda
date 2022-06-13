# thumbnail-rs-lambda
S3 Event Lambda for AWS to create thumbnails

Just a try-out how easy you can setup rust lambdas.

What I did:

1. Install rust toolchain (https://www.rust-lang.org/tools/install)
2. Install cargo-lambda (https://github.com/cargo-lambda/cargo-lambda)
3. Install the aws-sam-cli (https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install-mac.html)
4. Create new cargo lambda project `cargo lambda new --event-type s3::S3Event thumbnail-rs`
5. Write your function (see `src` folder)
6. Build for your target `cargo lambda build --release --arm64`
7. Write a SAM template for deployment `template.yaml` and deploy `sam deploy --guided`
