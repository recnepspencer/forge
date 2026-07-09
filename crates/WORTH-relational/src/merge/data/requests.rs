#[path = "requests/denial.rs"]
mod denial;
#[path = "requests/digest.rs"]
mod digest;
#[path = "requests/foundational.rs"]
mod foundational;
#[path = "requests/normalized.rs"]
mod normalized;
#[path = "requests/raw.rs"]
mod raw;

pub use denial::RelationalMergeRequestNormalizationDenial;
pub(crate) use digest::normalized_merge_request_digest;
pub use foundational::RelationalFoundationalMergeRequest;
pub use normalized::{
    NormalizedRelationalMergeRequest, RelationalMergeCorrespondencePosture,
    RelationalMergeRequestFamily, RelationalMergeSchemaReconciliationPosture, RelationalMergeScope,
    RelationalMergeTopologyIntent,
};
pub use raw::{MergeExecutionRequest, MergeIntent, MergePlanningRequest};
