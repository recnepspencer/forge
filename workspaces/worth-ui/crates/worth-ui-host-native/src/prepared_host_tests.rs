use std::cell::RefCell;
use std::rc::Rc;

use winit::dpi::PhysicalPosition;
use winit::event::{DeviceId, WindowEvent};
use worth_ui_host_contract::{
    UiHostObservationPayload, UiHostObservationPresentationBasis, UiHostPresentationEpoch,
    UiHostProtocolContract, UiHostProtocolNegotiation, UiMountedFrameIdentity,
    UiSurfaceBindingGeneration, WorthUiHostMechanicsAdapter,
};

use super::{
    WorthUiNativeMechanicsAdapter, WorthUiPreparedNativeHost, WorthUiPreparedNativeMechanics,
};
use crate::native::UiNativeHostState;

#[test]
fn ordinary_preparation_requires_the_process_main_thread() {
    assert_eq!(
        WorthUiPreparedNativeHost::prepare_qualified().event_loop_thread_posture,
        crate::UiNativeEventLoopThreadPosture::MainThreadRequired
    );
}

#[cfg(feature = "certification-support")]
#[test]
fn certification_preparation_requires_an_explicit_worker_posture() {
    let ordinary = crate::UiNativeQualificationPlan::ordinary();
    assert_eq!(
        WorthUiPreparedNativeHost::prepare_qualified_for_certification(ordinary)
            .event_loop_thread_posture,
        crate::UiNativeEventLoopThreadPosture::MainThreadRequired
    );
    let worker = ordinary.with_certification_worker_event_loop();
    assert_eq!(
        WorthUiPreparedNativeHost::prepare_qualified_for_certification(worker)
            .event_loop_thread_posture,
        crate::UiNativeEventLoopThreadPosture::CertificationWorker
    );
}

#[test]
fn prepared_mechanics_delegates_retained_observation_drain() {
    let host_session = 97;
    let state = Rc::new(RefCell::new(UiNativeHostState::new()));
    let prepared = WorthUiPreparedNativeMechanics {
        adapter: WorthUiNativeMechanicsAdapter::from_preparation(
            Rc::clone(&state),
            crate::UiNativePlatformProfileIdentity::WORTH_UI_WINDOWS_DX12_V1,
        ),
    };
    let protocol = match UiHostProtocolContract::current().negotiate() {
        UiHostProtocolNegotiation::Compatible(agreement) => agreement,
        UiHostProtocolNegotiation::Incompatible(_) => unreachable!(),
    };
    let binding = UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let basis = UiHostObservationPresentationBasis::new(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        binding,
        UiHostPresentationEpoch::issued_by_host(1),
    );
    {
        let mut state = state.borrow_mut();
        state
            .lifecycle_protocol
            .install_initial_profile(1.0, [800, 600]);
        assert_eq!(
            state
                .lifecycle_protocol
                .record_completed_presentation(protocol, host_session, basis)
                .effect(),
            crate::native::UiNativeLifecycleEffect::PresentationCompleted
        );
        let transition = state.lifecycle_protocol.observe_window_event_at(
            &WindowEvent::CursorMoved {
                device_id: DeviceId::dummy(),
                position: PhysicalPosition::new(12.0, 24.0),
            },
            0,
            None,
        );
        assert_eq!(
            transition.effect(),
            crate::native::UiNativeLifecycleEffect::Retained
        );
    }

    let drain = prepared
        .drain_mechanical_host_observations(host_session)
        .unwrap();
    let batches = drain.into_batches();
    assert_eq!(batches.len(), 1);
    assert_eq!(batches[0].canonical_core().presentation(), basis);
    assert!(matches!(
        batches[0].reports()[0].payload(),
        UiHostObservationPayload::PointerMotion { .. }
    ));
}
