use crate::utils::naming_convention::to_snake_case;

pub static AXUM_ROUTES_SYSTEM_TEMPLATE: &str = r#"
pub fn routes_system(services: Arc<ServicesState>) -> Router {
    Router::new()
        .route("/alive", get(alive))
        {axum_entity_routes}
        .with_state(services)
}
"#;

pub static AXUM_ENTITIY_COLLECTION_ROUTE_TEMPLATE: &str = r#"
        .route("/v1/{sc_entity_name}s", post(create_{sc_entity_name}))"#;

pub static AXUM_ENTITY_ROUTE_TEMPLATE: &str = r#"
        .route("/v1/{sc_entity_name}s/:id", get(get_{sc_entity_name}).put(update_{sc_entity_name}).delete(delete_{sc_entity_name}))"#;

pub trait AxumRoutesGenerator {
    fn generate_axum_routes(&self, entity_name: String) -> String {
        let sc_entity_name = to_snake_case(&entity_name);
        let entity_collection_endpoint = AXUM_ENTITIY_COLLECTION_ROUTE_TEMPLATE
            .replace("{sc_entity_name}", &sc_entity_name);
        let entity_endpoint = AXUM_ENTITY_ROUTE_TEMPLATE
            .replace("{sc_entity_name}", &sc_entity_name);
        let entity_routes = format!("{}{}", entity_collection_endpoint, entity_endpoint);
        AXUM_ROUTES_SYSTEM_TEMPLATE
            .replace("{axum_entity_routes}", &entity_routes)
            
    }
}