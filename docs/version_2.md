This document describes how version 2 is going to be implemented in order to have the the template as an input.

The template will be stored in a folder, the path of which can be provided as input.

The idea would be as the following:

We start from the following variables that the user can use as building blocks:
- service name
- entity name: snake case and camel case
- entity name plural: snake case and camel case
- attribute name
- attribute type

The user can enrich those variable with variables they define their own in order to introduce some abstraction and flexibility. For instance they can define:

```
GET_ENTITY_QUERY := SELECT * FROM {sc_entity_name_plural}
```

which can then be used in another place:
```
sqlx::query_as!({entity_name}, GET_ENTITY_QUERY)
```

In order for a file to generated for each entity, the name of the file can be `{entity_name}_model.rs` for example.

The user should provide mapping between the types used in the template and the types used by lightspeed. For instance, the user can define that the type `String` in the template should be mapped to the type `Text` in lightspeed. The mapping is only valid within the file in which it is defined. This way, it can be possible to have different mappings for different technologies (eg Rust, postgresql, etc).

