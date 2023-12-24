use crate::utils::naming_convention::to_snake_case;

pub static STRUCT_TEMPLATE: &str = r#"
#[derive(Serialize, Deserialize)]
pub struct {struct_name} {
    {attributes}
}
"#;

pub static ATTRIBUTE_TEMPLATE: &str = r#"
    pub {attribute_name}: {attribute_type},"#;


pub static CONTROLLER_CREATE_ENTITY_TEMPLATE: &str = r#"
pub async fn create_{sc_entity_name}(
    State(services): State<Arc<ServicesState>>,
    Json(payload): Json<Add{entity_name}Payload>
) -> Result<impl IntoResponse> {
    services
        .{sc_entity_name}_service
        .create_{sc_entity_name}(payload)
        .await
        .map(|{sc_entity_name}| {
            (StatusCode::CREATED, Json(entity)))
        })
}
"#;

pub static CONTROLLER_UPDATE_ENTITY_TEMPLATE: &str = r#"
pub async fn update_{sc_entity_name}(
    State(services): State<Arc<ServicesState>>,
    Json(payload): Json<Update{entity_name}Payload>
) -> Result<impl IntoResponse> {
    services
        .{sc_entity_name}_service
        .update_{sc_entity_name}(payload)
        .await
        .map(|{sc_entity_name}| {
            (StatusCode::UPDATED, Json(entity)))
        })
}
"#;

pub static CONTROLLER_DELETE_ENTITY_TEMPLATE: &str = r#"
pub async fn delete_{sc_entity_name}(
    State(services): State<Arc<ServicesState>>,
    Json(payload): Json<Delete{entity_name}Payload>
) -> Result<impl IntoResponse> {
    services
        .{sc_entity_name}_service
        .delete_{sc_entity_name}(payload)
        .await
}
"#;

pub trait ControllerGenerator {
    fn generate_create_fn(&self, entity_name: &str) -> String {
        println!("entity_name: {}", to_snake_case(entity_name));
        CONTROLLER_CREATE_ENTITY_TEMPLATE
            .replace("{sc_entity_name}", to_snake_case(entity_name).as_str())
            .replace("{entity_name}", &entity_name)
    }
    
}