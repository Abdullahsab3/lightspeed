use std::{path::Path, io};

use crate::models::ddr_req::DomainDrivenRequest;

use super::file_generator::FileGenerator;

pub static CONFIG_TOML_PATH: &str = ".cargo/config.toml";
pub static CARGO_TOML_PATH: &str = "Cargo.toml";
pub static SOURCE_DIR: &str = "src";
pub static CONTROLLERS_DIR: &str = "controllers";
pub static MODELS_DIR: &str = "models";
pub static SERVICES_DIR: &str = "services";

pub static STATIC_TEMPLATES_DIR: &str = "./static_templates";
pub static RUST_STATIC_TEMPLATE_DIR: &str = "/rust/microservice";

pub trait RustMicroserviceGenerator: FileGenerator {
    fn generate_rust_microservice(&self, domain_driven_request: DomainDrivenRequest, out_dir: &str) -> io::Result<()> {
        let rust_static_template_dir = format!("{}{}", STATIC_TEMPLATES_DIR, RUST_STATIC_TEMPLATE_DIR);
        let rust_static_template_path = Path::new(&rust_static_template_dir);

        /*
         * Generate Cargo.toml
         */
        let cargo_toml_static_template_path = rust_static_template_path.join(CARGO_TOML_PATH);
        let cargo_toml_static_template = std::fs::read_to_string(cargo_toml_static_template_path)?;
        let cargo_toml_dynamic_template = domain_driven_request.generate_cargo_toml();
        self.generate_file(cargo_toml_static_template, cargo_toml_dynamic_template, &format!("{}/{}", out_dir, CARGO_TOML_PATH))?;

        /*
         * Generate config.toml
         */
        let config_toml_static_template_path = rust_static_template_path.join(CONFIG_TOML_PATH);
        let config_toml_static_template = std::fs::read_to_string(config_toml_static_template_path)?;
        let config_toml_dynamic_template = domain_driven_request.generate_environment_definitions();
        self.generate_file(config_toml_static_template, config_toml_dynamic_template, &format!("{}/{}", out_dir, CONFIG_TOML_PATH))?;


        Ok(())

    }
}