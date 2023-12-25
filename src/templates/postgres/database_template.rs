use crate::utils::naming_convention::to_snake_case;

pub static DATABASE_CREATE_TEMPLATE: &str = r#"
CREATE DATABASE "{sc_service_name}";
"#;

pub static DATABASE_DROP_TEMPLATE: &str = r#"
DROP DATABASE IF EXISTS "{sc_service_name}";
"#;

pub trait DatabaseGenerator {
    fn generate_database_create(&self, service_name: &str) -> String {
        DATABASE_CREATE_TEMPLATE.replace("{sc_service_name}", &to_snake_case(service_name))
    }

    fn generate_database_drop(&self, service_name: &str) -> String {
        DATABASE_DROP_TEMPLATE.replace("{sc_service_name}", &to_snake_case(service_name))
    }
}