use forge_relational::facade::schema::{SchemaId, SchemaVersionId};

pub const WORTH_SCHEMA_ID: &str = "worth";
pub const WORTH_SCHEMA_VERSION_ID: u32 = 1;

pub fn schema_id() -> SchemaId {
    SchemaId(WORTH_SCHEMA_ID.to_string())
}

pub fn schema_version_id() -> SchemaVersionId {
    SchemaVersionId(WORTH_SCHEMA_VERSION_ID)
}
