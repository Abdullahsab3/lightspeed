use serde_json::Value;

use crate::utils::naming_convention::to_table_name;

pub static CREATE_ENTITY_QUERY: &str = r#"
            INSERT INTO {entity_name}
                ({entity_fields})
            VALUES
                ({entity_values})
            RETURNING *;
"#;

pub static UPDATE_ENTITY_QUERY: &str = r#"
            UPDATE {entity_name}
            SET 
                {entity_fields}
            WHERE id = {entity_id}
            RETURNING *;
"#;

pub static DELETE_ENTITY_QUERY: &str = r#"
            DELETE FROM {entity_name}
            WHERE id = {entity_id}
            RETURNING *;
"#;

pub trait CrudQueryGenerator {
    fn generate_create_query(&self, entity_name: &str, entity_description: &Value) -> String {
        let table_name = to_table_name(entity_name);
        let mut entity_fields = Vec::new();
        let mut entity_values = Vec::new();
        for (arg_num, (field_name, _)) in entity_description.as_object().unwrap().iter().enumerate() {
            entity_fields.push(format!("{}", field_name));
            entity_values.push(format!("${}", arg_num + 1));
        }
        let entity_fields = entity_fields.join(", ");
        let entity_values = entity_values.join(", ");
        CREATE_ENTITY_QUERY
            .replace("{entity_name}", &table_name)
            .replace("{entity_fields}", &entity_fields)
            .replace("{entity_values}", &entity_values)
    }
    fn generate_update_query(&self, entity_name: &str, entity_description: &Value) -> String {
        let table_name = to_table_name(entity_name);
        let mut entity_fields = Vec::new();
        for (arg_num, (field_name, _)) in entity_description.as_object().unwrap().iter().enumerate() {
            entity_fields.push(format!("{} = ${}", field_name, arg_num + 1 ));
        }
        let entity_fields = entity_fields.join(", ");
        UPDATE_ENTITY_QUERY
            .replace("{entity_name}", &table_name)
            .replace("{entity_fields}", &entity_fields)
            .replace("{entity_id}", &format!("${}", entity_description.as_object().unwrap().len() + 1))
    }
    fn generate_delete_query(&self, entity_name: &str) -> String {
        let table_name = to_table_name(entity_name);
        DELETE_ENTITY_QUERY
            .replace("{entity_name}", &table_name)
            .replace("{entity_id}", &format!("${}", 1))
    }
}