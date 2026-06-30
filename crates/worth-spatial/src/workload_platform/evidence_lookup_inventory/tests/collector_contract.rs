use super::super::{
    EvidenceLookupAuthorityKind, EvidenceLookupCertificationPosture, EvidenceLookupCostPosture,
    EvidenceLookupDisposition, EvidenceLookupInventoryCollector, EvidenceLookupInventoryErrorKind,
    EvidenceLookupInventoryRow, EvidenceLookupInventoryRowScope, EvidenceLookupOwner,
    EvidenceLookupQuerySurface, EvidenceLookupReplacementPhase,
};
use crate::workload_platform::evidence_lookup_inventory::coverage::EvidenceLookupCoverageGuardReport;

#[test]
fn duplicate_source_identity_is_denied_by_collector() {
    let guard = EvidenceLookupCoverageGuardReport::clean(2, 2, 2);
    let error = EvidenceLookupInventoryCollector::with_guard_report(guard)
        .admit_row(duplicate_behavior_row(
            "crates/worth-spatial/fixtures/evidence_lookup_inventory/classified_lookup_surface.rs",
        ))
        .expect("first source admitted")
        .admit_row(duplicate_behavior_row(
            "crates/worth-spatial/fixtures/evidence_lookup_inventory/classified_lookup_surface.rs",
        ))
        .expect_err("same source and surface identity is duplicate");

    assert_eq!(
        error.kind(),
        EvidenceLookupInventoryErrorKind::DuplicateInventoryRowIdentity
    );
}

#[test]
fn closeout_requires_one_row_per_covered_surface() {
    let guard = EvidenceLookupCoverageGuardReport::clean(2, 2, 2);
    let error = EvidenceLookupInventoryCollector::with_guard_report(guard)
        .admit_row(duplicate_behavior_row(
            "crates/worth-spatial/fixtures/evidence_lookup_inventory/classified_lookup_surface.rs",
        ))
        .expect("first source admitted")
        .closeout_without_catalog_validation()
        .expect_err("under-filled closeout must not pass");

    assert_eq!(
        error.kind(),
        EvidenceLookupInventoryErrorKind::ClassifiedRowCountMismatch
    );
}

fn duplicate_behavior_row(
    source_path: &'static str,
) -> super::super::EvidenceLookupInventoryRowBuilder {
    EvidenceLookupInventoryRow::builder()
        .source_path(source_path)
        .surface_name("same lookup behavior")
        .owner(EvidenceLookupOwner::WorthSpatial)
        .current_caller("test")
        .authority_kind(EvidenceLookupAuthorityKind::StageLocalNearbyLookup)
        .disposition(EvidenceLookupDisposition::Migrate)
        .replacement_phase(EvidenceLookupReplacementPhase::PhaseTwoFamilyCatalog)
        .blocker("same behavior must keep source identity")
        .removal_trigger("family catalog migration")
        .certification_posture(EvidenceLookupCertificationPosture::OrdinaryProductionReachable)
        .cost_posture(EvidenceLookupCostPosture::LocalTypedLookup)
        .query_surface(EvidenceLookupQuerySurface::NotQuery)
        .row_scope(EvidenceLookupInventoryRowScope::ConcreteSource)
}
