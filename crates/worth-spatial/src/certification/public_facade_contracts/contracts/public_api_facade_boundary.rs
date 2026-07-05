#[test]
fn spatial_facade_is_namespaced_and_no_longer_flat() {
    let facade = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/facade/mod.rs"));
    let lib = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

    assert!(!facade.contains("pub use crate::bindings::{"));
    assert!(!facade.contains("pub use crate::spatial_domain::{"));
    assert!(!facade.contains("pub use crate::test_support::SpatialFixtureWitnessCatalog"));
    assert!(facade.contains("pub mod anchor_binding;"));
    assert!(facade.contains("pub mod binding;"));
    assert!(facade.contains("pub mod bindings;"));
    assert!(facade.contains("pub mod continuation;"));
    assert!(facade.contains("pub mod inspection;"));
    assert!(facade.contains("pub mod neighborhood;"));
    assert!(facade.contains("pub mod planar_contracts;"));
    assert!(facade.contains("pub mod planar_m6_closeout;"));
    assert!(facade.contains("pub mod planar_overlap;"));
    assert!(facade.contains("pub mod planar_signed_area;"));
    assert!(facade.contains("pub mod planar_predicates;"));
    assert!(facade.contains("pub mod placement;"));
    assert!(facade.contains("pub mod projection;"));
    assert!(facade.contains("pub mod rebinding;"));
    assert!(facade.contains("pub mod recovery;"));
    assert!(facade.contains("pub mod support;"));
    assert!(facade.contains("pub mod tolerance;"));
    assert!(facade.contains("pub mod workload_inventory;"));
    assert!(!facade.contains("pub mod workload_operators;"));
    assert!(!facade.contains("pub mod policy;"));
    assert!(!facade.contains("pub mod birth;"));
    assert!(!facade.contains("pub mod motion;"));
    assert!(!facade.contains("pub mod constraints;"));
    assert!(!facade.contains("pub mod arbitration;"));
    assert!(!facade.contains("pub mod witness_resolution;"));
    assert!(!lib.contains("mod spatial_domain;"));
    assert!(!lib.contains("pub mod test_support;"));
    assert!(lib.contains("pub mod certification;"));
    assert!(lib.contains("pub mod facade;"));
}

#[test]
fn planar_boolean_overlap_region_extraction_facade_exports_phase_six_classification_surfaces() {
    use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
        PlanarBooleanOverlapCellContainmentInput, PlanarBooleanOverlapCellContainmentMap,
        PlanarBooleanOverlapCellContainmentRow, PlanarBooleanOverlapCellWindingField,
        PlanarBooleanOverlapCellWindingFieldInput, PlanarBooleanOverlapCellWindingRow,
    };

    let _: Option<PlanarBooleanOverlapCellContainmentInput<'static>> = None;
    let _: Option<PlanarBooleanOverlapCellContainmentMap> = None;
    let _: Option<PlanarBooleanOverlapCellContainmentRow> = None;
    let _: Option<PlanarBooleanOverlapCellWindingFieldInput<'static>> = None;
    let _: Option<PlanarBooleanOverlapCellWindingField> = None;
    let _: Option<PlanarBooleanOverlapCellWindingRow> = None;
}

#[test]
fn planar_boolean_overlap_region_extraction_facade_exports_phase_seven_island_component_surfaces() {
    use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
        PlanarBooleanAreaOverlapComponentSet, PlanarBooleanBoundaryContactComponentSet,
        PlanarBooleanOverlapIslandCandidateInput, PlanarBooleanOverlapIslandCandidateSet,
        PlanarBooleanOverlapIslandComponentBundle, PlanarBooleanOverlapIslandPartition,
        PlanarBooleanOverlapIslandSet,
    };

    let _: Option<PlanarBooleanOverlapIslandCandidateInput<'static>> = None;
    let _: Option<PlanarBooleanOverlapIslandCandidateSet> = None;
    let _: Option<PlanarBooleanOverlapIslandComponentBundle> = None;
    let _: Option<PlanarBooleanOverlapIslandPartition> = None;
    let _: Option<PlanarBooleanOverlapIslandSet> = None;
    let _: Option<PlanarBooleanBoundaryContactComponentSet> = None;
    let _: Option<PlanarBooleanAreaOverlapComponentSet> = None;
}

#[test]
fn planar_boolean_overlap_region_extraction_facade_exports_phase_eight_boundary_contact_classification_surfaces() {
    use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
        PlanarBooleanBoundaryContactClassificationBundle,
        PlanarBooleanBoundaryContactClassificationInput, PlanarBooleanPureBoundaryOnlyOutcomeSet,
        PlanarBooleanSharedBoundaryContactOutcomeSet,
    };

    let _: Option<PlanarBooleanBoundaryContactClassificationInput<'static>> = None;
    let _: Option<PlanarBooleanBoundaryContactClassificationBundle> = None;
    let _: Option<PlanarBooleanSharedBoundaryContactOutcomeSet> = None;
    let _: Option<PlanarBooleanPureBoundaryOnlyOutcomeSet> = None;
}

#[test]
fn planar_boolean_overlap_region_extraction_facade_exports_phase_nine_shared_area_admission_surfaces() {
    use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
        PlanarBooleanMixedBoundaryAreaOutcomeSet, PlanarBooleanSharedAreaAdmissionBundle,
        PlanarBooleanSharedAreaAdmissionInput, PlanarBooleanSharedAreaAdmissionOutcomeSet,
    };

    let _: Option<PlanarBooleanSharedAreaAdmissionInput<'static>> = None;
    let _: Option<PlanarBooleanSharedAreaAdmissionBundle> = None;
    let _: Option<PlanarBooleanSharedAreaAdmissionOutcomeSet> = None;
    let _: Option<PlanarBooleanMixedBoundaryAreaOutcomeSet> = None;
}

#[test]
fn planar_boolean_overlap_region_extraction_facade_exports_phase_ten_pre_region_normalization_surfaces() {
    use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
        PlanarBooleanOppositeSenseOverlapNormalizationSet,
        PlanarBooleanPreRegionNormalizationBundle, PlanarBooleanPreRegionNormalizationInput,
    };

    let _: Option<PlanarBooleanPreRegionNormalizationInput<'static>> = None;
    let _: Option<PlanarBooleanPreRegionNormalizationBundle> = None;
    let _: Option<PlanarBooleanOppositeSenseOverlapNormalizationSet> = None;
}

#[test]
fn planar_boolean_overlap_region_extraction_facade_exports_phase_eleven_region_candidate_surfaces() {
    use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
        PlanarBooleanAdmittedOverlapRegionSet, PlanarBooleanBoundaryOnlyOverlapOutcomeSet,
        PlanarBooleanDeniedOverlapRegionCandidateSet,
        PlanarBooleanOverlapRegionCandidateBoundaryBundle,
        PlanarBooleanOverlapRegionCandidateBoundaryInput, PlanarBooleanOverlapRegionCandidateSet,
    };

    let _: Option<PlanarBooleanOverlapRegionCandidateBoundaryInput<'static>> = None;
    let _: Option<PlanarBooleanOverlapRegionCandidateBoundaryBundle> = None;
    let _: Option<PlanarBooleanOverlapRegionCandidateSet> = None;
    let _: Option<PlanarBooleanDeniedOverlapRegionCandidateSet> = None;
    let _: Option<PlanarBooleanAdmittedOverlapRegionSet> = None;
    let _: Option<PlanarBooleanBoundaryOnlyOverlapOutcomeSet> = None;
}

#[test]
fn planar_boolean_overlap_region_extraction_facade_exports_phase_twelve_post_admission_normalization_surfaces() {
    use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
        PlanarBooleanOverlapRegionCanonicalWindingSet,
        PlanarBooleanPostAdmissionNormalizationBundle, PlanarBooleanPostAdmissionNormalizationInput,
    };

    let _: Option<PlanarBooleanPostAdmissionNormalizationInput<'static>> = None;
    let _: Option<PlanarBooleanPostAdmissionNormalizationBundle> = None;
    let _: Option<PlanarBooleanOverlapRegionCanonicalWindingSet> = None;
}

#[test]
fn planar_boolean_overlap_region_extraction_facade_exports_phase_thirteen_identity_lineage_surfaces() {
    use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
        PlanarBooleanOverlapRegionIdentityLineageBundle,
        PlanarBooleanOverlapRegionIdentityLineageInput, PlanarBooleanOverlapRegionIdentityMap,
        PlanarBooleanOverlapRegionPersistentNamePropagationMap,
        PlanarBooleanOverlapRegionSubshapeSignatureMap,
    };

    let _: Option<PlanarBooleanOverlapRegionIdentityLineageInput<'static>> = None;
    let _: Option<PlanarBooleanOverlapRegionIdentityLineageBundle> = None;
    let _: Option<PlanarBooleanOverlapRegionIdentityMap> = None;
    let _: Option<PlanarBooleanOverlapRegionPersistentNamePropagationMap> = None;
    let _: Option<PlanarBooleanOverlapRegionSubshapeSignatureMap> = None;
}

#[test]
fn planar_boolean_overlap_region_extraction_facade_exports_phase_fourteen_overlap_ledger_surfaces() {
    use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
        PlanarBooleanOverlapRegionDecisionLog, PlanarBooleanOverlapRegionLedger,
        PlanarBooleanOverlapRegionLedgerAssemblyBundle,
        PlanarBooleanOverlapRegionLedgerAssemblyInput, PlanarBooleanOverlapRegionLedgerReceipt,
    };

    let _: Option<PlanarBooleanOverlapRegionLedgerAssemblyInput<'static>> = None;
    let _: Option<PlanarBooleanOverlapRegionLedgerAssemblyBundle> = None;
    let _: Option<PlanarBooleanOverlapRegionDecisionLog> = None;
    let _: Option<PlanarBooleanOverlapRegionLedger> = None;
    let _: Option<PlanarBooleanOverlapRegionLedgerReceipt> = None;
}

#[test]
fn planar_boolean_overlap_region_extraction_facade_exports_phase_fifteen_replay_closeout_surfaces() {
    use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
        PlanarBooleanOverlapRegionCheckpointParityReceipt,
        PlanarBooleanOverlapRegionEvidenceInput, PlanarBooleanOverlapRegionEvidenceReceipt,
        PlanarBooleanOverlapRegionReplayParityInput, PlanarBooleanOverlapRegionReplayParityReceipt,
    };

    let _: Option<PlanarBooleanOverlapRegionEvidenceInput<'static>> = None;
    let _: Option<PlanarBooleanOverlapRegionEvidenceReceipt> = None;
    let _: Option<PlanarBooleanOverlapRegionReplayParityInput<'static>> = None;
    let _: Option<PlanarBooleanOverlapRegionReplayParityReceipt> = None;
    let _: Option<PlanarBooleanOverlapRegionCheckpointParityReceipt> = None;
}
