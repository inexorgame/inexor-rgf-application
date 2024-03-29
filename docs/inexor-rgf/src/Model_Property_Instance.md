# Model: Property Instance

A property instance is a data stream. The current value of the property instance can be queried. Data streams can be
connected to each other (both within an instance and to data streams from property instances of other entity or relation
instances).

The property type defines the name, data type and socket type of the property instance.

## Data Model

| Field      | DataType                                                          | Description               |
|------------|-------------------------------------------------------------------|---------------------------|
| Name       | String                                                            | The name of the property  |
| Value      | [Value](https://docs.serde.rs/serde_json/value/enum.Value.html)   | The value of the property |
| Type       | [Property Type](./Model_Property_Type.md)                         | The type of the property  |

## Graph

```mermaid
graph TD;
    PI1(Property Instance);
    PI2(Property Instance);
    PI3(Property Instance);
    EI(Entity Instance)--->PI1;
    EI(Entity Instance)--->PI2;
    EI(Entity Instance)--->PI3;

    PI4(Property Instance);
    PI5(Property Instance);
    PI6(Property Instance);
    RI(Relation Instance)--->PI4;
    RI(Relation Instance)--->PI5;
    RI(Relation Instance)--->PI6;
```

## ER Diagram

```mermaid
erDiagram
    Property-Type {
        string name
        enum DataType
        enum SocketType
        enum Mutability
    }
    Property-Instance {
        string name
        string value
    }
    Entity-Instance {
        string id
        string label
    }
    Relation-Instance {
        string instanceId
    }
    Property-Type ||--}o Property-Instance : is-a
    Entity-Instance ||--}o Property-Instance : stores
    Entity-Instance o{--}o Relation-Instance : outbound
    Entity-Instance o{--}o Relation-Instance : inbound
    Relation-Instance ||--}o Property-Instance : stores
```
