use crate::{models::entity::{Entity, FilterBy}, utils::naming_convention::to_snake_case};

pub static CREATE_ENTITY_QUERY: &str = r#"
            INSERT INTO {sc_plural_entity}
                ({entity_fields})
            VALUES
                ({entity_values})
            RETURNING *;
"#;

pub static GET_ENTITY_QUERY: &str = r#"
            SELECT * FROM {sc_plural_entity}
            WHERE {primary_key} = {entity_id};
"#;

pub static FILTER_BY_QUERY: &str = r#"
            SELECT * FROM {sc_plural_entity}
            WHERE {filter_by_fields};
"#;


pub static FILTER_BY_PAGINATED_QUERY: &str = r#"
            SELECT * FROM {sc_plural_entity}
            WHERE {filter_by_fields}
            LIMIT {limit} OFFSET {offset};
"#;

pub static FILTER_BY_PAGINATED_COUNT_QUERY: &str = r#"
            SELECT COUNT(*) FROM {sc_plural_entity}
            WHERE {filter_by_fields};
"#;

pub static FILTER_BY_FIELD: &str = r#"{field_name} = ${arg_num}"#;

pub static GET_PAGINATED_QUERY: &str = r#"
            SELECT * FROM {sc_plural_entity}
            LIMIT {limit} OFFSET {offset};
"#;

pub static COUNT_ENTITY_QUERY: &str = r#"
            SELECT COUNT(*) FROM {sc_plural_entity};
"#;

pub static UPDATE_ENTITY_QUERY: &str = r#"
            UPDATE {sc_plural_entity}
            SET 
                {entity_fields}
            WHERE {primary_key} = {entity_id}
            RETURNING *;
"#;

pub static DELETE_ENTITY_QUERY: &str = r#"
            DELETE FROM {sc_plural_entity}
            WHERE {primary_key} = {entity_id};
"#;

pub trait CrudQueryGenerator {
    fn generate_create_query(&self, entity: &Entity) -> String {
        let sc_plural_entity = to_snake_case(&entity.plural_name);
        let mut entity_fields = Vec::new();
        let mut entity_values = Vec::new();
        for (arg_num, (attribute_name, _)) in entity.attributes.iter().enumerate() {
            entity_fields.push(format!("{}", attribute_name));
            entity_values.push(format!("${}", arg_num + 1));
        }
        let entity_fields = entity_fields.join(", ");
        let entity_values = entity_values.join(", ");
        CREATE_ENTITY_QUERY
            .replace("{sc_plural_entity}", &sc_plural_entity)
            .replace("{entity_fields}", &entity_fields)
            .replace("{entity_values}", &entity_values)
    }

    fn generate_get_query(&self, entity: &Entity) -> String {
        let sc_plural_entity = to_snake_case(&entity.plural_name);
        let primary_key = &entity.primary_key;
        let entity_id = &format!("${}", 1);
        GET_ENTITY_QUERY
            .replace("{sc_plural_entity}", &sc_plural_entity)
            .replace("{primary_key}", &primary_key)
            .replace("{entity_id}", &entity_id)
    }

    fn generate_get_paginated_query(&self, entity: &Entity) -> String {
        let sc_plural_entity = to_snake_case(&entity.plural_name);
        let limit = &format!("${}", 1);
        let offset = &format!("${}", 2);
        GET_PAGINATED_QUERY
            .replace("{sc_plural_entity}", &sc_plural_entity)
            .replace("{limit}", &limit)
            .replace("{offset}", &offset)
    }

    fn generate_filter_by_paginated_query(&self, entity: &Entity, filter_attr: &FilterBy) -> String {
        let sc_plural_entity = to_snake_case(&entity.plural_name);
        let filter_by_fields = filter_attr.iter().enumerate().map(|(arg_num, field_name)| {
            FILTER_BY_FIELD
                .replace("{field_name}", &field_name)
                .replace("{arg_num}", &format!("{}", arg_num + 1))
        }).collect::<Vec<String>>().join(" AND ");
        FILTER_BY_PAGINATED_QUERY
                .replace("{sc_plural_entity}", &sc_plural_entity)
                .replace("{filter_by_fields}", &filter_by_fields)
                .replace("{limit}", &format!("${}", filter_attr.len() + 1))
                .replace("{offset}", &format!("${}", filter_attr.len() + 2))
    }

    fn generate_filter_by_paginated_count_query(&self, entity: &Entity, filter_attr: &FilterBy) -> String {
        let sc_plural_entity = to_snake_case(&entity.plural_name);
        let filter_by_fields = filter_attr.iter().enumerate().map(|(arg_num, field_name)| {
            FILTER_BY_FIELD
                .replace("{field_name}", &field_name)
                .replace("{arg_num}", &format!("{}", arg_num + 1))
        }).collect::<Vec<String>>().join(" AND ");
        FILTER_BY_PAGINATED_COUNT_QUERY
                .replace("{sc_plural_entity}", &sc_plural_entity)
                .replace("{filter_by_fields}", &filter_by_fields)
    }

    fn generate_filter_by_query(&self, entity: &Entity, filter_attr: &FilterBy) -> String {
        let sc_plural_entity = to_snake_case(&entity.plural_name);
        let filter_by_fields = filter_attr.iter().enumerate().map(|(arg_num, field_name)| {
            FILTER_BY_FIELD
                .replace("{field_name}", &field_name)
                .replace("{arg_num}", &format!("{}", arg_num + 1))
        }).collect::<Vec<String>>().join(" AND ");
        FILTER_BY_QUERY
                .replace("{sc_plural_entity}", &sc_plural_entity)
                .replace("{filter_by_fields}", &filter_by_fields)
    }

    fn generate_count_query(&self, entity: &Entity) -> String {
        COUNT_ENTITY_QUERY.replace("{sc_plural_entity}", &to_snake_case(&entity.plural_name))
    }
    
    fn generate_update_query(&self, entity: &Entity) -> String {
        let sc_plural_entity = to_snake_case(&entity.plural_name);
        let mut entity_fields = Vec::new();
        for (arg_num, (attribute_name, _)) in entity.attributes.iter().enumerate() {
            entity_fields.push(format!("{} = ${}", attribute_name, arg_num + 1 ));
        }
        let entity_fields = entity_fields.join(", ");
        UPDATE_ENTITY_QUERY
            .replace("{sc_plural_entity}", &sc_plural_entity)
            .replace("{entity_fields}", &entity_fields)
            .replace("{primary_key}", &entity.primary_key)
            .replace("{entity_id}", &format!("${}", entity.attributes.len() + 1))
    }
    fn generate_delete_query(&self, entity: &Entity) -> String {
        let sc_plural_entity = to_snake_case(&entity.plural_name);
        DELETE_ENTITY_QUERY
            .replace("{sc_plural_entity}", &sc_plural_entity)
            .replace("{primary_key}", &&entity.primary_key)
            .replace("{entity_id}", &format!("${}", 1))
    }
}