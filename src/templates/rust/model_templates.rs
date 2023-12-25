use serde_json::Value;
use crate::{models::ddr_req::AttributeType, utils::naming_convention::to_snake_case};

use super::import_templates::ImportGenerator;

pub static STRUCT_TEMPLATE: &str = r#"
#[derive(Serialize, Deserialize)]
pub struct {struct_name} {
    {attributes}
}
"#;

pub static NEW_FROM_PAYLOAD_TEMPLATE: &str = r#"
    pub fn new(payload: &Add{entity_name}Payload) -> Result<Self, Error> {
        Ok(Self {
            {new_attribute_from_payload}
        })
    }
"#;

pub static NEW_ATTRIBUTE_FROM_PAYLOAD: &str = r#"
            {attribute_name}: payload.{attribute_name},"#;

pub static UPDATE_FROM_PAYLOAD_TEMPLATE: &str = r#"
    pub fn update(&mut self, payload: &Update{entity_name}Payload) -> Result<Self, Error> {
        Ok(Self {
            {update_attribute_from_payload}
        })
    }
"#;

pub static UPDATE_ATTRIBUTE_FROM_PAYLOAD_NULLABLE: &str = r#"
            {attribute_name}: payload.{attribute_name}.or(self.{attribute_name}),"#;
pub static UPDATE_ATTRIBUTE_FROM_PAYLOAD: &str = r#"
            {attribute_name}: payload.{attribute_name}.unwrap_or(self.{attribute_name}),"#;

pub static ENTITY_IMPL_TEMPLATE: &str = r#"
impl {entity_name} {
    {new_from_payload}
    {update_from_payload}
}
"#;


pub static ATTRIBUTE_TEMPLATE: &str = r#"
    pub {attribute_name}: {attribute_type},"#;

pub static ENUM_TEMPLATE: &str = r#"
#[derive(Serialize, Deserialize, Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum {enum_name} {
    {enum_values}
}
"#;

pub static ENUM_VALUE_TEMPLATE: &str = r#"
    {enum_value},"#;

pub static MODEL_FILE_TEMPLATE: &str = r#"
use serde::Serialize;
use serde::Deserialize;
use uuid::Uuid;
use crate::error::Error;

{imports}


{model_definition}
{model_impl}

"#;

pub trait ModelGenerator: ImportGenerator {

    fn generate_model(&self, entity_name: &str, entity: &Value) -> String {
        let model_definition = self.generate_struct(entity_name, entity);
        let model_impl = self.generate_struct_impl(entity_name, entity);
        MODEL_FILE_TEMPLATE
            .replace("{imports}", &self.generate_controller_imports(entity_name))
            .replace("{model_definition}", &model_definition)
            .replace("{model_impl}", &model_impl)
    }
    fn generate_struct(&self, name: &str, entity: &Value) -> String {
        let mut attributes = String::new();
        for (key, value) in entity.as_object().unwrap() {
            let attribute_type = AttributeType::from_str(value.as_str().unwrap());
            attributes.push_str(&ATTRIBUTE_TEMPLATE
                .replace("{attribute_name}", key)
                .replace("{attribute_type}", &attribute_type.to_string()));
        }
        STRUCT_TEMPLATE
            .replace("{struct_name}", &name)
            .replace("{attributes}", &attributes)
    }

    fn generate_new_fn(&self, entity_name: &str, entity: &Value) -> String {
        let mut new_attribute_from_payload = String::new();
        for (key, value) in entity.as_object().unwrap() {
            new_attribute_from_payload.push_str(&NEW_ATTRIBUTE_FROM_PAYLOAD
                .replace("{attribute_name}", key)
                .replace("{attribute_type}", value.as_str().unwrap()));
        }
        NEW_FROM_PAYLOAD_TEMPLATE
            .replace("{entity_name}", &entity_name)
            .replace("{new_attribute_from_payload}", &new_attribute_from_payload)
    }

    fn generate_update_fn(&self, entity_name: &str, entity: &Value) -> String {
        let mut update_attribute_from_payload = String::new();
        for (key, value) in entity.as_object().unwrap() {
            let attribute_type = AttributeType::from_str(value.as_str().unwrap());
            let attribute_type_str = match &attribute_type {
                AttributeType::Option(t) => t.to_string(),
                _ => attribute_type.to_string()
            };
            match attribute_type {
                AttributeType::Option(_) => {
                    update_attribute_from_payload.push_str(&UPDATE_ATTRIBUTE_FROM_PAYLOAD_NULLABLE
                        .replace("{attribute_name}", key)
                        .replace("{attribute_type}", &attribute_type_str));
                },
                _ => {
                    update_attribute_from_payload.push_str(&UPDATE_ATTRIBUTE_FROM_PAYLOAD
                        .replace("{attribute_name}", key)
                        .replace("{attribute_type}", &attribute_type_str));
                }
            }
        }
        UPDATE_FROM_PAYLOAD_TEMPLATE
            .replace("{entity_name}", &entity_name)
            .replace("{update_attribute_from_payload}", &update_attribute_from_payload)
    }

    fn generate_struct_impl(&self, entity_name: &str, entity: &Value) -> String {
        let new_from_payload = self.generate_new_fn(entity_name, entity);
        let update_from_payload = self.generate_update_fn(entity_name, entity);
        ENTITY_IMPL_TEMPLATE
            .replace("{entity_name}", &entity_name)
            .replace("{new_from_payload}", &new_from_payload)
            .replace("{update_from_payload}", &update_from_payload)
    }

    // accessors for all the fields
    fn generate_entity_value_accessors(&self, entity_name: &str, entity: &Value) -> String {
        let mut entity_values = String::new();
        for (field_name, _) in entity.as_object().unwrap().iter() {
            entity_values.push_str(&format!("{}.{}, ", to_snake_case(entity_name), field_name));
        }
        entity_values
    }

    fn generate_enum(&self, name: &str, enum_values: Vec<String>) -> String {
        let mut values = String::new();
        for value in enum_values {
            values.push_str(&ENUM_VALUE_TEMPLATE.replace("{enum_value}", &value));
        }
        ENUM_TEMPLATE
            .replace("{enum_name}", &name)
            .replace("{enum_values}", &values)
    }

    
}