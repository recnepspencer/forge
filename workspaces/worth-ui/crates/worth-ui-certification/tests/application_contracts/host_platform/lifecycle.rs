#[path = "lifecycle_model.rs"]
mod lifecycle_model;

use worth_ui_host_native::{
    UiNativeLifecycleProtocolSchedule, UiNativeLifecycleProtocolWorld, UiNativePresentationFault,
    UiNativeProtocolClosePoint, UiNativeProtocolReadback, UiNativeProtocolSurfaceTransition,
};

use lifecycle_model::assert_world;

#[test]
fn lifecycle_faults_match_the_independent_transition_model() {
    for fault in [
        UiNativePresentationFault::Timeout,
        UiNativePresentationFault::Occluded,
        UiNativePresentationFault::Outdated,
        UiNativePresentationFault::SurfaceLost,
        UiNativePresentationFault::Validation,
        UiNativePresentationFault::DeviceLost,
    ] {
        assert_world(UiNativeLifecycleProtocolSchedule::ordinary().fault(fault));
    }
    for fault in [
        UiNativePresentationFault::Outdated,
        UiNativePresentationFault::SurfaceLost,
        UiNativePresentationFault::DeviceLost,
    ] {
        assert_world(
            UiNativeLifecycleProtocolSchedule::ordinary()
                .fault(fault)
                .recover_and_resume(),
        );
    }
}

#[test]
fn close_at_each_effect_stage_drains_actual_queued_readiness() {
    for close_at in [
        UiNativeProtocolClosePoint::PreparedUpload,
        UiNativeProtocolClosePoint::Prepared,
        UiNativeProtocolClosePoint::SurfaceAcquired,
        UiNativeProtocolClosePoint::Encoded,
        UiNativeProtocolClosePoint::Submitted,
        UiNativeProtocolClosePoint::PresentHandoff,
    ] {
        assert_world(
            UiNativeLifecycleProtocolSchedule::ordinary()
                .with_queued_readiness()
                .close_at(close_at),
        );
    }
    assert_world(
        UiNativeLifecycleProtocolSchedule::ordinary()
            .with_queued_readiness()
            .readback(UiNativeProtocolReadback::PendingThenComplete)
            .close_at(UiNativeProtocolClosePoint::Readback),
    );
}

#[test]
fn readback_settlement_and_indeterminacy_match_the_independent_model() {
    for readback in [
        UiNativeProtocolReadback::Complete,
        UiNativeProtocolReadback::PendingThenComplete,
        UiNativeProtocolReadback::Indeterminate,
    ] {
        assert_world(UiNativeLifecycleProtocolSchedule::ordinary().readback(readback));
    }
}

#[test]
fn one_physical_recovery_fact_reconstructs_every_affected_binding() {
    let schedule = UiNativeLifecycleProtocolSchedule::ordinary()
        .fault(UiNativePresentationFault::SurfaceLost)
        .with_recovery_bindings(2)
        .recover_and_resume();
    assert_world(schedule);

    let report = UiNativeLifecycleProtocolWorld::run(schedule);
    assert_eq!(report.surface_generation(), 2);
    assert_eq!(report.device_generation(), 1);
    assert_eq!(report.reconstructed_bindings(), 2);
    assert_eq!(report.peak_census().reconstruction_requirements, 2);
    assert_eq!(report.terminal_census().reconstruction_requirements, 0);
}

#[test]
fn zero_minimize_resize_and_dpi_use_the_production_lifecycle_orchestrator() {
    for transition in [
        UiNativeProtocolSurfaceTransition::ZeroSized,
        UiNativeProtocolSurfaceTransition::Minimized,
        UiNativeProtocolSurfaceTransition::Resize,
        UiNativeProtocolSurfaceTransition::Dpi,
    ] {
        assert_world(UiNativeLifecycleProtocolSchedule::ordinary().surface_transition(transition));
    }
}
