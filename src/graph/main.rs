use async_graphql::{
    EmptySubscription, Request as GraphqlRequest, Response as GraphqlResponse, Result, Schema,
    ServerError as GraphqlError,
};

use lambda_http::{
    http::{Method, StatusCode},
    Body as LambdaBody, Error as LambdaError, Request, Response as LambdaResponse,
};

use lambda_runtime::service_fn;

use serde_json;

use std::sync::LazyLock;

use tokio;

static SCHEMA: LazyLock<Schema<Query, Mutation, EmptySubscription>> =
    LazyLock::new(|| Schema::build(Query, Mutation, EmptySubscription).finish());

mod db;
mod schema;
mod consts;

use schema::*;

#[derive(Debug)]
enum ClientError {
    MethodNotAllowed,
    EmptyBody,
    InvalidJson(serde_json::Error),
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        ClientError::InvalidJson(err)
    }
}

fn graphql_error(client_error: ClientError) -> GraphqlError {
    let message = match client_error {
        ClientError::MethodNotAllowed => "Method not allowed",
        ClientError::EmptyBody => "Empty request body",
        ClientError::InvalidJson(_) => "Invalid JSON in request body",
    };
    GraphqlError::new(message.to_string(), None)
}

fn error_response(
    status: StatusCode,
    error: GraphqlError,
) -> Result<LambdaResponse<LambdaBody>, LambdaError> {
    let error_response = GraphqlResponse::from_errors(vec![error]);
    let response_body = serde_json::to_string(&error_response).unwrap();
    Ok(LambdaResponse::builder()
        .status(status.as_u16())
        .body(LambdaBody::Text(response_body))?)
}

async fn handle_request(request: Request) -> Result<LambdaResponse<LambdaBody>, LambdaError> {
    let query = if request.method() == Method::POST {
        graphql_request_from_post(request)
    } else {
        Err(ClientError::MethodNotAllowed)
    };
    let query = match query {
        Err(e) => {
            return error_response(StatusCode::BAD_REQUEST, graphql_error(e));
        }
        Ok(query) => query,
    };
    let response_body = serde_json::to_string(&SCHEMA.execute(query).await).unwrap();
    Ok(LambdaResponse::builder()
        .status(200)
        .body(LambdaBody::Text(response_body))?)
}

fn graphql_request_from_post(request: Request) -> Result<GraphqlRequest, ClientError> {
    match request.into_body() {
        LambdaBody::Empty => Err(ClientError::EmptyBody),
        LambdaBody::Text(text) => {
            serde_json::from_str::<GraphqlRequest>(&text).map_err(ClientError::from)
        }
        LambdaBody::Binary(binary) => {
            serde_json::from_slice::<GraphqlRequest>(&binary).map_err(ClientError::from)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    lambda_http::run(service_fn(handle_request)).await
}
