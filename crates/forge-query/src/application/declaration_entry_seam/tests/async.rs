use crate::application::{
    ForgeQueryAsyncLegalityDenialKind, ForgeQueryDeclarationAdmissionOrLegalityError,
    ForgeQueryDeclarationEntryInspectionInput, ForgeQueryDeclarationEntryProgressionError,
    ForgeQueryDeclarationEntryReadinessRequest, ForgeQueryDeclarationEntryReadinessStatus,
};

use super::support::{
    async_current_envelope, handle, AsyncCurrentFamily, AsyncInput, AsyncPreviewFamily,
    DeferredAsyncFamily,
};

#[test]
fn async_current_family_projects_runtime_async_debt_into_bridge_readiness() {
    let handle = handle("async-current");
    let readiness = handle.declaration_entry_readiness::<AsyncInput<AsyncCurrentFamily>>();
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
        "async declaration-entry readiness remains deferred until the async-resource runtime facade is admitted"
    );

    assert!(matches!(
        handle.declare_review_and_progress(AsyncInput::<AsyncCurrentFamily>::bridge_blocking("edge:42")),
        Err(ForgeQueryDeclarationEntryProgressionError::Entry(
            ForgeQueryDeclarationAdmissionOrLegalityError::Legality(
                crate::application::ForgeQueryDeclarationLegalityDenial::AsyncProjectionUnsupported {
                    kind: ForgeQueryAsyncLegalityDenialKind::RuntimeFacadeDeferred,
                    ..
                }
            )
        ))
    ));
}

#[test]
fn deferred_async_family_projects_family_level_async_debt_before_runtime_row_meaning() {
    let handle = handle("async-deferred-family");
    let readiness = handle.declaration_entry_readiness::<AsyncInput<DeferredAsyncFamily>>();
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
        "async declaration-entry readiness remains deferred until this family's async declaration surface is admitted"
    );
}

#[test]
fn async_preview_family_keeps_preview_basis_invalid_and_localized() {
    let handle = handle("async-preview");
    let readiness = handle.declaration_entry_readiness::<AsyncInput<AsyncPreviewFamily>>();
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
        "async declaration-entry readiness does not currently admit preview bridge truth context"
    );
    assert_eq!(
        signal_row.status(),
        ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis
    );
    assert_eq!(
        signal_row.reason(),
        "async declaration-entry readiness does not currently admit preview-derived signal basis families"
    );
}

#[test]
fn retained_non_async_subject_does_not_inherit_family_level_async_debt() {
    let handle = handle("async-current");
    let envelope =
        async_current_envelope(&handle, AsyncInput::<AsyncCurrentFamily>::plain("edge:42"));

    let readiness = handle
        .try_declaration_entry_readiness::<AsyncInput<AsyncCurrentFamily>>(
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
                async_current_envelope(&handle, AsyncInput::<AsyncCurrentFamily>::plain("edge:43")),
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

#[test]
fn unsupported_async_source_is_rejected_before_route_planning() {
    let handle = handle("async-current");

    assert!(matches!(
        handle.declare_review_and_progress(AsyncInput::<AsyncCurrentFamily>::external_refresh("edge:99")),
        Err(ForgeQueryDeclarationEntryProgressionError::Entry(
            ForgeQueryDeclarationAdmissionOrLegalityError::Legality(
                crate::application::ForgeQueryDeclarationLegalityDenial::AsyncProjectionUnsupported {
                    kind: ForgeQueryAsyncLegalityDenialKind::UnsupportedSourceFamily(
                        crate::application::ForgeQueryAsyncSourceFamily::ExternalResource,
                    ),
                    ..
                }
            )
        ))
    ));
}
