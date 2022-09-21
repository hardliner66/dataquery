use {
    async_trait::async_trait,
    gluesql::{
        core::{
            ast::{AstLiteral, ColumnDef, Expr},
            data::{Key, Row, Schema},
            result::{MutResult, Result},
            store::{GStore, GStoreMut, Index, IndexMut, Metadata},
            store::{RowIter, Store, StoreMut},
        },
        prelude::DataType,
    },
    serde::{Deserialize, Serialize},
    std::iter::empty,
};

// Err(Error::StorageMsg(
//     "[Storage] Index::scan_indexed_data is not supported".to_owned(),
// ))


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DataQuery {}

fn key_to_string(key: &Key) -> String {
    use Key::*;
    match key {
        I8(value) => format!("{}", value),
        I16(value) => format!("{}", value),
        I32(value) => format!("{}", value),
        I64(value) => format!("{}", value),
        I128(value) => format!("{}", value),
        Bool(value) => format!("{}", value),
        Str(value) => format!("{}", value),
        Bytea(value) => format!("{:?}", value),
        Date(value) => format!("{}", value),
        Timestamp(value) => format!("{}", value),
        Time(value) => format!("{}", value),
        Interval(value) => format!("{:?}", value),
        Uuid(value) => format!("{}", value),
        Decimal(value) => format!("{}", value),
        None => "".to_owned(),
    }
}

#[async_trait(?Send)]
impl Store for DataQuery {
    async fn fetch_schema(&self, table_name: &str) -> Result<Option<Schema>> {
        Ok(Some(Schema {
            table_name: table_name.to_string(),
            column_defs: vec![ColumnDef {
                name: "name".to_owned(),
                data_type: DataType::Text,
                options: vec![],
            }],
            indexes: vec![],
        }))
    }

    async fn fetch_data(&self, table_name: &str, key: &Key) -> Result<Option<Row>> {
        let schema = self.fetch_schema(table_name).await.unwrap().unwrap();
        Ok(Some(
            Row::new(
                &schema.column_defs,
                &["name".to_owned()],
                &[Expr::Literal(AstLiteral::QuotedString(format!(
                    "asdf {}",
                    key_to_string(key)
                )))],
            )
            .unwrap(),
        ))
    }

    async fn scan_data(&self, _table_name: &str) -> Result<RowIter> {
        Ok(Box::new(empty()))
    }
}

impl DataQuery {
    pub fn new() -> Self {
        Self {}
    }
    pub fn insert_schema_impl(&mut self, _schema: &Schema) {}

    pub fn delete_schema_impl(&mut self, _table_name: &str) {}

    pub fn append_data_impl(&mut self, _table_name: &str, _rows: Vec<Row>) {}

    pub fn insert_data_impl(&mut self, _table_name: &str, _rows: Vec<(Key, Row)>) {}

    pub fn delete_data_impl(&mut self, _table_name: &str, _keys: Vec<Key>) {}
}

#[async_trait(?Send)]
impl StoreMut for DataQuery {
    async fn insert_schema(self, schema: &Schema) -> MutResult<Self, ()> {
        let mut storage = self;

        DataQuery::insert_schema_impl(&mut storage, schema);

        Ok((storage, ()))
    }

    async fn delete_schema(self, table_name: &str) -> MutResult<Self, ()> {
        let mut storage = self;

        DataQuery::delete_schema_impl(&mut storage, table_name);

        Ok((storage, ()))
    }

    async fn append_data(self, table_name: &str, rows: Vec<Row>) -> MutResult<Self, ()> {
        let mut storage = self;

        DataQuery::append_data_impl(&mut storage, table_name, rows);

        Ok((storage, ()))
    }

    async fn insert_data(self, table_name: &str, rows: Vec<(Key, Row)>) -> MutResult<Self, ()> {
        let mut storage = self;

        DataQuery::insert_data_impl(&mut storage, table_name, rows);

        Ok((storage, ()))
    }

    async fn delete_data(self, table_name: &str, keys: Vec<Key>) -> MutResult<Self, ()> {
        let mut storage = self;

        DataQuery::delete_data_impl(&mut storage, table_name, keys);

        Ok((storage, ()))
    }
}

#[async_trait(?Send)]
impl Metadata for DataQuery {}

#[async_trait(?Send)]
impl Index for DataQuery {}

#[async_trait(?Send)]
impl IndexMut for DataQuery {}

impl GStore for DataQuery {}
impl GStoreMut for DataQuery {}
