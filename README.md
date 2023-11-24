<div align="center">

![Atmosphere](./docs/assets/banner.png)

# `🌍 Atmosphere`

**A lightweight sql framework for sustainable database reliant systems**

[![SQLx](https://img.shields.io/badge/sqlx-framework-blueviolet.svg)]()
[![Crate](https://img.shields.io/crates/v/atmosphere.svg)](https://crates.io/crates/atmosphere)
[![Book](https://img.shields.io/badge/book-latest-0f5225.svg)](https://mara-schulke.github.io/atmosphere)
[![Docs](https://img.shields.io/badge/docs-latest-153f66.svg)](https://docs.rs/atmosphere)

</div>

## Overview

Atmosphere is a lightweight SQL framework designed for sustainable,
database-reliant systems. It leverages Rust's powerful type and macro systems
to derive SQL schemas from your rust struct definitions into an advanced trait
system.

## Key Features

- SQL schema derivation from Rust structs.
- Advanced trait system for query generation.
- Automated database code testing with `atmosphere::testing`
- ORM-like CRUD traits.
- Code reusability across API layers using generics.
- Compile-time introspection for type-safe schema generation.

## Quickstart

```rust
use atmosphere::prelude::*;

#[derive(Schema)]
#[table(schema = "public", name = "user")]
struct User {
    #[sql(pk)]
    id: i32,
    name: String,
    #[sql(unique)]
    email: String,
}

#[derive(Schema)]
#[table(schema = "public", name = "post")]
struct Post {
    #[sql(pk)]
    id: i32,
    #[sql(fk -> User, rename = "author_id")]
    author: i32,
    #[sql(unique)]
    title: String,
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let pool = atmosphere::Pool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;

    // CRUD operations

    let user = User { id: 0, name: "demo".to_owned(), email: "some@email.com".to_owned(), };

    user.save(&pool).await?;
    user.delete(&pool).await?;

    // Field Queries

    assert!(
        User::find(&0, &pool).await?,
        User::find_by_email("some@email.com", &pool).await?
    );

    // Relationships

    Post { id: 0, author: 0, title: "test".to_owned() }
        .save(&pool)
        .await?;

    Post::find_by_author(&0, &pool).await?;
    Post::delete_by_author(&0, &pool).await?;

    // Inter-Table Operations

    Post { id: 1, author: 0, title: "test1".to_owned() }
        .author(&pool).await?;

    user.posts(&pool).await?;
    user.delete_posts(&pool).await?;

    Ok(())
}
```

Atmosphere introspects the `User` and `Post` structs at compile time and
generates `const` available type information about the schema into the `Table`
trait:

```rust
impl Table for User {
    const SCHEMA: &str = "public"
    const TABLE: &str = "user"
    const PRIMARY_KEY: Column = Column { name: "id", ty: PrimaryKey, .. };
    const FOREIGN_KEYS: &'static [Column; 0] = &[];
    const DATA: &'static [Column; 2] = &[
        Column { name: "name", ty: Value, .. },
        Column { name: "email", ty: Value, .. }
    ];
}
```

## Roadmap

### Alpha Release
- [x] Advanced SQL Trait System (`Table`, `Column`, `Relation` ..)
- [x] Derive Macro (`Schema`)
- [x] SQL Field Attributes (`#[sql(pk)]`, `#[sql(fk -> Model)]` and so on)
- [x] SQL Query Generation
- [x] Automated Integration Testing
- [x] Attribute Macro (`#[table]`)

### Beta Release
- [x] Transaction Support
- [x] Getting Database Agnostic
- [ ] Hook into query execution using `atmosphere::hooks`
- [ ] Errors using `miette`
- [ ] Combined Primary and Foreign Keys

### Stable Release
- [x] Postgres Composite Types
- [x] Support Custom Types
- [x] Runtime Inspection
- [x] Provide Application Utils
- [ ] Stabilize Traits
- [ ] Stabilize Query Generation
- [ ] Table Lenses (subsets / views)

### Advanced
- [ ] Virtual Columns using (`#[virtual = "<sql>"]`)
- [ ] Soft Delete Support
- [ ] Attribute Macro (`#[query]`)
- [ ] Custom queries
- [ ] Auto Timestamping
- [ ] `validator` support

### Longterm
- [ ] Generate GraphQL + HTTP Servers?
- [ ] Generate Graphs


## Contribution

We welcome contributions! Please see our contribution guidelines for more details.

## License

Atmosphere is licensed under Apache 2.0.
