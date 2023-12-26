use crate::utils::naming_convention::{to_snake_case, to_snake_case_plural, to_plural};

use super::{import_templates::ImportGenerator, model_templates::ATTRIBUTE_TEMPLATE};

pub static VERIFY_ENTITY_CREATION_FN: &str = r##"
    pub async fn verify_{sc_entity_name}_creation_constraints(
        &self,
        {sc_entity_name}: &{entity_name}
    ) -> Result<(), Error> {
        {verify_constraints}
    }
"##;

pub static VERIFY_ENTITY_UPDATE_FN: &str = r##"
    pub async fn verify_{sc_entity_name}_update_constraints(
        &self,
        {sc_entity_name}: &{entity_name}
    ) -> Result<(), Error> {
        {verify_constraints}
    }
"##;

pub static VERIFY_ENTITY_DELETE_FN: &str = r##"
    pub async fn verify_{sc_entity_name}_delete_constraints(
        &self,
        {sc_entity_name}_id: Uuid
    ) -> Result<(), Error> {
        {verify_constraints}
    }
"##;

pub static CREATE_ENTITY_FN: &str = r##"
    pub async fn create_{sc_entity_name}(
        &self,
        {sc_entity_name}_payload: Add{entity_name}Payload
    ) -> Result<{entity_name}, Error> {
        let {sc_entity_name} = {entity_name}::new({sc_entity_name}_payload)?;
        self.verify_{sc_entity_name}_creation_constraints(&{sc_entity_name}).await?;

        match self.{table_name}_table.create_{sc_entity_name}(&{sc_entity_name}).await {
            Ok({sc_entity_name}) => Ok({sc_entity_name}),
            Err(e) => Err(Error::{entity_name}CreationError(e.to_string()))
        }
    }
"##;

pub static GET_ENTITY_FN: &str = r##"
    pub async fn get_{sc_entity_name}(
        &self,
        {sc_entity_name}_id: Uuid
    ) -> Result<{entity_name}, Error> {
        match self.{table_name}_table.get_{sc_entity_name}(&{sc_entity_name}_id).await {
            Ok({sc_entity_name}) => Ok({sc_entity_name}),
            Err(e) => Err(Error::{entity_name}FetchError(e.to_string()))
        }
    }
"##;

pub static GET_PAGINATED_ENTITY_FN: &str = r##"
    pub async fn get_{sc_plural_entity}(
        &self,
        page: i64,
        limit: i64,
    ) -> Result<PaginatedResult<{entity_name}>, Error> {
        let {sc_plural_entity} = self.{sc_plural_entity}_table.get_{sc_plural_entity}(page, limit).await;
        match {sc_plural_entity} {
            Ok({sc_plural_entity}) => {
                let total = self
                    .{sc_plural_entity}_table
                    .get_{sc_plural_entity}_count()
                    .await
                    .map_err(|_| {
                        Error::{entity_name}FetchError("Could not fetch the total number of {sc_plural_entity}".to_string())
                    })?;
                Ok(PaginatedResult {
                    results: {sc_plural_entity},
                    total: total,
                    page: page,
                    page_size: limit,
                })
            }
            Err(e) => Err(Error::{entity_name}FetchError(e.to_string())),
        }
    }

"##;

pub static UPDATE_ENTITY_FN: &str = r##"
    pub async fn update_{sc_entity_name}(
        &self,
        {sc_entity_name}_id: Uuid,
        {sc_entity_name}_payload: Update{entity_name}Payload
    ) -> Result<{entity_name}, Error> {
        let {sc_entity_name} = self.get_{sc_entity_name}({sc_entity_name}_id).await?.update({sc_entity_name}_payload)?;
        self.verify_{sc_entity_name}_update_constraints(&{sc_entity_name}).await?;

        match self.{table_name}_table.update_{sc_entity_name}(&{sc_entity_name}).await {
            Ok({sc_entity_name}) => Ok({sc_entity_name}),
            Err(e) => Err(Error::{entity_name}UpdateError(e.to_string()))
        }
    }
"##;

pub static DELETE_ENTITY_FN: &str = r##"
    pub async fn delete_{sc_entity_name}(
        &self,
        {sc_entity_name}_id: Uuid
    ) -> Result<(), Error> {
        self.verify_{sc_entity_name}_delete_constraints({sc_entity_name}_id).await?;

        match self.{table_name}_table.delete_{sc_entity_name}(&{sc_entity_name}_id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::{entity_name}DeleteError(e.to_string()))
        }
    }
"##;

pub static SERVICE_FILE_TEMPLATE: &str = r#"
use std::sync::Arc;

use sqlx::{Pool, Postgres};
{entity_imports}
use crate::error::Error;
use uuid::Uuid;
use crate::models::PaginatedResult;

pub struct {entity_plural}Service {
    {sc_entity_plural}_table: {entity_plural}Table,
}

impl {entity_plural}Service {
    pub fn new(db_pool: &Arc<Pool<Postgres>>) -> Self {
        Self {
            {sc_entity_plural}_table: {entity_plural}Table::new(db_pool.clone()),
        }
    }

    {service_functions}
}
"#;

pub static SERVICES_STATE_TEMPLATE: &str = r#"
pub struct ServicesState {
    {services_as_fields}
}
"#;

pub static CREATE_SERVICES_FN_TEMPLATE: &str = r#"

pub async fn create_services(
    pool: PgPool,
) -> Result<ServicesState> {
    let arc_pool = Arc::new(pool);
    {service_definitions}
    Ok(ServicesState {
        {services_as_fields}
    })
}
"#;

pub static SERVICE_DEFINITION: &str = r#"
let {sc_entity_plural}_service = {sc_entity_plural}_service::{entity_plural}Service::new(&arc_pool);
"#;


pub trait ServiceGenerator: ImportGenerator {

    fn generate_verify_entity_creation_constraints_fn(&self, entity_name: &str) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        // TODO: Generate constraints
        let verify_constraints = "Ok(())".to_string();
        VERIFY_ENTITY_CREATION_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity_name)
            .replace("{verify_constraints}", &verify_constraints)
    }

    fn generate_verify_entity_update_constraints_fn(&self, entity_name: &str) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        let verify_constraints = "Ok(())".to_string();
        VERIFY_ENTITY_UPDATE_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity_name)
            .replace("{verify_constraints}", &verify_constraints)
    }

    fn generate_verify_entity_delete_constraints_fn(&self, entity_name: &str) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        let verify_constraints = "Ok(())".to_string();
        VERIFY_ENTITY_DELETE_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity_name)
            .replace("{verify_constraints}", &verify_constraints)
    }

    fn generate_create_entity_fn(&self, entity_name: &str) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        let table_name = to_snake_case_plural(entity_name);
        
        CREATE_ENTITY_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity_name)
            .replace("{table_name}", &table_name)
    }

    fn generate_get_entity_fn(&self, entity_name: &str) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        let table_name = to_snake_case_plural(entity_name);
        
        GET_ENTITY_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity_name)
            .replace("{table_name}", &table_name)
    }

    fn generate_get_entities_paginated_fn(&self, entity_name: &str) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        let sc_plural_entity = to_snake_case_plural(entity_name);
        
        GET_PAGINATED_ENTITY_FN
            .replace("{sc_plural_entity}", &sc_plural_entity)
            .replace("{entity_name}", &entity_name)
            .replace("{sc_entity_name}", &sc_entity_name)
    }

    fn generate_update_entity_fn(&self, entity_name: &str) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        let table_name = to_snake_case_plural(entity_name);
        
        UPDATE_ENTITY_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity_name)
            .replace("{table_name}", &table_name)
    }

    fn generate_delete_entity_fn(&self, entity_name: &str) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        let table_name = to_snake_case_plural(entity_name);
        
        DELETE_ENTITY_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity_name)
            .replace("{table_name}", &table_name)
    }

    fn generate_service(&self, entity_name: &str) -> String {
        let mut entity_imports = String::new();
        entity_imports.push_str(&self.generate_model_imports(entity_name));
        entity_imports.push_str(&self.generate_source_imports(entity_name));
        entity_imports.push_str(&self.generate_controller_imports(entity_name));

        let mut service_functions = String::new();
        service_functions.push_str(&self.generate_verify_entity_creation_constraints_fn(entity_name));
        service_functions.push_str(&self.generate_verify_entity_update_constraints_fn(entity_name));
        service_functions.push_str(&self.generate_verify_entity_delete_constraints_fn(entity_name));

        service_functions.push_str(&self.generate_create_entity_fn(entity_name));
        service_functions.push_str(&self.generate_get_entity_fn(entity_name));
        service_functions.push_str(&self.generate_get_entities_paginated_fn(entity_name));
        service_functions.push_str(&self.generate_update_entity_fn(entity_name));
        service_functions.push_str(&self.generate_delete_entity_fn(entity_name));

        SERVICE_FILE_TEMPLATE
            .replace("{entity_imports}", &entity_imports)
            .replace("{entity_plural}", &to_plural(entity_name))
            .replace("{sc_entity_plural}", &to_snake_case_plural(entity_name))
            .replace("{service_functions}", &service_functions)
    }

    fn generate_services_state(&self, entity_names: Vec<String>) -> String {
        let services_as_fields = entity_names
            .iter()
            .map(|entity_name| {
                let service_key = to_snake_case_plural(entity_name) + "_service";
                let service_value = to_snake_case_plural(&entity_name) + "_service::" + to_plural(entity_name).as_str() + "Service";
                ATTRIBUTE_TEMPLATE
                    .replace("{attribute_name}", &service_key)
                    .replace("{attribute_type}", &service_value)
            })
            .collect::<Vec<String>>()
            .join("\n");

        SERVICES_STATE_TEMPLATE
            .replace("{services_as_fields}", &services_as_fields)
    }

    fn generate_service_definition(&self, entity_name: &str) -> String {
        let sc_entity_plural = to_snake_case_plural(entity_name);
        let entity_plural = to_plural(entity_name);
        SERVICE_DEFINITION
            .replace("{sc_entity_plural}", &sc_entity_plural)
            .replace("{entity_plural}", &entity_plural)
    }

    fn generate_create_services_fn(&self, entity_names: Vec<String>) -> String {
        let service_definitions = entity_names
            .iter()
            .map(|entity_name| self.generate_service_definition(entity_name))
            .collect::<Vec<String>>()
            .join("\n");

        let service_names = entity_names
            .iter()
            .map(|entity_name| to_snake_case_plural(entity_name) + "_service")
            .collect::<Vec<String>>()
            .join(",\n");

        CREATE_SERVICES_FN_TEMPLATE
            .replace("{service_definitions}", &service_definitions)
            .replace("{services_as_fields}", &service_names)

    }
}