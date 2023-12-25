# lightspeed
Lightspeed: Speed up the development process and generate CRUD based (reactive) microservices from your domain-specific entities


# TODO (MVP)
- Add support for reading operations
- Add support for filtering operations
- Add support for kafka events
- Add support for foreign keys, primary keys, unique keys -> In progress
- Add support for IaC (Kubernetes & Docker)
- Generate docker-compose to spin up the database -> Done
- Generate config.toml file -> Done
- Generate Cargo.toml file -> Done 

# Nice to Have
- The bigger picture: A universal template engine for CRUD microservices: All you have to do, is define the template in a language of choice, using the template keywords and the language conventions.
- Support for custom indentation and linting. Everything related to this is at this point hardcoded.
- Optimisations: Incremental computation for example can speed up stuff.
- Support for self reference within an entity (if you have nested entities like categories for example)
- And a whole list of other things :)
