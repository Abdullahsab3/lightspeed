use crate::{models::entity::Entity, utils::naming_convention::to_snake_case};

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
        .route("/v1/{sc_plural_entity}", get(filter_{sc_plural_entity}).post(create_{sc_entity_name}))"#;

pub static AXUM_ENTITY_ROUTE_TEMPLATE: &str = r#"
        .route("/v1/{sc_plural_entity}/:id", get(get_{sc_entity_name}).put(update_{sc_entity_name}).delete(delete_{sc_entity_name}))"#;

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
    fn generate_axum_routes(&self, entities: &Vec<&Entity>) -> String {
        let mut axum_routes = String::new();
        for entity in entities {
            let entity_route = AXUM_ENTITY_ROUTE_TEMPLATE
            .replace("{sc_entity_name}", &to_snake_case(&entity.name))
            .replace("{sc_plural_entity}", &to_snake_case(&entity.plural_name));
            let entity_collection_route = AXUM_ENTITIY_COLLECTION_ROUTE_TEMPLATE
            .replace("{sc_entity_name}", &to_snake_case(&entity.name))
            .replace("{sc_plural_entity}", &to_snake_case(&entity.plural_name));
            axum_routes.push_str(&entity_collection_route);
            axum_routes.push_str(&entity_route);
        }
        axum_routes
    }

    fn generate_axum_routes_system(&self, entities: &Vec<&Entity>) -> String {
        let axum_routes = self.generate_axum_routes(entities);
        AXUM_ROUTES_SYSTEM_TEMPLATE.replace("{axum_entity_routes}", &axum_routes)
    }

    fn generate_routes_file(&self, entities: Vec<&Entity>) -> String {
        let axum_routes_system = self.generate_axum_routes_system(&entities);
        let controller_imports = entities.iter().map(|entity| self.generate_controller_imports(&entity)).collect::<Vec<String>>().join("\n");
        ROUTES_FILE_TEMPLATE
        .replace("{routes_system}", &axum_routes_system)
        .replace("{controller_imports}", &controller_imports)
    }
}