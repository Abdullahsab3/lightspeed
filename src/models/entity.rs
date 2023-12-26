use serde::{Serialize, Deserialize};
use serde_json::Value;
use strum::EnumProperty;

pub type RawEntities = Value;
pub type RawEntity = Value;

pub type AttributeName = String;
pub type EntityName = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct ForeginKey {
    pub entity_name: String,
    pub attribute_name: String,
}

impl ForeginKey {
    pub fn from_str(foreign_reference: &str) -> Option<Self> {
        let foreign_reference = foreign_reference.split(".").collect::<Vec<&str>>();
        if foreign_reference.len() != 2 {
            return None;
        }
        let entity_name = foreign_reference[0].to_string();
        let attribute_name = foreign_reference[1].to_string();
        Some(ForeginKey {
            entity_name,
            attribute_name,
        })
    
    }
}

pub type FilterBy = Vec<AttributeName>;
pub type UniqueAttributes = Vec<AttributeName>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub attributes: Vec<(AttributeName, AttributeType)>,
    pub primary_key: String,
    pub foreign_keys: Vec<ForeginKey>,
    pub unique_attributes: Vec<UniqueAttributes>,
    pub filter_by: Vec<FilterBy>,
}

impl Entity {
    pub fn is_last(&self, attribute_name: &str) -> bool {
        attribute_name == self.attributes.last().unwrap().0.as_str()
    }

    pub fn primary_key_type(&self) -> &AttributeType {
        &self.attributes.iter().find(|(attribute_name, _)| attribute_name == &self.primary_key).unwrap().1
    }
}

impl From<(EntityName, RawEntity, RawEntities)> for Entity {
    fn from((entity_name, raw_entity, raw_entities): (EntityName, RawEntity, RawEntities)) -> Self {
        let mut attributes = raw_entity.as_object().unwrap().iter().map(|(attribute_name, attribute_type)| {
            let str_attribute_type = attribute_type.as_str().unwrap_or("Unknown");
            let attribute_type = AttributeType::from_str(str_attribute_type);
            (attribute_name.to_string(), attribute_type)
        }).collect::<Vec<(AttributeName, AttributeType)>>();
        
        let mut unknown_attributes = Vec::new();

        let mut foreign_keys = Vec::new();

        for attribute in attributes.iter_mut() {
            match &attribute.1 {
                AttributeType::Unknown(raw_attr_type) => {
                    let foreign_key_ref = ForeginKey::from_str(&raw_attr_type);
                    match foreign_key_ref {
                        Some(foreign_key_ref) => {
                            let foreign_key_entity = raw_entities.as_array().unwrap().iter().find(|entity| {
                                entity.as_object().unwrap().contains_key(&foreign_key_ref.entity_name)
                            }).unwrap().as_object().unwrap().get(&foreign_key_ref.entity_name).unwrap().as_object().unwrap();
                            let foreign_key_attribute_type = foreign_key_entity.get(&foreign_key_ref.attribute_name).unwrap();
                            let foreign_key_attribute_type = AttributeType::from_str(foreign_key_attribute_type.as_str().unwrap());
                            attribute.1 = foreign_key_attribute_type;
                            foreign_keys.push(foreign_key_ref);
                        },
                        None => {
                            unknown_attributes.push(attribute.0.clone());
                        }
                    }

                }
                _ => {}
            }
        }

        let attributes = attributes.into_iter().filter(|attribute| !unknown_attributes.contains(&attribute.0)).collect::<Vec<(AttributeName, AttributeType)>>();

        let primary_key = raw_entity.get("primary_key").map(|primary_key| primary_key.as_str().unwrap().to_string()).unwrap_or("id".to_string());

        let unique_attributes = raw_entity.get("unique_attributes").map(|unique_attributes| {
            unique_attributes.as_array().unwrap().iter().map(|unique_attribute| {
                match unique_attribute.as_array() {
                    Some(unique_attribute) => unique_attribute.iter().map(|attribute| attribute.as_str().unwrap().to_string()).collect::<UniqueAttributes>(),
                    None => vec![unique_attribute.as_str().unwrap().to_string()],
                }
            }).collect::<Vec<UniqueAttributes>>()
        }).unwrap_or(vec![]);

        let filter_by = raw_entity.get("filter_by").map(|filter_by| {
            filter_by.as_array().unwrap().iter().map(|filter_by| {
                match filter_by.as_array() {
                    Some(filter_by) => filter_by.iter().map(|attribute| attribute.as_str().unwrap().to_string()).collect::<FilterBy>(),
                    None => vec![filter_by.as_str().unwrap().to_string()],
                }
            }).collect::<Vec<FilterBy>>()
        }).unwrap_or(vec![]);


        Entity {
            name: entity_name,
            attributes,
            primary_key,
            foreign_keys,
            unique_attributes,
            filter_by,
        }
    }
        
}


#[derive(PartialEq, Debug, Serialize, Deserialize)]
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
impl From<&AttributeType> for PostgresAttributeType {
    fn from(attribute_type: &AttributeType) -> Self {
        match attribute_type {
            AttributeType::String => PostgresAttributeType::VARCHAR,
            AttributeType::Uuid => PostgresAttributeType::UUID,
            AttributeType::I32 => PostgresAttributeType::INT,
            AttributeType::I64 => PostgresAttributeType::BIGINT,
            AttributeType::F32 => PostgresAttributeType::REAL,
            AttributeType::F64 => PostgresAttributeType::DOUBLE_PRECISION,
            AttributeType::Boolean => PostgresAttributeType::BOOLEAN,
            AttributeType::UTCDateTime => PostgresAttributeType::TIMESTAMP,
            AttributeType::Option(attribute_type) => PostgresAttributeType::OPTION(Box::new(Into::<PostgresAttributeType>::into(attribute_type.as_ref()))),
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