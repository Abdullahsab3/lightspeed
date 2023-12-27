# lightspeed
Lightspeed: Speed up the development process and generate CRUD based (reactive) microservices from your domain-specific entities


# TODO (MVP)
- Add support for reading operations -> Done
- Add support for filtering operations -> Done
- Add support for constraints and indexing -> Done
- Add support for foreign keys, primary keys, unique keys -> Done
- Generate docker-compose to spin up the database -> Done
- Generate config.toml file -> Done
- Generate Cargo.toml file -> Done 

# TODO next MVP version
- Add support for IaC (Kubernetes & Docker)
- Fix technical debt:
    - Unify all the templates to always use the same keywords-
    - Unify the generation functions into one set of functions that always fill the same fields
- Add support for tests and swagger documentation
- Add support for kafka events
- Add support for foreign keys


# Long term vision
- The bigger picture: A universal template engine for CRUD microservices: All you have to do, is define the template in a language of choice, using the template keywords and the language conventions.
- True modularity: The ability to accumulate different templates in order to introduce new features in the generated service (for example generating kafka when you need it)
- Support for custom indentation and linting. Everything related to this is at this point hardcoded.
- Optimisations: Incremental computation for example can speed up stuff.
- Support for self reference within an entity (if you have nested entities like categories for example)
- Code injection in existing services. If you want to add a new entity to a service for instance. Incremental computation can also be interesting here to apply.
- And a whole list of other things :)
