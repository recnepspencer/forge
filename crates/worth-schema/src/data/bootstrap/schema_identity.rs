use forge_relational::facade::schema::{SchemaId, SchemaVersionId};

pub const SCHEMA_ID: &str = "";
pub const SCHEMA_VERSION_ID: u32 = 1;

pub fn schema_id() -> SchemaId {
    SchemaId(SCHEMA_ID.to_string())
}

pub fn schema_version_id() -> SchemaVersionId {
    SchemaVersionId(SCHEMA_VERSION_ID)
}
