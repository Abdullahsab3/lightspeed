pub mod models;

fn main() {
    let test_json = r#"
    {
        "service_name": "test",
        "entities": [
            {
                "Person": {
                    "name": "String",
                    "age": "i32",
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
}