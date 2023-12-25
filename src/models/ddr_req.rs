use serde::{Serialize, Deserialize};
use serde_json::Value;


use crate::templates::{rust::{controller_templates::ControllerGenerator, model_templates::ModelGenerator, source_templates::SourceGenerator, service_templates::ServiceGenerator, axum_routes_templates::AxumRoutesGenerator, error_templates::ErrorGenerator, import_templates::ImportGenerator, project_config_templates::ProjectConfigGenerator, mod_template::ModGenerator}, postgres::{table_templates::PostgresTableGenerator, crud_query_templates::CrudQueryGenerator, database_template::DatabaseGenerator}, docker::docker_compose::DockerComposeGenerator};

use super::entity::{Entity, EntityName};



#[derive(Serialize, Deserialize)]
pub struct RawDomainDrivenRequest {
    pub service_name: String,
    pub entities: Value,
}

impl RawDomainDrivenRequest {
    pub fn generate_entities(&self) -> Vec<Entity> {
        let mut entities = Vec::new();
        // extract entities in key value pairs
        for(entity_name, entity_description) in self.get_entity_names_and_values()  {
            let entity = Entity::from((entity_name.to_string(), entity_description.clone(), self.entities.clone()));
            entities.push(entity);
        }
        entities
    }

    pub fn get_entity_names_and_values(&self) -> Vec<(String, Value)> {
        self
            .entities
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|x| x.as_object().unwrap()).map(|(entity_name, entity_value)| (entity_name.to_string(), entity_value.clone())).collect::<Vec<(String, Value)>>()
    }
}

pub struct DomainDrivenRequest {
    pub service_name: String,
    pub entities: Vec<Entity>,
}

impl From<RawDomainDrivenRequest> for DomainDrivenRequest {
    fn from(raw_ddr: RawDomainDrivenRequest) -> Self {
        let entities = raw_ddr.generate_entities();
        DomainDrivenRequest {
            service_name: raw_ddr.service_name,
            entities,
        }
    }
}

impl DomainDrivenRequest {
    pub fn generate_models(&self) -> Vec<(&String, String)> {
        let mut models = Vec::new();
        // extract entities in key value pairs
        for entity in &self.entities {
            let model = self.generate_model(&entity);
            models.push((&entity.name, model));
        }
        models
    }

    fn get_entity_names(&self) -> Vec<EntityName> {
        self.entities.iter().map(|entity| entity.name.clone()).collect::<Vec<EntityName>>()
    }

    pub fn generate_routes_file(&self) -> String {
        AxumRoutesGenerator::generate_routes_file(self, self.get_entity_names())
    }

    pub fn generate_controllers(&self) -> Vec<(&String, String)> {
        let mut controller = Vec::new();
        // extract entities in key value pairs
        for entity in &self.entities {
            let controller_fn = ControllerGenerator::generate_controller(self, &entity);
            controller.push((&entity.name, controller_fn));
        }
        controller
    }

    pub fn generate_cargo_toml(&self) -> String {
        ProjectConfigGenerator::generate_cargo_toml(self, &self.service_name)
    }

    pub fn generate_environment_definitions(&self) -> String {
        ProjectConfigGenerator::generate_config_toml(self, &self.service_name)
    }

    pub fn generate_database_config(&self) -> String {
        DatabaseGenerator::generate_database_drop(self, self.service_name.as_str())
        +
        DatabaseGenerator::generate_database_create(self, self.service_name.as_str()).as_str()
    }

    pub fn generate_postgres_tables(&self) -> Vec<(&String, String)> {
        let mut tables = Vec::new();
        // extract entities in key value pairs
        for entity in &self.entities  {
            let table = self.generate_table_query(&entity);
            tables.push((&entity.name, table));
        }
        tables
    }


    pub fn generate_http(&self) -> String {
        self.generate_create_services_fn(self.get_entity_names())

    }

    pub fn generate_services(&self) -> Vec<(String, String)> {
        let mut service = Vec::new();
        // extract entities in key value pairs
        for entity_name in self.get_entity_names()  {
            let service_fn = ServiceGenerator::generate_service(self, &entity_name.to_string());
            service.push((entity_name, service_fn));
        }
        service
    }
    
    pub fn generate_sources(&self) -> Vec<(&String, String)> {
        let mut sources = Vec::new();
        // extract entities in key value pairs
        for entity in &self.entities {
            let source_file = SourceGenerator::generate_source(self, &entity);
            sources.push((&entity.name, source_file));
        }
        sources
    }
    
    pub fn generate_docker_compose(&self) -> String {
        DockerComposeGenerator::generate_docker_compose(self, &self.service_name)
    }

    pub fn generate_error(&self) -> String {
        ErrorGenerator::generate_error(self, self.get_entity_names())
    }

    pub fn generate_model_mods(&self) -> String {
        let mut model_mods = String::new();
        // extract entities in key value pairs
        for entity_name in self.get_entity_names()  {
            let model_mod = ModGenerator::generate_model_mod(self, &entity_name.to_string());
            model_mods.push_str(model_mod.as_str());
        }
        model_mods
    
    }

    pub fn generate_service_mods(&self) -> String {
        let mut service_mods = String::new();
        // extract entities in key value pairs
        for entity_name in self.get_entity_names()  {
            let service_mod = ModGenerator::generate_service_mod(self, &entity_name.to_string());
            service_mods.push_str(service_mod.as_str());
        }
        let services_state = ServiceGenerator::generate_services_state(self, self.get_entity_names());
        service_mods + services_state.as_str()
    }

    pub fn generate_source_mods(&self) -> String {
        let mut source_mods = String::new();
        // extract entities in key value pairs
        for entity_name in self.get_entity_names()  {
            let source_mod = ModGenerator::generate_source_mod(self, &entity_name.to_string());
            source_mods.push_str(source_mod.as_str());
        }
        source_mods
    }

    pub fn generate_controller_mods(&self) -> String {
        let mut controller_mods = String::new();
        // extract entities in key value pairs
        for entity_name in self.get_entity_names()  {
            let controller_mod = ModGenerator::generate_controller_mod(self, &entity_name.to_string());
            controller_mods.push_str(controller_mod.as_str());
        }
        controller_mods
    
    }

    

}

impl ImportGenerator for DomainDrivenRequest {}
impl ModelGenerator for DomainDrivenRequest {}
impl AxumRoutesGenerator for DomainDrivenRequest {}
impl ControllerGenerator for DomainDrivenRequest {}
impl ProjectConfigGenerator for DomainDrivenRequest {}
impl DatabaseGenerator for DomainDrivenRequest {}
impl PostgresTableGenerator for DomainDrivenRequest {}
impl ServiceGenerator for DomainDrivenRequest {}
impl SourceGenerator for DomainDrivenRequest {}
impl CrudQueryGenerator for DomainDrivenRequest {}
impl DockerComposeGenerator for DomainDrivenRequest {}
impl ErrorGenerator for DomainDrivenRequest {}
impl ModGenerator for DomainDrivenRequest {}