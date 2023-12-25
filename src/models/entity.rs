use serde::{Serialize, Deserialize};
use serde_json::Value;

use super::ddr_req::AttributeType;

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

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub attributes: Vec<(AttributeName, AttributeType)>,
    pub primary_key: Option<String>,
    pub foreign_keys: Vec<ForeginKey>,
    pub unique_attributes: Vec<UniqueAttributes>,
    pub filter_by: Vec<FilterBy>,
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

        let primary_key = raw_entity.get("primary_key").map(|primary_key| primary_key.as_str().unwrap().to_string()).or(Some("id".to_string()));

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