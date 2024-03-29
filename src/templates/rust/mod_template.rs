use crate::{models::entity::Entity, utils::naming_convention::to_snake_case};

pub static MOD_TEMPLATE: &str = r#"
pub mod {module_name};
"#;

pub static CONTROLLER_MOD_TEMPLATE: &str = r#"{sc_plural_entity}_controller"#;

pub static MODEL_MOD_TEMPLATE: &str = r#"{sc_entity_name}"#;

pub static SERVICE_MOD_TEMPLATE: &str = r#"{sc_entity_name}_service"#;

pub static SOURCE_MOD_TEMPLATE: &str = r#"{sc_entity_name}_table"#;

pub trait ModGenerator {
    fn generate_controller_mod(&self, entity: &Entity) -> String {
        let module_name = CONTROLLER_MOD_TEMPLATE.replace("{sc_plural_entity}", &to_snake_case(&entity.plural_name));
        MOD_TEMPLATE.replace("{module_name}", &module_name)
    }

    fn generate_model_mod(&self, entity: &Entity) -> String {
        let module_name = MODEL_MOD_TEMPLATE.replace("{sc_entity_name}", &to_snake_case(&entity.name));
        MOD_TEMPLATE.replace("{module_name}", &module_name)
    }
    
    fn generate_service_mod(&self, entity: &Entity) -> String {
        let module_name = SERVICE_MOD_TEMPLATE.replace("{sc_entity_name}", &to_snake_case(&entity.plural_name));
        MOD_TEMPLATE.replace("{module_name}", &module_name)
    }

    fn generate_source_mod(&self, entity: &Entity) -> String {
        let module_name = SOURCE_MOD_TEMPLATE.replace("{sc_entity_name}", &to_snake_case(&entity.plural_name));
        MOD_TEMPLATE.replace("{module_name}", &module_name)
    }
}