use crate::application::{
    ForgeQueryDeclarationAdmissionOrLegalityError, ForgeQueryDeclarationEntryInspectionInput,
    ForgeQueryDeclarationEntryProgressionError, ForgeQueryDeclarationEntryReadinessRequest,
    ForgeQueryDeclarationEntryReadinessStatus, ForgeQueryTemporalLegalityDenialKind,
};

use super::support::{
    handle, temporal_current_envelope, TemporalCurrentFamily, TemporalHistoricalFamily,
    TemporalInput, TemporalPreviewFamily,
};

#[test]
fn temporal_current_family_projects_runtime_temporal_debt_into_bridge_readiness() {
    let handle = handle("temporal-current");
    let readiness = handle.declaration_entry_readiness::<TemporalInput<TemporalCurrentFamily>>();
    let bridge_row = readiness
        .rows()
        .iter()
        .find(|row| row.crossing_row().bridge_continuation_family().is_some())
        .expect("bridge row should exist");

    assert_eq!(
        bridge_row.status(),
        ForgeQueryDeclarationEntryReadinessStatus::Deferred
    );
    assert_eq!(
        bridge_row.reason(),
        "temporal declaration-entry readiness remains deferred until the temporal runtime facade is admitted"
    );

    assert!(matches!(
        handle.declare_review_and_progress(TemporalInput::<TemporalCurrentFamily>::stale("edge:42")),
        Err(ForgeQueryDeclarationEntryProgressionError::Entry(
            ForgeQueryDeclarationAdmissionOrLegalityError::Legality(
                crate::application::ForgeQueryDeclarationLegalityDenial::TemporalProjectionUnsupported {
                    kind: ForgeQueryTemporalLegalityDenialKind::RuntimeFacadeDeferred,
                    ..
                }
            )
        ))
    ));
}

#[test]
fn temporal_preview_family_keeps_preview_truth_basis_invalid_and_localized() {
    let handle = handle("temporal-preview");
    let readiness = handle.declaration_entry_readiness::<TemporalInput<TemporalPreviewFamily>>();
    let bridge_row = readiness
        .rows()
        .iter()
        .find(|row| row.crossing_row().bridge_continuation_family().is_some())
        .expect("bridge row should exist");
    let signal_row = readiness
        .rows()
        .iter()
        .find(|row| row.crossing_row().signal_execution_family().is_some())
        .expect("signal row should exist");

    assert_eq!(
        bridge_row.status(),
        ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis
    );
    assert_eq!(
        bridge_row.reason(),
        "temporal declaration-entry readiness does not currently admit preview bridge truth context"
    );
    assert_eq!(
        signal_row.status(),
        ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis
    );
    assert_eq!(
        signal_row.reason(),
        "temporal declaration-entry readiness does not currently admit preview-derived signal basis families"
    );
}

#[test]
fn temporal_historical_family_keeps_historical_truth_basis_invalid_and_localized() {
    let handle = handle("temporal-historical");
    let readiness = handle.declaration_entry_readiness::<TemporalInput<TemporalHistoricalFamily>>();
    let bridge_row = readiness
        .rows()
        .iter()
        .find(|row| row.crossing_row().bridge_continuation_family().is_some())
        .expect("bridge row should exist");

    assert_eq!(
        bridge_row.status(),
        ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis
    );
    assert_eq!(
        bridge_row.reason(),
        "temporal declaration-entry readiness does not currently admit historical bridge truth context"
    );
}

#[test]
fn retained_non_temporal_subject_does_not_inherit_family_level_temporal_debt() {
    let handle = handle("temporal-current");
    let envelope = temporal_current_envelope(
        &handle,
        TemporalInput::<TemporalCurrentFamily>::plain("edge:42"),
    );

    let readiness = handle
        .try_declaration_entry_readiness::<TemporalInput<TemporalCurrentFamily>>(
            ForgeQueryDeclarationEntryReadinessRequest::base().for_retained_subject(
                crate::application::ForgeQueryDeclarationEntryRetainedSubjectInput::envelope_checked(
                    crate::application::ForgeQueryDeclarationEnvelopeChecked::Enveloped(
                        envelope,
                    ),
                ),
            ),
        )
        .unwrap_or_else(|_| panic!("subject-aware readiness should succeed"));
    let bridge_row = readiness
        .rows()
        .iter()
        .find(|row| row.crossing_row().bridge_continuation_family().is_some())
        .expect("bridge row should exist");
    assert_eq!(
        bridge_row.status(),
        ForgeQueryDeclarationEntryReadinessStatus::Admitted
    );

    let inspection = handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Enveloped(
                temporal_current_envelope(
                    &handle,
                    TemporalInput::<TemporalCurrentFamily>::plain("edge:43"),
                ),
            ),
        ))
        .unwrap_or_else(|_| panic!("inspection should succeed"));
    let inspected_bridge_row = inspection
        .readiness()
        .rows()
        .iter()
        .find(|row| row.crossing_row().bridge_continuation_family().is_some())
        .expect("bridge row should exist");
    assert_eq!(
        inspected_bridge_row.status(),
        ForgeQueryDeclarationEntryReadinessStatus::Admitted
    );
}
