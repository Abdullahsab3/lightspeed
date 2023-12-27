use crate::{models::entity::{Entity, AttributeType}, utils::naming_convention::{to_snake_case, to_plural}};

use super::import_templates::ImportGenerator;

pub static STRUCT_TEMPLATE: &str = r#"
#[derive(Serialize, Deserialize, Debug)]
pub struct {struct_name} {
    {attributes}
}
"#;

pub static NEW_FROM_PAYLOAD_TEMPLATE: &str = r#"
    pub fn new(payload: Add{entity_name}Payload) -> Result<Self, Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            {new_attribute_from_payload}
        })
    }
"#;

pub static NEW_ATTRIBUTE_FROM_PAYLOAD: &str = r#"
            {attribute_name}: payload.{attribute_name},"#;

pub static UPDATE_FROM_PAYLOAD_TEMPLATE: &str = r#"
    pub fn update(self, payload: Update{entity_name}Payload) -> Result<Self, Error> {
        Ok(Self {
            id: self.id,
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

pub static FILTER_PARAMS_TEMPLATE: &str = r#"
#[derive(Deserialize)]
pub struct {entity_name}FilterParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    {attributes}
}
"#;

pub static FILTER_IMPL_TEMPLATE: &str = r#"
impl {entity_name}FilterParams {
    {is_filter_functions}
}
"#;
pub static IS_FILTER_FN: &str = r#"
    pub fn is_{attribute_name}_filter(&self) -> bool {{
       {check_if_attribute_is_not_null}
    }}
"#;

pub static CHECK_IF_ATTRIBUTE_IS_NOT_NULL: &str = r#"
        self.{attribute_name}.is_some()"#;


pub static ATTRIBUTE_TEMPLATE: &str = r#"
    pub {attribute_name}: {attribute_type},"#;

pub static ENUM_TEMPLATE: &str = r#"
#[derive(Serialize, Deserialize, Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum {enum_name} {
    {enum_values}
}
"#;

pub static RESPONSE_ENUM_TEMPLATE: &str = r#"
#[derive(Deserialize, Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum {enum_name} {
    {enum_values}
}
"#;


pub static SERIALIZE_ENUM_TEMPLATE: &str = r#"
impl Serialize for {enum_name} {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            {enum_match_values}
        }
    }
}
"#;

pub static SINGLE_ENUM: &str = r#"{entity_name}({entity_name})"#;
pub static PAGINATED_ENUM: &str = r#"{entity_plural}(PaginatedResult<{entity_name}>)"#;
pub static ENUM_MATCH_VALUES: &str = r#"
                {enum_name}::{enum_value}({sc_entity_name}) => {sc_entity_name}.serialize(serializer),"#;

pub static ENUM_VALUE_TEMPLATE: &str = r#"
    {enum_value},"#;

pub static MODEL_FILE_TEMPLATE: &str = r#"
use serde::Serialize;
use serde::Deserialize;
use uuid::Uuid;
use crate::error::Error;
use crate::models::PaginatedResult;
use serde::Serializer;

{imports}


{model_definition}
{model_impl}

{filter_definition}
{filter_impl}

{response_definition}
{response_impl}


"#;

pub trait ModelGenerator: ImportGenerator {

    fn generate_model(&self, entity: &Entity) -> String {
        let model_definition = self.generate_struct(entity);
        let model_impl = self.generate_struct_impl(entity);
        MODEL_FILE_TEMPLATE
            .replace("{imports}", &self.generate_controller_imports(&entity.name))
            .replace("{model_definition}", &model_definition)
            .replace("{model_impl}", &model_impl)
            .replace("{filter_definition}", &self.generate_filter_params(&entity))
            .replace("{filter_impl}", &self.generate_filter_params_impl(&entity))
            .replace("{response_definition}", &self.generate_response_enum(&entity))
            .replace("{response_impl}", &self.generate_response_serialize_impl(&entity))
            
    }
    fn generate_struct(&self, entity: &Entity) -> String {
        let mut attributes = String::new();
        for (key, value) in &entity.attributes {
            let attribute_type = AttributeType::from_str(&value.to_string());
            attributes.push_str(&ATTRIBUTE_TEMPLATE
                .replace("{attribute_name}", &key)
                .replace("{attribute_type}", &attribute_type.to_string()));
        }
        STRUCT_TEMPLATE
            .replace("{struct_name}", &entity.name)
            .replace("{attributes}", &attributes)
    }

    fn generate_new_fn(&self, entity: &Entity) -> String {
        let mut new_attribute_from_payload = String::new();
        for (key, value) in &entity.attributes {
            if key == &entity.primary_key {
                continue;
            }
            new_attribute_from_payload.push_str(&NEW_ATTRIBUTE_FROM_PAYLOAD
                .replace("{attribute_name}", &key)
                .replace("{attribute_type}", &value.to_string()));
        }
        NEW_FROM_PAYLOAD_TEMPLATE
            .replace("{entity_name}", &entity.name)
            .replace("{new_attribute_from_payload}", &new_attribute_from_payload)
    }

    fn generate_update_fn(&self, entity: &Entity) -> String {
        let mut update_attribute_from_payload = String::new();
        for (key, value) in &entity.attributes {
            if key == &entity.primary_key {
                continue;
            }
            let attribute_type = AttributeType::from_str(&value.to_string());
            let attribute_type_str = match &attribute_type {
                AttributeType::Option(t) => t.to_string(),
                _ => attribute_type.to_string()
            };
            match attribute_type {
                AttributeType::Option(_) => {
                    update_attribute_from_payload.push_str(&UPDATE_ATTRIBUTE_FROM_PAYLOAD_NULLABLE
                        .replace("{attribute_name}", &key)
                        .replace("{attribute_type}", &attribute_type_str));
                },
                _ => {
                    update_attribute_from_payload.push_str(&UPDATE_ATTRIBUTE_FROM_PAYLOAD
                        .replace("{attribute_name}", &key)
                        .replace("{attribute_type}", &attribute_type_str));
                }
            }
        }
        UPDATE_FROM_PAYLOAD_TEMPLATE
            .replace("{entity_name}", &entity.name)
            .replace("{update_attribute_from_payload}", &update_attribute_from_payload)
    }

    fn generate_struct_impl(&self, entity: &Entity) -> String {
        let new_from_payload = self.generate_new_fn(entity);
        let update_from_payload = self.generate_update_fn(entity);
        ENTITY_IMPL_TEMPLATE
            .replace("{entity_name}", &entity.name)
            .replace("{new_from_payload}", &new_from_payload)
            .replace("{update_from_payload}", &update_from_payload)
    }

    // accessors for all the fields
    fn generate_entity_value_accessors(&self, entity: &Entity) -> String {
        let mut entity_values = String::new();
        for (field_name, _) in &entity.attributes {
            entity_values.push_str(&format!("{}.{}, ", to_snake_case(&entity.name), field_name));
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

    fn generate_response_enum(&self, entity: &Entity) -> String {
        let enum_values = vec![
            SINGLE_ENUM
            .replace("{entity_name}", &entity.name), 
            PAGINATED_ENUM
            .replace("{entity_name}", &entity.name)
            .replace("{entity_plural}", &to_plural(&entity.name))];
        let enum_name = format!("{}Response", &entity.name);
        RESPONSE_ENUM_TEMPLATE
            .replace("{enum_name}", &enum_name)
            .replace("{enum_values}", &enum_values.join(",\n"))
    }

    fn generate_response_serialize_impl(&self, entity: &Entity) -> String {
        let enum_values = vec![
            entity.name.to_string(), 
           to_plural(&entity.name)];

        let enum_match_values = enum_values.iter().map(|enum_value| ENUM_MATCH_VALUES
            .replace("{enum_name}", &format!("{}Response", &entity.name))
            .replace("{enum_value}", enum_value)
            .replace("{sc_entity_name}", &to_snake_case(&entity.name)))
            .collect::<Vec<String>>()
            .join("");
        SERIALIZE_ENUM_TEMPLATE
            .replace("{enum_name}", &format!("{}Response", &entity.name))
            .replace("{enum_match_values}", &enum_match_values)
    }

    /**
     * Idea: One struct that contains all the filter attributes
     * The struct has an impl that consists of functions to check which type of filter is applied
     * by checking which options are not null
     * 
     * In the controllers: Check which filter you have, starting from the most specific (ie the longest filter_by vector)
     */
    fn generate_filter_params(&self, entity: &Entity) -> String {
        let filter_attributes = entity.filter_by.iter().fold(entity.filter_by.first().unwrap().to_owned(), |attributes, filter_by| {
            let mut attributes = attributes.clone();
            for attribute in filter_by {
                if !attributes.contains(attribute) {
                    attributes.push(attribute.clone());
                }
            }
            attributes
        }).iter().map(|attribute_name| {
            let attribute_type = &entity.attributes.iter().find(|(key, _)| key == attribute_name).unwrap().1;
            let attribute_type_str = 
            "Option<".to_string()
            +
            match attribute_type {
                AttributeType::Option(t) => t.to_string(),
                _ => attribute_type.to_string()
            }
            .as_str()
            +
            ">";
            ATTRIBUTE_TEMPLATE
            .replace("{attribute_name}", &attribute_name)
            .replace("{attribute_type}", &attribute_type_str)
    })
            .collect::<Vec<String>>()
            .join("");

        FILTER_PARAMS_TEMPLATE
            .replace("{entity_name}", &entity.name)
            .replace("{attributes}", filter_attributes.as_str())
    }

    fn generate_filter_params_impl(&self, entity: &Entity) -> String {
        let mut is_filter_functions = String::new();
        for filter_by in &entity.filter_by {
            let most_specific_attribute = filter_by.last().unwrap();
            let check_if_attributes_are_not_null = filter_by
                .iter()
                .map(|attribute_name| CHECK_IF_ATTRIBUTE_IS_NOT_NULL
                    .replace("{attribute_name}", &attribute_name))
                .collect::<Vec<String>>()
                .join(" && ");
            is_filter_functions.push_str(&IS_FILTER_FN
                .replace("{attribute_name}", &most_specific_attribute)
                .replace("{check_if_attribute_is_not_null}", &check_if_attributes_are_not_null));
        }
        FILTER_IMPL_TEMPLATE
            .replace("{entity_name}", &entity.name)
            .replace("{is_filter_functions}", &is_filter_functions)
    }

    
}