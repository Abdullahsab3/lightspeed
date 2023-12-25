use std::sync::Arc;

use anyhow::Context;
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::{middleware, Json, Router};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::controllers::*;
use crate::services::*;
use crate::error::{Error, Result};
use crate::log_request::log_request;

pub fn app(services: ServicesState) -> Router {
    routes::routes_system(services.into())
        .layer(middleware::map_response(main_response_mapper))
}

pub async fn serve(
    pool: PgPool,
) -> Result<()> {
    let services = create_services(pool).await?;
    match axum::Server::bind(&"0.0.0.0:9000".parse().expect("Could not parse the address"))
        .serve(app(services).into_make_service())
        .await
        .context("failed to run the server")
    {
        Ok(_) => println!("Server started"),
        Err(e) => panic!("Could not start the server: {e:?}"),
    }
    Ok(())
}

async fn main_response_mapper(uri: Uri, req_method: Method, res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(Error::client_status_and_error);

    // -- If client error, build the new reponse.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });
            println!("    ->> client_error_body: {client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    // TODO: Need to hander if log_request fail (but should not fail request)
    let _ = log_request(uuid, req_method, uri, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
}
