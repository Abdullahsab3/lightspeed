use crate::{utils::naming_convention::{to_snake_case, to_snake_case_plural, to_plural}, models::{entity::AttributeType, entity::Entity}};

use super::{model_templates::ATTRIBUTE_TEMPLATE, import_templates::ImportGenerator};

pub static CONTROLLER_CREATE_ENTITY_TEMPLATE: &str = r#"
pub async fn create_{sc_entity_name}(
    State(services): State<Arc<ServicesState>>,
    Json(payload): Json<Add{entity_name}Payload>
) -> Result<impl IntoResponse> {
    services
        .{sc_plural_entity}_service
        .create_{sc_entity_name}(payload)
        .await
        .map(|{sc_entity_name}| {
            (StatusCode::CREATED, Json({sc_entity_name}))
        })
}
"#;

pub static CONTROLLER_GET_ENTITY_TEMPLATE: &str = r#"
pub async fn get_{sc_entity_name}(
    Path(id): Path<Uuid>,
    State(services): State<Arc<ServicesState>>,
) -> Result<impl IntoResponse> {
    return services
            .{sc_plural_entity}_service
            .get_{sc_entity_name}(&id)
            .await
            .map(|{sc_entity_name}| {
                (StatusCode::OK, Json({entity_name}Response::{entity_name}({sc_entity_name})))
            });
}
"#;

pub static CONTROLLER_GET_PAGINATED_ENTITIES_TEMPLATE: &str = r#"
pub async fn get_{sc_plural_entity}(
    Query(filter_params): Query<{entity_name}FilterParams>,
    State(services): State<Arc<ServicesState>>,
) -> Result<impl IntoResponse> {
    {filter_by}
    return services
            .{sc_plural_entity}_service
            .get_{sc_plural_entity}(
                filter_params.page.unwrap_or(1),
                filter_params.page_size.unwrap_or(10)
            )
            .await
            .map(|{sc_plural_entity}| {
                (StatusCode::OK, Json({entity_name}Response::{plural_entity}({sc_plural_entity})))
            });
    }
"#;

pub static FILTER_BY_PAGINATED_TEMPLATE: &str = r#"
    if filter_params.is_{attribute_name}_filter() {
        return services
                .{sc_plural_entity}_service
                .filter_{sc_plural_entity}_by_{attribute_name}(
                    {filter_by_fields},
                    filter_params.page.unwrap_or(1),
                    filter_params.page_size.unwrap_or(10)
                )
                .await
                .map(|{sc_plural_entity}| {
                    (StatusCode::OK, Json({entity_name}Response::{plural_entity_name}({sc_plural_entity})))
                });
    }
"#;

pub static FILTER_BY_TEMPLATE: &str = r#"
    if filter_params.is_{attribute_name}_filter() {
        return  services
                .{sc_plural_entity}_service
                .get_{sc_plural_entity}_by_{attribute_name}(
                    {filter_by_fields}
                )
                .await
                .map(|{sc_entity_name}| {
                    (StatusCode::OK, Json({entity_name}Response::{entity_name}({sc_entity_name})))
                })
    }
"#;

pub static CONTROLLER_CREATE_ENTITY_PAYLOAD_TEMPLATE: &str = r#"
#[derive(Deserialize)]
pub struct Add{entity_name}Payload {
    {attributes}
}
"#;


pub static CONTROLLER_UPDATE_ENTITY_TEMPLATE: &str = r#"
pub async fn update_{sc_entity_name}(
    Path(id): Path<Uuid>,
    State(services): State<Arc<ServicesState>>,
    Json(payload): Json<Update{entity_name}Payload>
) -> Result<impl IntoResponse> {
    services
        .{sc_plural_entity}_service
        .update_{sc_entity_name}(&id, payload)
        .await
        .map(|{sc_entity_name}| {
            (StatusCode::OK, Json({sc_entity_name}))
        })
}
"#;

pub static CONTROLLER_UPDATE_ENTITY_ATTRIBUTE_TEMPLATE: &str = r#"
    pub {attribute_name}: Option<{attribute_type}>,"#;

pub static CONTROLLER_UPDATE_ENTITY_PAYLOAD_TEMPLATE: &str = r#"
#[derive(Deserialize)]
pub struct Update{entity_name}Payload {
    {attributes}
}
"#;

pub static CONTROLLER_DELETE_ENTITY_TEMPLATE: &str = r#"
pub async fn delete_{sc_entity_name}(
    Path(id): Path<Uuid>,
    State(services): State<Arc<ServicesState>>,
) -> Result<impl IntoResponse> {
    services
        .{sc_plural_entity}_service
        .delete_{sc_entity_name}(&id)
        .await
}
"#;

pub static CONTROLLER_FILE_TEMPLATE: &str = r#"
use std::sync::Arc;

use crate::models::PaginatedParams;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::Result;
use crate::services::ServicesState;

{imports}

{controller_functions}

{controller_payloads}
"#;



pub trait ControllerGenerator: ImportGenerator {
    fn generate_create_fn(&self, entity: &Entity) -> String {
        CONTROLLER_CREATE_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(&entity.name).as_str())
            .replace("{sc_plural_entity}", to_snake_case_plural(&entity.name).as_str())
            .replace("{entity_name}", &entity.name)
    }

    fn generate_get_fn(&self, entity: &Entity) -> String {
        CONTROLLER_GET_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(&entity.name).as_str())
            .replace("{sc_plural_entity}", to_snake_case_plural(&entity.name).as_str())
            .replace("{plural_entity}", to_plural(&entity.name).as_str())
            .replace("{entity_name}", &entity.name)
    }

    fn generate_get_paginated_fn(&self, entity: &Entity) -> String {
        let filters = entity.filter_by.iter().map(|filter_by| {
            let filter_by_fields = filter_by.iter().map(|field| {
                format!("&filter_params.{}.unwrap()", field)
            }).collect::<Vec<String>>().join(", ");
            let most_specific_filter_by = filter_by.last().unwrap();
            if filter_by.iter().filter(|field| entity.is_unique(field)).count() > 0 {
                FILTER_BY_TEMPLATE
                .replace("{attribute_name}", most_specific_filter_by)
                .replace("{filter_by_fields}", &filter_by_fields)
                .replace("{sc_plural_entity}", to_snake_case_plural(&entity.name).as_str())
                .replace("{plural_entity_name}", to_plural(&entity.name).as_str())
                .replace("{sc_entity_name}", to_snake_case(&entity.name).as_str())
                .replace("{entity_name}", &entity.name)

            } else {
                FILTER_BY_PAGINATED_TEMPLATE
                .replace("{attribute_name}", most_specific_filter_by)
                .replace("{filter_by_fields}", &filter_by_fields)
                .replace("{sc_plural_entity}", to_snake_case_plural(&entity.name).as_str())
                .replace("{plural_entity_name}", to_plural(&entity.name).as_str())
                .replace("{entity_name}", &entity.name)
                

            }
           
        }).collect::<Vec<String>>().join("\n");
        
        CONTROLLER_GET_PAGINATED_ENTITIES_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(&entity.name).as_str())
            .replace("{sc_plural_entity}", to_snake_case_plural(&entity.name).as_str())
            .replace("{entity_name}", &entity.name)
            .replace("{filter_by}", &filters)
            .replace("{plural_entity}", to_plural(&entity.name).as_str())
    }

    fn generate_update_fn(&self, entity: &Entity) -> String {
        CONTROLLER_UPDATE_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(&entity.name).as_str())
            .replace("{sc_plural_entity}", to_snake_case_plural(&entity.name).as_str())
            .replace("{entity_name}", &entity.name)
    }

    fn generate_delete_fn(&self, entity: &Entity) -> String {
        CONTROLLER_DELETE_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(&entity.name).as_str())
            .replace("{sc_plural_entity}", to_snake_case_plural(&entity.name).as_str())
            .replace("{entity_name}", &entity.name)
    }

    fn generate_create_payload(&self, entity: &Entity) -> String {
        let mut attributes = String::new();
        for (attribute_name, attribute_type) in &entity.attributes {
            if attribute_name == &entity.primary_key {
                continue;
            }
            
            attributes.push_str(&ATTRIBUTE_TEMPLATE
                .replace("{attribute_name}", &attribute_name)
                .replace("{attribute_type}", &attribute_type.to_string()));
        }
        CONTROLLER_CREATE_ENTITY_PAYLOAD_TEMPLATE
            .replace("{entity_name}", &entity.name)
            .replace("{attributes}", &attributes)
    }
    

    fn generate_update_payload(&self, entity: &Entity) -> String {
        let mut attributes = String::new();
        for (attribute_name, attribute_type) in &entity.attributes {
            if attribute_name == &entity.primary_key {
                continue;
            }
            let attribute_type_str = match attribute_type {
                AttributeType::Option(t) => t.to_string(),
                _ => attribute_type.to_string()
            };
            attributes.push_str(&CONTROLLER_UPDATE_ENTITY_ATTRIBUTE_TEMPLATE
                .replace("{attribute_name}", &attribute_name)
                .replace("{attribute_type}", &attribute_type_str));
        }
        CONTROLLER_UPDATE_ENTITY_PAYLOAD_TEMPLATE
            .replace("{entity_name}", &entity.name)
            .replace("{attributes}", &attributes)
    }

    fn generate_controller(&self, entity: &Entity) -> String {
        let mut controller_functions = String::new();
        controller_functions.push_str(&self.generate_create_fn(&entity));
        controller_functions.push_str(&self.generate_get_fn(&entity));
        controller_functions.push_str(&self.generate_get_paginated_fn(&entity));
        controller_functions.push_str(&self.generate_update_fn(&entity));
        controller_functions.push_str(&self.generate_delete_fn(&entity));
        

        let mut controller_payloads = String::new();
        controller_payloads.push_str(&self.generate_create_payload(&entity));
        controller_payloads.push_str(&self.generate_update_payload(&entity));

        CONTROLLER_FILE_TEMPLATE
            .replace("{imports}", &self.generate_model_imports(&entity.name))
            .replace("{controller_functions}", &controller_functions)
            .replace("{controller_payloads}", &controller_payloads)
    }
    
}