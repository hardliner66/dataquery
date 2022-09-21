```c++
// impl Store
  ERROR_CODE fetch_schema(void* self, char* table_name) -> Result<Option<Schema>>
  ERROR_CODE fetch_data_i8(void* self, char* table_name, i8 key) -> Result<Option<Row>>
  ERROR_CODE fetch_data_i16(void* self, char* table_name, i16 key)
  ERROR_CODE fetch_data_i32(void* self, char* table_name, i32 key)
  ERROR_CODE fetch_data_i64(void* self, char* table_name, i64 key)
  ERROR_CODE fetch_data_i128(void* self, char* table_name, i128 key)
  ERROR_CODE fetch_data_bool(void* self, char* table_name, bool key)
  ERROR_CODE fetch_data_str(void* self, char* table_name, char* key)
  ERROR_CODE fetch_data_bytea(void* self, char* table_name, usize length, u8* key)
  ERROR_CODE fetch_data_date(void* self, char* table_name, NaiveDate key)
  ERROR_CODE fetch_data_timestamp(void* self, char* table_name, NaiveDateTime key)
  ERROR_CODE fetch_data_time(void* self, char* table_name, NaiveTime key)
  ERROR_CODE fetch_data_interval(void* self, char* table_name, Interval key)
  ERROR_CODE fetch_data_uuid(void* self, char* table_name, u128 key)
  ERROR_CODE fetch_data_decimal(void* self, char* table_name, Decimal key)
  ERROR_CODE fetch_data_none(void* self, char* table_name)
  ERROR_CODE scan_data(void* self, char* table_name) -> Result<RowIter>

// impl StoreMut
  ERROR_CODE insert_schema(void* self, schema: &Schema) -> MutResult<Self, ()>
  ERROR_CODE delete_schema(void* self, table_name: char*) -> MutResult<Self, ()>
  ERROR_CODE append_data(void* self, table_name: char*, rows: Vec<Row>) -> MutResult<Self, ()>
  ERROR_CODE insert_data(void* self, table_name: char*, rows: Vec<(Key, Row)>) -> MutResult<Self, ()>
  ERROR_CODE delete_data(void* self, table_name: char*, keys: Vec<Key>) -> MutResult<Self, ()>
```
