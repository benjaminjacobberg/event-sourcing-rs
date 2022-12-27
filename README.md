# event-sourcing-rs

🚧 This library is under construction! 🚧

## Summary

An opinionated event sourcing library that is designed to be focused on speed, durability, and ease of use.

## Setup

### Prerequisites

- Apache Cassandra or ScyllaDB
  - Make sure to have CDC enabled
- Apache Kafka
- Debezium
  - Configured the Debezium Connector for Cassandra or ScyllaDB
- Any preferred database to store your projected states

Add the following dependencies to your `Cargo.toml`:
```toml
[dependencies]
event-sourcing = { git = "https://github.com/benjaminjacobberg/event-sourcing-rs" } # Core
event-store-cassandra = { git = "https://github.com/benjaminjacobberg/event-sourcing-rs" } # Cassandra event store implementation
event-stream-kafka = { git = "https://github.com/benjaminjacobberg/event-sourcing-rs" } # Kafka event stream implementation
```

## Diagrams

### Sequence Diagrams

#### Execute Command

```mermaid
sequenceDiagram
    actor User

    autonumber
    User->>+Controller: command request
    Controller->>+CommandHandler: handle command
    CommandHandler->>+EventStore: read existing events
    EventStore-->>-CommandHandler: exisintg events
    CommandHandler->>+Aggregate: apply existing events
    Aggregate-->>-CommandHandler: aggregate result
    CommandHandler->>+Aggregate: apply new event
    Aggregate-->>-CommandHandler: aggregate result
    CommandHandler->>+EventStore: persist new event
    EventStore->>+Cassandra: save
    Cassandra-->>-EventStore: result
    EventStore-->>-CommandHandler: result
    CommandHandler-->>-Controller: result
    Controller-->>-User: response
```

#### Flow of Data to Projection

```mermaid
sequenceDiagram
    autonumber
    loop
        Debezium->>+Cassandra: read events from CDC (change data capture)
        Debezium->>+Kafka: write events
    end

    loop
        EventListener->>+Kafka: consume events
        EventListener->>+Projection: apply projection
        Projection->>+Repository: save
        Repository->>+Database: save
        Database-->>-Repository: result
        Repository-->>-Projection: result
    end
```

#### Query State

```mermaid
sequenceDiagram
    actor User

    autonumber
    User->>+Controller: query request
    Controller->>+QueryHandler: handle query
    QueryHandler->>+Repository: query
    Repository-->>-QueryHandler: result
    QueryHandler-->>-Controller: result
    Controller-->>-User: response
```

#### Entire Overview

```mermaid
sequenceDiagram
    actor User

    autonumber
    User->>+Controller: command request
    Controller->>+CommandHandler: handle command
    CommandHandler->>+EventStore: read existing events
    EventStore-->>-CommandHandler: exisintg events
    CommandHandler->>+Aggregate: apply existing events
    Aggregate-->>-CommandHandler: aggregate result
    CommandHandler->>+Aggregate: apply new event
    Aggregate-->>-CommandHandler: aggregate result
    CommandHandler->>+EventStore: persist new event
    EventStore->>+Cassandra: save
    Cassandra-->>-EventStore: result
    EventStore-->>-CommandHandler: result
    CommandHandler-->>-Controller: result
    Controller-->>-User: response
    
    loop
        Debezium->>+Cassandra: read events from CDC (change data capture)
        Debezium->>+Kafka: write events
    end

    loop
        EventListener->>+Kafka: consume events
        EventListener->>+Projection: apply projection
        Projection->>+Repository: save
        Repository->>+Database: save
        Database-->>-Repository: result
        Repository-->>-Projection: result
    end

    User->>+Controller: query request
    Controller->>+QueryHandler: handle query
    QueryHandler->>+Repository: query
    Repository-->>-QueryHandler: result
    QueryHandler-->>-Controller: result
    Controller-->>-User: response
```