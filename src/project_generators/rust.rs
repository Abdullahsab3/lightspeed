use std::{path::Path, io, fs};

use chrono::Utc;

use crate::{models::ddr_req::DomainDrivenRequest, utils::naming_convention::{to_snake_case_plural, to_snake_case}};

use super::file_generator::FileGenerator;

pub static CONFIG_TOML_PATH: &str = ".cargo/config.toml";
pub static CARGO_TOML_PATH: &str = "Cargo.toml";
pub static CONTROLLERS_DIR: &str = "src/controllers";
pub static MODELS_DIR: &str = "src/models";
pub static HTTP_PATH: &str = "src/http/mod.rs";
pub static SERVICES_DIR: &str = "src/services";
pub static DATABASE_CONFIG_PATH: &str = "docker/postgres/01.sql";
pub static MIGRATIONS_DIR: &str = "migrations";
pub static STATIC_TEMPLATES_DIR: &str = "./static_templates";
pub static RUST_STATIC_TEMPLATE_DIR: &str = "/rust/microservice";

pub trait RustMicroserviceGenerator: FileGenerator {
    fn generate_rust_microservice(&self, domain_driven_request: DomainDrivenRequest, out_dir: &str) -> io::Result<()>;
}

pub struct RustMicroserviceGeneratorImpl {}
impl FileGenerator for RustMicroserviceGeneratorImpl {}

impl RustMicroserviceGenerator for RustMicroserviceGeneratorImpl {
    fn generate_rust_microservice(&self, domain_driven_request: DomainDrivenRequest, out_dir: &str) -> io::Result<()> {
        fs::create_dir_all(out_dir)?;
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
        //let config_toml_static_template_path = rust_static_template_path.join(CONFIG_TOML_PATH);
        let config_toml_dynamic_template = domain_driven_request.generate_environment_definitions();
        self.generate_file(String::new(), config_toml_dynamic_template, &format!("{}/{}", out_dir, CONFIG_TOML_PATH))?;

        /*
         * Generate database config
         */
        let database_dynamic_template = domain_driven_request.generate_database_config();
        self.generate_file(String::new(), database_dynamic_template, &format!("{}/{}", out_dir, DATABASE_CONFIG_PATH))?;

        /*
         * Generate migrations
         */
        let migrations_dynamic_template = domain_driven_request.generate_postgres_tables();
        for (entity_name, migration) in migrations_dynamic_template {
            let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
            let migration_path = format!("{}_{}.sql", timestamp, to_snake_case_plural(&entity_name));
            self.generate_file(String::new(), migration, &format!("{}/{}/{}", out_dir, MIGRATIONS_DIR, migration_path))?;
        }

        /*
         * Generate controllers
         */
        let controller_mods_dynamic_template = domain_driven_request.generate_controller_mods();
        self.generate_file(String::new(), controller_mods_dynamic_template, &format!("{}/{}/mod.rs", out_dir, CONTROLLERS_DIR))?;
        let controllers_dynamic_templates = domain_driven_request.generate_controllers();
        for (entity_name, controller) in controllers_dynamic_templates {
            let controller_path = format!("{}_controller.rs", to_snake_case_plural(&entity_name));
            self.generate_file(String::new(), controller, &format!("{}/{}/{}", out_dir, CONTROLLERS_DIR, controller_path))?;
        }

        /*
         * Generate http
         */
        let http_static_template = rust_static_template_path.join(HTTP_PATH);
        let http_static_template = std::fs::read_to_string(http_static_template)?;
        let http_dynamic_template = domain_driven_request.generate_http();
        self.generate_file(http_static_template, http_dynamic_template, &format!("{}/{}", out_dir, HTTP_PATH))?;

        /*
         * Generate models
         */
        let models_mods_dynamic_template = domain_driven_request.generate_model_mods();
        self.generate_file(String::new(), models_mods_dynamic_template, &format!("{}/{}/mod.rs", out_dir, MODELS_DIR))?;
        let models_dynamic_templates = domain_driven_request.generate_models();
        for (entity_name, model) in models_dynamic_templates {
            let model_path = format!("{}.rs", to_snake_case(&entity_name));
            self.generate_file(String::new(), model, &format!("{}/{}/{}", out_dir, MODELS_DIR, model_path))?;
        }

        /*
         * Generate services
         */
        let services_mods_dynamic_template = domain_driven_request.generate_service_mods();
        self.generate_file(String::new(), services_mods_dynamic_template, &format!("{}/{}/mod.rs", out_dir, SERVICES_DIR))?;
        let services_dynamic_templates = domain_driven_request.generate_services();
        for (entity_name, service) in services_dynamic_templates {
            let service_path = format!("{}_service.rs", to_snake_case(&entity_name));
            self.generate_file(String::new(), service, &format!("{}/{}/{}", out_dir, SERVICES_DIR, service_path))?;
        }

        /*
         * Generate errors
         */
        let errors_dynamic_template = domain_driven_request.generate_error();
        self.generate_file(String::new(), errors_dynamic_template, &format!("{}/src/errors.rs", out_dir))?;

        /*
         * Generate Docker compose
         */
        let docker_compose_dynamic_template = domain_driven_request.generate_docker_compose();
        self.generate_file(String::new(), docker_compose_dynamic_template, &format!("{}/docker-compose.yml", out_dir))?;
        
        Ok(())

    }


}
