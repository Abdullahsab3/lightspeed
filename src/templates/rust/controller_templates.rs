use crate::{utils::naming_convention::{to_snake_case, to_snake_case_plural}, models::{entity::AttributeType, entity::Entity}};

use super::{model_templates::ATTRIBUTE_TEMPLATE, import_templates::ImportGenerator};

pub static CONTROLLER_CREATE_ENTITY_TEMPLATE: &str = r#"
pub async fn create_{sc_entity_name}(
    State(services): State<Arc<ServicesState>>,
    Json(payload): Json<Add{entity_name}Payload>
) -> Result<impl IntoResponse> {
    services
        .{sc_plural_entity}_service
        .create_{sc_entity_name}(payload)
        .await
        .map(|{sc_entity_name}| {
            (StatusCode::CREATED, Json({sc_entity_name}))
        })
}
"#;

pub static CONTROLLER_CREATE_ENTITY_PAYLOAD_TEMPLATE: &str = r#"
#[derive(Deserialize)]
pub struct Add{entity_name}Payload {
    {attributes}
}
"#;

pub static CONTROLLER_UPDATE_ENTITY_TEMPLATE: &str = r#"
pub async fn update_{sc_entity_name}(
    Path(id): Path<Uuid>,
    State(services): State<Arc<ServicesState>>,
    Json(payload): Json<Update{entity_name}Payload>
) -> Result<impl IntoResponse> {
    services
        .{sc_plural_entity}_service
        .update_{sc_entity_name}(id, payload)
        .await
        .map(|{sc_entity_name}| {
            (StatusCode::OK, Json({sc_entity_name}))
        })
}
"#;

pub static CONTROLLER_UPDATE_ENTITY_ATTRIBUTE_TEMPLATE: &str = r#"
    pub {attribute_name}: Option<{attribute_type}>,"#;

pub static CONTROLLER_UPDATE_ENTITY_PAYLOAD_TEMPLATE: &str = r#"
#[derive(Deserialize)]
pub struct Update{entity_name}Payload {
    {attributes}
}
"#;

pub static CONTROLLER_DELETE_ENTITY_TEMPLATE: &str = r#"
pub async fn delete_{sc_entity_name}(
    Path(id): Path<Uuid>,
    State(services): State<Arc<ServicesState>>,
) -> Result<impl IntoResponse> {
    services
        .{sc_plural_entity}_service
        .delete_{sc_entity_name}(id)
        .await
}
"#;

pub static CONTROLLER_FILE_TEMPLATE: &str = r#"
use std::sync::Arc;

use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::Result;
use crate::services::ServicesState;

{controller_functions}

{controller_payloads}
"#;



pub trait ControllerGenerator: ImportGenerator {
    fn generate_create_fn(&self, entity: &Entity) -> String {
        CONTROLLER_CREATE_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(&entity.name).as_str())
            .replace("{sc_plural_entity}", to_snake_case_plural(&entity.name).as_str())
            .replace("{entity_name}", &entity.name)
    }

    fn generate_update_fn(&self, entity: &Entity) -> String {
        CONTROLLER_UPDATE_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(&entity.name).as_str())
            .replace("{sc_plural_entity}", to_snake_case_plural(&entity.name).as_str())
            .replace("{entity_name}", &entity.name)
    }

    fn generate_delete_fn(&self, entity: &Entity) -> String {
        CONTROLLER_DELETE_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(&entity.name).as_str())
            .replace("{sc_plural_entity}", to_snake_case_plural(&entity.name).as_str())
            .replace("{entity_name}", &entity.name)
    }

    fn generate_create_payload(&self, entity: &Entity) -> String {
        let mut attributes = String::new();
        for (attribute_name, attribute_type) in &entity.attributes {
            if attribute_name == &entity.primary_key {
                continue;
            }
            
            attributes.push_str(&ATTRIBUTE_TEMPLATE
                .replace("{attribute_name}", &attribute_name)
                .replace("{attribute_type}", &attribute_type.to_string()));
        }
        CONTROLLER_CREATE_ENTITY_PAYLOAD_TEMPLATE
            .replace("{entity_name}", &entity.name)
            .replace("{attributes}", &attributes)
    }

    fn generate_update_payload(&self, entity: &Entity) -> String {
        let mut attributes = String::new();
        for (attribute_name, attribute_type) in &entity.attributes {
            if attribute_name == &entity.primary_key {
                continue;
            }
            let attribute_type_str = match attribute_type {
                AttributeType::Option(t) => t.to_string(),
                _ => attribute_type.to_string()
            };
            attributes.push_str(&CONTROLLER_UPDATE_ENTITY_ATTRIBUTE_TEMPLATE
                .replace("{attribute_name}", &attribute_name)
                .replace("{attribute_type}", &attribute_type_str));
        }
        CONTROLLER_UPDATE_ENTITY_PAYLOAD_TEMPLATE
            .replace("{entity_name}", &entity.name)
            .replace("{attributes}", &attributes)
    }

    fn generate_controller(&self, entity: &Entity) -> String {
        let mut controller_functions = String::new();
        controller_functions.push_str(&self.generate_create_fn(&entity));
        controller_functions.push_str(&self.generate_update_fn(&entity));
        controller_functions.push_str(&self.generate_delete_fn(&entity));

        let mut controller_payloads = String::new();
        controller_payloads.push_str(&self.generate_create_payload(&entity));
        controller_payloads.push_str(&self.generate_update_payload(&entity));

        CONTROLLER_FILE_TEMPLATE
            .replace("{controller_functions}", &controller_functions)
            .replace("{controller_payloads}", &controller_payloads)
    }
    
}