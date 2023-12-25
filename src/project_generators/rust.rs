use std::{path::Path, io};

use crate::models::ddr_req::DomainDrivenRequest;

pub static CONFIG_TOML_PATH: &str = ".cargo/config.toml";
pub static CARGO_TOML_PATH: &str = "Cargo.toml";
pub static SOURCE_DIR: &str = "src";
pub static CONTROLLERS_DIR: &str = "controllers";
pub static MODELS_DIR: &str = "models";
pub static SERVICES_DIR: &str = "services";




pub trait RustMicroserviceGenerator {
    fn generate_rust_microservice(&self, domain_driven_request: DomainDrivenRequest, static_template_dir: &str, out_dir: &str) -> io::Result<()> {
        let static_template_path = Path::new(static_template_dir);


        Ok(())

    }
}