pub trait WorthQueryOperationOutput: 'static {
    fn operation_output_identity(&self) -> String;
}

impl WorthQueryOperationOutput for () {
    fn operation_output_identity(&self) -> String {
        "unit".into()
    }
}

macro_rules! scalar_output {
    ($type:ty, $label:literal) => {
        impl WorthQueryOperationOutput for $type {
            fn operation_output_identity(&self) -> String {
                format!(concat!($label, ":{}"), self)
            }
        }
    };
}

scalar_output!(bool, "bool");
scalar_output!(i64, "i64");
scalar_output!(u64, "u64");
scalar_output!(String, "text");

impl WorthQueryOperationOutput for crate::ordinary::read::WorthQueryReadCompletion {
    fn operation_output_identity(&self) -> String {
        crate::identity::hash_parts(&[
            "worth_query_installed_read_output_v1".into(),
            format!(
                "canonical_query:{}",
                self.result().receipt().canonical_query_digest()
            ),
            format!("result:{}", self.result().receipt().result_digest()),
            format!(
                "snapshot:{}",
                self.result()
                    .receipt()
                    .snapshot_evidence_identity()
                    .as_str()
            ),
        ])
    }
}
