from __future__ import annotations

from typing import Any

from worth_ui_3141_p5_raster_sources import (
    COLOR_RASTER_SOURCES,
    GLYPH_RASTER_SOURCES,
)


CERT_ROOT = "workspaces/worth-ui/crates/worth-ui-certification/tests"
LEDGER = f"{CERT_ROOT}/milestone_3141_phase1_ledger"
NATIVE = "workspaces/worth-ui/crates/worth-ui-host-native/src/native"
PHYSICAL = f"{NATIVE}/physical_work_signal"
ATLAS = f"{NATIVE}/text_atlas"
TEXT = "workspaces/worth-ui/crates/worth-ui-text/src"
TEXT_RASTER = f"{TEXT}/raster"
PLATFORM_PULSE = "workspaces/worth-ui/apps/platform-pulse"
RUNTIME = "workspaces/worth-ui/crates/worth-ui-runtime/src"
QUERY_BINDING = "workspaces/worth-ui/crates/worth-ui-query-binding/src/presentation_async"

P5_CASE_AUTHORITY_SOURCES = (
    "scripts/ci/worth_ui_3141_case_contracts.py",
    "scripts/ci/worth_ui_3141_p5_case_contracts.py",
    f"{LEDGER}/phase_five_case_contract.rs",
    f"{LEDGER}/result_artifact.rs",
)

PHYSICAL_SIGNAL_SOURCES = (
    f"{PHYSICAL}/mod.rs",
    f"{PHYSICAL}/completion_reconciliation.rs",
    f"{PHYSICAL}/construction.rs",
    f"{PHYSICAL}/counters.rs",
    f"{PHYSICAL}/declarations/mod.rs",
    f"{PHYSICAL}/declarations/aspects.rs",
    f"{PHYSICAL}/declarations/resources.rs",
    f"{PHYSICAL}/identity.rs",
    f"{PHYSICAL}/lifecycle_observation.rs",
    f"{PHYSICAL}/locality.rs",
    f"{PHYSICAL}/observation.rs",
    f"{PHYSICAL}/readiness_handoff.rs",
    f"{PHYSICAL}/routing/mod.rs",
    f"{PHYSICAL}/routing/progression.rs",
    f"{PHYSICAL}/routing/request.rs",
    f"{PHYSICAL}/routing/external_observation.rs",
    f"{PHYSICAL}/shutdown.rs",
    f"{PHYSICAL}/temporal_progression.rs",
    f"{PHYSICAL}/transition_observation.rs",
    f"{PHYSICAL}/wake_delivery.rs",
    f"{PHYSICAL}/worker.rs",
    f"{PHYSICAL}/worker_graph.rs",
    f"{PHYSICAL}/tests.rs",
    f"{PHYSICAL}/tests/request_locality.rs",
)

ATLAS_TRANSACTION_SOURCES = PHYSICAL_SIGNAL_SOURCES + P5_CASE_AUTHORITY_SOURCES + (
    f"{NATIVE}/host_state.rs",
    f"{NATIVE}/host_state/text_atlas_commit.rs",
    f"{NATIVE}/host_state/text_atlas_lifecycle.rs",
    f"{NATIVE}/mechanics_adapter.rs",
    f"{NATIVE}/readiness.rs",
    f"{NATIVE}/resource_census.rs",
    f"{NATIVE}/resource_ownership.rs",
    f"{NATIVE}/resource_registry.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_admission.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_gate_d_evidence.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_dx12_upload_port.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_retry_correlation_tests.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_rasterization.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_settlement.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_transaction.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_upload.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_physical_ownership.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_upload_tests.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_upload_sink.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_signal_failure_tests.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_tests.rs",
    f"{NATIVE}/mechanics_adapter/presentation.rs",
    f"{NATIVE}/mechanics_adapter/presentation/text_atlas.rs",
    f"{NATIVE}/mechanics_adapter/presentation/text_atlas_tests.rs",
    f"{NATIVE}/graphics/adapter_selection.rs",
    f"{ATLAS}/mod.rs",
    f"{ATLAS}/admission.rs",
    f"{ATLAS}/alpha.rs",
    f"{ATLAS}/boundary_tests.rs",
    f"{ATLAS}/candidate_store.rs",
    f"{ATLAS}/capacity.rs",
    f"{ATLAS}/census.rs",
    f"{ATLAS}/cleanup.rs",
    f"{ATLAS}/color.rs",
    f"{ATLAS}/content_extent_tests.rs",
    f"{ATLAS}/demand.rs",
    f"{ATLAS}/demand_admission.rs",
    f"{ATLAS}/entry.rs",
    f"{ATLAS}/eviction.rs",
    f"{ATLAS}/eviction_tests.rs",
    f"{ATLAS}/gate_d_model_evidence.rs",
    f"{ATLAS}/in_flight.rs",
    f"{ATLAS}/key.rs",
    f"{ATLAS}/model_key.rs",
    f"{ATLAS}/model_oracle.rs",
    f"{ATLAS}/model_placement.rs",
    f"{ATLAS}/model_records.rs",
    f"{ATLAS}/ownership.rs",
    f"{ATLAS}/ownership_tests.rs",
    f"{ATLAS}/pinning.rs",
    f"{ATLAS}/placement.rs",
    f"{ATLAS}/placement_model_tests.rs",
    f"{ATLAS}/pinning_capacity_tests.rs",
    f"{ATLAS}/planning.rs",
    f"{ATLAS}/recovery.rs",
    f"{ATLAS}/recovery_identity_tests.rs",
    f"{ATLAS}/raster_upload.rs",
    f"{ATLAS}/settlement.rs",
    f"{ATLAS}/settling.rs",
    f"{ATLAS}/test_device_tests.rs",
    f"{ATLAS}/transaction.rs",
    f"{ATLAS}/transaction_plan_snapshot.rs",
    f"{ATLAS}/upload.rs",
    f"{ATLAS}/upload/correlation.rs",
    f"{ATLAS}/upload_batch.rs",
    f"{ATLAS}/upload_staging.rs",
)

RUNTIME_PIN_SOURCES = (
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator/semantic_text_raster.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator/text_pins.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator/text_pins_tests.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/preparation.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/preparation/demand_join.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/preparation/mounted_work.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/rasterization.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/recovery.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/transaction.rs",
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/qualified_text/raster_key.rs",
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/qualified_text/raster_batch_view.rs",
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/qualified_text/raster_transaction.rs",
)


def unique_sources(*sources: str) -> tuple[str, ...]:
    return tuple(dict.fromkeys(sources))


HOST_OWNER_PRODUCTION_SOURCES = (
    f"{QUERY_BINDING}/host_owner.rs",
    f"{QUERY_BINDING}/host_owner/admission.rs",
    f"{QUERY_BINDING}/host_owner/admission_recovery.rs",
    f"{QUERY_BINDING}/host_owner/cancellation.rs",
    f"{QUERY_BINDING}/host_owner/completion_semantic_changes.rs",
    f"{QUERY_BINDING}/host_owner/correspondence.rs",
    f"{QUERY_BINDING}/host_owner/installation.rs",
    f"{QUERY_BINDING}/host_owner/pending_progress.rs",
    f"{QUERY_BINDING}/host_owner/receipts.rs",
    f"{QUERY_BINDING}/host_owner/rejection.rs",
    f"{QUERY_BINDING}/host_owner/settlement.rs",
    f"{QUERY_BINDING}/host_owner/stops.rs",
    f"{QUERY_BINDING}/host_owner/superseded_settlement.rs",
    f"{QUERY_BINDING}/host_owner/superseded_observation.rs",
    f"{QUERY_BINDING}/host_owner/terminal_close.rs",
    f"{QUERY_BINDING}/host_owner/transition_trace.rs",
    f"{QUERY_BINDING}/host_owner/unresolved.rs",
)

PRESENTATION_ASYNC_PRODUCTION_SOURCES = HOST_OWNER_PRODUCTION_SOURCES + (
    "workspaces/worth-ui/crates/worth-ui-query-binding/src/presentation_async.rs",
    f"{QUERY_BINDING}/declaration.rs",
    f"{QUERY_BINDING}/observation.rs",
    f"{QUERY_BINDING}/request_basis.rs",
    f"{QUERY_BINDING}/request_basis/identity_parts.rs",
    f"{QUERY_BINDING}/request_basis/raster_key_set.rs",
    f"{QUERY_BINDING}/retained_posture.rs",
    f"{QUERY_BINDING}/runtime_bridge.rs",
    f"{QUERY_BINDING}/runtime_bridge/completion_access.rs",
    f"{QUERY_BINDING}/runtime_bridge/completion_progress.rs",
    f"{QUERY_BINDING}/runtime_bridge/schema.rs",
    f"{QUERY_BINDING}/semantic_invalidation.rs",
    f"{QUERY_BINDING}/semantic_invalidation/bridge_registrations.rs",
    f"{QUERY_BINDING}/semantic_invalidation/compute.rs",
    f"{QUERY_BINDING}/semantic_invalidation/graph_participation.rs",
    f"{QUERY_BINDING}/semantic_invalidation/installed_operation.rs",
    f"{QUERY_BINDING}/semantic_invalidation/installed_operation/contracts.rs",
    f"{QUERY_BINDING}/semantic_invalidation/installed_operation/executor.rs",
    f"{QUERY_BINDING}/semantic_invalidation/installed_operation/operation.rs",
    f"{QUERY_BINDING}/semantic_invalidation/instance.rs",
    f"{QUERY_BINDING}/semantic_registry.rs",
    f"{QUERY_BINDING}/semantic_registry/admission_index.rs",
    f"{QUERY_BINDING}/semantic_registry/execution.rs",
    f"{QUERY_BINDING}/semantic_registry/partition_scope.rs",
    f"{QUERY_BINDING}/semantic_registry/partitions.rs",
    f"{QUERY_BINDING}/semantic_registry/partitions/evidence_digest.rs",
    f"{QUERY_BINDING}/semantic_registry/partitions/subscriber_identity.rs",
    f"{QUERY_BINDING}/semantic_registry/subscriber_index.rs",
    f"{QUERY_BINDING}/semantic_transition.rs",
    f"{QUERY_BINDING}/terminal_projection.rs",
)

QUERY_ASYNC_PRESENTATION_SOURCES = (
    "workspaces/worth-query/crates/worth-query/src/application/declaration/async_resource/request_identity.rs",
    "workspaces/worth-query/crates/worth-query/src/facade/exports_application.rs",
    "workspaces/worth-query/crates/worth-query/src/runtime/async_result_identity.rs",
    "workspaces/worth-query/crates/worth-query/src/runtime/async_result_projection.rs",
    "workspaces/worth-query/crates/worth-query/src/runtime/async_result_state.rs",
    "workspaces/worth-query/crates/worth-query/src/runtime/async_source_binding.rs",
    "workspaces/worth-query/crates/worth-query/src/runtime/async_source_transition.rs",
    "workspaces/worth-query/crates/worth-query/src/runtime/async_source_transition_plan.rs",
    "workspaces/worth-query/crates/worth-query/src/runtime/bridge_async_live_view_declaration.rs",
    "workspaces/worth-query/crates/worth-query/src/runtime/owned_async_source.rs",
    "workspaces/worth-query/crates/worth-query/src/runtime/owned_async_supersession.rs",
    "workspaces/worth-query/crates/worth-query/src/runtime/owned_conditional_instance.rs",
    "workspaces/worth-query/crates/worth-query/src/runtime/workspace/owned_async_source.rs",
    "workspaces/worth-query/crates/worth-query/src/runtime/workspace/owned_conditional_instance.rs",
)

RUNTIME_BRIDGE_ASYNC_COMPLETION_SOURCES = (
    "crates/worth-runtime-bridge/src/facade/exports_core.rs",
    "crates/worth-runtime-bridge/src/source/async_declaration/completion/mod.rs",
    "crates/worth-runtime-bridge/src/source/async_declaration/completion/admitted.rs",
    "crates/worth-runtime-bridge/src/source/async_declaration/completion/completion.rs",
    "crates/worth-runtime-bridge/src/source/async_declaration/completion/indeterminate.rs",
    "crates/worth-runtime-bridge/src/source/async_declaration/completion/rejection.rs",
)

DPI_REPLACEMENT_SOURCES = (
    f"{TEXT_RASTER}/demand.rs",
    f"{TEXT_RASTER}/demand/derivation.rs",
    f"{TEXT_RASTER}/demand_candidate.rs",
    f"{TEXT_RASTER}/demand_identity.rs",
    f"{TEXT_RASTER}/demand_identity_tests.rs",
    f"{TEXT}/phase5_ledger_evidence.rs",
)

RUNTIME_PAINT_SPAN_SOURCES = (
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/projection/frame_storage/mechanic_source.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/projection/frame_storage/mechanic_source_tests.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/projection/frame_storage/semantic_mechanics/paint_only.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/preparation.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/preparation/demand_join.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/preparation/mounted_work.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/preparation_tests.rs",
    f"{TEXT_RASTER}/demand.rs",
    f"{TEXT_RASTER}/demand/derivation.rs",
    f"{TEXT_RASTER}/demand_candidate.rs",
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/qualified_text/glyph_run_view.rs",
)

PHASE_F_SUPERSESSION_SOURCES = (
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_frame/presentation.rs",
    "workspaces/worth-ui/crates/worth-ui/src/facade/app.rs",
    "workspaces/worth-ui/crates/worth-ui/tests/ui/facade/app_journey_pass/mounted_frame_uses_app_surface.rs",
    f"{NATIVE}/event_loop.rs",
    f"{NATIVE}/event_loop/callback_thread.rs",
    f"{NATIVE}/event_loop/close_request.rs",
    f"{NATIVE}/event_loop/readiness_progress.rs",
    f"{NATIVE}/event_loop/contract.rs",
    f"{NATIVE}/event_loop/physical_progression.rs",
    f"{NATIVE}/event_loop/window_port.rs",
    f"{NATIVE}/host_state/presentation_lifecycle.rs",
    f"{NATIVE}/host_state/presentation_lifecycle/owner_poll.rs",
    f"{NATIVE}/host_state/presentation_lifecycle/ready_token.rs",
    f"{NATIVE}/host_state/presentation_lifecycle/settlement.rs",
    f"{NATIVE}/host_state/qualification.rs",
    f"{NATIVE}/mechanics_adapter/presentation/pending_completion.rs",
    f"{NATIVE}/presentation/pending_settlement.rs",
    f"{NATIVE}/presentation/port.rs",
    f"{NATIVE}/presentation/qualified_external_obligation.rs",
    f"{NATIVE}/presentation/transaction_state.rs",
    f"{NATIVE}/physical_work_signal/completion_reconciliation.rs",
    f"{NATIVE}/physical_work_signal/transition_observation.rs",
    f"{NATIVE}/physical_work_signal/worker.rs",
    f"{RUNTIME}/facade/entry/mounted_frame_execution.rs",
    f"{RUNTIME}/runtime/presentation_state.rs",
    f"{RUNTIME}/runtime/presentation_state_tests.rs",
    f"{RUNTIME}/facade/entry/native_application_shell.rs",
    f"{RUNTIME}/facade/entry/active_framework_turn/mounted_projection.rs",
    f"{RUNTIME}/facade/mounted.rs",
    f"{RUNTIME}/mounting/presentation/coordinator.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/duplicate_observation.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/pending_completion.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/pending_completion/presented.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/presentation_attempt.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/presentation_outcome.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/presented_semantic_settlement.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/settlement.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/surface_uncertainty.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/presented.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/superseding_admission.rs",
    f"{RUNTIME}/mounting/presentation/outcome.rs",
    f"{RUNTIME}/mounting/presentation/state.rs",
    f"{RUNTIME}/mounting/publication.rs",
    f"{RUNTIME}/mounting/retention/authority.rs",
    f"{RUNTIME}/mounting/retention/budget.rs",
    f"{RUNTIME}/mounting/retention/coordinator.rs",
    f"{RUNTIME}/mounting/retention/reservation.rs",
    f"{RUNTIME}/mounting/retention/successor_admission.rs",
    f"{RUNTIME}/mounting/session_state/publication.rs",
    f"{RUNTIME}/native_platform/application_driver.rs",
    f"{RUNTIME}/native_platform/application_driver/program_progress.rs",
    f"{RUNTIME}/native_platform/application_driver/program_progress/physical_progress.rs",
    f"{RUNTIME}/native_platform/application_driver/program_progress/physical_progress/pending_completion.rs",
    f"{RUNTIME}/native_platform/application_driver/program_progress/physical_progress/recovery_progress.rs",
    f"{RUNTIME}/native_platform/application_driver/program_progress/physical_progress/settlement_progress.rs",
    f"{RUNTIME}/native_platform/application_driver/program_progress/superseding_pair.rs",
    f"{RUNTIME}/native_platform/text_presentation/async_correspondence.rs",
)

PIXEL_WORLD_SOURCES = unique_sources(
    f"{PLATFORM_PULSE}/Cargo.toml",
    f"{PLATFORM_PULSE}/src/main.rs",
    f"{PLATFORM_PULSE}/src/native_phase_f_application.rs",
    f"{PLATFORM_PULSE}/src/native_phase_f_evidence.rs",
    f"{PLATFORM_PULSE}/src/native_phase_f_world.rs",
    f"{PLATFORM_PULSE}/src/native_phase_f_world_evidence.rs",
    f"{PLATFORM_PULSE}/src/native_phase_f_cancellation_world.rs",
    f"{PLATFORM_PULSE}/src/query_source/installation.rs",
    f"{PLATFORM_PULSE}/tests/executable_world/courtroom/native_phase_f.rs",
    f"{PLATFORM_PULSE}/tests/executable_world/courtroom/native_phase_f/authored_pixel_contract.rs",
    f"{PLATFORM_PULSE}/tests/executable_world/courtroom/native_phase_f/pixels.rs",
    f"{PLATFORM_PULSE}/tests/executable_world/courtroom/native_phase_f/lineage.rs",
    f"{PLATFORM_PULSE}/tests/executable_world/courtroom/native_phase_f/retained_paint.rs",
    f"{PLATFORM_PULSE}/tests/executable_world/courtroom/native_phase_f/physical_trace.rs",
    f"{PLATFORM_PULSE}/tests/executable_world/native_platform/contract.rs",
    f"{PLATFORM_PULSE}/tests/executable_world/native_platform/windows.rs",
    f"{PLATFORM_PULSE}/tests/executable_world/native_platform/windows/observation_readiness.rs",
    f"{PLATFORM_PULSE}/tests/executable_world/product_process/launch.rs",
    f"{PLATFORM_PULSE}/tests/executable_world/product_process/output_capture.rs",
    f"{NATIVE}/presentation/port/transaction.rs",
    f"{NATIVE}/presentation/pending_wgpu_readback.rs",
    f"{NATIVE}/presentation/qualified_external_obligation.rs",
    f"{NATIVE}/presentation.rs",
    f"{NATIVE}/presentation/glyph_observation.rs",
    f"{NATIVE}/presentation/pending_settlement.rs",
    f"{NATIVE}/presentation/retained_draw_list.rs",
    f"{NATIVE}/mechanics_adapter/presentation.rs",
    f"{NATIVE}/mechanics_adapter/presentation/retained_frame.rs",
    f"{NATIVE}/observation.rs",
    f"{NATIVE}/mod.rs",
    f"{NATIVE}/host_state.rs",
    f"{NATIVE}/event_loop/finish.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/lib.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/qualification.rs",
    f"{NATIVE}/host_state/qualification.rs",
    f"{NATIVE}/host_state/presentation_lifecycle.rs",
    f"{NATIVE}/event_loop/physical_progression.rs",
    f"{NATIVE}/event_loop/contract/client_shutdown.rs",
    f"{NATIVE}/event_loop/contract/client_shutdown/text_presentation_work.rs",
    f"{NATIVE}/resource_census.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/pending_completion.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/presentation_attempt.rs",
    f"{RUNTIME}/facade/entry/native_application_program.rs",
    f"{RUNTIME}/native_platform/application_driver/program_progress.rs",
    f"{RUNTIME}/facade/entry/native_application_shell.rs",
    f"{RUNTIME}/native_platform/application_driver/program_progress/physical_progress.rs",
    f"{RUNTIME}/native_platform/application_driver/shutdown_observation.rs",
    f"{RUNTIME}/native_platform/outcome.rs",
    f"{RUNTIME}/native_platform/mod.rs",
    f"{RUNTIME}/native_platform/text_presentation/work_observation.rs",
    f"{RUNTIME}/native_platform/text_presentation/work_observation/transcript.rs",
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/qualified_text/glyph_run_view.rs",
    "workspaces/worth-ui/crates/worth-ui-native-platform/src/lib.rs",
    f"{QUERY_BINDING}/host_owner/settlement.rs",
    f"{QUERY_BINDING}/host_owner/unresolved.rs",
    *PHASE_F_SUPERSESSION_SOURCES,
)

RECONSTRUCTION_SOURCES = PIXEL_WORLD_SOURCES + (
    f"{PLATFORM_PULSE}/src/native_phase_f_reconstruction_world.rs",
    f"{PLATFORM_PULSE}/src/native_phase_f_reconstruction_world/exact_reconstruction.rs",
    f"{PLATFORM_PULSE}/tests/executable_world/courtroom/native_phase_f_reconstruction.rs",
    f"{NATIVE}/derived_state_reconstruction.rs",
    f"{NATIVE}/host_state/derived_state_loss.rs",
    f"{NATIVE}/presentation/reconstruction.rs",
    f"{NATIVE}/presentation/retained_draw_list/reconstruction_tests.rs",
    f"{RUNTIME}/facade/entry/native_application_shell/presentation_recovery.rs",
    f"{RUNTIME}/mounting/identity_state/layout_reconstruction.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/raster_cache_reconstruction.rs",
    f"{RUNTIME}/native_platform/application_driver/program_reconstruction.rs",
)

LOCALITY_MATRIX_SOURCES = (
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/application.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/case.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/ci_join.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/dependency_model.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/execution.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/hostile_cost_model.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/hostile_cost_model/mutant_execution.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/hostile_cost_model/performed_basis.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/oracle.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/oracle/evidence_row.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/oracle/semantic_frontier.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/oracle/semantic_frontier/fixture_identity.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/oracle/semantic_frontier/identity_model.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/oracle/semantic_frontier/mechanic_evidence.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/presentation_cost_model.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/retained_order_reference.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/process_execution.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/shard_orchestration.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/scenario/phase5_locality_matrix/timings.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/src/bin/phase5_locality_matrix.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/phase5_locality_closure.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/phase5_locality_hostile_control.rs",
    "workspaces/worth-ui/crates/worth-ui-certification/tests/suites/phase5_closure.rs",
    f"{LEDGER}/result_artifact_cost.rs",
    f"{LEDGER}/result_artifact_phase_five_cost_tests.rs",
    f"{RUNTIME}/mounting/presentation/coordinator/semantic_text_raster.rs",
    f"{RUNTIME}/native_platform/application_driver/shutdown_observation.rs",
    f"{RUNTIME}/native_platform/application_driver.rs",
    f"{RUNTIME}/native_platform/mod.rs",
    f"{RUNTIME}/facade/entry/native_application_shell/component_presence.rs",
    f"{RUNTIME}/facade/entry/native_application_shell/launch.rs",
    f"{RUNTIME}/facade/entry/native_application_shell/query_close.rs",
    f"{RUNTIME}/facade/entry/native_application_shell/shutdown.rs",
    f"{RUNTIME}/mounting/presentation/work_producer_tests/producer_slope.rs",
    f"{NATIVE}/event_loop/contract/client_shutdown.rs",
    f"{NATIVE}/event_loop/contract/client_shutdown/mounted_identity.rs",
    f"{NATIVE}/event_loop/contract.rs",
    f"{NATIVE}/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/lib.rs",
    "workspaces/worth-ui/crates/worth-ui-native-platform/src/lib.rs",
    f"{QUERY_BINDING}/host_owner.rs",
    f"{QUERY_BINDING}/host_owner/settlement.rs",
    f"{QUERY_BINDING}/semantic_registry.rs",
    f"{QUERY_BINDING}/semantic_registry/partitions.rs",
    f"{QUERY_BINDING}/semantic_registry/partitions/evidence_digest.rs",
    f"{QUERY_BINDING}/semantic_registry/partitions/subscriber_identity.rs",
    f"{QUERY_BINDING}/semantic_registry/partition_scope.rs",
    f"{QUERY_BINDING}/semantic_registry/subscriber_index.rs",
    f"{PHYSICAL}/identity.rs",
    f"{PHYSICAL}/transition_observation.rs",
    ".github/workflows/ci.yml",
)

PINNING_PRODUCT_SOURCES = ATLAS_TRANSACTION_SOURCES + RUNTIME_PIN_SOURCES + (
    "scripts/ci/test_worth_ui_phase5_portfolio_dependency.py",
    "scripts/ci/worth_ui_3141_supporting_world.py",
    "scripts/ci/worth_ui_3141_p5_contracts.py",
    "scripts/ci/worth_ui_ledger_dependency.py",
    "scripts/ci/worth_ui_ledger_hostile_control_evidence.py",
    "scripts/ci/worth_ui_ledger_observation.py",
    "scripts/ci/worth_ui_ledger_phase_five_portfolio.py",
    "scripts/ci/worth_ui_ledger_portfolio_row.py",
    "scripts/ci/worth_ui_ledger_row_evidence.py",
    "scripts/ci/worth_ui_ledger_verifier_rebinding.py",
    f"{LEDGER}/dependency_row.rs",
    f"{LEDGER}/execution_posture.rs",
    f"{LEDGER}/result_artifact_cost.rs",
    f"{LEDGER}/result_artifact_gate_d_pin.rs",
    f"{LEDGER}/result_artifact_gate_d_pin_tests.rs",
    f"{LEDGER}/runner_artifact_authentication.rs",
    f"{LEDGER}/supporting_world_artifact.rs",
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/lib.rs",
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_frame/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_frame/surface_stop.rs",
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/operational_adapter.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/cleanup.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/contract.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/finish.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/physical_clock.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/physical_progression.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/run.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/terminal_cleanup.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/window_port.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/graphics.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/graphics/ownership.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/graphics/port.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/lib.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/prepared_host.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/certification_support/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/certification_support/presentation_mechanics.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/certification_support/semantic_text_projection.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/lib.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/host_session_authority.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/prepared_application_authority/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/prepared_application_authority/host_session_plan.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/host/adapter/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/host/adapter/operational_contract.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/host/adapter/session_authority.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/platform.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/application_driver.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/native_platform_binding.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/authorized_native_host.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/outcome.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/mounted_coordinator.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/qualified_text_test_support.rs",
    "workspaces/worth-ui/apps/platform-pulse/src/main.rs",
    "workspaces/worth-ui/apps/platform-pulse/src/native_gate_d_application.rs",
    "workspaces/worth-ui/apps/platform-pulse/Cargo.toml",
    "workspaces/worth-ui/crates/worth-ui/src/facade/certification.rs",
    "workspaces/worth-ui/crates/worth-ui/src/facade/mod.rs",
    "workspaces/worth-ui/crates/worth-ui/src/lib.rs",
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_gate_d_pin.rs",
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/product_process/kill_on_close_job.rs",
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/product_process/launch.rs",
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/product_process/mod.rs",
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/product_process/native_desktop_lease.rs",
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/product_process/output_capture.rs",
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/product_process/shutdown.rs",
    "_docs/worth-ui/milestone-3.14.1-evidence/p5-atlas-01.json",
)


def build_p5_proofs(
    proof_type: Any,
    control_type: Any,
    predecessor_artifact: str,
) -> dict[str, Any]:
    result = {
        "P5-PREDECESSOR-01": predecessor_proof(
            proof_type, control_type, predecessor_artifact
        ),
        "P5-GLYPH-RASTER-01": glyph_raster_proof(proof_type, control_type),
        "P5-COLOR-EMOJI-01": color_raster_proof(proof_type, control_type),
        "P5-ATLAS-01": atlas_proof(proof_type, control_type),
        "P5-ATLAS-PINNING-01": pinning_proof(proof_type, control_type),
        "P5-TEXT-DPI-01": dpi_replacement_proof(proof_type, control_type),
        "P5-TEXT-SPAN-PAINT-01": paint_span_proof(proof_type, control_type),
        "P5-TEXT-PIXELS-01": pixel_world_proof(proof_type, control_type),
        "P5-TEXT-RECONSTRUCTION-01": reconstruction_proof(proof_type, control_type),
        "P5-TEXT-COST-01": locality_cost_proof(proof_type, control_type),
        "P5-TEXT-ASYNC-PRESENTATION-01": async_presentation_proof(
            proof_type, control_type
        ),
        "P5-CLOSE-01": phase_five_close_proof(proof_type, control_type),
    }
    return result


def glyph_raster_proof(proof_type: Any, control_type: Any) -> Any:
    return proof_type(
        "worth-ui-text",
        ("lib", "lib"),
        "phase5_ledger_evidence::qualified_alpha_and_color_raster_cross_exact_production_authority",
        f"{TEXT_RASTER}/demand/derivation.rs::derive_glyph_raster_demand",
        f"{TEXT}/phase5_ledger_evidence.rs::qualified_alpha_and_color_raster_cross_exact_production_authority",
        GLYPH_RASTER_SOURCES + P5_CASE_AUTHORITY_SOURCES,
        control=control_type(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_topology::phase_five_raster_authority::consumer_raster_authority_mutants_are_rejected",
            f"{CERT_ROOT}/milestone_3141_phase1_topology/phase_five_raster_authority.rs",
        ),
    )


def color_raster_proof(proof_type: Any, control_type: Any) -> Any:
    return proof_type(
        "worth-ui-text",
        ("lib", "lib"),
        "phase5_ledger_evidence::every_qualified_color_source_and_rgi_sequence_crosses_production_raster",
        f"{TEXT_RASTER}/color/mod.rs::rasterize_intrinsic_color",
        f"{TEXT}/phase5_ledger_evidence.rs::every_qualified_color_source_and_rgi_sequence_crosses_production_raster",
        COLOR_RASTER_SOURCES + P5_CASE_AUTHORITY_SOURCES,
        control=control_type(
            "worth-ui-text",
            ("lib", "lib"),
            "phase5_ledger_evidence::emoji_tint_split_and_unqualified_color_sources_are_rejected",
            f"{TEXT}/phase5_ledger_evidence.rs",
        ),
    )


def atlas_proof(proof_type: Any, control_type: Any) -> Any:
    main = "native::mechanics_adapter::text_atlas::tests::gate_d_evidence::real_dx12_signal_transaction_matches_the_independent_atlas_model_and_closes_exactly"
    control = "native::mechanics_adapter::text_atlas::tests::gate_d_evidence::host_atlas_escape_and_lifecycle_faults_are_causally_rejected"
    return proof_type(
        "worth-ui-host-native",
        ("lib", "lib"),
        main,
        f"{NATIVE}/mechanics_adapter/text_atlas_transaction.rs::perform",
        f"{NATIVE}/mechanics_adapter/text_atlas_gate_d_evidence.rs::real_dx12_signal_transaction_matches_the_independent_atlas_model_and_closes_exactly",
        ATLAS_TRANSACTION_SOURCES,
        control=control_type(
            "worth-ui-host-native",
            ("lib", "lib"),
            control,
            f"{NATIVE}/mechanics_adapter/text_atlas_gate_d_evidence.rs",
        ),
    )


def pinning_proof(proof_type: Any, control_type: Any) -> Any:
    tests = "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator/text_pins_tests.rs"
    return proof_type(
        "worth-ui-platform-pulse",
        ("test", "executable_world"),
        "courtroom::native_gate_d_pin::live_layout_pins_cross_runtime_native_signal_and_release_at_last_owner",
        "workspaces/worth-ui/apps/platform-pulse/src/main.rs::run_native_gate_d_pin_world",
        "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_gate_d_pin.rs::live_layout_pins_cross_runtime_native_signal_and_release_at_last_owner",
        PINNING_PRODUCT_SOURCES,
        features=("executable-world",),
        control=control_type(
            "worth-ui-runtime",
            ("lib", "lib"),
            "mounting::presentation::coordinator::text_pins::tests::shared_pins_release_only_after_the_last_binding_is_deregistered",
            tests,
        ),
    )


def dpi_replacement_proof(proof_type: Any, control_type: Any) -> Any:
    evidence = f"{TEXT}/phase5_ledger_evidence.rs"
    return proof_type(
        "worth-ui-text",
        ("lib", "lib"),
        "phase5_ledger_evidence::pure_dpi_replaces_raster_identity_without_relayout",
        f"{TEXT_RASTER}/demand/derivation.rs::derive_glyph_raster_demand",
        f"{evidence}::pure_dpi_replaces_raster_identity_without_relayout",
        DPI_REPLACEMENT_SOURCES,
        control=control_type(
            "worth-ui-text",
            ("lib", "lib"),
            "phase5_ledger_evidence::stale_dpi_raster_is_rejected_by_complete_successor_keys",
            evidence,
        ),
    )


def paint_span_proof(proof_type: Any, control_type: Any) -> Any:
    evidence = "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/preparation_tests.rs"
    return proof_type(
        "worth-ui-runtime",
        ("lib", "lib"),
        "native_platform::text_presentation::preparation::tests::mixed_bidi_native_runs_keep_logical_paint_ownership",
        "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/preparation/demand_join.rs::prepare_demands",
        f"{evidence}::mixed_bidi_native_runs_keep_logical_paint_ownership",
        RUNTIME_PAINT_SPAN_SOURCES,
        control=control_type(
            "worth-ui-runtime",
            ("lib", "lib"),
            "native_platform::text_presentation::preparation::tests::single_color_and_logical_order_mutants_disagree_with_native_runs",
            evidence,
        ),
    )


def pixel_world_proof(proof_type: Any, control_type: Any) -> Any:
    test = "courtroom::native_phase_f::query_async_reconstruction_joins_exact_transitions_to_external_pixels_and_cleanup"
    control = "courtroom::native_phase_f::pixels::compositor_edges_and_unrelated_bright_pixels_cannot_satisfy_the_authored_text_oracle"
    oracle = f"{PLATFORM_PULSE}/tests/executable_world/courtroom/native_phase_f.rs"
    return proof_type(
        "worth-ui-platform-pulse",
        ("test", "executable_world"),
        test,
        f"{NATIVE}/presentation/port/transaction.rs::present",
        f"{oracle}::query_async_reconstruction_joins_exact_transitions_to_external_pixels_and_cleanup",
        PIXEL_WORLD_SOURCES + P5_CASE_AUTHORITY_SOURCES,
        features=("executable-world",),
        control=control_type(
            "worth-ui-platform-pulse",
            ("test", "executable_world"),
            control,
            oracle,
            features=("executable-world",),
        ),
    )


def reconstruction_proof(proof_type: Any, control_type: Any) -> Any:
    oracle = f"{PLATFORM_PULSE}/tests/executable_world/courtroom/native_phase_f_reconstruction.rs"
    return proof_type(
        "worth-ui-platform-pulse",
        ("test", "executable_world"),
        "courtroom::native_phase_f_reconstruction::every_derived_state_reconstructs_in_a_fresh_product_world",
        f"{RUNTIME}/facade/entry/native_application_shell/presentation_recovery.rs::reconstruct_current_presentation",
        f"{oracle}::every_derived_state_reconstructs_in_a_fresh_product_world",
        RECONSTRUCTION_SOURCES + P5_CASE_AUTHORITY_SOURCES,
        features=("executable-world",),
        control=control_type(
            "worth-ui-host-native",
            ("lib", "lib"),
            "native::presentation::retained_draw_list::tests::reconstruction_tests::cold_reconstruction_rebuilds_every_index_then_next_delta_remains_local",
            f"{NATIVE}/presentation/retained_draw_list/reconstruction_tests.rs",
        ),
    )


def locality_cost_proof(proof_type: Any, control_type: Any) -> Any:
    oracle = "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/phase5_locality_closure.rs"
    return proof_type(
        "worth-ui-certification",
        ("test", "phase5_closure"),
        "phase5_locality_closure::all_32_fresh_native_locality_worlds_retain_owner_issued_evidence",
        f"{RUNTIME}/mounting/presentation/coordinator/semantic_text_raster.rs::present",
        f"{oracle}::all_32_fresh_native_locality_worlds_retain_owner_issued_evidence",
        LOCALITY_MATRIX_SOURCES + P5_CASE_AUTHORITY_SOURCES,
        control=control_type(
            "worth-ui-certification",
            ("test", "phase5_closure"),
            "phase5_locality_hostile_control::exact_owner_cost_mutants_are_convicted_by_performed_small_worlds",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/phase5_locality_hostile_control.rs",
        ),
    )


def async_presentation_proof(proof_type: Any, control_type: Any) -> Any:
    oracle = f"{PLATFORM_PULSE}/tests/executable_world/courtroom/native_phase_f.rs"
    return proof_type(
        "worth-ui-platform-pulse",
        ("test", "executable_world"),
        "courtroom::native_phase_f::query_async_reconstruction_joins_exact_transitions_to_external_pixels_and_cleanup",
        f"{QUERY_BINDING}/host_owner/settlement.rs::admit_presented",
        f"{oracle}::query_async_reconstruction_joins_exact_transitions_to_external_pixels_and_cleanup",
        unique_sources(
            *PIXEL_WORLD_SOURCES,
            *PHYSICAL_SIGNAL_SOURCES,
            *PRESENTATION_ASYNC_PRODUCTION_SOURCES,
            f"{QUERY_BINDING}/host_owner_authority_tests.rs",
            f"{QUERY_BINDING}/host_owner_hostile_control_tests.rs",
            f"{QUERY_BINDING}/host_owner_tests.rs",
            f"{QUERY_BINDING}/host_owner_tests/completion.rs",
            f"{QUERY_BINDING}/host_owner_unresolved_tests.rs",
            *QUERY_ASYNC_PRESENTATION_SOURCES,
            "crates/worth-runtime-bridge/src/conditional_execution.rs",
            "crates/worth-runtime-bridge/src/conditional_execution/contract.rs",
            "crates/worth-runtime-bridge/src/conditional_execution/owned_async.rs",
            "crates/worth-runtime-bridge/src/conditional_execution/owned_async_observation.rs",
            *RUNTIME_BRIDGE_ASYNC_COMPLETION_SOURCES,
            "workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_frame/presentation_work/authority.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/fixtures/compile_contracts/Cargo.toml",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/suites/compile_contract_execution.csv",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/live_authority_cannot_be_reconstructed_or_substituted.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/live_authority_cannot_be_reconstructed_or_substituted.stderr",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/live_authority_flows_through_owner_issued_values.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/physical_signal_cannot_authorize_query_effect.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/physical_signal_cannot_authorize_query_effect.stderr",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/recovery_authority_is_not_serializable.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/recovery_authority_is_not_serializable.stderr",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/reporting_material_cannot_open_authority.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/reporting_material_cannot_open_authority.stderr",
            f"{RUNTIME}/mounting/presentation/coordinator/cancellation.rs",
            f"{RUNTIME}/mounting/presentation/coordinator/cancellation_settlement.rs",
            f"{RUNTIME}/mounting/presentation/coordinator/pending_completion.rs",
            f"{RUNTIME}/mounting/presentation/coordinator/settlement.rs",
            f"{RUNTIME}/mounting/presentation/outcome.rs",
            f"{RUNTIME}/mounting/presentation/terminal.rs",
            f"{RUNTIME}/native_platform/application_driver/physical_recovery_tracker.rs",
            f"{RUNTIME}/native_platform/application_driver/program_progress.rs",
            f"{RUNTIME}/native_platform/application_driver/program_progress/superseding_pair.rs",
            f"{NATIVE}/mechanics_adapter/presentation/pending_completion.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/mounted_presentation/query_uncertainty.rs",
            "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json",
            "scripts/ci/run_worth_ui_compile_contracts.py",
            *P5_CASE_AUTHORITY_SOURCES,
        ),
        features=("executable-world",),
        control=control_type(
            "worth-ui-query-binding",
            ("lib", "lib"),
            "presentation_async::host_owner::hostile_control_tests::typed_async_hostile_family_matches_the_independent_transition_adjudicator",
            f"{QUERY_BINDING}/host_owner_hostile_control_tests.rs",
        ),
    )


def phase_five_close_proof(proof_type: Any, control_type: Any) -> Any:
    ledger = f"{CERT_ROOT}/milestone_3141_phase1_ledger.rs"
    mutation = f"{LEDGER}/mutation_tests.rs"
    return proof_type(
        "worth-ui-certification",
        ("test", "topology_contracts"),
        "milestone_3141_phase1_ledger::phase_five_closure_requires_every_predecessor_and_phase_five_row",
        f"{ledger}::validate_phase_closure",
        f"{ledger}::phase_five_closure_requires_every_predecessor_and_phase_five_row",
        (
            ledger,
            f"{LEDGER}/phase_progression.rs",
            mutation,
            "scripts/ci/worth_ui_3141_p5_proofs.py",
            "scripts/ci/worth_ui_ledger_phase_five_portfolio.py",
        ),
        control=control_type(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_ledger::mutation_tests::phase_closure_mode_rejects_open_rows_at_or_before_its_gate",
            mutation,
        ),
    )


def predecessor_proof(
    proof_type: Any, control_type: Any, predecessor_artifact: str
) -> Any:
    validator = f"{LEDGER}/predecessor_artifact.rs"
    handoff = f"{LEDGER}/predecessor_handoff.rs"
    return proof_type(
        "worth-ui-certification",
        ("test", "topology_contracts"),
        "milestone_3141_phase1_ledger::predecessor_handoff::phase_five_predecessor_handoff_is_current",
        f"{validator}::validate",
        f"{handoff}::phase_five_predecessor_handoff_is_current",
        (
            validator,
            handoff,
            "scripts/ci/worth_ui_3141_p5_proofs.py",
            "scripts/ci/worth_ui_predecessor_handoff.py",
            "scripts/ci/verify_worth_ui_3141_ledger.py",
            "scripts/ci/worth_ui_ledger_operational_successors.py",
            "scripts/ci/worth_ui_ledger_phase_five_portfolio.py",
            "scripts/ci/worth_ui_ledger_portfolio_row.py",
            "scripts/ci/worth_ui_ledger_source_state.py",
            predecessor_artifact,
        ),
        control=control_type(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_ledger::predecessor_artifact::tests::phase_five_stale_source_or_missing_row_is_rejected",
            validator,
        ),
    )
