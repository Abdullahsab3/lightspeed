# General Idea

Generating a reactive microservice can look like this:

- Get the domain driven design from the user as a JSON file for example. This JSON file can be something like this:
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
                "age": "Int",
                "email": "String",
                "primary_key": "id",
                "filter_by": ["name", "age", ["name", "surname"]],
                "unique_attributes": ["email"]
                // composite filters are filtered like this: major, minor, subminor, etc.
            }
        },
        {
            "Car" : 
            {
                "id": "Uuid",
                "name" : "String",
                "brand": "String",
                "price": "Int",
                "ownedBy": "User.id",
                "primary_key": "id",
                "filter_by": ["name", "brand"]
            }
        }
    ],
    "kafka": {
        "topic": "my_topic",
        "bootstrap_servers": "localhost:9092"
    },

    "container_registry": {
        "uri": "eu.gcr.io/my-project",
        "username": "_json_key",
        "password": "my-key.json"
    }
}

```
The idea can be futher extended to support more complex types like arrays, maps, enums, etc, or to have more complex relationships between entities.

The things I would like to do in the future are:
- Possibility to generate the code in different languages, for different frameworks and libraries
- Possibility to generate different kinds of services like gRPC, event sourcing (CQRS), ingestion pipeline services, etc.
- Possibility to have more complex relations between attributes.
- Though described in the JSON file, custom container registry, Kafka configurations and Kubernetes configurations are a nice to have.


Why not get a database schema instead of a JSON file? I would like to decouple traditional relational databases from this idea. If you would like to generate a CQRS based service or a gRPC service that relies on event sourcing for example, you may not have a traditional database.

# High-Level Technical Details

An index can be added to the filter attributes to speed up the filtering process.