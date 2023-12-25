use project_generators::rust::{RustMicroserviceGeneratorImpl, RustMicroserviceGenerator};

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
    let ddr: models::ddr_req::DomainDrivenRequest = serde_json::from_str(test_json).unwrap();
    let entities: Vec<models::entity::Entity> = ddr.generate_entities();
    for entitiy in entities {
        println!("{:#?}", entitiy);
    }
    
}
