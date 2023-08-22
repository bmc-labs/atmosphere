use async_trait::async_trait;
use std::marker::PhantomData;

pub trait Table: Sized + Send + for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + 'static
where
    Self::PrimaryKey: for<'q> sqlx::Encode<'q, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + Send,
{
    type PrimaryKey: Sized + 'static;

    const SCHEMA: &'static str;
    const TABLE: &'static str;
    const PRIMARY_KEY: Column<Self>;
    const FOREIGN_KEYS: &'static [Column<Self>];
    const DATA: &'static [Column<Self>];
}

#[async_trait]
pub trait Create: Table {
    /// Insert a new row
    async fn insert(&self, pool: &sqlx::PgPool) -> Result<()>;
}

//#[async_trait]
//pub trait CreateMany {
//async fn insert_all(&self, pool: &sqlx::PgPool) -> Result<()>;
//}

#[async_trait]
pub trait Read: Table {
    /// Find a row by its primary key
    async fn find(pk: &Self::PrimaryKey, pool: &sqlx::PgPool) -> Result<Self>;
    /// Find all rows in the list of primary keys
    async fn find_many(
        pks: &[impl AsRef<Self::PrimaryKey>],
        pool: &sqlx::PgPool,
    ) -> Result<Vec<Self>>;

    // TODO(mara): stream
    // Read all rows from the database
    //async fn all(pool: &sqlx::PgPool) -> Result<Vec<Self>>;
}

#[async_trait]
pub trait Update: Table {
    /// Reload this row
    async fn reload(&mut self, pool: &sqlx::PgPool) -> Result<()>;
    /// Update the row in the database
    async fn update(&self, pool: &sqlx::PgPool) -> Result<()>;
    /// Save to the database (upsert behavior)
    async fn save(&self, pool: &sqlx::PgPool) -> Result<()>;
}

#[async_trait]
pub trait Delete: Table {
    /// Delete row in database
    async fn delete(&self, pool: &sqlx::PgPool) -> Result<()>;
    /// Delete row in database by primary key
    async fn delete_by(pk: &Self::PrimaryKey, pool: &sqlx::PgPool) -> Result<()>;
    /// Delete all rows in the list of primary keys
    async fn delete_many(pks: &[impl AsRef<Self::PrimaryKey>], pool: &sqlx::PgPool) -> Result<()>;
}

//#[async_trait]
//pub trait DeleteMany {
//async fn delete_all(&self, pool: &sqlx::PgPool) -> Result<()>;
//}

#[derive(Debug)]
pub struct Column<T: Table> {
    pub name: &'static str,
    pub data_type: DataType,
    pub col_type: ColType,
    marker: PhantomData<T>,
}

impl<T: Table> Column<T> {
    pub const fn new(name: &'static str, data_type: DataType, col_type: ColType) -> Self {
        Self {
            name,
            data_type,
            col_type,
            marker: PhantomData,
        }
    }
}

/// All possible types for postgres
#[derive(Debug)]
pub enum DataType {
    Unknown,
    Text,
    Number,
}

#[derive(Debug)]
pub enum ColType {
    Value,
    PrimaryKey,
    ForeignKey,
}

pub type Result<T> = std::result::Result<T, ()>;
