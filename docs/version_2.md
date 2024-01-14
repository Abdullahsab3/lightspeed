This document describes how version 2 is going to be implemented in order to have the the template as an input.

The template will be stored in a folder, the path of which can be provided as input.

The idea would be as the following:

We start from the following variables that the user can use as building blocks:
- service name
- entity name: snake case and camel case
- entity name plural: snake case and camel case
- attribute name
- attribute type

We can also use some basic functions that might be useful:
- rep: It can be used for: repetition for entities, attributes, numbers, letters.

The user can enrich those variable with variables they define their own in order to introduce some abstraction and flexibility. For instance they can define:

```
GET_ENTITY_QUERY := SELECT * FROM {sc_entity_name_plural}
```

which can then be used in another place:
```
sqlx::query_as!({entity_name}, GET_ENTITY_QUERY)
```

In order for a file to generated for each entity, the name of the file can be `rep{entity_name}_model.rs` for example.

The user should provide mapping between the types used in the template and the types used by lightspeed. For instance, the user can define that the type `String` in the template should be mapped to the type `Text` in lightspeed. The mapping is only valid within the directory in which it is defined. This way, it can be possible to have different mappings for different technologies (eg Rust, postgresql, etc)

We also need a way to make a distinction between files that contain templating code which should only be used in places in which that templating code is used, and files that should be copied to the output. Postgres queries for example will be injected in places where they are needed, but the folder and files containing the definitions for the postgres templates should not be copied to the output.

This means that we have several kinds of files:
- Static files. They do not contain any templating code and should be copied to the output as is.
- Templating files that are used by other files, but should not be copied to the output.
- Templating files that are used by other files, and are part of the output.

## How to define the templates?

We can use `a# #a` as braces to define templates within the curly braces. This can be for example:
```
a#
pub struct {camel_case_entity_name} {
    {entity_attributes}
}

entity_attributes := rep{entity_attribute}

entity_attribute := {snake_case_attribute_name} : {attribute_type}
#a
```

We can have several types of template expressions:
- Static text: These expressions are defined outside the  `a# #a` braces and are copied to the output as is.
- Dynamic text: These expressions are defined inside the `a# #a` braces and are evaluated and copied to the output. We can have two parts of dynamic expressions:
    - anonymous expressions: These expressions are evaluated and copied to the output. The `struct` template defined above for example is an anonymous expression.
    - named expressions: These expressions are used to define variables that can be used in anonymous expressions. They are only evaluated when an anonymous expression that uses them is evaluated. For example, the `entity_attributes` template defined above is a named expression.
    named expressions are defined using the following syntax:
        - `name := expression` where `name` is the name of the expression and `expression` is the expression itself. The expression can use other named expressions that are defined before it. For example, the `entity_attributes` template uses the `entity_attribute` template that is defined before it.

We need to have a way to encapsulate the dynamic expressions in a way that we can easily implement the evaluation of the expressions. We can encapsulate the following properties:
- For anonymous expressions:
    - The place in which the expression is defined
    - The expression itself
    - The variables that are used in the expression
- For named expressions:
    - The expression name
    - The place in which the expression is defined
    - The expression itself
    - The variables that are used in the expression

There should be some restrictions added in order to make the implementation of this version easier:
- It is not possible to mix static and dynamic expressions in the same file. This means that a file should either contain only static expressions, or only dynamic expressions. However, this is a restriction I would like to get rid of in the next version, hence the `a# #a` braces.

## How to define the type mappings

Type mappings is a mapping between the types used in the template and the types used by lightspeed. For instance, the user can define that the type `Int` in the template should be mapped to the type `i32` in lightspeed. The mapping is only valid within the directory in which it is defined. This way, it can be possible to have different mappings for different technologies (eg Rust, postgresql, etc). It should be defined in a seperate file called `type_mappings.json` in the same directory as the template files that use this mapping. The subdirectories of the directory in which the `type_mappings.json` file is defined should also use the same mapping.

## How to evaluate the templates
Evaluating the template and generating the output happens by the following steps:
- Read the template files and the type mappings
- Parse the type mappings, keeping in mind that the environment in which the mapping is valid, is the directory in which the mapping is defined and its subdirectories. A mapping can be overridden in a subdirectory.
- parse the template files. Parse the dynamic expressions in the template files and encapsulate them. Parsing the dynamic expressions would require:
    - parsing the variables
    - parsing the functions that can be used (eg `rep`)
- Evaluate the dynamic expressions. This would require:
    - For named expressions: lookup the expressions in the environment and evaluate it.
    - For `entity_name` variable: lookup the entity in the environment and replace it with the name of the entity. Evaluate the other entity variables (eg `entity_name_plural`) in the same way.
    - For `attribute_name` variable: lookup the attribute in the environment and replace it with the name of the attribute.
    - For `attribute_type` variable: lookup the type mapping in the environment, and lookup the attribute in the environment and replaced it with the mapped type. There is one restriction: `attribute_type` can only be used when `attribute_name` is used in the same expression.

There is one important resitrction: `attribute_name` and `attribute_type` can only be used
    