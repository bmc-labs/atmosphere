use sqlx::{Database, Encode, FromRow, Type};

mod create;
mod delete;
mod read;
mod update;

pub use create::Create;
pub use delete::Delete;
pub use read::Read;
pub use update::Update;

pub use self::column::{Column, DataColumn, DynamicForeignKey, ForeignKey, MetaColumn, PrimaryKey};

/// SQL Table Definition
pub trait Table
where
    Self: Sized + Send + for<'r> FromRow<'r, <crate::Driver as Database>::Row> + 'static,
    Self::PrimaryKey: for<'q> Encode<'q, crate::Driver> + Type<crate::Driver> + Send,
{
    type PrimaryKey: Sync + Sized + 'static;

    const SCHEMA: &'static str;
    const TABLE: &'static str;

    /// The primary key of this table
    const PRIMARY_KEY: PrimaryKey<Self>;
    /// Columns that are used as a foreign key
    const FOREIGN_KEYS: &'static [DynamicForeignKey<Self>];
    /// Columns that are treated as data
    const DATA_COLUMNS: &'static [DataColumn<Self>];
    /// Columns that are treated as metadata
    const META_COLUMNS: &'static [MetaColumn<Self>];

    fn pk(&self) -> &Self::PrimaryKey;
}

/// A entity is a table that implements [`Create`], [`Read`], [`Update`] & [`Create`]
pub trait Entity: Create + Read + Update + Delete {}

impl<E: Create + Read + Update + Delete> Entity for E {}

pub mod column {
    use crate::Table;
    use std::marker::PhantomData;

    /// Column Variants
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Column<'c, T: Table> {
        PrimaryKey(&'c PrimaryKey<T>),
        ForeignKey(&'c DynamicForeignKey<T>),
        DataColumn(&'c DataColumn<T>),
        MetaColumn(&'c MetaColumn<T>),
    }

    impl<'c, T: Table> Column<'c, T> {
        pub const fn name(&self) -> &'static str {
            match self {
                Self::PrimaryKey(pk) => pk.name,
                Self::ForeignKey(fk) => fk.name,
                Self::DataColumn(data) => data.name,
                Self::MetaColumn(meta) => meta.name,
            }
        }
    }

    /// Descriptor type of a sql data column
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct PrimaryKey<T: Table> {
        pub name: &'static str,
        table: PhantomData<T>,
    }

    impl<T: Table> PrimaryKey<T> {
        pub const fn new(name: &'static str) -> Self {
            Self {
                name,
                table: PhantomData,
            }
        }
    }

    /// Descriptor type of a sql foreign key column
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct DynamicForeignKey<T: Table> {
        pub name: &'static str,
        table: PhantomData<T>,
    }

    impl<T: Table> DynamicForeignKey<T> {
        pub const fn new(name: &'static str) -> Self {
            Self {
                name,
                table: PhantomData,
            }
        }
    }

    /// Descriptor type of a sql foreign key column
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ForeignKey<F: Table, T: Table> {
        pub name: &'static str,
        from: PhantomData<F>,
        to: PhantomData<T>,
    }

    impl<F: Table, T: Table> ForeignKey<F, T> {
        pub const fn new(name: &'static str) -> Self {
            Self {
                name,
                from: PhantomData,
                to: PhantomData,
            }
        }
    }

    /// Descriptor type of a sql data column
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct DataColumn<T: Table> {
        pub name: &'static str,
        table: PhantomData<T>,
    }

    impl<T: Table> DataColumn<T> {
        pub const fn new(name: &'static str) -> Self {
            Self {
                name,
                table: PhantomData,
            }
        }
    }

    /// Descriptor type of a sql metadata column
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct MetaColumn<T: Table> {
        pub name: &'static str,
        table: PhantomData<T>,
    }

    impl<T: Table> MetaColumn<T> {
        pub const fn new(name: &'static str) -> Self {
            Self {
                name,
                table: PhantomData,
            }
        }
    }
}
