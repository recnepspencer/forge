use crate::application::{
    ForgeQueryDeclarationEntryInspectionInput, ForgeQueryDeclarationEntryReadinessStatus,
    ForgeQueryDeclarationEntrySeamClassification, ForgeQueryDeclarationSignalCompatibilityInput,
};

use super::support::{
    bridge_signal_envelope, handle, signal_disabled_handle, BridgeSignalFamily,
    DeferredSignalFamily, Input, MixedFamily, RelationalFamily,
};

#[test]
fn inventory_digest_changes_when_admitted_world_changes() {
    let left = handle("alpha").declaration_entry_crossing_inventory::<Input<RelationalFamily>>();
    let right = handle("beta").declaration_entry_crossing_inventory::<Input<RelationalFamily>>();

    assert_ne!(left.inventory_digest(), right.inventory_digest());
}

#[test]
fn mixed_family_inventory_keeps_relational_and_bridge_rows_distinct() {
    let inventory = handle("mixed").declaration_entry_crossing_inventory::<Input<MixedFamily>>();
    assert!(inventory
        .rows()
        .iter()
        .any(|row| row.relational_truth_claim().is_some()));
    assert!(inventory
        .rows()
        .iter()
        .any(|row| row.bridge_continuation_family().is_some()));
    assert!(inventory
        .rows()
        .iter()
        .any(|row| row.entrypoint_key() == "route-relational-truth-checked"));
    assert!(inventory
        .rows()
        .iter()
        .any(|row| row.entrypoint_key() == "route-bridge-continuation-from-progressed"));
}

#[test]
fn deferred_signal_family_uses_deferred_neighbor_classification() {
    let inventory =
        handle("deferred").declaration_entry_crossing_inventory::<Input<DeferredSignalFamily>>();
    let signal_row = inventory
        .rows()
        .iter()
        .find(|row| {
            matches!(
                row.surface(),
                crate::application::ForgeQueryDeclarationEntryCrossingSurface::SignalCompatibility
            )
        })
        .expect("deferred signal row should exist");
    assert_eq!(
        signal_row.seam_classification(),
        ForgeQueryDeclarationEntrySeamClassification::DeferredNeighbor
    );
}

#[test]
fn readiness_projection_matches_signal_support_status() {
    let handle = handle("preview");
    let readiness = handle.declaration_entry_readiness::<Input<BridgeSignalFamily>>();
    let signal_row = readiness
        .rows()
        .iter()
        .find(|row| row.crossing_row().signal_execution_family().is_some())
        .expect("signal row should exist");
    assert_eq!(
        signal_row.status(),
        ForgeQueryDeclarationEntryReadinessStatus::Admitted
    );

    let signal_support = handle.signal_compatibility_support::<Input<BridgeSignalFamily>>();
    assert_eq!(signal_support.rows().len(), 1);
    assert_eq!(signal_support.rows()[0].reason(), signal_row.reason());
}

#[test]
fn mixed_family_readiness_matches_relational_and_bridge_support_rows() {
    let handle = handle("mixed");
    let readiness = handle.declaration_entry_readiness::<Input<MixedFamily>>();
    let relational_row = readiness
        .rows()
        .iter()
        .find(|row| row.crossing_row().relational_truth_claim().is_some())
        .expect("relational readiness row should exist");
    let bridge_row = readiness
        .rows()
        .iter()
        .find(|row| row.crossing_row().bridge_continuation_family().is_some())
        .expect("bridge readiness row should exist");

    let relational_support = handle.relational_truth_support::<Input<MixedFamily>>();
    assert_eq!(relational_support.rows().len(), 1);
    assert_eq!(
        relational_support.rows()[0].reason(),
        relational_row.reason()
    );

    let bridge_support = handle.bridge_continuation_support::<Input<MixedFamily>>();
    assert_eq!(bridge_support.rows().len(), 1);
    assert_eq!(bridge_support.rows()[0].reason(), bridge_row.reason());
}

#[test]
fn signalless_world_keeps_invalid_basis_distinct() {
    let readiness = signal_disabled_handle("preview")
        .declaration_entry_readiness::<Input<BridgeSignalFamily>>();
    let signal_row = readiness
        .rows()
        .iter()
        .find(|row| row.crossing_row().signal_execution_family().is_some())
        .expect("signal row should exist");
    assert_eq!(
        signal_row.status(),
        ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis
    );
}

#[test]
fn wrong_handle_retained_artifacts_are_denied_before_inspection() {
    let source = handle("source");
    let target = handle("target");
    let envelope = bridge_signal_envelope(&source, "edge:42");

    let error = match target.inspect_declaration_entry(
        ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope),
        ),
    ) {
        Ok(_) => panic!("wrong-handle inspection must deny"),
        Err(error) => error,
    };
    assert!(error.reason().contains("same admitted handle"));
}

#[test]
fn envelope_only_inspection_does_not_fake_lower_authority_posture() {
    let handle = handle("preview");
    let envelope = bridge_signal_envelope(&handle, "edge:42");

    let inspection = match handle.inspect_declaration_entry(
        ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope),
        ),
    ) {
        Ok(inspection) => inspection,
        Err(_) => panic!("inspection should succeed"),
    };

    assert!(inspection.relational_posture().is_none());
    assert!(inspection.bridge_posture().is_none());
    assert!(inspection.signal_posture().is_none());
    assert_eq!(inspection.matching_row_digests().len(), 5);
}

#[test]
fn signal_checked_inspection_exposes_signal_posture() {
    let handle = handle("preview");
    let envelope = bridge_signal_envelope(&handle, "edge:42");
    let checked = handle.signal_compatibility_checked(
        ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope),
    );
    let inspection = match handle.inspect_declaration_entry(
        ForgeQueryDeclarationEntryInspectionInput::signal_compatibility_checked(checked),
    ) {
        Ok(inspection) => inspection,
        Err(_) => panic!("inspection should succeed"),
    };

    assert!(inspection.signal_posture().is_some());
    assert!(inspection.matching_row_digests().len() >= 2);
}
