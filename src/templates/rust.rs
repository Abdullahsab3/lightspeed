pub static STRUCT_TEMPLATE: &str = r#"
#[derive(Serialize, Deserialize)]
pub struct {struct_name} {
    {attributes}
}
"#;

pub static ATTRIBUTE_TEMPLATE: &str = r#"
    pub {attribute_name}: {attribute_type},"#;
