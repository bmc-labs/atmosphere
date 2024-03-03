use std::collections::HashSet;

use syn::parse::{Parse, ParseStream};
use syn::{Error, Fields, Generics, Ident, LitStr, Token, Visibility};

use crate::hooks::Hooks;
use crate::schema::column::{Column, DataColumn, TimestampColumn};
use crate::schema::keys::{ForeignKey, PrimaryKey};

#[derive(Clone, Debug)]
pub struct TableIdRaw {
    pub schema: Option<String>,
    pub table: Option<String>,
}

impl Parse for TableIdRaw {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut schema = None;
        let mut table = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let value: LitStr = input.parse()?;

            match ident.to_string().as_str() {
                "schema" => schema = Some(value.value()),
                "name" => table = Some(value.value()),
                _ => {
                    return Err(syn::Error::new_spanned(
                        ident,
                        "`#[table]` supports only the values `schema` and `name`",
                    ))
                }
            }

            if !input.peek(Token![,]) {
                break;
            }

            input.parse::<Token![,]>()?;
        }

        Ok(Self { schema, table })
    }
}

impl TableIdRaw {
    fn build(self, item: &syn::ItemStruct) -> TableId {
        TableId {
            schema: self.schema.unwrap_or_else(|| "public".into()),
            table: self.table.unwrap_or_else(|| item.ident.to_string()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TableId {
    pub schema: String,
    pub table: String,
}

#[derive(Clone, Debug)]
pub struct Table {
    pub vis: Visibility,
    pub generics: Generics,
    pub ident: Ident,

    pub id: TableId,

    pub primary_key: PrimaryKey,
    pub foreign_keys: HashSet<ForeignKey>,
    pub data_columns: HashSet<DataColumn>,
    pub timestamp_columns: HashSet<TimestampColumn>,

    pub hooks: Hooks,
}

impl Parse for Table {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item: syn::ItemStruct = input.parse()?;

        let id: TableIdRaw = item
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("table"))
            .ok_or(syn::Error::new(
                input.span(),
                "You need to use the `#[table]` attribute if you want to derive `Schema`",
            ))?
            .parse_args()?;

        let id = id.build(&item);

        let hooks: Hooks = {
            let attr = item.attrs.iter().find(|attr| attr.path().is_ident("hooks"));

            if let Some(attr) = attr {
                attr.parse_args()?
            } else {
                Hooks::default()
            }
        };

        let ident = item.ident;

        let fields = match item.fields {
            Fields::Named(n) => n,
            Fields::Unnamed(_) | Fields::Unit => {
                return Err(Error::new(
                    ident.span(),
                    format!(
                        "{} must use named fields in order to derive `Schema`",
                        ident
                    ),
                ))
            }
        };

        let columns = fields
            .named
            .into_iter()
            .map(Column::try_from)
            .collect::<syn::Result<HashSet<Column>>>()?;

        let primary_key = {
            let primary_keys: HashSet<PrimaryKey> = columns
                .iter()
                .filter_map(|c| c.as_primary_key())
                .cloned()
                .collect();

            if primary_keys.len() > 1 {
                return Err(Error::new(
                    input.span(),
                    format!(
                        "{} declares more than one column as its primary key – only one is allowed",
                        ident
                    ),
                ));
            }

            primary_keys.into_iter().next().ok_or(Error::new(
                input.span(),
                format!(
                    "{} must declare one field as its primary key (using `#[primary_key]`",
                    ident
                ),
            ))?
        };

        let foreign_keys = columns
            .iter()
            .filter_map(|c| c.as_foreign_key())
            .cloned()
            .collect();

        let data_columns = columns
            .iter()
            .filter_map(|c| c.as_data_column())
            .cloned()
            .collect();

        let timestamp_columns = columns
            .iter()
            .filter_map(|c| c.as_timestamp_column())
            .cloned()
            .collect();

        Ok(Self {
            vis: item.vis,
            generics: item.generics,
            ident,
            id,
            primary_key,
            foreign_keys,
            data_columns,
            timestamp_columns,
            hooks,
        })
    }
}
