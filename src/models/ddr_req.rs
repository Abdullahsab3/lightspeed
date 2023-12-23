use serde::{Serialize, Deserialize};
use serde_json::Value;


#[derive(Serialize, Deserialize)]
pub struct DomainDrivenRequest {
    pub service_name: String,
    pub entities: Value,
}

impl DomainDrivenRequest {
    pub fn generate_models(&self) -> Vec<String> {
        let mut models = Vec::new();
        // extract entities in key value pairs
        for(entity_name, entity_description) in self.entities.as_array().unwrap().iter().flat_map(|x| x.as_object().unwrap())  {
            println!("{}: {}", entity_name, entity_description);
            let model = self.generate_struct(entity_name.to_string(), entity_description.clone());
            models.push(model);
        }
        models
    }

}

impl ModelGenerator for DomainDrivenRequest {}

pub trait ModelGenerator {
    fn generate_struct(&self, name: String, value: Value) -> String {
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
}

enum AttributeType {
    String,
    Uuid,
    I32,
    I64,
    F32,
    F64,
    Boolean,
    UTCDateTime,
}

impl AttributeType {
    pub fn from_str(s: &str) -> AttributeType {
        match s {
            "String" => AttributeType::String,
            "Uuid" => AttributeType::Uuid,
            "i32" => AttributeType::I32,
            "i64" => AttributeType::I64,
            "f32" => AttributeType::F32,
            "f64" => AttributeType::F64,
            "bool" => AttributeType::Boolean,
            "UTCDateTime" => AttributeType::UTCDateTime,
            _ => panic!("Unknown attribute type: {}", s),
        }
    }
}

impl ToString for AttributeType {
    fn to_string(&self) -> String {
        match self {
            AttributeType::String => "String".to_string(),
            AttributeType::Uuid => "Uuid".to_string(),
            AttributeType::I32 => "i32".to_string(),
            AttributeType::I64 => "i64".to_string(),
            AttributeType::F32 => "f32".to_string(),
            AttributeType::F64 => "f64".to_string(),
            AttributeType::Boolean => "bool".to_string(),
            AttributeType::UTCDateTime => "UTCDateTime".to_string(),
        }
    }
}

static STRUCT_TEMPLATE: &str = r#"
#[derive(Serialize, Deserialize)]
pub struct {struct_name} {
    {attributes}
}
"#;

static ATTRIBUTE_TEMPLATE: &str = r#"
    pub {attribute_name}: {attribute_type},
"#;