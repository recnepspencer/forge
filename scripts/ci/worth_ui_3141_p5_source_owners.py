from __future__ import annotations


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
