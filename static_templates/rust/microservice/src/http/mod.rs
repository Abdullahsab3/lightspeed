pub async fn create_services(
    pool: Pool<Postgres>,
    effy_producers: EffyProducers,
) -> Result<ServicesState> {
    let arc_pool = Arc::new(pool);
    let tags_service: TagsService =
        TagsService::new(Arc::clone(&arc_pool), effy_producers.tag_producer).await?;
    let readings_service: ReadingsService = ReadingsService::new(Arc::clone(&arc_pool)).await?;
    let folders_service: folders_service::FoldersService = folders_service::FoldersService::new(
        Arc::clone(&arc_pool),
        effy_producers.tag_relations_producer.clone(),
    )
    .await?;
    let parameters_service: parameters_service::ParametersService =
        parameters_service::ParametersService::new(Arc::clone(&arc_pool)).await?;
    let sources_service: crate::services::sources_service::SourcesService =
        crate::services::sources_service::SourcesService::new(Arc::clone(&arc_pool)).await?;
    let asset_linking_service: crate::services::asset_linking_service::AssetLinkingService =
        crate::services::asset_linking_service::AssetLinkingService::new(
            Arc::clone(&arc_pool),
            effy_producers.tag_relations_producer,
        )
        .await?;
    Ok(ServicesState {
        tags_service,
        readings_service,
        folders_service,
        parameters_service,
        sources_service,
        asset_linking_service,
    })
}

pub fn app(services: ServicesState) -> Router {
    routes::routes_system(services.into())
        .layer(middleware::map_response(main_response_mapper))
}

pub async fn serve(
    pool: Pool<Postgres>,
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
