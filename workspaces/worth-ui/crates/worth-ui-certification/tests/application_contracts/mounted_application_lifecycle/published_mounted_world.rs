use worth_ui::facade::app::WorthUiActiveApplicationSession;
use worth_ui_host_contract::UiHostPresentationEpoch;
use worth_ui_runtime::facade::mounted::{
    UiMountedFrameIdentity, UiMountedFrameOutcome, UiMountedInspectionReceipt,
    UiMountedInspectionRequest, UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity,
    UiPresentationDeadline, UiSurfaceBindingGeneration,
};
use worth_ui_test_support::WorthUiMountedIdentityCertificationExt;
use worth_ui_test_support::WorthUiMountedPublicationCertificationExt;

use super::in_flight_presentation_world::{mounted_session, prepared};
use crate::mounted_host_protocol::scripted_host::ScriptedPresentationHost;

#[derive(Clone, Copy)]
pub(crate) struct PresentedObservationBasis {
    pub(crate) frame: UiMountedFrameIdentity,
    pub(crate) epoch: UiHostPresentationEpoch,
    pub(crate) instance: UiMountedInstanceIdentity,
    pub(crate) receipt: UiMountedNodeReceiptIdentity,
}

pub(crate) struct MultiSurfaceObservationWorld {
    pub(crate) session: WorthUiActiveApplicationSession,
    pub(crate) surfaces: Box<[(UiSurfaceBindingGeneration, PresentedObservationBasis)]>,
}

pub(crate) struct PublishedObservationWorld {
    pub(crate) session: WorthUiActiveApplicationSession,
    pub(crate) host: ScriptedPresentationHost,
    pub(crate) binding: UiSurfaceBindingGeneration,
    pub(crate) predecessor: PresentedObservationBasis,
    pub(crate) current: PresentedObservationBasis,
}

pub(crate) fn published_observation_world(label: &str) -> PublishedObservationWorld {
    published_observation_world_with_host(label, ScriptedPresentationHost::default())
}

pub(crate) fn published_observation_world_with_host(
    label: &str,
    host: ScriptedPresentationHost,
) -> PublishedObservationWorld {
    let (mut session, bindings) = mounted_session(host.clone(), label, 1);
    let binding = bindings[0];
    let instance = session.inspect_mounted_identity().mounted_instances()[0].identity();
    let predecessor = publish(&mut session, &host, instance);
    let current = publish(&mut session, &host, instance);
    PublishedObservationWorld {
        session,
        host,
        binding,
        predecessor,
        current,
    }
}

pub(crate) fn multi_surface_observation_world(
    label: &str,
    surface_count: usize,
) -> MultiSurfaceObservationWorld {
    let host = ScriptedPresentationHost::default();
    let (mut session, bindings) = mounted_session(host.clone(), label, surface_count);
    for _ in 0..surface_count {
        host.push_presented();
    }
    let frame = prepared(&mut session);
    let frame_identity = frame.canonical_core().frame();
    match session.present_prepared_mounted_frame(frame, UiPresentationDeadline::at_tick(1_000), 0) {
        UiMountedFrameOutcome::Published(_) => {}
        _ => panic!("scripted multi-surface frame must publish"),
    }
    let presentation = inspect_current_presentation(&session);
    let identity = session.inspect_mounted_identity();
    let surfaces = bindings
        .into_iter()
        .map(|binding| {
            let surface = identity
                .surface_bindings()
                .iter()
                .find(|candidate| candidate.binding_generation() == binding)
                .expect("published binding remains indexed")
                .semantic_surface_identity();
            let instance = identity
                .mounted_instances()
                .iter()
                .find(|candidate| candidate.basis().semantic_surface_identity() == surface)
                .expect("each surface owns one mounted instance")
                .identity();
            let receipt = identity
                .frame_receipts()
                .iter()
                .find(|candidate| candidate.mounted_instance_identity() == instance)
                .expect("published instance has a receipt")
                .node_receipt_identity();
            let epoch = presentation
                .surfaces()
                .iter()
                .find(|candidate| candidate.binding() == binding)
                .expect("published binding has one surface presentation receipt")
                .epoch();
            (
                binding,
                PresentedObservationBasis {
                    frame: frame_identity,
                    epoch,
                    instance,
                    receipt,
                },
            )
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    drop(identity);
    MultiSurfaceObservationWorld { session, surfaces }
}

pub(crate) fn publish(
    session: &mut WorthUiActiveApplicationSession,
    host: &ScriptedPresentationHost,
    instance: UiMountedInstanceIdentity,
) -> PresentedObservationBasis {
    host.push_presented();
    let frame = prepared(session);
    let frame_identity = frame.canonical_core().frame();
    match session.present_prepared_mounted_frame(frame, UiPresentationDeadline::at_tick(1_000), 0) {
        UiMountedFrameOutcome::Published(_) => {}
        _ => panic!("scripted complete frame must publish"),
    }
    let receipt = session
        .inspect_mounted_identity()
        .frame_receipts()
        .iter()
        .find(|receipt| receipt.mounted_instance_identity() == instance)
        .expect("published instance has one frame-scoped receipt")
        .node_receipt_identity();
    let presentation = inspect_current_presentation(session);
    assert_eq!(
        presentation.surfaces().len(),
        1,
        "the shared single-surface publication helper requires one presented surface"
    );
    PresentedObservationBasis {
        frame: frame_identity,
        epoch: presentation.surfaces()[0].epoch(),
        instance,
        receipt,
    }
}

pub(crate) fn presented_epoch(
    session: &WorthUiActiveApplicationSession,
    frame: UiMountedFrameIdentity,
    binding: UiSurfaceBindingGeneration,
) -> UiHostPresentationEpoch {
    let inspected = match session.inspect_mounted_frame(UiMountedInspectionRequest::frame(frame)) {
        UiMountedInspectionReceipt::Available(frame) => frame,
        other => panic!("presented frame must retain presentation evidence, got {other:?}"),
    };
    inspected
        .presentation()
        .surfaces()
        .iter()
        .find(|surface| surface.binding() == binding)
        .expect("presented binding has one surface receipt")
        .epoch()
}

fn inspect_current_presentation(
    session: &WorthUiActiveApplicationSession,
) -> worth_ui_runtime::facade::mounted::UiMountedPresentationReceipt {
    match session.inspect_mounted_frame(UiMountedInspectionRequest::current()) {
        UiMountedInspectionReceipt::Available(frame) => frame.presentation().clone(),
        other => panic!("published frame must retain presentation evidence, got {other:?}"),
    }
}
