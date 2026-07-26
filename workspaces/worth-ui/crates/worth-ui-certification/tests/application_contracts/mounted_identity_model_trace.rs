use std::collections::BTreeMap;
use worth_ui_test_support::WorthUiMountedIdentityCertificationExt;

use super::mounted_application_lifecycle::known_empty_surface_world::{
    active_session, first_node, profile, registered_surface,
};
use super::mounted_protocol_model::{MountedIdentityModel, MountedIdentityModelOperation};
use worth_ui::facade::app::WorthUiActiveApplicationSession;
use worth_ui_runtime::facade::mounted::{
    UiMountedInstanceIdentity, UiSurfaceBindingGeneration, UiSurfaceBindingIdentityView,
};

struct TraceEvidence<'a> {
    identities: &'a BTreeMap<u8, UiMountedInstanceIdentity>,
    bindings: &'a [UiSurfaceBindingGeneration],
    frames: &'a [worth_ui_runtime::facade::mounted::UiMountedFrameIdentity],
    trace: &'static str,
}

#[test]
fn bounded_identity_trace_agrees_with_independent_model_after_every_step() {
    let mut session = active_session();
    let surface = registered_surface(&mut session);
    let node = first_node(&session);
    let mut model = MountedIdentityModel::known_empty_surface();
    let mut identities = BTreeMap::new();
    let mut bindings = vec![binding(&session).binding_generation()];
    let mut frames = Vec::new();

    assert_agreement(
        &session,
        &model,
        evidence(&identities, &bindings, &frames, "known-empty"),
    );
    let first = session.mount_instance(node, surface).unwrap();
    identities.insert(1, first);
    model.apply(MountedIdentityModelOperation::Mount(1));
    assert_agreement(
        &session,
        &model,
        evidence(&identities, &bindings, &frames, "mount(1)"),
    );

    let second = session.mount_instance(node, surface).unwrap();
    identities.insert(2, second);
    model.apply(MountedIdentityModelOperation::Mount(2));
    assert_agreement(
        &session,
        &model,
        evidence(&identities, &bindings, &frames, "mount(1), mount(2)"),
    );

    session.reorder_mounted_instances(&[second, first]).unwrap();
    model.apply(MountedIdentityModelOperation::Reorder(&[2, 1]));
    assert_agreement(
        &session,
        &model,
        evidence(
            &identities,
            &bindings,
            &frames,
            "mount(1), mount(2), reorder(2,1)",
        ),
    );

    frames.push(session.advance_mounted_identity_frame().unwrap());
    model.apply(MountedIdentityModelOperation::AdvanceFrame);
    assert_agreement(
        &session,
        &model,
        evidence(
            &identities,
            &bindings,
            &frames,
            "mount(1), mount(2), reorder(2,1), frame",
        ),
    );

    let rebound = session
        .rebind_host_surface(
            bindings[0],
            binding(&session).presentation_mode(),
            profile(2),
        )
        .unwrap();
    bindings.push(rebound.binding_generation());
    model.apply(MountedIdentityModelOperation::RebindSurface);
    assert_agreement(
        &session,
        &model,
        evidence(
            &identities,
            &bindings,
            &frames,
            "mount(1), mount(2), reorder(2,1), frame, rebind",
        ),
    );

    session.unmount_instance(first).unwrap();
    model.apply(MountedIdentityModelOperation::Unmount(1));
    assert_agreement(
        &session,
        &model,
        evidence(
            &identities,
            &bindings,
            &frames,
            "mount(1), mount(2), reorder(2,1), frame, rebind, unmount(1)",
        ),
    );

    let third = session.mount_instance(node, surface).unwrap();
    identities.insert(3, third);
    model.apply(MountedIdentityModelOperation::Mount(3));
    assert_agreement(
        &session,
        &model,
        evidence(
            &identities,
            &bindings,
            &frames,
            "mount(1), mount(2), reorder(2,1), frame, rebind, unmount(1), mount(3)",
        ),
    );

    session.reorder_mounted_instances(&[third, second]).unwrap();
    model.apply(MountedIdentityModelOperation::Reorder(&[3, 2]));
    assert_agreement(
        &session,
        &model,
        evidence(
            &identities,
            &bindings,
            &frames,
            "mount(1), mount(2), reorder(2,1), frame, rebind, unmount(1), mount(3), reorder(3,2)",
        ),
    );

    frames.push(session.advance_mounted_identity_frame().unwrap());
    model.apply(MountedIdentityModelOperation::AdvanceFrame);
    assert_agreement(
        &session,
        &model,
        evidence(
            &identities,
            &bindings,
            &frames,
            "mount(1), mount(2), reorder(2,1), frame, rebind, unmount(1), mount(3), reorder(3,2), frame",
        ),
    );
}

fn assert_agreement(
    session: &WorthUiActiveApplicationSession,
    model: &MountedIdentityModel,
    evidence: TraceEvidence<'_>,
) {
    let production = session.inspect_mounted_identity();
    assert_eq!(
        production.mounted_instances().len(),
        model.snapshot().live_count(),
        "trace: {}",
        evidence.trace
    );
    let reverse = evidence
        .identities
        .iter()
        .map(|(model, production)| (*production, *model))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        production
            .mounted_instances()
            .iter()
            .map(|entry| reverse[&entry.identity()])
            .collect::<Vec<_>>(),
        model.snapshot().visible_order(),
        "trace: {}",
        evidence.trace
    );
    assert_eq!(
        evidence.bindings.len(),
        usize::from(model.snapshot().binding_generation()),
        "trace: {}",
        evidence.trace
    );
    assert_eq!(
        evidence.frames.len(),
        usize::from(model.snapshot().frame_generation()),
        "trace: {}",
        evidence.trace
    );
    assert_eq!(
        evidence.identities.len(),
        usize::from(model.snapshot().incarnation_generation()),
        "trace: {}",
        evidence.trace
    );
    assert_eq!(
        production.current_frame().is_some(),
        model.snapshot().frame_current(),
        "trace: {}",
        evidence.trace
    );
    if model.snapshot().frame_current() {
        assert_eq!(
            production.current_frame(),
            evidence.frames.last().copied(),
            "trace: {}",
            evidence.trace
        );
    }
}

fn evidence<'a>(
    identities: &'a BTreeMap<u8, UiMountedInstanceIdentity>,
    bindings: &'a [UiSurfaceBindingGeneration],
    frames: &'a [worth_ui_runtime::facade::mounted::UiMountedFrameIdentity],
    trace: &'static str,
) -> TraceEvidence<'a> {
    TraceEvidence {
        identities,
        bindings,
        frames,
        trace,
    }
}

fn binding(session: &WorthUiActiveApplicationSession) -> UiSurfaceBindingIdentityView {
    session.inspect_mounted_identity().surface_bindings()[0]
}
