use crate::application::{
    ForgeQueryDeclarationEntryInspectionInput, ForgeQueryDeclarationEntryReadinessStatus,
};
use crate::runtime::tests::support::stateful_bridge_task_runtime;
use crate::runtime::{
    runtime_state_snapshot_basis_label_identity,
    runtime_state_snapshot_result_shape_label_identity, ForgeQueryInspection,
    ForgeQueryRuntimeStateKind,
};

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
        world_state.basis_for_reporting(),
        runtime_state_snapshot_basis_label_identity(
            world_basis.basis_lifecycle_support_identity(),
        )
        .as_str()
    );
    assert_eq!(
        world_state.result_shape_for_reporting(),
        runtime_state_snapshot_result_shape_label_identity(world_basis.handle_identity(),).as_str()
    );

    match world_inspection {
        ForgeQueryInspection::BasisLifecycle(inspection) => {
            assert_eq!(inspection.subject_label(), "admitted_world_basis");
            assert_eq!(inspection.state_kind(), ForgeQueryRuntimeStateKind::Ready);
            assert_eq!(
                inspection.support_digest(),
                Some(world_basis.support_snapshot_digest())
            );
            assert_eq!(inspection.basis_digest(), world_state.basis_for_reporting());
            assert_eq!(
                inspection.shape_digest(),
                world_state.result_shape_for_reporting()
            );
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

    let stale_envelope = temporal_current_envelope(
        &handle,
        TemporalInput::<TemporalCurrentFamily>::stale("edge:stale"),
    );
    let stale_inspection = handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Enveloped(stale_envelope),
        ))
        .unwrap_or_else(|_| panic!("stale temporal-family envelope should inspect"));
    let stale_bridge_row = stale_inspection
        .readiness()
        .rows()
        .iter()
        .find(|row| row.crossing_row().bridge_continuation_family().is_some())
        .expect("stale bridge row should exist");
    assert_eq!(
        stale_bridge_row.status(),
        ForgeQueryDeclarationEntryReadinessStatus::Admitted
    );
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

    let blocking_envelope = async_current_envelope(
        &handle,
        AsyncInput::<AsyncCurrentFamily>::bridge_blocking("edge:blocking"),
    );
    let blocking_inspection = handle
        .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Enveloped(blocking_envelope),
        ))
        .unwrap_or_else(|_| panic!("blocking async-family envelope should inspect"));
    let blocking_bridge_row = blocking_inspection
        .readiness()
        .rows()
        .iter()
        .find(|row| row.crossing_row().bridge_continuation_family().is_some())
        .expect("blocking bridge row should exist");
    assert_eq!(
        blocking_bridge_row.status(),
        ForgeQueryDeclarationEntryReadinessStatus::Admitted
    );
}
