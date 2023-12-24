use crate::utils::naming_convention::{to_snake_case, to_table_name};

pub static VERIFY_ENTITY_CREATION_FN: &str = r##"
pub async fn verify_{sc_entity_name}_creation_constraints(
    &self,
    {sc_entity_name}: &{entity_name}
) -> Result<{(), sqlx::Error> {
    {verify_constraints}
}
"##;

pub static VERIFY_ENTITY_UPDATE_FN: &str = r##"
pub async fn verify_{sc_entity_name}_update_constraints(
    &self,
    {sc_entity_name}: &{entity_name}
) -> Result<{(), sqlx::Error> {
    {verify_constraints}
}
"##;

pub static VERIFY_ENTITY_DELETE_FN: &str = r##"
pub async fn verify_{sc_entity_name}_delete_constraints(
    &self,
    {sc_entity_name}_id: i32
) -> Result<{(), sqlx::Error> {
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

pub static UPDATE_ENTITY_FN: &str = r##"
pub async fn update_{sc_entity_name}(
    &self,
    {sc_entity_name}_id: i32,
    {sc_entity_name}_payload: Update{entity_name}Payload
) -> Result<{entity_name}, Error> {
    let mut {sc_entity_name} = self.get_{sc_entity_name}({sc_entity_name}_id).await?;
    {sc_entity_name}.update({sc_entity_name}_payload)?;
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
    {sc_entity_name}_id: i32
) -> Result<(), Error> {
    self.verify_{sc_entity_name}_delete_constraints({sc_entity_name}_id).await?;

    match self.{table_name}_table.delete_{sc_entity_name}({sc_entity_name}_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::{entity_name}DeleteError(e.to_string()))
    }
}
"##;

pub trait ServiceGenerator {

    fn generate_create_entity_fn(&self, entity_name: &str) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        let table_name = to_table_name(entity_name);
        
        CREATE_ENTITY_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity_name)
            .replace("{table_name}", &table_name)
    }

    fn generate_update_entity_fn(&self, entity_name: &str) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        let table_name = to_table_name(entity_name);
        
        UPDATE_ENTITY_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity_name)
            .replace("{table_name}", &table_name)
    }

    fn generate_delete_entity_fn(&self, entity_name: &str) -> String {
        let sc_entity_name = to_snake_case(entity_name);
        let table_name = to_table_name(entity_name);
        
        DELETE_ENTITY_FN
            .replace("{sc_entity_name}", &sc_entity_name)
            .replace("{entity_name}", &entity_name)
            .replace("{table_name}", &table_name)
    }
}