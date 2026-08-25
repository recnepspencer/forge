#[path = "requests/denial.rs"]
mod denial;
#[path = "requests/digest.rs"]
mod digest;
#[path = "requests/foundational.rs"]
mod foundational;
#[path = "requests/normalized.rs"]
mod normalized;
#[path = "requests/owner_bound.rs"]
mod owner_bound;
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
pub use owner_bound::{
    OwnerBoundMergeExecutionRequest, OwnerBoundMergePlanningRequest,
    RelationalMergeRequestBindingDenial,
};
pub use raw::{MergeExecutionRequest, MergeIntent, MergePlanningRequest};
