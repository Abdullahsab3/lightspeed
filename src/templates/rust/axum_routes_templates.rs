use crate::utils::naming_convention::to_snake_case;

use super::import_templates::ImportGenerator;

pub static AXUM_ROUTES_SYSTEM_TEMPLATE: &str = r#"
pub fn routes_system(services: Arc<ServicesState>) -> Router {
    Router::new()
        .route("/alive", get(alive))
        {axum_entity_routes}
        .with_state(services)
}
"#;

pub static AXUM_ENTITIY_COLLECTION_ROUTE_TEMPLATE: &str = r#"
        .route("/v1/{sc_entity_name}s", get(get_{sc_entity_name}s).post(create_{sc_entity_name}))"#;

pub static AXUM_ENTITY_ROUTE_TEMPLATE: &str = r#"
        .route("/v1/{sc_entity_name}s/:id", get(get_{sc_entity_name}).put(update_{sc_entity_name}).delete(delete_{sc_entity_name}))"#;

pub static ROUTES_FILE_TEMPLATE: &str = r#"
use std::sync::Arc;

use axum::routing::{delete, get, post};
use axum::Router;
use crate::controllers::system_controller::alive;
use crate::services::ServicesState;

{controller_imports}

{routes_system}
"#;


pub trait AxumRoutesGenerator: ImportGenerator {
    fn generate_axum_routes(&self, entity_names: &Vec<String>) -> String {
        let sc_entity_names = entity_names.iter().map(|x| to_snake_case(x)).collect::<Vec<String>>();
        let mut axum_routes = String::new();
        for sc_entity_name in sc_entity_names{
            let entity_route = AXUM_ENTITY_ROUTE_TEMPLATE.replace("{sc_entity_name}", &sc_entity_name);
            let entity_collection_route = AXUM_ENTITIY_COLLECTION_ROUTE_TEMPLATE
            .replace("{sc_entity_name}", &sc_entity_name);
            axum_routes.push_str(&entity_collection_route);
            axum_routes.push_str(&entity_route);
        }
        axum_routes
    }

    fn generate_axum_routes_system(&self, entity_names: &Vec<String>) -> String {
        let axum_routes = self.generate_axum_routes(entity_names);
        AXUM_ROUTES_SYSTEM_TEMPLATE.replace("{axum_entity_routes}", &axum_routes)
    }

    fn generate_routes_file(&self, entity_names: Vec<String>) -> String {
        let axum_routes_system = self.generate_axum_routes_system(&entity_names);
        let controller_imports = entity_names.iter().map(|entity_name| self.generate_controller_imports(entity_name)).collect::<Vec<String>>().join("\n");
        ROUTES_FILE_TEMPLATE
        .replace("{routes_system}", &axum_routes_system)
        .replace("{controller_imports}", &controller_imports)
    }
}