#[cfg(test)]
mod admission_test_support;
mod authority;
mod counter_honesty;
mod denial;
mod digest;
#[cfg(test)]
mod guard_contract_tests;
mod input;
#[cfg(test)]
mod ledger_locality_tests;
mod lookup;
#[cfg(test)]
mod lookup_boundary_denial_tests;
#[cfg(test)]
mod lookup_equivalence_tests;
#[cfg(test)]
mod lookup_locality_tests;
#[cfg(test)]
mod lookup_separation_tests;
mod operating_world;
#[cfg(test)]
mod phase_three_tests;
mod preview;
#[cfg(test)]
mod query_descriptor_denial_tests;
#[cfg(test)]
mod query_descriptor_parity_tests;
#[cfg(test)]
mod query_gap_tests;
mod query_lowering;
#[cfg(test)]
mod query_milestone_boundary_tests;
#[cfg(test)]
mod query_selector_precision_tests;
#[cfg(test)]
mod receipt_backed_admission_test_authority;
#[cfg(test)]
mod receipt_coverage_tests;
#[cfg(test)]
mod rejection_boundary_tests;
#[cfg(test)]
mod replay_equivalence_tests;
mod request;
mod stage_vocabulary;
#[cfg(test)]
mod tests;

pub use authority::SpatialGeometryEvidenceTouchAuthority;
pub use counter_honesty::{
    SpatialGeometryEvidenceTouchCounterHonesty, SpatialGeometryEvidenceTouchCounterViolationRow,
};
pub use denial::{
    SpatialGeometryEvidenceTouchDenial, SpatialGeometryEvidenceTouchDenialKind,
    SpatialGeometryEvidenceTouchDenialPrecedence,
};
#[allow(unused_imports)]
pub use digest::{SpatialGeometryEvidenceParticipantDigest, SpatialGeometryEvidenceTouchDigest};
#[cfg(test)]
pub(crate) use input::SpatialGeometryEvidenceTouchRejectedInput;
pub(crate) use input::SpatialGeometryEvidenceTouchRejectedInputKind;
pub use lookup::{
    deny_query_descriptor_digest_as_spatial_evidence_lookup_authority, SpatialEvidenceLookupDenial,
    SpatialEvidenceLookupDenialKind, SpatialEvidenceLookupExpectation, SpatialEvidenceLookupKey,
    SpatialEvidenceLookupProduct, SpatialEvidenceLookupProductDigest,
};
pub use operating_world::SpatialGeometryEvidenceTouchOperatingWorld;
pub use preview::{
    SpatialGeometryEvidenceTouchDiagnosticStatus, SpatialGeometryEvidenceTouchReceiptOnlyPreview,
};
pub use query_lowering::{
    deny_copied_receipt_fields_as_spatial_query_lowering_authority,
    deny_query_descriptor_as_spatial_query_lowering_authority,
    deny_raw_row_as_spatial_query_lowering_authority,
    deny_topology_touched_basis_as_spatial_query_lowering_authority,
    lower_spatial_touch_authority_to_query_descriptor, SpatialEvidenceQueryGapKind,
    SpatialEvidenceQueryGapRow, SpatialEvidenceQueryLoweringCounters,
    SpatialEvidenceQueryLoweringDenial, SpatialEvidenceQueryLoweringDenialKind,
    SpatialEvidenceQueryTouchDescriptor, SpatialEvidenceQueryTouchDescriptorDigest,
};
#[cfg(test)]
pub(crate) use receipt_backed_admission_test_authority::{
    receipt_backed_event_ledger_touch_authority_for_admission_tests,
    receipt_backed_touch_authority_for_admission_tests,
    receipt_backed_touch_authority_for_admission_tests_with_declared_world,
};
pub(crate) use request::SpatialGeometryEvidenceTouchRowRequest;
pub use request::{
    SpatialGeometryEvidenceTouchAdmissionInput, SpatialGeometryEvidenceTouchRequest,
};
pub use stage_vocabulary::{
    spatial_touch_workload_evidence_stage, SPATIAL_TOUCH_BOOLEAN_EVIDENCE_STAGE_KINDS,
};
