pub static DATABASE_CREATE_TEMPLATE: &str = r#"
CREATE DATABASE IF NOT EXISTS {sc_service_name};
"#;

pub static DATABASE_DROP_TEMPLATE: &str = r#"
DROP DATABASE IF EXISTS {sc_service_name};
"#;

pub trait DatabaseGenerator {
    fn generate_database_create(&self, service_name: &str) -> String {
        DATABASE_CREATE_TEMPLATE.replace("{sc_service_name}", &service_name.to_lowercase())
    }

    fn generate_database_drop(&self, service_name: &str) -> String {
        DATABASE_DROP_TEMPLATE.replace("{sc_service_name}", &service_name.to_lowercase())
    }
}