use schema::facade::platform::relations::TopologyRelationKind;

use super::catalog_test_fixtures::{
    current_catalog, loop_cycle_graph_facts, loop_cycle_record_input, phase_two_seed,
    rewire_loop_endpoint_declared_basis, synthetic_basis_from_relation_kind,
};
use super::{
    DerivedInvalidationFamilyCatalog, DerivedInvalidationFamilyCatalogCloseout,
    DerivedInvalidationFamilyCatalogErrorKind, DerivedTopologyConsumedGraphFacts,
    DerivedTopologyProductFamilyIdentity, DerivedTopologyQueryReceiptPosture,
    DerivedTopologySpatialEvidencePosture, DerivedTopologyUpdatePosture,
};
use crate::topology_operators::{LoopEndpointKind, TopologyTouchedAspect};

mod source_coverage;

#[test]
fn current_family_catalog_closes_and_emits_phase_three_seed() {
    let catalog = current_catalog();
    let closeout = DerivedInvalidationFamilyCatalogCloseout::close(catalog).unwrap();
    let counters = closeout.catalog().counters();

    assert_eq!(counters.family_count(), 7);
    assert_eq!(counters.required_family_count(), 7);
    assert_eq!(counters.query_required_family_count(), 7);
    assert_eq!(counters.legality_required_family_count(), 7);
    assert_eq!(counters.spatial_receipt_required_family_count(), 0);
    assert_eq!(counters.no_spatial_evidence_family_count(), 7);
    assert_eq!(counters.bounded_rebuild_family_count(), 3);
    assert_eq!(counters.incremental_eligible_family_count(), 4);
    assert_eq!(
        closeout.phase_three_seed().catalog_digest(),
        closeout.catalog().catalog_digest()
    );
    assert_eq!(
        closeout.phase_three_seed().inventory_seed_digest(),
        closeout.catalog().phase_two_seed().seed_digest()
    );
    assert!(!closeout.phase_three_seed().seed_digest().is_empty());
}

#[test]
fn declare_once_family_record_matches_multiple_matching_touched_bases() {
    let catalog = current_catalog();
    let family = catalog
        .family(DerivedTopologyProductFamilyIdentity::WireViews)
        .unwrap();
    let start_rewire_basis = rewire_loop_endpoint_declared_basis(LoopEndpointKind::Start, 31);
    let end_rewire_basis = rewire_loop_endpoint_declared_basis(LoopEndpointKind::End, 41);

    assert!(family.matches_touched_basis(&start_rewire_basis));
    assert!(family.matches_touched_basis(&end_rewire_basis));
    assert!(family
        .consumed_graph_facts()
        .relation_kinds()
        .iter()
        .any(|kind| start_rewire_basis.relation_kinds().contains(kind)));
    assert!(family
        .consumed_graph_facts()
        .relation_kinds()
        .iter()
        .any(|kind| end_rewire_basis.relation_kinds().contains(kind)));
    assert_eq!(
        family.spatial_evidence_posture(),
        DerivedTopologySpatialEvidencePosture::NoSpatialEvidenceConsumed
    );
}

#[test]
fn declare_once_family_record_rejects_unrelated_touched_basis() {
    let catalog = current_catalog();
    let family = catalog
        .family(DerivedTopologyProductFamilyIdentity::RadialRings)
        .unwrap();

    assert!(
        !family.matches_touched_basis(&synthetic_basis_from_relation_kind(
            TopologyRelationKind::ModelOwnsBody
        ))
    );
}

#[test]
fn closeout_rejects_missing_required_family() {
    let mut families = current_catalog().families().to_vec();
    families.retain(|family| family.identity() != DerivedTopologyProductFamilyIdentity::WireViews);
    let catalog = DerivedInvalidationFamilyCatalog::new(phase_two_seed(), families);

    let error = DerivedInvalidationFamilyCatalogCloseout::close(catalog).unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::MissingRequiredFamily {
            family: "wire_views"
        }
    );
}

#[test]
fn family_record_rejects_missing_consumed_graph_facts() {
    let error = loop_cycle_record_input(Some(DerivedTopologyConsumedGraphFacts::new(
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![TopologyTouchedAspect::TopologyBoundary],
    )))
    .with_consumed_graph_facts(None)
    .build()
    .unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::MissingConsumedGraphFacts {
            family: "loop_cycles"
        }
    );
}

#[test]
fn family_record_rejects_empty_consumed_graph_facts() {
    let error = loop_cycle_record_input(Some(DerivedTopologyConsumedGraphFacts::new(
        Vec::new(),
        Vec::new(),
    )))
    .build()
    .unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::EmptyConsumedGraphFacts {
            family: "loop_cycles"
        }
    );
}

#[test]
fn family_record_rejects_missing_invalidation_predicate() {
    let error = loop_cycle_record_input(Some(DerivedTopologyConsumedGraphFacts::new(
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![TopologyTouchedAspect::TopologyBoundary],
    )))
    .with_invalidation_predicate(None)
    .build()
    .unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::MissingInvalidationPredicate {
            family: "loop_cycles"
        }
    );
}

#[test]
fn family_record_rejects_missing_query_posture() {
    let error = loop_cycle_record_input(Some(DerivedTopologyConsumedGraphFacts::new(
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![TopologyTouchedAspect::TopologyBoundary],
    )))
    .with_query_receipt_posture(None)
    .build()
    .unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::MissingQueryReceiptPosture {
            family: "loop_cycles"
        }
    );
}

#[test]
fn family_record_rejects_missing_spatial_evidence_posture() {
    let error = loop_cycle_record_input(Some(DerivedTopologyConsumedGraphFacts::new(
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![TopologyTouchedAspect::TopologyBoundary],
    )))
    .with_spatial_evidence_posture(None)
    .build()
    .unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::MissingSpatialEvidencePosture {
            family: "loop_cycles"
        }
    );
}

#[test]
fn family_record_rejects_missing_update_posture() {
    let error = loop_cycle_record_input(Some(loop_cycle_graph_facts()))
        .with_update_posture(None)
        .build()
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::MissingUpdatePosture {
            family: "loop_cycles"
        }
    );
}

#[test]
fn family_record_rejects_missing_legality_receipt_posture() {
    let error = loop_cycle_record_input(Some(loop_cycle_graph_facts()))
        .with_legality_receipt_posture(None)
        .build()
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::MissingLegalityReceiptPosture {
            family: "loop_cycles"
        }
    );
}

#[test]
fn family_record_rejects_missing_diagnostic_posture() {
    let error = loop_cycle_record_input(Some(loop_cycle_graph_facts()))
        .with_diagnostic_posture(None)
        .build()
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::MissingDiagnosticPosture {
            family: "loop_cycles"
        }
    );
}

#[test]
fn family_record_rejects_missing_support_posture() {
    let error = loop_cycle_record_input(Some(loop_cycle_graph_facts()))
        .with_support_posture(None)
        .build()
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::MissingSupportPosture {
            family: "loop_cycles"
        }
    );
}

#[test]
fn closeout_rejects_duplicate_family_identity() {
    let mut families = current_catalog().families().to_vec();
    let duplicate = families
        .iter()
        .find(|family| family.identity() == DerivedTopologyProductFamilyIdentity::WireViews)
        .unwrap()
        .clone();
    families.push(duplicate);
    let catalog = DerivedInvalidationFamilyCatalog::new(phase_two_seed(), families);

    let error = DerivedInvalidationFamilyCatalogCloseout::close(catalog).unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::DuplicateFamily {
            family: "wire_views"
        }
    );
}

#[test]
fn family_digest_changes_when_consumed_graph_facts_change() {
    let original = loop_cycle_record_input(Some(loop_cycle_graph_facts()))
        .build()
        .unwrap();
    let changed = loop_cycle_record_input(Some(DerivedTopologyConsumedGraphFacts::new(
        vec![TopologyRelationKind::HalfEdgeRadialNext],
        vec![TopologyTouchedAspect::TopologyRadial],
    )))
    .build()
    .unwrap();

    assert_ne!(original.family_digest(), changed.family_digest());
}

#[test]
fn family_digest_changes_when_query_posture_changes() {
    let original = loop_cycle_record_input(Some(loop_cycle_graph_facts()))
        .build()
        .unwrap();
    let changed = loop_cycle_record_input(Some(loop_cycle_graph_facts()))
        .with_query_receipt_posture(Some(
            DerivedTopologyQueryReceiptPosture::ProjectionConsumptionRequired,
        ))
        .build()
        .unwrap();

    assert_ne!(original.family_digest(), changed.family_digest());
}

#[test]
fn family_digest_changes_when_update_posture_changes() {
    let original = loop_cycle_record_input(Some(loop_cycle_graph_facts()))
        .build()
        .unwrap();
    let changed = loop_cycle_record_input(Some(loop_cycle_graph_facts()))
        .with_update_posture(Some(DerivedTopologyUpdatePosture::BoundedRebuildRequired))
        .build()
        .unwrap();

    assert_ne!(original.family_digest(), changed.family_digest());
}

#[test]
fn family_digest_changes_when_spatial_posture_changes() {
    let original = loop_cycle_record_input(Some(loop_cycle_graph_facts()))
        .build()
        .unwrap();
    let changed = loop_cycle_record_input(Some(loop_cycle_graph_facts()))
        .with_spatial_evidence_posture(Some(
            DerivedTopologySpatialEvidencePosture::SpatialReceiptRequired,
        ))
        .build()
        .unwrap();

    assert_ne!(original.family_digest(), changed.family_digest());
}

#[test]
fn query_required_posture_reports_typed_denial_when_support_is_absent() {
    let closeout = DerivedInvalidationFamilyCatalogCloseout::close(current_catalog()).unwrap();

    let error = closeout
        .require_family_query_support_present(DerivedTopologyProductFamilyIdentity::LoopCycles)
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::QuerySupportRequired {
            family: "loop_cycles"
        }
    );
}
