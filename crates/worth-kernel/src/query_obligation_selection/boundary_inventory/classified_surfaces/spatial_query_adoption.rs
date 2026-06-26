use crate::query_obligation_selection::boundary_inventory::inventory_record::{
    QuerySelectionAuthorityPosture as Posture, QuerySelectionBoundaryInventoryRow,
    QuerySelectionDeletionAction as Action, QuerySelectionProofStrength as Proof,
    QuerySelectionSurfaceClassification as Class,
};
use crate::query_obligation_selection::boundary_inventory::row_constructors::{
    spatial, spatial_residue,
};

pub(super) fn rows() -> Vec<QuerySelectionBoundaryInventoryRow> {
    vec![
        spatial("SpatialEvidenceQueryTouchDescriptor", "workload_platform/evidence_ledger/spatial_touch_admission/query_lowering.rs", Class::SourceDescriptor, Posture::DescriptorInput, Proof::SourceDescriptorOnly, Action::KeepAsSourceDescriptor, None),
        spatial("SpatialEvidenceQueryTouchDescriptorDigest", "workload_platform/evidence_ledger/spatial_touch_admission/query_lowering.rs", Class::SourceDescriptor, Posture::DescriptorInput, Proof::SourceDescriptorOnly, Action::KeepAsSourceDescriptor, None),
        spatial("SpatialEvidenceQueryTouchDescriptor::product_digest", "workload_platform/evidence_ledger/spatial_touch_admission/query_lowering.rs", Class::SourceDescriptor, Posture::DescriptorInput, Proof::SourceDescriptorOnly, Action::KeepAsSourceDescriptor, None),
        spatial("spatial_query_graph_obligation_adoption_proof", "query_adoption/consumer_kit.rs", Class::QueryOwnedSelection, Posture::ExecutionBackedSelectionAdoption, Proof::ExecutionBackedAdoption, Action::KeepAsQueryOwnedSelection, Some("worth_spatial::facade::query_adoption")),
        spatial("spatial_query_graph_obligation_adoption_proof_for_descriptor", "query_adoption/consumer_kit.rs", Class::QueryOwnedSelection, Posture::ExecutionBackedSelectionAdoption, Proof::ExecutionBackedAdoption, Action::KeepAsQueryOwnedSelection, Some("worth_spatial::facade::query_adoption")),
        spatial("current_spatial_query_consumer_kit_adoption_status", "query_adoption/consumer_kit.rs", Class::CertificationOnlySupport, Posture::PublicFacadeStatus, Proof::PublicStatusOnly, Action::CertificationOnly, Some("worth_spatial::facade::query_adoption")),
        spatial_residue("spatial_query_graph_obligation_residue_manifest", "query_adoption/consumer_kit.rs", "1 row max for current_spatial_workload_support_pin_rows facade projection", "older query-native closeout consumers still read support-pin row projection", "public facade support projection is deleted or replaced by Query-owned selected-obligation status"),
        spatial("current_spatial_workload_support_pin_rows", "query_adoption/support_projection.rs", Class::CappedResidue, Posture::SupportPin, Proof::SupportOnly, Action::CappedResidue, Some("worth_spatial::facade::query_adoption")),
    ]
}
