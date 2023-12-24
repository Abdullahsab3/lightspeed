use serde_json::Value;

use crate::{models::ddr_req::{AttributeType, PostgresAttributeType}, utils::naming_convention::{to_snake_case, to_plural}};


pub static SQL_TABLE_QUERY_TEMPLATE: &str = r#"
CREATE TABLE IF NOT EXISTS {table_name} (
    {attributes}
);
"#;

pub static SQL_ATTRIBUTE_TEMPLATE: &str = r#"
    {attribute_name} {attribute_type}"#;

pub trait PostgresTableGenerator {
    fn generate_table_query(&self, entity_name: String, entity: Value) -> String {
        let plural_entity_name = to_plural(&to_snake_case(&entity_name));
        let mut attributes = String::new();
        for (key, value) in entity.as_object().unwrap() {
            let attribute_type = AttributeType::from_str(value.as_str().unwrap());
            let postgres_attribute_type: PostgresAttributeType = attribute_type.into();
            // Latest attribute cannot have a comma.
            if key == entity.as_object().unwrap().keys().last().unwrap() {
                attributes.push_str(&SQL_ATTRIBUTE_TEMPLATE
                    .replace("{attribute_name}", key)
                    .replace("{attribute_type}", &postgres_attribute_type.to_string()));
            } else {
                attributes.push_str(&SQL_ATTRIBUTE_TEMPLATE
                    .replace("{attribute_name}", key)
                    .replace("{attribute_type}", (postgres_attribute_type.to_string() + ",").as_str()));
            }

            
        }
        SQL_TABLE_QUERY_TEMPLATE
            .replace("{table_name}", &plural_entity_name)
            .replace("{attributes}", &attributes)
    }
}