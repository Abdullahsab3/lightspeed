use serde_json::Value;

use crate::{utils::naming_convention::{to_snake_case, to_snake_case_plural}, models::ddr_req::AttributeType};

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
    fn generate_create_fn(&self, entity_name: &str) -> String {
        CONTROLLER_CREATE_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(entity_name).as_str())
            .replace("{sc_plural_entity}", to_snake_case_plural(entity_name).as_str())
            .replace("{entity_name}", &entity_name)
    }

    fn generate_update_fn(&self, entity_name: &str) -> String {
        CONTROLLER_UPDATE_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(entity_name).as_str())
            .replace("{sc_plural_entity}", to_snake_case_plural(entity_name).as_str())
            .replace("{entity_name}", &entity_name)
    }

    fn generate_delete_fn(&self, entity_name: &str) -> String {
        CONTROLLER_DELETE_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(entity_name).as_str())
            .replace("{sc_plural_entity}", to_snake_case_plural(entity_name).as_str())
            .replace("{entity_name}", &entity_name)
    }

    fn generate_create_payload(&self, entity_name: &str, entity: Value) -> String {
        let mut attributes = String::new();
        for (key, value) in entity.as_object().unwrap() {
            if key == "id" {
                continue;
            }
            let attribute_type = AttributeType::from_str(value.as_str().unwrap());
            attributes.push_str(&ATTRIBUTE_TEMPLATE
                .replace("{attribute_name}", key)
                .replace("{attribute_type}", &attribute_type.to_string()));
        }
        CONTROLLER_CREATE_ENTITY_PAYLOAD_TEMPLATE
            .replace("{entity_name}", &entity_name)
            .replace("{attributes}", &attributes)
    }

    fn generate_update_payload(&self, entity_name: &str, entity: Value) -> String {
        let mut attributes = String::new();
        for (key, value) in entity.as_object().unwrap() {
            if key == "id" {
                continue;
            }
            let attribute_type = AttributeType::from_str(value.as_str().unwrap());
            let attribute_type_str = match attribute_type {
                AttributeType::Option(t) => t.to_string(),
                _ => attribute_type.to_string()
            };
            attributes.push_str(&CONTROLLER_UPDATE_ENTITY_ATTRIBUTE_TEMPLATE
                .replace("{attribute_name}", key)
                .replace("{attribute_type}", &attribute_type_str));
        }
        CONTROLLER_UPDATE_ENTITY_PAYLOAD_TEMPLATE
            .replace("{entity_name}", &entity_name)
            .replace("{attributes}", &attributes)
    }

    fn generate_controller(&self, entity_name: &str, entity: Value) -> String {
        let mut controller_functions = String::new();
        controller_functions.push_str(&self.generate_create_fn(entity_name));
        controller_functions.push_str(&self.generate_update_fn(entity_name));
        controller_functions.push_str(&self.generate_delete_fn(entity_name));

        let mut controller_payloads = String::new();
        controller_payloads.push_str(&self.generate_create_payload(entity_name, entity.clone()));
        controller_payloads.push_str(&self.generate_update_payload(entity_name, entity));

        CONTROLLER_FILE_TEMPLATE
            .replace("{controller_functions}", &controller_functions)
            .replace("{controller_payloads}", &controller_payloads)
    }
    
}