use serde_json::Value;

use crate::{models::ddr_req::AttributeType, utils::naming_convention::to_snake_case};

pub static STRUCT_TEMPLATE: &str = r#"
#[derive(Serialize, Deserialize)]
pub struct {struct_name} {
    {attributes}
}
"#;

pub static ATTRIBUTE_TEMPLATE: &str = r#"
    pub {attribute_name}: {attribute_type},"#;



pub trait ModelGenerator {
    fn generate_struct(&self, name: &str, value: Value) -> String {
        let mut attributes = String::new();
        for (key, value) in value.as_object().unwrap() {
            let attribute_type = AttributeType::from_str(value.as_str().unwrap());
            attributes.push_str(&ATTRIBUTE_TEMPLATE
                .replace("{attribute_name}", key)
                .replace("{attribute_type}", &attribute_type.to_string()));
        }
        STRUCT_TEMPLATE
            .replace("{struct_name}", &name)
            .replace("{attributes}", &attributes)
    }

    // accessors for all the fields
    fn generate_entity_value_accessors(&self, entity_name: &str, entity: &Value) -> String {
        let mut entity_values = String::new();
        for (field_name, _) in entity.as_object().unwrap().iter() {
            entity_values.push_str(&format!("{}.{}, ", to_snake_case(entity_name), field_name));
        }
        entity_values

    }
}