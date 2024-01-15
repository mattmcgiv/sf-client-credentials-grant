# Client Credentials Grant

## Overview

The "sf-client-credentials-grant" project is a Rust-based AWS Lambda function that facilitates the OAuth 2.0 Client Credentials Grant flow. This Lambda function is designed to request and receive access tokens from an OAuth 2.0 server, making it suitable for server-to-server authentication where a client can authenticate using its own credentials.

## Features

- Utilizes AWS Lambda and Rust for efficient and scalable serverless computing.
- Implements the OAuth 2.0 Client Credentials Grant flow.
- Securely handles sensitive information like client ID and secret.
- Parses and returns access token information in JSON format.

## Dependencies

The project relies on several Rust crates, including:

- `lambda_runtime`: For AWS Lambda functions.
- `reqwest`: For making HTTP requests.
- `serde` and `serde_json`: For serializing and deserializing JSON data.
- `tokio`: For asynchronous runtime.
- `tracing` and `tracing-subscriber`: For logging and tracing.
- `openssl`: For secure network connections.

Refer to `Cargo.toml` for the complete list of dependencies and their versions.

## Setup and Configuration

Before deploying the Lambda function, ensure the following environment variables are set:

- `CLIENT_ID`: The OAuth 2.0 client ID.
- `CLIENT_SECRET`: The OAuth 2.0 client secret.
- `REQUEST_URL`: The URL to request the access token from.

## Deployment

To deploy this Lambda function:

1. Compile the Rust project for the AWS Lambda environment:
   ```sh
   cargo lambda build --release
   ```
2. Deploy the Rust lambda:
   ```sh
   cargo lambda deploy --profile=<YOUR_AWS_PROFILE_NAME>
   ```
3. Configure the Lambda function's environment variables in the AWS Management Console or via the AWS CLI.

## Usage

Once deployed, the Lambda function can be invoked through `cargo lambda invoke`, AWS SDKs, the AWS CLI, or the AWS Management Console. The function will perform the Client Credentials Grant flow and return an access token response.

## Response Structure

The Lambda function returns a response in JSON format containing the following fields:

- `access_token`: The obtained access token.
- `expires_in`: The lifetime of the access token.
- `refresh_token`: The refresh token, if provided.
- `token_type`: The type of the token (typically "Bearer").
- `req_id`: The AWS Lambda request ID.
- `msg`: A message indicating the status

of the request (e.g., "Ok").

## Development

To contribute to this project or modify it for your own use, follow these steps:

1. Clone the repository to your local machine.
2. Install Rust and Cargo, following the official Rust installation guide.
3. Install `cargo lambda`
4. Add a `.env` file containing the necessary environment variables.
5. Build the lambda and run a lambda emulator locally with your env vars:
   ```sh
   cargo lambda watch --env-file .env
   ```
6. Invoke the lambda from the command line:
   ```sh
   cargo lambda invoke sf-client-credentials-grant --data-ascii '{}'
   ```

## Security

Do not hard-code sensitive information, such as client credentials, into the application. Use environment variables or AWS Secrets Manager for handling sensitive data.

## Support and Issues

For support or to report any issues, please file an issue on the project's GitHub repository.

## License

This project is licensed under the [MIT License](LICENSE). See the LICENSE file in the project repository for more information.
