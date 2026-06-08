use crate::application::{
    ForgeQueryDeclarationAdmissionOrLegalityError, ForgeQueryDeclarationEntryInspectionInput,
    ForgeQueryDeclarationEntryProgressionError, ForgeQueryDeclarationEntryReadinessStatus,
};
use crate::runtime::tests::support::stateful_bridge_task_runtime;
use crate::runtime::{ForgeQueryInspection, ForgeQueryRuntimeStateKind};

use super::support::{
    async_current_envelope, handle, temporal_current_envelope, AsyncCurrentFamily, AsyncInput,
    TemporalCurrentFamily, TemporalInput,
};

#[test]
fn retained_world_basis_and_subject_aware_temporal_readiness_stay_stitched() {
    let handle = handle("cross-phase-temporal");
    let runtime = stateful_bridge_task_runtime();
    let workspace = runtime
        .workspace("cross.phase.temporal")
        .expect("workspace should open");
    let world_basis = handle.retained_world_basis();

    let world_state = workspace
        .state(&world_basis)
        .expect("retained world basis should snapshot");
    let world_inspection = workspace
        .inspect(&world_basis)
        .expect("retained world basis should inspect");

    assert_eq!(world_state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(
        world_state.basis_digest(),
        world_basis.basis_lifecycle_support_digest()
    );
    assert_eq!(
        world_state.result_shape_digest(),
        world_basis.handle_identity_digest()
    );

    match world_inspection {
        ForgeQueryInspection::BasisLifecycle(inspection) => {
            assert_eq!(inspection.subject_label(), "admitted_world_basis");
            assert_eq!(inspection.state_kind(), ForgeQueryRuntimeStateKind::Ready);
            assert_eq!(
                inspection.support_digest(),
                Some(world_basis.support_snapshot_digest())
            );
            assert_eq!(inspection.basis_digest(), world_state.basis_digest());
            assert_eq!(inspection.shape_digest(), world_state.result_shape_digest());
        }
        other => panic!("expected basis lifecycle inspection, got {other:?}"),
    }

    let plain_envelope = temporal_current_envelope(
        &handle,
        TemporalInput::<TemporalCurrentFamily>::plain("edge:plain"),
    );
    let plain_inspection = handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Enveloped(plain_envelope),
        ))
        .unwrap_or_else(|_| panic!("plain temporal-family envelope should inspect"));
    let bridge_row = plain_inspection
        .readiness()
        .rows()
        .iter()
        .find(|row| row.crossing_row().bridge_continuation_family().is_some())
        .expect("bridge row should exist");
    assert_eq!(
        bridge_row.status(),
        ForgeQueryDeclarationEntryReadinessStatus::Admitted
    );

    assert!(matches!(
        handle.declare_review_and_progress(TemporalInput::<TemporalCurrentFamily>::stale("edge:stale")),
        Err(ForgeQueryDeclarationEntryProgressionError::Entry(
            ForgeQueryDeclarationAdmissionOrLegalityError::Legality(
                crate::application::ForgeQueryDeclarationLegalityDenial::TemporalProjectionUnsupported { .. }
            )
        ))
    ));
}

#[test]
fn retained_world_basis_and_subject_aware_async_readiness_stay_stitched() {
    let handle = handle("cross-phase-async");
    let runtime = stateful_bridge_task_runtime();
    let workspace = runtime
        .workspace("cross.phase.async")
        .expect("workspace should open");
    let world_basis = handle.retained_world_basis();

    let world_inspection = workspace
        .inspect(&world_basis)
        .expect("retained world basis should inspect");
    match world_inspection {
        ForgeQueryInspection::BasisLifecycle(inspection) => {
            assert_eq!(inspection.subject_label(), "admitted_world_basis");
            assert_eq!(
                inspection.support_digest(),
                Some(world_basis.support_snapshot_digest())
            );
        }
        other => panic!("expected basis lifecycle inspection, got {other:?}"),
    }

    let plain_envelope = async_current_envelope(
        &handle,
        AsyncInput::<AsyncCurrentFamily>::plain("edge:plain"),
    );
    let plain_inspection = handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Enveloped(plain_envelope),
        ))
        .unwrap_or_else(|_| panic!("plain async-family envelope should inspect"));
    let bridge_row = plain_inspection
        .readiness()
        .rows()
        .iter()
        .find(|row| row.crossing_row().bridge_continuation_family().is_some())
        .expect("bridge row should exist");
    assert_eq!(
        bridge_row.status(),
        ForgeQueryDeclarationEntryReadinessStatus::Admitted
    );

    assert!(matches!(
        handle.declare_review_and_progress(AsyncInput::<AsyncCurrentFamily>::bridge_blocking("edge:blocking")),
        Err(ForgeQueryDeclarationEntryProgressionError::Entry(
            ForgeQueryDeclarationAdmissionOrLegalityError::Legality(
                crate::application::ForgeQueryDeclarationLegalityDenial::AsyncProjectionUnsupported { .. }
            )
        ))
    ));
}
