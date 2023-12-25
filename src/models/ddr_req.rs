use serde::{Serialize, Deserialize};
use serde_json::Value;
use strum::EnumProperty;

use crate::templates::{rust::{controller_templates::ControllerGenerator, model_templates::ModelGenerator, source_templates::SourceGenerator, service_templates::ServiceGenerator, axum_routes_templates::AxumRoutesGenerator, error_templates::ErrorGenerator, import_templates::ImportGenerator, project_config_templates::ProjectConfigGenerator}, postgres::{table_templates::PostgresTableGenerator, crud_query_templates::CrudQueryGenerator}, docker::docker_compose::DockerComposeGenerator};



#[derive(Serialize, Deserialize)]
pub struct DomainDrivenRequest {
    pub service_name: String,
    pub entities: Value,
}

impl DomainDrivenRequest {
    pub fn generate_models(&self) -> Vec<String> {
        let mut models = Vec::new();
        // extract entities in key value pairs
        for(entity_name, entity_description) in self.get_entity_names_and_values()  {
            let model = self.generate_struct(&entity_name.to_string(), entity_description.clone());
            models.push(model);
        }
        models
    }

    pub fn generate_postgres_tables(&self) -> Vec<String> {
        let mut tables = Vec::new();
        // extract entities in key value pairs
        for(entity_name, entity_description) in self.get_entity_names_and_values()  {
            let table = self.generate_table_query(entity_name.to_string(), entity_description.clone());
            tables.push(table);
        }
        tables
    }

    pub fn generate_payloads(&self) -> Vec<String> {
        let mut payloads = Vec::new();
        // extract entities in key value pairs
        for(entity_name, entity_description) in self.get_entity_names_and_values()  {
            let create_payload = self.generate_create_payload(&entity_name.to_string(), entity_description.clone());
            let update_payload = self.generate_update_payload(&entity_name.to_string(), entity_description.clone());
            
            payloads.push(create_payload);
            payloads.push(update_payload);
        }
        payloads
    }

    pub fn generate_queries(&self) -> Vec<String> {
        let mut queries = Vec::new();
        // extract entities in key value pairs
        for(entity_name, entity_description) in self.get_entity_names_and_values()  {
            let create_query = self.generate_create_query(&entity_name.to_string(), &entity_description);
            let update_query = self.generate_update_query(&entity_name.to_string(), &entity_description);
            let delete_query = self.generate_delete_query(&entity_name.to_string());
            
            queries.push(create_query);
            queries.push(update_query);
            queries.push(delete_query);
        }
        queries
    }

    pub fn generate_axum_routes_system(&self) -> String {
        AxumRoutesGenerator::generate_axum_routes_system(self, self.get_entity_names())
    }

    pub fn get_entity_names(&self) -> Vec<String> {
        self
            .entities
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|x| x.as_object().unwrap()).map(|(entity_name, _)| entity_name.to_string()).collect::<Vec<String>>()
    }

    pub fn get_entity_names_and_values(&self) -> Vec<(String, Value)> {
        self
            .entities
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|x| x.as_object().unwrap()).map(|(entity_name, entity_value)| (entity_name.to_string(), entity_value.clone())).collect::<Vec<(String, Value)>>()
    }

    pub fn generate_errors(&self) -> Vec<String> {
        let mut errors = Vec::new();
        // extract entities in key value pairs
        errors.push(ErrorGenerator::generate_server_error_enums(self, self.get_entity_names()));
        errors.push(ErrorGenerator::generate_client_error_enums(self, self.get_entity_names()));
        errors.push(ErrorGenerator::generate_error_impl(self, self.get_entity_names()));
        errors
    }

    pub fn generate_imports(&self) -> Vec<String> {
        let mut imports = Vec::new();
        // extract entities in key value pairs
        for entity_name in self.get_entity_names()  {
            let model_import = ImportGenerator::generate_model_imports(self, &entity_name.to_string());
            let source_import = ImportGenerator::generate_source_imports(self, &entity_name.to_string());
            let service_import = ImportGenerator::generate_service_imports(self, &entity_name.to_string());
            let controller_import = ImportGenerator::generate_controller_imports(self, &entity_name.to_string());
            imports.push(model_import);
            imports.push(source_import);
            imports.push(service_import);
            imports.push(controller_import);
        }
        imports
    }

    pub fn generate_controllers(&self) -> Vec<String> {
        let mut controller = Vec::new();
        // extract entities in key value pairs
        for(entity_name, entity_value) in self.entities.as_array().unwrap().iter().flat_map(|x| x.as_object().unwrap())  {
            let controller_fn = ControllerGenerator::generate_controller(self, &entity_name.to_string(), entity_value.clone());
            controller.push(controller_fn);
        }
        controller
    }

    pub fn generate_services(&self) -> Vec<String> {
        let mut service = Vec::new();
        // extract entities in key value pairs
        for entity_name in self.get_entity_names()  {
            let service_fn = ServiceGenerator::generate_service(self, &entity_name.to_string());
            service.push(service_fn);
        }
        service
    }

    pub fn generate_sources(&self) -> Vec<String> {
        let mut sources = Vec::new();
        // extract entities in key value pairs
        for (entity_name, entity_value) in self.entities.as_array().unwrap().iter().flat_map(|x| x.as_object().unwrap())  {
            let source_file = SourceGenerator::generate_source(self, &entity_name.to_string(), &entity_value.clone());
            sources.push(source_file);
        }
        sources
    }

    pub fn generate_docker_compose(&self) -> String {
        DockerComposeGenerator::generate_docker_compose(self, &self.service_name)
    }

    pub fn generate_environment_definitions(&self) -> String {
        ProjectConfigGenerator::generate_config_toml(self, &self.service_name)
    }

    pub fn generate_cargo_toml(&self) -> String {
        ProjectConfigGenerator::generate_cargo_toml(self, &self.service_name)
    }
    
    
}

impl ModelGenerator for DomainDrivenRequest {}
impl PostgresTableGenerator for DomainDrivenRequest {}
impl ControllerGenerator for DomainDrivenRequest {}
impl CrudQueryGenerator for DomainDrivenRequest {}
impl SourceGenerator for DomainDrivenRequest {}
impl ServiceGenerator for DomainDrivenRequest {}
impl AxumRoutesGenerator for DomainDrivenRequest {}
impl ErrorGenerator for DomainDrivenRequest {}
impl ImportGenerator for DomainDrivenRequest {}
impl DockerComposeGenerator for DomainDrivenRequest {}
impl ProjectConfigGenerator for DomainDrivenRequest {}

pub enum AttributeType {
    String,
    Uuid,
    I32,
    I64,
    F32,
    F64,
    Boolean,
    UTCDateTime,
    Option(Box<AttributeType>),
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, strum_macros::EnumProperty)]
pub enum PostgresAttributeType {

    VARCHAR,
    UUID,
    INT,
    BIGINT,
    REAL,
    DOUBLE_PRECISION,
    BOOLEAN,
    TIMESTAMP,
    #[strum(props(is_nullable = "true"))]
    OPTION(Box<PostgresAttributeType>),
}

impl PostgresAttributeType {
    pub fn is_nullable(&self) -> bool {
        self.get_bool("is_nullable").unwrap_or(false)
    }
}
impl From<AttributeType> for PostgresAttributeType {
    fn from(attribute_type: AttributeType) -> Self {
        match attribute_type {
            AttributeType::String => PostgresAttributeType::VARCHAR,
            AttributeType::Uuid => PostgresAttributeType::UUID,
            AttributeType::I32 => PostgresAttributeType::INT,
            AttributeType::I64 => PostgresAttributeType::BIGINT,
            AttributeType::F32 => PostgresAttributeType::REAL,
            AttributeType::F64 => PostgresAttributeType::DOUBLE_PRECISION,
            AttributeType::Boolean => PostgresAttributeType::BOOLEAN,
            AttributeType::UTCDateTime => PostgresAttributeType::TIMESTAMP,
            AttributeType::Option(attribute_type) => PostgresAttributeType::OPTION(Box::new(Into::<PostgresAttributeType>::into(*attribute_type))),
        }
    }
}


impl ToString for PostgresAttributeType {
    fn to_string(&self) -> String {
        let attr_type = match self {
            PostgresAttributeType::VARCHAR => "VARCHAR(255)".to_string(),
            PostgresAttributeType::UUID => "UUID".to_string(),
            PostgresAttributeType::INT => "INT".to_string(),
            PostgresAttributeType::BIGINT => "BIGINT".to_string(),
            PostgresAttributeType::REAL => "REAL".to_string(),
            PostgresAttributeType::DOUBLE_PRECISION => "DOUBLE PRECISION".to_string(),
            PostgresAttributeType::BOOLEAN => "BOOLEAN".to_string(),
            PostgresAttributeType::TIMESTAMP => "TIMESTAMP".to_string(),
            PostgresAttributeType::OPTION(attribute_type) => format!("{}", attribute_type.to_string()),
        };
        attr_type + if self.is_nullable() { "" } else { " NOT NULL" }
    }
}

impl AttributeType {
    pub fn from_str(s: &str) -> AttributeType {
        match s {
            "String" => AttributeType::String,
            "Uuid" => AttributeType::Uuid,
            "i32" => AttributeType::I32,
            "i64" => AttributeType::I64,
            "f32" => AttributeType::F32,
            "f64" => AttributeType::F64,
            "bool" => AttributeType::Boolean,
            "UTCDateTime" => AttributeType::UTCDateTime,
            _ if s.starts_with("Option<") && s.ends_with(">") => {
                let inner_type = s[7..s.len() - 1].to_string();
                AttributeType::Option(Box::new(AttributeType::from_str(inner_type.as_str())))
            }
            _ => panic!("Invalid attribute type: {}", s),
        }
}
}

impl ToString for AttributeType {
    fn to_string(&self) -> String {
        match self {
            AttributeType::String => "String".to_string(),
            AttributeType::Uuid => "Uuid".to_string(),
            AttributeType::I32 => "i32".to_string(),
            AttributeType::I64 => "i64".to_string(),
            AttributeType::F32 => "f32".to_string(),
            AttributeType::F64 => "f64".to_string(),
            AttributeType::Boolean => "bool".to_string(),
            AttributeType::UTCDateTime => "UTCDateTime".to_string(),
            AttributeType::Option(attribute_type) => format!("Option<{}>", attribute_type.to_string()),
        }
    }
}