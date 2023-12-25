use serde_json::Value;

use crate::{utils::naming_convention::{to_snake_case, to_plural}, templates::postgres::crud_query_templates::CrudQueryGenerator};

use super::{model_templates::ModelGenerator, import_templates::ImportGenerator};

pub static CREATE_ENTITY_FN: &str = r##"
    pub async fn create_{sc_entity_name}(
        &self,
        {sc_entity_name}: {entity_name}
    ) -> Result<{entity_name}, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        let new_{sc_entity_name} = sqlx::query_as!(
            {entity_name},
            r#"{create_query}
            "#,
            {entity_values}
        )
        .fetch_one(transaction.as_mut())
        .await?;
        transaction.commit().await?;
        Ok(new_{sc_entity_name})
    }
"##;

pub static UPDATE_ENTITY_FN: &str = r##"
    pub async fn update_{sc_entity_name}(
        &self,
        {sc_entity_name}: {entity_name}
    ) -> Result<{entity_name}, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        let updated_{sc_entity_name} = sqlx::query_as!(
            {entity_name},
            r#"{update_query}
            "#,
            {entity_values}
        )
        .fetch_one(transaction.as_mut())
        .await?;
        transaction.commit().await?;
        Ok(updated_{sc_entity_name})
    }
"##;

pub static DELETE_ENTITY_FN: &str = r##"
    pub async fn delete_{sc_entity_name}(
        &self,
        {sc_entity_name}_id: Uuid
    ) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query_as!(
            {entity_name},
            r#"{delete_query}
            "#,
            {sc_entity_name}_id
        )
        .execute(transaction.as_mut())
        .await?;
        transaction.commit().await?;
        Ok(())
    }
"##;

pub static SOURCE_FILE_TEMPLATE: &str = r#"
use std::sync::Arc;
use uuid::Uuid;
use sqlx::{Pool, Postgres};

{entity_imports}

pub struct {entity_plural}Table {
    pool: Arc<Pool<Postgres>>
}

impl {entity_plural}Table {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            pool
        }
    }

    {source_functions}
}"#;


pub trait SourceGenerator : CrudQueryGenerator + ModelGenerator + ImportGenerator {
    fn generate_create_fn(&self, entity_name: &str, entity: &Value) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        let create_query = self.generate_create_query(entity_name, &entity);
        let entity_values = self.generate_entity_value_accessors(entity_name, &entity);
        CREATE_ENTITY_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity_name)
            .replace("{create_query}", &create_query)
            .replace("{entity_values}", &entity_values)
    }

    fn generate_update_fn(&self, entity_name: &str, entity: &Value) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        let update_query = self.generate_update_query(entity_name, &entity);
        let entity_values = self.generate_entity_value_accessors(entity_name, &entity);
        UPDATE_ENTITY_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity_name)
            .replace("{update_query}", &update_query)
            .replace("{entity_values}", &entity_values)
    }

    fn generate_delete_fn(&self, entity_name: &str) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        let delete_query = self.generate_delete_query(entity_name);
        DELETE_ENTITY_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity_name)
            .replace("{delete_query}", &delete_query)
    }

    fn generate_source(&self, entity_name: &str, entity: &Value) -> String {
        let entity_imports = self.generate_model_imports(entity_name);

        let mut source_functions = String::new();
        source_functions.push_str(SourceGenerator::generate_create_fn(self, entity_name, &entity).as_str());
        source_functions.push_str(SourceGenerator::generate_update_fn(self, entity_name, &entity).as_str());
        source_functions.push_str(SourceGenerator::generate_delete_fn(self, entity_name).as_str());

        SOURCE_FILE_TEMPLATE
            .replace("{entity_imports}", &entity_imports)
            .replace("{entity_plural}", &to_plural(entity_name))
            .replace("{source_functions}", &source_functions)
    }
    
}