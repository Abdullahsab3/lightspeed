pub static DATABASE_CONTAINER_TEMPLATE: &str = r#"
    {service_name}-db:
        image: postgres:13-alpine
        container_name: {service_name}-db
        restart: always
        command: [ "-c", "shared_buffers=256MB", "-c", "max_connections=500", "-c", "log_statement=all" ]
        environment:
            TZ: "Europe/Brussels"
            PGTZ: "Europe/Brussels"
            POSTGRES_USER: "postgres"
            POSTGRES_PASSWORD: "postgres"
      
        ports:
            - 5432:5432
        networks:
        - {service_name}-network
        volumes:
            - "{service_name}-data:/var/lib/postgresql/data"
            -"./docker/postgres/01.sql:/docker-entrypoint-initdb.d/01_db.sql"
"#;

pub static NETWORK_TEMPLATE: &str = r#"
networks:
    {service_name}-network:
        driver: bridge
"#;

pub static VOLUME_TEMPLATE: &str = r#"
volumes:
    {service_name}-data:
        driver: local
"#;

pub static ZOOKEEPER_CONTAINER_TEMPLATE: &str = r#"
    {service_name}-zookeeper:
        image: confluentinc/cp-zookeeper:latest
        restart: always
        container_name: {service_name}-zookeeper
        environment:
            ZOOKEEPER_CLIENT_PORT: 2181
            ZOOKEEPER_TICK_TIME: 2000
        ports:
            - 2181:2181
        networks:
            - {service_name}-network
"#;

pub static KAFKA_CONTAINER_TEMPLATE: &str = r#"
    {service_name}-kafka:
        image: confluentinc/cp-kafka:latest
        restart: always
        container_name: {service_name}-kafka
        depends_on:
            - {service_name}-zookeeper
        environment:
            KAFKA_BROKER_ID: 1
            KAFKA_ZOOKEEPER_CONNECT: {service_name}-zookeeper:2181
            KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://{service_name}-kafka:29092,PLAINTEXT_HOST://localhost:9092
            KAFKA_LISTENER_SECURITY_PROTOCOL_MAP: PLAINTEXT:PLAINTEXT,PLAINTEXT_HOST:PLAINTEXT
            KAFKA_INTER_BROKER_LISTENER_NAME: PLAINTEXT
            KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
            KAFKA_NUM_PARTITIONS: 20
        ports:
            - 9092:9092
            - 29092:29092
        networks:
            - {service_name}-network
"#;

pub static KAFDROP_CONTAINER_TEMPLATE: &str = r#"
    {service_name}-kafdrop:
        image: obsidiandynamics/kafdrop:latest
        restart: "no"
        container_name: {service_name}-kafdrop
        depends_on:
            - {service_name}-kafka
        environment:
            KAFKA_BROKERCONNECT: {service_name}-kafka:29092
        ports:
            - 7777:9000
        networks:
            - {service_name}-network
"#;

pub static SERVICES_TEMPLATE: &str = r#"
services:
{database_container}
{zookeeper_container}
{kafka_container}
{kafdrop_container}
"#;

pub static DOCKER_COMPOSE_TEMPLATE: &str = r#"
version: "3.9"
{services}
{networks}
{volumes}
"#;

pub trait DockerComposeGenerator {
    fn generate_database_container(&self, service_name: &str) -> String {
        DATABASE_CONTAINER_TEMPLATE.replace("{service_name}", service_name)
    }

    fn generate_zookeeper_container(&self, service_name: &str) -> String {
        ZOOKEEPER_CONTAINER_TEMPLATE.replace("{service_name}", service_name)
    }

    fn generate_kafka_container(&self, service_name: &str) -> String {
        KAFKA_CONTAINER_TEMPLATE.replace("{service_name}", service_name)
    }

    fn generate_kafdrop_container(&self, service_name: &str) -> String {
        KAFDROP_CONTAINER_TEMPLATE.replace("{service_name}", service_name)
    }

    fn generate_services(&self, service_name: &str) -> String {
        SERVICES_TEMPLATE
            .replace("{database_container}", &self.generate_database_container(service_name))
            .replace("{zookeeper_container}", &self.generate_zookeeper_container(service_name))
            .replace("{kafka_container}", &self.generate_kafka_container(service_name))
            .replace("{kafdrop_container}", &self.generate_kafdrop_container(service_name))
    }

    fn generate_network(&self, service_name: &str) -> String {
        NETWORK_TEMPLATE.replace("{service_name}", service_name)
    }

    fn generate_volume(&self, service_name: &str) -> String {
        VOLUME_TEMPLATE.replace("{service_name}", service_name)
    }

    fn generate_docker_compose(&self, service_name: &str) -> String {
        DOCKER_COMPOSE_TEMPLATE
            .replace("{services}", &self.generate_services(service_name))
            .replace("{networks}", &self.generate_network(service_name))
            .replace("{volumes}", &self.generate_volume(service_name))
    }
}