use crate::utils::naming_convention::kebab_to_snake_case;

pub static CARGO_TOML_PACKAGE_TEMPLATE: &str = r#"
[package]
name = "{service_name}"
version = "0.1.0"
edition = "2021"
default-run = "{service_name}
"#;

pub static CARGO_TOML_BIN_TEMPLATE: &str = r#"
[bin]
name = "{service_name}"
path = "src/main.rs"
"#;

pub static CONFIG_TOML_TEMPLATE: &str = r#"
[env]
RUST_LOG = "info"
DATABASE_URL = "postgres://postgres:postgres@localhost:5432/{sc_service_name}"
DATABASE_CONNECTION_RETRIES = "10"
"#;

pub trait ProjectConfigGenerator {
    fn generate_cargo_toml_package(&self, service_name: &str) -> String {
        CARGO_TOML_PACKAGE_TEMPLATE.replace("{service_name}", service_name)
    }

    fn generate_cargo_toml_bin(&self, service_name: &str) -> String {
        CARGO_TOML_BIN_TEMPLATE.replace("{service_name}", service_name)
    }

    fn generate_config_toml(&self, service_name: &str) -> String {
        CONFIG_TOML_TEMPLATE.replace("{sc_service_name}", &kebab_to_snake_case(service_name))
    }
}