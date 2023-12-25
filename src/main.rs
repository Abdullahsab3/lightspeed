use project_generators::rust::RustMicroserviceGeneratorImpl;
use crate::project_generators::rust::RustMicroserviceGenerator;


pub mod models;
pub mod templates;
pub mod utils;
pub mod project_generators;

fn main() {
    
    let test_json = r#"
    {
        "service_name": "MyService",
        "entities": [
            {
                "User" : 
                {
                    "id": "Uuid",
                    "name" : "String",
                    "surname": "String",
                    "age": "Int",
                    "email": "String",
                    "primary_key": "id",
                    "filter_by": ["name", "age", ["name", "surname"]] ,
                    "unique_attributes": ["email"]
                }
            },
            {
                "Car" : 
                {
                    "id": "Uuid",
                    "name" : "String",
                    "brand": "String",
                    "price": "Int",
                    "ownedBy": "User.id",
                    "primary_key": "id",
                    "filter_by": ["name", "brand"]
                }
            }
        ]
    }
    "#;
    let raw_ddr: models::ddr_req::RawDomainDrivenRequest = serde_json::from_str(test_json).unwrap();
    let ddr = models::ddr_req::DomainDrivenRequest::from(raw_ddr);
    let rust_microservice_generator = RustMicroserviceGeneratorImpl {};
    rust_microservice_generator.generate_rust_microservice(ddr, "/Users/abdullahsabaaallil/test/generated_microservice").unwrap()
    
}
