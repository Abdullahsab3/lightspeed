use crate::utils::naming_convention::{to_snake_case, to_plural};

pub static IMPORT_TEMPLATE: &str = r#"
use crate::{import};"#;

pub static IMPORT_MODEL_TEMPLATE: &str = r#"models::{sc_entity_name}::{entity_name}"#;

pub static IMPORT_SOURCE_TEMPLATE: &str = r#"sources::{sc_entity_name}::{entity_plural}Table"#;

pub static IMPORT_SERVICE_TEMPLATE: &str = r#"services::{sc_entity_name}::{entity_plural}Service"#;

pub static IMPORT_CONTROLLER_TEMPLATE: &str = r#"controllers::{sc_entity_name}_controller::*"#;

pub trait ImportGenerator {
    fn generate_model_imports(&self, entity_name: &str) -> String {
        let import = IMPORT_MODEL_TEMPLATE
            .replace("{sc_entity_name}", &to_snake_case(entity_name))
            .replace("{entity_name}", &entity_name);
        IMPORT_TEMPLATE.replace("{import}", &import)
    }

    fn generate_source_imports(&self, entity_name: &str) -> String {
        let import = IMPORT_SOURCE_TEMPLATE
            .replace("{sc_entity_name}", &to_snake_case(entity_name))
            .replace("{entity_plural}", &to_plural(entity_name));
        IMPORT_TEMPLATE.replace("{import}", &import)
    }

    fn generate_service_imports(&self, entity_name: &str) -> String {
        let import = IMPORT_SERVICE_TEMPLATE
            .replace("{sc_entity_name}", &to_snake_case(entity_name))
            .replace("{entity_plural}", &to_plural(entity_name));
        IMPORT_TEMPLATE.replace("{import}", &import)
    }

    fn generate_controller_imports(&self, entity_name: &str) -> String {
        let import = IMPORT_CONTROLLER_TEMPLATE
            .replace("{sc_entity_name}", &to_snake_case(entity_name));
        IMPORT_TEMPLATE.replace("{import}", &import)
    }
}