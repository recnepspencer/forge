from __future__ import annotations


HOST = "workspaces/worth-ui/crates/worth-ui-host-native/src/native"
RUNTIME = "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform"
ARTIFACT_MUTATION = (
    "workspaces/worth-ui/crates/worth-ui-certification/tests/"
    "milestone_3141_phase1_ledger/result_artifact_mutation.rs"
)


def control(
    control_type, package, test_name, source, target=("lib", "lib"), features=()
):
    return control_type(package, target, test_name, source, features)


def boundary_control(control_type):
    return control(
        control_type,
        "worth-ui-certification",
        "milestone_3141_phase1_ledger::result_artifact::mutation_tests::phase_two_boundary_observation_rejects_each_causal_mutation",
        ARTIFACT_MUTATION,
        ("test", "topology_contracts"),
    )


def application_proofs(control_type, proof_factory):
    return {
        "P2-APPLICATION-01": proof_factory(
            f"{RUNTIME}/application_driver.rs::run",
            control(
                control_type,
                "worth-ui-certification",
                "milestone_3141_phase1_topology::compile_contract_artifact::product_native_driver_substitution_is_compiler_rejected",
                "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/compile_contract_artifact.rs",
                ("test", "topology_contracts"),
            ),
        ),
        "P2-EVENT-LOOP-01": proof_factory(
            f"{HOST}/event_loop.rs::transition_callback_thread",
            control(
                control_type,
                "worth-ui-host-native",
                "native::event_loop::tests::callback_thread_transition_rejects_off_thread_run",
                f"{HOST}/event_loop/tests.rs",
            ),
        ),
        "P2-READINESS-01": proof_factory(
            f"{HOST}/readiness.rs::commit_latest",
            control(
                control_type,
                "worth-ui-host-native",
                "native::readiness::tests::committed_readiness_requests_exactly_one_redraw_and_preserves_the_latest_generation",
                f"{HOST}/readiness.rs",
            ),
        ),
    }


def graphics_proofs(control_type, proof_factory):
    mutation = boundary_control(control_type)
    return {
        "P2-GRAPHICS-01": proof_factory(
            f"{HOST}/graphics/adapter_selection.rs::select_eligible_adapter_index",
            control(
                control_type,
                "worth-ui-host-native",
                "native::graphics::tests::adapter_selection_returns_the_exact_qualified_candidate_and_rejects_substitutes",
                f"{HOST}/graphics.rs",
            ),
        ),
        "P2-PIXELS-01": proof_factory(
            "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/native_platform/windows.rs::capture_exposed_client_area",
            control(
                control_type,
                "worth-ui-platform-pulse",
                "native_platform::windows::independent_window_capture_rejects_monitor_pixel_substitution",
                "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/native_platform/windows.rs",
                ("test", "executable_world"),
                ("executable-world",),
            ),
        ),
        "P2-PRESENT-01": proof_factory(
            f"{HOST}/presentation.rs::present_initial",
            control(
                control_type,
                "worth-ui-host-native",
                "native::presentation::raster::tests::geometry_and_color_are_derived_from_the_admitted_command",
                f"{HOST}/presentation/raster.rs",
            ),
        ),
        "P2-WINDOW-01": proof_factory(
            f"{HOST}/graphics.rs::basis_changed",
            control(
                control_type,
                "worth-ui-host-native",
                "native::graphics::tests::window_basis_classifier_rearms_only_for_new_scale_or_nonzero_extent",
                f"{HOST}/graphics.rs",
            ),
        ),
    }


def lifecycle_proofs(control_type, proof_factory):
    return {
        "P2-CLOSE-01": proof_factory(
            f"{HOST}/event_loop.rs::terminal_cleanup_complete",
            control(
                control_type,
                "worth-ui-host-native",
                "native::event_loop::tests::held_resource_with_clean_client_cannot_report_a_clean_stop",
                f"{HOST}/event_loop/tests.rs",
            ),
        ),
        "P2-PORTS-01": proof_factory(
            f"{HOST}/mechanics_adapter.rs::settle_presentation_failure",
            control(
                control_type,
                "worth-ui-host-native",
                "native::mechanics_adapter::tests::external_port_orchestration_and_effect_postures_are_exact",
                f"{HOST}/mechanics_adapter.rs",
            ),
            f"{HOST}/event_loop/window_port.rs",
            f"{HOST}/graphics/port.rs",
            f"{HOST}/presentation/port.rs",
            f"{HOST}/presentation/readback_port.rs",
            f"{HOST}/event_loop.rs",
            f"{HOST}/presentation.rs",
        ),
        "P2-WORLD-01": proof_factory(
            "workspaces/worth-ui/apps/platform-pulse/src/main.rs::run_native_phase2_world",
            boundary_control(control_type),
            "workspaces/worth-ui/apps/platform-pulse/src/native_seed_application.rs",
        ),
    }


def build_p2_proofs(control_type, proof_factory):
    result = application_proofs(control_type, proof_factory)
    result.update(graphics_proofs(control_type, proof_factory))
    result.update(lifecycle_proofs(control_type, proof_factory))
    return result
