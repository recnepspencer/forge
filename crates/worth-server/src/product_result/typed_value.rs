pub trait WorthServerProductResultValue: serde::Serialize {
    fn result_schema_identity(&self) -> &str;

    fn result_schema_version(&self) -> u32;
}
