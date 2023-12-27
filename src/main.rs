use project_generators::rust::RustMicroserviceGeneratorImpl;
use crate::project_generators::rust::RustMicroserviceGenerator;


pub mod models;
pub mod templates;
pub mod utils;
pub mod project_generators;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Lightspeed")
    .version("0.0.1")
    .author("Abdullah Sabaa Allil")
    .about("Generate a service from your DDD!")
    .arg(Arg::new("input")
                .short('i')
                .long("input")
                .help("The input file, containing the JSON representation of the DDD"))
    .arg(Arg::new("output")
                .short('o')
                .long("output")
                .help("The output directory, where the generated service will be placed"))
    .get_matches();
    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output").unwrap();

    let json = std::fs::read_to_string(input).unwrap();
    let raw_ddr: models::ddr_req::RawDomainDrivenRequest = serde_json::from_str(&json).unwrap();
    let ddr = models::ddr_req::DomainDrivenRequest::from(raw_ddr);
    let rust_microservice_generator = RustMicroserviceGeneratorImpl {};
    rust_microservice_generator.generate_rust_microservice(ddr, output).unwrap()
    
}
