use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

use tokio;

#[derive(Deserialize)]
// No Request format required at this time
struct Request {
    // Example:
    // command: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    access_token: String,
    expires_in: String,
    msg: String,
    refresh_token: String,
    req_id: String,
    token_type: String,
}

#[derive(Deserialize)]
struct ServiceFusionClientCredentialsGrantResponse {
    access_token: String,
    expires_in: String,
    refresh_token: String,
    token_type: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Set up request values
    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;
    let request_url = env::var("REQUEST_URL")?;

    // Set up some data
    let data = json!({
        "client_id": client_id,
        "client_secret": client_secret,
        "grant_type": "client_credentials"
    });

    let client = reqwest::Client::new();

    // Make the request
    let response = client
        .post(request_url)
        .json(&data)
        .send()
        .await?;

    let data = response.json::<ServiceFusionClientCredentialsGrantResponse>().await?;
    let resp = Response {
        access_token: data.access_token,
        expires_in: data.expires_in,
        msg: "Ok".to_string(),
        refresh_token: data.refresh_token,
        req_id: event.context.request_id,
        token_type: data.token_type,
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
