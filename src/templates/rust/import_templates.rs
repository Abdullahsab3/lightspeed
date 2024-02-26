use crate::{models::entity::Entity, utils::naming_convention::to_snake_case};

pub static IMPORT_TEMPLATE: &str = r#"
use crate::{import};"#;

pub static IMPORT_MODEL_TEMPLATE: &str = r#"models::{sc_entity_name}::*"#;

pub static IMPORT_SOURCE_TEMPLATE: &str = r#"sources::{sc_entity_name_plural}_table::{entity_plural}Table"#;

pub static IMPORT_SERVICE_TEMPLATE: &str = r#"services::{sc_entity_name_plural}_service::{entity_plural}Service"#;

pub static IMPORT_CONTROLLER_TEMPLATE: &str = r#"controllers::{sc_plural_entity}_controller::*"#;

pub static ERROR_IMPORT_TEMPLATE: &str = r#"use crate::error::Error;"#;

pub trait ImportGenerator {
    fn generate_model_imports(&self, entity: &Entity) -> String {
        let import = IMPORT_MODEL_TEMPLATE
            .replace("{sc_entity_name}", &to_snake_case(&entity.name));
        IMPORT_TEMPLATE.replace("{import}", &import)
    }

    fn generate_source_imports(&self, entity: &Entity) -> String {
        let import = IMPORT_SOURCE_TEMPLATE
            .replace("{sc_entity_name_plural}", &to_snake_case(&entity.plural_name))
            .replace("{entity_plural}", &entity.plural_name);
        IMPORT_TEMPLATE.replace("{import}", &import)
    }

    fn generate_service_imports(&self, entity: &Entity) -> String {
        let import = IMPORT_SERVICE_TEMPLATE
            .replace("{sc_entity_name_plural}", &to_snake_case(&entity.plural_name))
            .replace("{entity_plural}", &entity.plural_name);
        IMPORT_TEMPLATE.replace("{import}", &import)
    }

    fn generate_controller_imports(&self, entity: &Entity) -> String {
        let import = IMPORT_CONTROLLER_TEMPLATE
            .replace("{sc_plural_entity}", &to_snake_case(&entity.plural_name));
        IMPORT_TEMPLATE.replace("{import}", &import)
    }

    fn generate_error_imports(&self) -> String {
        ERROR_IMPORT_TEMPLATE.to_string()
    }
}