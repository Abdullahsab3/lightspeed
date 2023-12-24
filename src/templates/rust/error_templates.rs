use crate::utils::naming_convention::to_upper_snake_case;

use super::model_templates::ModelGenerator;

pub static ENTITY_ALREADY_EXISTS_ERROR_TEMPLATE: &str = r#"{entity_name}AlreadyExists"#;
pub static ENTITY_CREATION_ERROR_TEMPLATE: &str = r#"{entity_name}CreationError(String)"#;
pub static ENTITY_DOES_NOT_EXIST_ERROR_TEMPLATE: &str = r#"{entity_name}DoesNotExist"#;
pub static ENTITY_UPDATE_ERROR_TEMPLATE: &str = r#"{entity_name}UpdateError(String)"#;
pub static ENTITY_DELETION_ERROR_TEMPLATE: &str = r#"{entity_name}DeletionError(String)"#;
pub static ENTITY_FETCH_ERROR_TEMPLATE: &str = r#"{entity_name}FetchError(String)"#;

pub static CLIENT_ENTITY_ALREADY_EXISTS_ERROR_TEMPLATE: &str = r#"{usc_entity_name}_ALREADY_EXISTS"#;
pub static CLIENT_ENTITY_CREATION_ERROR_TEMPLATE: &str = r#"{usc_entity_name}_CREATION_ERROR"#;
pub static CLIENT_ENTITY_DOES_NOT_EXIST_ERROR_TEMPLATE: &str = r#"{usc_entity_name}_DOES_NOT_EXIST"#;
pub static CLIENT_ENTITY_UPDATE_ERROR_TEMPLATE: &str = r#"{usc_entity_name}_UPDATE_ERROR"#;
pub static CLIENT_ENTITY_DELETION_ERROR_TEMPLATE: &str = r#"{usc_entity_name}_DELETION_ERROR"#;
pub static CLIENT_ENTITY_FETCH_ERROR_TEMPLATE: &str = r#"{usc_entity_name}_FETCH_ERROR"#;

pub static ERROR_TO_CLIENT_ERROR_EXIST_TEMPLATE: &str = r#"
            Error::{entity_name}AlreadyExists => (StatusCode::Conflict, ClientError::{usc_entity_name}_ALREADY_EXISTS),"#;
pub static ERROR_TO_CLIENT_ERROR_CREATION_TEMPLATE: &str = r#"
            Error::{entity_name}CreationError(_) => (StatusCode::InternalServerError, ClientError::{usc_entity_name}_CREATION_ERROR),"#;
pub static ERROR_TO_CLIENT_ERROR_DOES_NOT_EXIST_TEMPLATE: &str = r#"
            Error::{entity_name}DoesNotExist => (StatusCode::NotFound, ClientError::{usc_entity_name}_DOES_NOT_EXIST),"#;
pub static ERROR_TO_CLIENT_ERROR_UPDATE_TEMPLATE: &str = r#"
            Error::{entity_name}UpdateError(_) => (StatusCode::InternalServerError, ClientError::{usc_entity_name}_UPDATE_ERROR),"#;
pub static ERROR_TO_CLIENT_ERROR_DELETION_TEMPLATE: &str = r#"
            Error::{entity_name}DeletionError(_) => (StatusCode::InternalServerError, ClientError::{usc_entity_name}_DELETION_ERROR),"#;
pub static ERROR_TO_CLIENT_ERROR_FETCH_TEMPLATE: &str = r#"
            Error::{entity_name}FetchError(_) => (StatusCode::InternalServerError, ClientError::{usc_entity_name}_FETCH_ERROR),"#;

pub static ERROR_IMPL_TEMPLATE: &str = r#"
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            {error_to_client_errors}
        }
}
"#;

pub trait ErrorGenerator : ModelGenerator {
    fn generate_server_error_enums(&self, entity_names: Vec<String>) -> String {
        let mut error_enums: Vec<String> = Vec::new();
        for entity_name in entity_names {
            error_enums.push(ENTITY_ALREADY_EXISTS_ERROR_TEMPLATE.replace("{entity_name}", &entity_name));
            error_enums.push(ENTITY_CREATION_ERROR_TEMPLATE.replace("{entity_name}", &entity_name));
            error_enums.push(ENTITY_DOES_NOT_EXIST_ERROR_TEMPLATE.replace("{entity_name}", &entity_name));
            error_enums.push(ENTITY_UPDATE_ERROR_TEMPLATE.replace("{entity_name}", &entity_name));
            error_enums.push(ENTITY_DELETION_ERROR_TEMPLATE.replace("{entity_name}", &entity_name));
            error_enums.push(ENTITY_FETCH_ERROR_TEMPLATE.replace("{entity_name}", &entity_name));
        }
        self.generate_enum("Error", error_enums)
    }

    fn generate_client_error_enums(&self, entity_names: Vec<String>) -> String {
        let mut error_enums: Vec<String> = Vec::new();
        for entity_name in entity_names {
            let usc_entity_name = to_upper_snake_case(&entity_name);
            error_enums.push(CLIENT_ENTITY_ALREADY_EXISTS_ERROR_TEMPLATE.replace("{usc_entity_name}", &usc_entity_name));
            error_enums.push(CLIENT_ENTITY_CREATION_ERROR_TEMPLATE.replace("{usc_entity_name}", &usc_entity_name));
            error_enums.push(CLIENT_ENTITY_DOES_NOT_EXIST_ERROR_TEMPLATE.replace("{usc_entity_name}", &usc_entity_name));
            error_enums.push(CLIENT_ENTITY_UPDATE_ERROR_TEMPLATE.replace("{usc_entity_name}", &usc_entity_name));
            error_enums.push(CLIENT_ENTITY_DELETION_ERROR_TEMPLATE.replace("{usc_entity_name}", &usc_entity_name));
            error_enums.push(CLIENT_ENTITY_FETCH_ERROR_TEMPLATE.replace("{usc_entity_name}", &usc_entity_name));
        }
        self.generate_enum("ClientError", error_enums)
    }

    fn generate_error_impl(&self, entity_names: Vec<String>) -> String {
        let mut error_to_client_errors = String::new();
        for entity_name in entity_names {
            let usc_entity_name = to_upper_snake_case(&entity_name);
            error_to_client_errors.push_str(&ERROR_TO_CLIENT_ERROR_EXIST_TEMPLATE
                .replace("{entity_name}", &ENTITY_ALREADY_EXISTS_ERROR_TEMPLATE.replace("{entity_name}", &entity_name))
                .replace("{usc_entity_name}", &usc_entity_name));
            error_to_client_errors.push_str(&ERROR_TO_CLIENT_ERROR_CREATION_TEMPLATE
                .replace("{entity_name}", &ENTITY_CREATION_ERROR_TEMPLATE.replace("{entity_name}", &entity_name))
                .replace("{usc_entity_name}", &usc_entity_name));
            error_to_client_errors.push_str(&ERROR_TO_CLIENT_ERROR_DOES_NOT_EXIST_TEMPLATE
                .replace("{entity_name}", &ENTITY_DOES_NOT_EXIST_ERROR_TEMPLATE.replace("{entity_name}", &entity_name))
                .replace("{usc_entity_name}", &usc_entity_name));
            error_to_client_errors.push_str(&ERROR_TO_CLIENT_ERROR_UPDATE_TEMPLATE
                .replace("{entity_name}", &ENTITY_UPDATE_ERROR_TEMPLATE.replace("{entity_name}", &entity_name))
                .replace("{usc_entity_name}", &usc_entity_name));
            error_to_client_errors.push_str(&ERROR_TO_CLIENT_ERROR_DELETION_TEMPLATE
                .replace("{entity_name}", &ENTITY_DELETION_ERROR_TEMPLATE.replace("{entity_name}", &entity_name))
                .replace("{usc_entity_name}", &usc_entity_name));
            error_to_client_errors.push_str(&ERROR_TO_CLIENT_ERROR_FETCH_TEMPLATE
                .replace("{entity_name}", &ENTITY_FETCH_ERROR_TEMPLATE.replace("{entity_name}", &entity_name))
                .replace("{usc_entity_name}", &usc_entity_name));
        }
        ERROR_IMPL_TEMPLATE.replace("{error_to_client_errors}", &error_to_client_errors)
    }

}