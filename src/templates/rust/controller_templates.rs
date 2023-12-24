use serde_json::Value;

use crate::{utils::naming_convention::to_snake_case, models::ddr_req::AttributeType};

use super::model_template::ATTRIBUTE_TEMPLATE;

pub static CONTROLLER_CREATE_ENTITY_TEMPLATE: &str = r#"
pub async fn create_{sc_entity_name}(
    State(services): State<Arc<ServicesState>>,
    Json(payload): Json<Add{entity_name}Payload>
) -> Result<impl IntoResponse> {
    services
        .{sc_entity_name}_service
        .create_{sc_entity_name}(payload)
        .await
        .map(|{sc_entity_name}| {
            (StatusCode::CREATED, Json(entity)))
        })
}
"#;

pub static CONTROLLER_CREATE_ENTITY_PAYLOAD_TEMPLATE: &str = r#"
#[derive(Serialize, Deserialize)]
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
        .{sc_entity_name}_service
        .update_{sc_entity_name}(id, payload)
        .await
        .map(|{sc_entity_name}| {
            (StatusCode::UPDATED, Json(entity)))
        })
}
"#;

pub static CONTROLLER_UPDATE_ENTITY_ATTRIBUTE_TEMPLATE: &str = r#"
    pub {attribute_name}: Option<{attribute_type}>,"#;

pub static CONTROLLER_UPDATE_ENTITY_PAYLOAD_TEMPLATE: &str = r#"
#[derive(Serialize, Deserialize)]
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
        .{sc_entity_name}_service
        .delete_{sc_entity_name}(id, payload)
        .await
}
"#;


pub trait ControllerGenerator {
    fn generate_create_fn(&self, entity_name: &str) -> String {
        CONTROLLER_CREATE_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(entity_name).as_str())
            .replace("{entity_name}", &entity_name)
    }

    fn generate_update_fn(&self, entity_name: &str) -> String {
        CONTROLLER_UPDATE_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(entity_name).as_str())
            .replace("{entity_name}", &entity_name)
    }

    fn generate_delete_fn(&self, entity_name: &str) -> String {
        CONTROLLER_DELETE_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(entity_name).as_str())
            .replace("{entity_name}", &entity_name)
    }

    fn generate_create_payload(&self, entity_name: &str, value: Value) -> String {
        let mut attributes = String::new();
        for (key, value) in value.as_object().unwrap() {
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

    fn generate_update_payload(&self, entity_name: &str, value: Value) -> String {
        let mut attributes = String::new();
        for (key, value) in value.as_object().unwrap() {
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
    
}