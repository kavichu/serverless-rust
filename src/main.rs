use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use lambda_runtime::{service_fn, LambdaEvent, Error as LambdaError};
use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main()  -> Result<(), LambdaError> {
    let func  = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, LambdaError> {

    let (event, _context) = event.into_parts();

    let first_name = event["first_name"].as_str().unwrap();
    let last_name = event["last_name"].as_str().unwrap();
    let uuid = Uuid::new_v4().to_string();

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let request = client.put_item()
        .table_name("Users")
        .item("uuid", AttributeValue::S(String::from(uuid)))
        .item("first_name", AttributeValue::S(String::from(first_name)))
        .item("last_name", AttributeValue::S(String::from(last_name)));

    request.send().await?;

    Ok(json!({"message": "Record written"}))
}