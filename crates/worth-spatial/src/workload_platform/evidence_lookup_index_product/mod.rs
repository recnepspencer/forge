mod basis;
mod construction;
mod counters;
mod disposal_posture;
mod error;
mod identity;
mod lifecycle_posture;
mod product;
mod query_support;
mod topology_support;

#[cfg(test)]
mod basis_audit;

#[cfg(test)]
pub(crate) mod tests;

pub use crate::workload_platform::evidence_lookup_reuse_decision::EvidenceLookupIndexReuseResolution;
pub(crate) use basis::EvidenceLookupLedgerBasis;
#[cfg(test)]
pub(crate) use basis_audit::{
    audit_evidence_lookup_index_product_basis, EvidenceLookupIndexBasisAuditScope,
};
pub use construction::require_persistent_evidence_lookup_index_product;
pub(crate) use construction::{
    lifecycle_posture, selected_lookup_counters, selected_plan_query_surface_contract_rows,
};
pub use counters::EvidenceLookupIndexProductCounters;
pub use disposal_posture::{
    EvidenceLookupIndexDisposalPosture, EvidenceLookupIndexDisposalPostureKind,
};
pub use error::{EvidenceLookupIndexProductError, EvidenceLookupIndexProductErrorKind};
pub(crate) use identity::{
    admit_and_lower_index_family_identity, rebuild_required_identity,
};
#[cfg(test)]
pub(crate) use identity::{
    admit_and_lower_index_family_identity_from_basis, lower_index_family_identity_from_basis,
};
pub use lifecycle_posture::{
    EvidenceLookupIndexLifecyclePosture, EvidenceLookupIndexLifecyclePostureKind,
};
pub use product::EvidenceLookupIndexProduct;
pub(crate) use query_support::selected_query_support_digest;
pub(crate) use topology_support::selected_topology_support_digest;
