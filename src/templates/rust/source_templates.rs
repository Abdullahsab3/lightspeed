use serde_json::Value;

use crate::{utils::naming_convention::to_snake_case, templates::postgres::crud_query_templates::CrudQueryGenerator};

use super::model_templates::ModelGenerator;

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
    .fetch_one(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_{sc_entity_name})
}
"##;

pub trait SourceGenerator : CrudQueryGenerator + ModelGenerator {
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
}