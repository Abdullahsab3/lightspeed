use crate::{utils::naming_convention::{to_snake_case, to_plural}, templates::postgres::crud_query_templates::CrudQueryGenerator, models::entity::Entity};

use super::{model_templates::ModelGenerator, import_templates::ImportGenerator};

pub static CREATE_ENTITY_FN: &str = r##"
    pub async fn create_{sc_entity_name}(
        &self,
        {sc_entity_name}: &{entity_name}
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

pub static GET_ENTITY_FN: &str = r##"
    pub async fn get_{sc_entity_name}(
        &self,
        {primary_key}: &{primary_key_type}
    ) -> Result<{entity_name}, sqlx::Error> {
        let {sc_entity_name} = sqlx::query_as!(
            {entity_name},
            r#"{get_query}
            "#,
            {primary_key}
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok({sc_entity_name})
    }
"##;

pub static GET_PAGINATED_FN: &str = r##"
    pub async fn get_{sc_plural_entity}(
        &self,
        page: i64,
        limit: i64
    ) -> Result<Vec<{entity_name}>, sqlx::Error> {
        let {sc_plural_entity} = sqlx::query_as!(
            {entity_name},
            r#"{get_paginated_query}
            "#,
            limit,
            (page - 1) * limit
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok({sc_plural_entity})
    }
"##;

pub static FILTER_BY_FN: &str = r##"
    pub async fn get_{sc_plural_entity}_by_{most_specific_attribute}(
        &self,
        {filter_by_fields}
    ) -> Result<Vec<{entity_name}>, sqlx::Error> {
        let {sc_plural_entity} = sqlx::query_as!(
            {entity_name},
            r#"{filter_by_query}
            "#,
            {filter_by_values}
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok({sc_plural_entity})
    }
"##;

pub static FILTER_BY_PAGINATED_FN: &str = r##"
    pub async fn filter_{sc_plural_entity}_by_{most_specific_attribute}(
        &self,
        {filter_by_fields},
        page: i64,
        limit: i64
    ) -> Result<Vec<{entity_name}>, sqlx::Error> {
        let {sc_plural_entity} = sqlx::query_as!(
            {entity_name},
            r#"{filter_by_paginated_query}
            "#,
            {filter_by_values},
            limit,
            (page - 1) * limit
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok({sc_plural_entity})
    }
"##;

pub static FILTER_BY_FIELD: &str = r#"{attribute_name}: {attribute_type}"#;

pub static GET_COUNT_FN: &str = r##"
    pub async fn get_{sc_plural_entity}_count(
        &self
    ) -> Result<i64, sqlx::Error> {
        let {sc_plural_entity}_count = sqlx::query!(
            r#"{count_query}
            "#
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok({sc_plural_entity}_count.count.unwrap())
    }
"##;

pub static UPDATE_ENTITY_FN: &str = r##"
    pub async fn update_{sc_entity_name}(
        &self,
        {sc_entity_name}: &{entity_name}
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
        {primary_key}: &{primary_key_type}
    ) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query_as!(
            {entity_name},
            r#"{delete_query}
            "#,
            {primary_key}
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
    fn generate_create_fn(&self, entity: &Entity) -> String {
        let sc_entity_name = to_snake_case(&entity.name);
        let create_query = self.generate_create_query(&entity);
        let entity_values = self.generate_entity_value_accessors(&entity);
        CREATE_ENTITY_FN
            .replace("{primary_key}", &entity.primary_key)
            .replace("{primary_key_type}", &entity.primary_key_type().to_string())
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity.name)
            .replace("{create_query}", &create_query)
            .replace("{entity_values}", &entity_values)
    }

    fn generate_get_fn(&self, entity: &Entity) -> String {
        let sc_entity_name = to_snake_case(&entity.name);
        let get_query = self.generate_get_query(&entity);
        GET_ENTITY_FN
            .replace("{primary_key}", &entity.primary_key)
            .replace("{primary_key_type}", &entity.primary_key_type().to_string())
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity.name)
            .replace("{get_query}", &get_query)
    }

    fn generate_get_paginated_fn(&self, entity: &Entity) -> String {
        let sc_entity_name = to_snake_case(&entity.name);
        let get_paginated_query = self.generate_get_paginated_query(&entity);
        GET_PAGINATED_FN
            .replace("{sc_plural_entity}", &to_plural(&sc_entity_name))
            .replace("{entity_name}", &entity.name)
            .replace("{get_paginated_query}", &get_paginated_query)
    }

    fn generate_filter_by_fn(&self, entity: &Entity) -> String {
        entity.filter_by.iter().map(|filter_by| {
          
            let filter_by_values = filter_by.iter().map(|field_name| {
                format!("&{}", field_name)
            }).collect::<Vec<String>>().join(", ");
            let filter_by_fields = filter_by.iter().map(|field_name| {
                FILTER_BY_FIELD
                .replace("{attribute_name}", &field_name)
                .replace("{attribute_type}", &entity.attributes.iter().find(|(attribute_name, _)| {println!("attr name: {}, field name: {}", attribute_name, field_name); attribute_name == field_name}).unwrap().1.to_string())
            }).collect::<Vec<String>>().join("\n, ");
            if filter_by.iter().filter(|field_name| entity.is_unique(&field_name)).count() > 0 {
                let filter_by_query = self.generate_filter_by_query(&entity, &filter_by);
                FILTER_BY_FN
                .replace("{sc_plural_entity}", &to_plural(&to_snake_case(&entity.name)))
                .replace("{entity_name}", &entity.name)
                .replace("{filter_by_query}", &filter_by_query)
                .replace("{filter_by_values}", &filter_by_values)
                .replace("{most_specific_attribute}", &filter_by.last().unwrap())
                .replace("{filter_by_fields}", &filter_by_fields)


            } else {
                let filter_by_paginated_query = self.generate_filter_by_paginated_query(&entity, &filter_by);
                FILTER_BY_PAGINATED_FN
                .replace("{sc_plural_entity}", &to_plural(&to_snake_case(&entity.name)))
                .replace("{entity_name}", &entity.name)
                .replace("{filter_by_paginated_query}", &filter_by_paginated_query)
                .replace("{filter_by_values}", &filter_by_values)
                .replace("{most_specific_attribute}", &filter_by.last().unwrap())
                .replace("{filter_by_fields}", &filter_by_fields)
                

            }

        }).collect::<Vec<String>>().join("\n")
    }

    fn generate_get_count_fn(&self, entity: &Entity) -> String {
        let sc_entity_name = to_snake_case(&entity.name);
        let count_query = self.generate_count_query(&entity);
        GET_COUNT_FN
            .replace("{sc_plural_entity}", &to_plural(&sc_entity_name))
            .replace("{entity_name}", &entity.name)
            .replace("{count_query}", &count_query)
    }

    fn generate_update_fn(&self, entity: &Entity) -> String {
        let sc_entity_name = to_snake_case(&entity.name);
        let update_query = self.generate_update_query(entity);
        let entity_values = self.generate_entity_value_accessors(entity) + &format!("{}.{}, ", to_snake_case(&entity.name), entity.primary_key);
        
        UPDATE_ENTITY_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity.name)
            .replace("{update_query}", &update_query)
            .replace("{entity_values}", &entity_values)
    }

    fn generate_delete_fn(&self, entity: &Entity) -> String {
        let sc_entity_name = to_snake_case(&entity.name);
        let delete_query = self.generate_delete_query(&entity);
        DELETE_ENTITY_FN
            .replace("{primary_key}", &entity.primary_key)
            .replace("{primary_key_type}", &entity.primary_key_type().to_string())
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity.name)
            .replace("{delete_query}", &delete_query)
    }

    fn generate_source(&self, entity: &Entity) -> String {
        let entity_imports = self.generate_model_imports(&entity.name);

        let mut source_functions = String::new();
        source_functions.push_str(SourceGenerator::generate_create_fn(self, &entity).as_str());
        source_functions.push_str(SourceGenerator::generate_get_fn(self, &entity).as_str());
        source_functions.push_str(SourceGenerator::generate_get_paginated_fn(self, &entity).as_str());
        source_functions.push_str(SourceGenerator::generate_filter_by_fn(self, &entity).as_str());
        source_functions.push_str(SourceGenerator::generate_get_count_fn(self, &entity).as_str());
        source_functions.push_str(SourceGenerator::generate_update_fn(self, &entity).as_str());
        source_functions.push_str(SourceGenerator::generate_delete_fn(self, &entity).as_str());

        SOURCE_FILE_TEMPLATE
            .replace("{entity_imports}", &entity_imports)
            .replace("{entity_plural}", &to_plural(&entity.name))
            .replace("{source_functions}", &source_functions)
    }
    
}