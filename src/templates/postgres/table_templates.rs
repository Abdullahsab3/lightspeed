use crate::{models::entity::{PostgresAttributeType, Entity}, utils::naming_convention::to_snake_case_plural};


pub static SQL_TABLE_QUERY_TEMPLATE: &str = r#"
CREATE TABLE IF NOT EXISTS {table_name} (
    {attributes}
);
"#;

pub static SQL_INDEX_QUERY_TEMPLATE: &str = r#"
CREATE INDEX IF NOT EXISTS {table_name}_{attribute_name}_index ON {table_name} ({attribute_names});
"#;

pub static SQL_INDEX_QUERY_TEMPLATE_UNIQUE: &str = r#"
CREATE UNIQUE INDEX IF NOT EXISTS unique_{attribute_name}_index ON {table_name} ({attribute_names});
"#;

pub static SQL_ATTRIBUTE_NAMES_TEMPLATE: &str = r#"{attribute_names}"#;

pub static SQL_ATTRIBUTE_TEMPLATE: &str = r#"
    {attribute_name} {attribute_type}"#;

pub trait PostgresTableGenerator {
    fn generate_attribute_names(&self, attributes: Vec<String>) -> String {
        attributes.join(",")
    }

    fn generate_index_queries(&self, entity: &Entity) -> String {
        let table_name = to_snake_case_plural(&entity.name);
        entity.filter_by.iter().map(|attribute_names| {
            let most_specific_attribute_name = attribute_names.last().unwrap();
            let attribute_names = self.generate_attribute_names(attribute_names.clone());
            SQL_INDEX_QUERY_TEMPLATE
                .replace("{table_name}", &table_name)
                .replace("{attribute_name}", &most_specific_attribute_name)
                .replace("{attribute_names}", &attribute_names)
        }).collect::<Vec<String>>().join("\n")
    }

    fn generate_unique_index_queries(&self, entity: &Entity) -> String {
        let table_name = to_snake_case_plural(&entity.name);
        entity.unique_attributes.iter().map(|attribute_names| {
            let most_specific_attribute_name = attribute_names.last().unwrap();
            let attribute_names = self.generate_attribute_names(attribute_names.clone());
            SQL_INDEX_QUERY_TEMPLATE_UNIQUE
                .replace("{table_name}", &table_name)
                .replace("{attribute_name}", &most_specific_attribute_name)
                .replace("{attribute_names}", &attribute_names)
        }).collect::<Vec<String>>().join("\n")
    }
    fn generate_table_query(&self, entity: &Entity) -> String {
        let table_name = to_snake_case_plural(&entity.name);
        let mut attributes = String::new();
        for (attribute_name, attribute_type) in &entity.attributes {
            let postgres_attribute_type: PostgresAttributeType = attribute_type.into();
            // Latest attribute cannot have a comma.
            if entity.is_last(&attribute_name) {
                attributes.push_str(&SQL_ATTRIBUTE_TEMPLATE
                    .replace("{attribute_name}", &attribute_name)
                    .replace("{attribute_type}", &postgres_attribute_type.to_string()));
            } else {
                attributes.push_str(&SQL_ATTRIBUTE_TEMPLATE
                    .replace("{attribute_name}", &attribute_name)
                    .replace("{attribute_type}", (postgres_attribute_type.to_string() + ",").as_str()));
            }

            
        }
        SQL_TABLE_QUERY_TEMPLATE
            .replace("{table_name}", &table_name)
            .replace("{attributes}", &attributes)
        + 
        &self.generate_index_queries(entity)
        +
        &self.generate_unique_index_queries(entity)
    }
}