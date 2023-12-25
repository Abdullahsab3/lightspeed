use project_generators::rust::{RustMicroserviceGeneratorImpl, RustMicroserviceGenerator};

pub mod models;
pub mod templates;
pub mod utils;
pub mod project_generators;

fn main() {
    
    let test_json = r#"
    {
        "service_name": "test",
        "entities": [
            {
                "Person": {
                    "id": "Uuid",
                    "name": "String",
                    "age": "Option<i32>",
                    "height": "f32",
                    "is_cool": "bool"
                },
                "Car": {
                    "id": "Uuid",
                    "make": "String",
                    "model": "String",
                    "year": "i32",
                    "is_fast": "bool"
                }
            }
        ]
    }
    "#;
    let ddr: models::ddr_req::DomainDrivenRequest = serde_json::from_str(test_json).unwrap();
    let rust_microservice_generator = RustMicroserviceGeneratorImpl {};
    rust_microservice_generator.generate_rust_microservice(ddr, "/Users/abdullahsabaaallil/test/generated_microservice").unwrap();
    
}
