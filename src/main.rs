use std::io::Cursor;
use aws_lambda_events::event::s3::S3Event;
use aws_sdk_s3::Client;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::types::ByteStream;
use image::DynamicImage;
use image::io::Reader as ImageReader;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-runtime/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<S3Event>) -> Result<(), Error> {
    for s3rec in event.payload.records.iter() {
        info!("lambda called for record");
        if let Some(name) = s3rec.event_name.as_deref() {
            info!("event:{}",name);
            if name.starts_with("ObjectCreated:") {
                let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
                let config = aws_config::from_env().region(region_provider).load().await;
                let s3 = Client::new(&config);
                info!("client created");
                let bucket_name = s3rec.s3.bucket.name.as_deref().unwrap();
                let original_name = s3rec.s3.object.key.as_deref().unwrap();
                let thumb_name = format!("thumbs/{}",original_name.split("/").last().unwrap());
                let image = read_image(&s3, bucket_name, original_name).await?;
                info!("image read");
                let thumb = image.thumbnail(200,200);
                info!("thumb created");
                write_image(s3, bucket_name, thumb_name.as_str(), thumb).await?;
                info!("thumb written to {}",thumb_name);
            }
        }
    }
    info!("lambda ended no error");
    Ok(())
}

async fn write_image(s3: Client, bucket_name: &str, key_name: &str, thumb: DynamicImage) -> Result<(),Error> {
    let mut bytes: Vec<u8> = Vec::new();
    thumb.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Jpeg(75))?;
    s3.put_object()
        .body(ByteStream::from(bytes))
        .bucket(bucket_name)
        .key(key_name)
        .content_type("image/jpeg")
        .send().await?;
    Ok(())
}

async fn read_image(s3: &Client, bucket_name: &str, key_name: &str) -> Result<DynamicImage,Error> {
    let s3object = s3
        .get_object()
        .bucket(bucket_name)
        .key(key_name)
        .send().await?;
    let bytes = s3object.body.collect().await?.into_bytes();
    let image = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?.decode()?;
    Ok(image)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
