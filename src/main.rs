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
    let input = matches.get_one::<String>("input").expect("You must provide an input file");
    let output = matches.get_one::<String>("output").expect("You must provide an output directory");

    let json = std::fs::read_to_string(input).expect("Could not read the input file");
    let raw_ddr: models::ddr_req::RawDomainDrivenRequest = serde_json::from_str(&json).expect("Could not parse the input file");
    let ddr = models::ddr_req::DomainDrivenRequest::from(raw_ddr);
   // println!("{:#?}", ddr);
    let rust_microservice_generator = RustMicroserviceGeneratorImpl {};
    rust_microservice_generator.generate_rust_microservice(ddr, output).expect("Could not generate the service");
     

/*      let input_string = "a# 5 + 3 #a you";

     match parse_dynamic_expression(input_string) {
         Ok((remaining, expression)) => {
             println!("Found expression: {}", expression);
             println!("Remaining: {}", remaining);
         }
         Err(err) => {
             println!("Error parsing expression: {:?}", err);
         }
     }
     */
}

use nom::{
    character::complete::{char, multispace0, multispace1, alphanumeric1},
    sequence::{delimited, tuple},
    IResult, complete::tag, bytes::complete::{take_while, tag_no_case, is_not},
};

fn parse_dynamic_expression(input: &str) -> IResult<&str, &str> {
    delimited(
        tag_no_case("a#"),
        is_not("#a"),
        tag_no_case("#a"),
    )(input)
}


pub struct AnonymousExpression {
    pub file_path: String,
    pub expression: String,
    pub used_variables: Vec<String>,
}

pub struct NamedExpression {
    pub name: String,
    pub file_path: String,
    pub expression: String,
    pub used_variables: Vec<String>,
}

pub enum DynamicExpression {
    Anonymous(AnonymousExpression),
    Named(NamedExpression),
}