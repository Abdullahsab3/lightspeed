pub mod models;
pub mod templates;
pub mod utils;

fn main() {
    let test_json = r#"
    {
        "service_name": "test",
        "entities": [
            {
                "Person": {
                    "name": "String",
                    "age": "Option<i32>",
                    "height": "f32",
                    "is_cool": "bool"
                },
                "Car": {
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
    let models = ddr.generate_models();
    for model in models {
        println!("{}", model);
    }
    let tables = ddr.generate_postgres_tables();
    for table in tables {
        println!("{}", table);
    }
    let controllers = ddr.generate_controllers();
    for controller in controllers {
        println!("{}", controller);
    }
    let payloads = ddr.generate_payloads();
    for payload in payloads {
        println!("{}", payload);
    }
    
}
