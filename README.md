# lightspeed
<p align="center">
<img src="md_assets/logo.jpeg"
     alt="Markdown Monster icon"
     width=300 />
</p>

Lightspeed: Speed up the development process and generate CRUD based (reactive) microservices from your domain-specific entities


# Motivation
In day-to-day software development, software engineers find themselves writing a lot of CRUD based (micro)services. Manually creating this kind of services is a tedious and repetitive job, and it can be prone to errors and bugs due to its nature of having a significant amount of boilerplate. A developer would typically need to set up the code for a new service, as well as the infrastructure that is needed in order to connect with the database, process the incoming requests, create the API, etc. This can distract the developer from the actual domain problems that they need to tackle. CRUD based services typically have the same characteristics, even though they might be implemented in different technologies or using different patterns. Abstracting the CRUD implementation away from the developer might let them focus on the domain-specific, more complex problems that they need to solve. This will also allow teams to deliver much faster.

The idea of generating programs and services from domain-specific description is not a new one. Generating Java programs from UML diagrams for instance has been existing for decades. The issue with these solutions, however, is that they are too coupled with certain technologies and patterns, which makes new technologies and ideas difficult to adapt. Furthermore, companies might have their own in-house technologies or patterns that they utilize. Therefore, it would make sense to have some kind of decoupling the programs that need to be generated, and the common technologies and patterns.

My vision on this issue is that a universal, template-driven language can be used to define service templates. This way, we decouple the technology from the domain specifications. It can be possible to switch to a new kind of technology at any time. The system architect will then have to just define a new template in the new technology.

The domain-driven description can also be decoupled from modelling technologies. It is certainly possible for a certain team to choose alternatives over UML. To this end, the request to generate a service is a well-defined JSON object, containing the domain definitions in a simple format. This allows for more options. It is possible to compile a UML diagram to that JSON, but you can also have an interactive user interface that allows the user to define the domain entities and their relationships.

# How it works: The Request
The domain entities can be described using the following JSON object:
```json
    {
        "service_name": "MyService",
        "entities": [
            {
                "User" : 
                {
                    "id": "Uuid",
                    "name" : "String",
                    "surname": "String",
                    "age": "i32",
                    "email": "String",
                    "primary_key": "id",
                    "filter_by": ["name", "age", ["name", "surname"], "email"] ,
                    "unique_attributes": ["email"]
                }
            },
            {
                "Car" : 
                {
                    "id": "Uuid",
                    "name" : "String",
                    "brand": "String",
                    "price": "Int",
                    "owned_by": "User.id",
                    "primary_key": "id",
                    "filter_by": ["name", "brand"]
                }
            }
        ]
    }
```
`primary_key`, `filter_by` and `unique_attributes` are reserved keywords and cannot be used as an attribute for the entity.

Defining a foreign attribute is done by using the following syntax: `entity_name.attribute_name`. In the example above, the `Car` entity has a foreign attribute `owned_by` that references the `id` attribute of the `User` entity.

# How it works: The Response
From the provided request, Lightspeed 'fills in' the spaces that are defined in the template, and stores the generated code in the provided folder. 

For now, only Rust is (partly) supported in a not-so-maintainable, hardcooded template. This is still a proof of concept that I wanted to solidify as fast as possible. The idea is to make it more modular and maintainable in the future.

# Usage
The program accepts two flas: A path to the input file containing the JSON representation, and a path to a dircetory where the generated code will be stored. The command is as follows:
```bash
cargo run -- -i <path_to_input_file> -o <path_to_output_directory>
```

# Current Version Supports
-  reading operations
-  filtering operations
-  constraints and indexing 
- foreign keys (in a very limited way), primary keys, unique keys
- docker-compose to spin up the database
- config.toml file
- Cargo.toml file

# TODO next MVP version
- Add support for IaC (Kubernetes & Docker)
- Fix technical debt:
    - Unify all the templates to always use the same keywords-
    - Unify the generation functions into one set of functions that always fill the same fields
- Add support for tests and swagger documentation
- Add support for kafka events
- Add support for foreign keys
- Fix issues of the initial MVP


# Long term vision
- The bigger picture: A universal template engine for CRUD microservices: All you have to do, is define the template in a language of choice, using the template keywords and the language conventions.
- True modularity: The ability to accumulate different templates in order to introduce new features in the generated service (for example generating kafka when you need it)
- Support for custom indentation and linting. Everything related to this is at this point hardcoded.
- Optimisations: Incremental computation for example can speed up stuff.
- Support for self reference within an entity (if you have nested entities like categories for example)
- Code injection in existing services. If you want to add a new entity to a service for instance. Incremental computation can also be interesting here to apply.
- And a whole list of other things :)
