use serde::{Serialize, Deserialize};
use serde_json::Value;
use strum::EnumProperty;

use crate::templates::{rust::{controller_templates::ControllerGenerator, model_templates::ModelGenerator, source_templates::SourceGenerator, service_templates::ServiceGenerator, axum_routes_templates::AxumRoutesGenerator, error_templates::ErrorGenerator, import_templates::ImportGenerator, project_config_templates::ProjectConfigGenerator, mod_template::ModGenerator}, postgres::{table_templates::PostgresTableGenerator, crud_query_templates::CrudQueryGenerator, database_template::DatabaseGenerator}, docker::docker_compose::DockerComposeGenerator};



#[derive(Serialize, Deserialize)]
pub struct DomainDrivenRequest {
    pub service_name: String,
    pub entities: Value,
}

impl DomainDrivenRequest {
    pub fn generate_models(&self) -> Vec<(String, String)> {
        let mut models = Vec::new();
        // extract entities in key value pairs
        for(entity_name, entity_description) in self.get_entity_names_and_values()  {
            let model = self.generate_model(&entity_name, &entity_description);
            models.push((entity_name, model));
        }
        models
    }

    pub fn generate_postgres_tables(&self) -> Vec<(String, String)> {
        let mut tables = Vec::new();
        // extract entities in key value pairs
        for(entity_name, entity_description) in self.get_entity_names_and_values()  {
            let table = self.generate_table_query(entity_name.to_string(), entity_description.clone());
            tables.push((entity_name, table));
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

    pub fn generate_controllers(&self) -> Vec<(String, String)> {
        let mut controller = Vec::new();
        // extract entities in key value pairs
        for(entity_name, entity_value) in self.get_entity_names_and_values()  {
            let controller_fn = ControllerGenerator::generate_controller(self, &entity_name.to_string(), entity_value.clone());
            controller.push((entity_name, controller_fn));
        }
        controller
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

    pub fn generate_sources(&self) -> Vec<(String, String)> {
        let mut sources = Vec::new();
        // extract entities in key value pairs
        for (entity_name, entity_value) in self.get_entity_names_and_values() {
            let source_file = SourceGenerator::generate_source(self, &entity_name.to_string(), &entity_value.clone());
            sources.push((entity_name, source_file));
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

    pub fn generate_database_config(&self) -> String {
        DatabaseGenerator::generate_database_drop(self, self.service_name.as_str())
        +
        DatabaseGenerator::generate_database_create(self, self.service_name.as_str()).as_str()
    }

    pub fn generate_http(&self) -> String {
        self.generate_create_services_fn(self.get_entity_names())

    }

    pub fn generate_error(&self) -> String {
        ErrorGenerator::generate_error(self, self.get_entity_names())
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
impl DatabaseGenerator for DomainDrivenRequest {}
impl ModGenerator for DomainDrivenRequest {}

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
    Unknown(String),
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
    UNKNOWN,
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
            AttributeType::Unknown(_) => PostgresAttributeType::UNKNOWN,
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
            PostgresAttributeType::UNKNOWN => panic!("Unknown attribute type"),
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
            _ => AttributeType::Unknown(s.to_string()),
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
            AttributeType::Unknown(_) => panic!("Unknown attribute type"),
        }
    }
}