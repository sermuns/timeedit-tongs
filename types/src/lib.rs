use serde::Deserialize;
use serde_repr::Deserialize_repr;
use wincode::{SchemaRead, SchemaWrite};

#[derive(Deserialize, Debug)]
pub struct ObjectSearchResponse {
    pub count: u16,
    pub records: Vec<ObjectRecord>,
}

#[derive(Deserialize, Debug, SchemaWrite, SchemaRead, Clone)]
pub struct ObjectRecord {
    pub id: u32,
    pub values: String,
    #[serde(rename = "typeId")]
    pub r#type: ObjectType,
}

#[derive(Deserialize_repr, Debug, SchemaWrite, SchemaRead, Clone)]
#[repr(i32)]
pub enum ObjectType {
    StudentGroup = 205,
    Course = 219,
}
