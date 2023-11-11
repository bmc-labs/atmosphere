<div align="center">

![Atmosphere](./docs/assets/banner.png)

# `🌍 Atmosphere`

**A lightweight sqlx framework for safe and fast postgres interactions**

[![Hemisphere](https://img.shields.io/badge/hemisphere-open%20source-blueviolet.svg)](https://hemisphere.studio)
[![Hemisphere](https://img.shields.io/badge/postgresql-orm-blue.svg)]()

</div>

## Roadmap

### Alpha Release
- [x] Trait System (`Table`, `Column`, `Relation` ..)
- [x] Derive Macro (`Schema`)
- [x] Field Attributes (`#[id]`, and so on)
- [x] Query Generation
- [x] Compile Time Verification
- [x] Attribute Macro (`#[table]`)
- [ ] Attribute Macro (`#[relation]`)
- [ ] Attribute Macro (`#[query]`)

### Beta Release
- [ ] Transaction Support
- [ ] Custom queries

### Stable Release
- [ ] Stabilize Traits
- [ ] Table Lenses (subsets)

### Advanced
- [ ] Postgres Composite Types
- [ ] Support custom types
- [ ] Runtime Inspection
- [ ] Generate Graphs
- [ ] `validator` support

### Longterm
- [ ] Generate GraphQL Server

## Concept

## Macros

###### `derive(Schema)`

Builds compile time schema of struct and inserts into global database schema.
This automatically derives the atmosphere base traits for the following common
operations:

**Create**
- `Table::insert(&self)`
- `&[AsRef<Table>]::insert_all(&self)`

**Read**
- `Table::find(id: &Id)`
- `Table::find_all(ids: &[&Id])`

**Update**
- `Table::reload(&mut self)`
- `Table::update(&self)`
- `Table::upsert(&self)`

 **Delete**
- `Table::delete(&mut self)`
- `Table::delete_by(id: &Id)`
- `Table::delete_all_by(ids: &[&Id])`
- `&[AsRef<Table>]::delete_all_by(ids: &[&Id])`

###### `#[query]`
Enables custom queries on the struct

```rust
impl Forest {
    /// Select a forest by its name
    #[query(
        SELECT * FROM ${Forest}
        WHERE name = ${name}
        ORDER BY name
    )]
    pub async fn by_name(name: &str) -> query::Result<Self>;

    /// Select the newest forest
    #[query(
        SELECT * FROM ${Forest}
        ORDER BY created_at DESC
        LIMIT 1
    )]
    pub async fn newest() -> query::Result<Self>;
}
```

---

##### Advanced Macros

###### `#[table(schema = "public", name = <name>, id = (<a>, <b>))]`
configures a table name and schema (`schema.table`)
id optionally tells atmosphere to use combined primary keys

###### `#[relation(grouped_by = Forest)]` and `#[fk(Forest)]`
enable `Tree::by_forest(&forest.id)`

###### `#[relation(groups = Tree)]` and `#[fk(Forest)]`  (on the Tree)
enable `Forest::collect(&self)`

###### `#[relation(links = Forest, as = neighbour)]` and `#[fk(Forest)]`
enable `Tree::neighbour(&self)`

###### `#[virtual(<sql>)]`
marks a virtual column

###### `#[lens(Forest)]`
data lenses on big structs

###### `#[form(Forest)]`
data forms for mutating tables
