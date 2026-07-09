//! Single source of truth for Milestone 9.6 identity-boundary covered inventory,
//! folklore residue scanning, and session-entrypoint audits.

#[path = "identity_boundary_inventory_sources.rs"]
mod identity_boundary_inventory_sources;

pub use identity_boundary_inventory_sources::{
    source_for_format_digest_path, source_for_session_admission_path,
    source_for_string_carried_session_identity_path, source_for_string_matching_path,
};

pub const EVIDENCE_IDENTITY_COVERED_SURFACES: &[&str] = &[
    "runtime_public_support_matrix_row",
    "runtime_public_support_matrix",
    "runtime_public_api_family_contract",
    "runtime_public_api_contract",
    "runtime_public_api_transcript_evidence",
    "runtime_public_api_naming_row",
    "runtime_public_api_naming_contract",
    "runtime_state_snapshot",
    "basis_admission_evidence_row",
    "preview_basis_admission",
    "branch_basis_admission",
    "preview_intent_admission",
    "preview_intent_receipt",
    "intent_execution_provenance_chain",
    "authoritative_intent_receipt",
    "effect_intent_receipt",
    "preview_intent_receipt_inspection_basis",
    "preview_intent_receipt_inspection",
    "intent_inspection_delivery_counters",
    "intent_receipt_inspection",
    "intent_denial_inspection",
    "effect_intent_receipt_phase",
    "effect_intent_receipt_inspection",
    "feedback_phase_graph",
    "feedback_phase_graph_inspection",
    "branch_intent_receipt_inspection_basis",
    "branch_intent_receipt_inspection",
    "generic_inspection_intent_seed",
    "authoritative_mutation_intent_seed",
    "authoritative_mutation_batch_intent_seed",
    "authoritative_mutation_execution_handoff",
    "branch_intent_admission",
    "branch_intent_receipt",
    "intent_denial_evidence",
    "preview_closeout_evidence",
    "preview_promotion_denial_evidence",
    "preview_execution_evidence",
    "preview_promotion_rebinding",
    "write_receipt_inspection_artifact",
    "write_receipt_declared_aspect_operation",
    "write_receipt_mutation_metadata_entry",
    "batch_write_receipt",
    "batch_write_receipt_inspection_artifact",
    "batch_write_receipt_component",
    "batch_write_receipt_symbolic_aspect_resolution",
    "batch_write_receipt_graph_resolution",
    "retained_existing_truth_assertion_evidence",
    "graph_composition_domain_invariant_denial",
    "read_domain_invariant_denial",
    "application_support_report",
];

pub const STOP_CLASS_COVERED_CONTRACTS: &[&str] = &[
    "missing_runtime_component",
    "existing_truth_assertion_denied",
    "existing_truth_probe_denied",
    "mutation_binding_denied",
    "mutation_continuity_denied",
    "graph_composition_denied",
    "graph_composition_domain_invariant_denied",
    "mutation_naming_denied",
    "mutation_target_reference_denied",
    "read_composition_denied",
    "read_composition_domain_invariant_denied",
    "workspace",
    "program",
    "runtime_lookup_failed",
    "missing_runtime_artifact",
    "shared_read_stale_basis",
    "runtime_declaration_failed",
    "preview_operation_effect_denied",
    "session_label_collision",
    "unsupported_authority_requirement",
    "existing_truth_assertion_requires_authority_lane",
    "intent_commit_denied",
    "intent_execution_routing_failed",
    "effect_policy_denied",
    "preview_promotion_denied",
    "family_admission_denied",
];

pub const SESSION_LABEL_ORDINARY_ENTRYPOINTS: &[&str] = &[
    "runtime.preview",
    "runtime.branch",
    "runtime.try_preview",
    "runtime.try_branch",
    "workspace.preview",
    "workspace.branch",
];

pub const EXACT_ZERO_FORMAT_DIGEST_PATHS: &[&str] = &[
    "application/support/report.rs",
    "runtime/support_matrix.rs",
    "runtime/state_snapshot.rs",
    "runtime/public_api_transcript.rs",
    "runtime/public_api.rs",
    "runtime/support/profile.rs",
    "runtime/public_api_naming.rs",
    "application/declaration_bridge_routing/digest.rs",
    "application/declaration_bridge_routing/lower.rs",
    "application/declaration_bridge_routing/lower_identity.rs",
    "application/support/registry.rs",
    "continuation_pipeline/execution/execute.rs",
    "continuation_pipeline/execution/readmission.rs",
    "continuation_pipeline/execution/support.rs",
    "runtime/intent/preview.rs",
    "runtime/intent/preview_receipt_identity.rs",
    "runtime/intent/receipt.rs",
    "runtime/intent/receipt_identity.rs",
    "runtime/intent/effect_triggered.rs",
    "runtime/intent/provenance.rs",
    "runtime/intent/provenance_identity.rs",
    "runtime/intent/denial.rs",
    "runtime/intent/failure.rs",
    "runtime/intent/branch.rs",
    "runtime/branch.rs",
    "preview/scoped.rs",
    "preview/mod.rs",
    "query_context/basis.rs",
    "query_context/execution.rs",
    "query_context/metadata.rs",
    "query_context/support.rs",
    "projection_consumption/certification/audits/boundary.rs",
    "projection_consumption/certification/audits/forbidden_fallback.rs",
    "projection_consumption/certification/audits/lane_local_hostile.rs",
    "projection_consumption/certification/audits/mod.rs",
    "projection_consumption/certification/audits/proof_shape.rs",
    "projection_consumption/certification/audits/support_matrix.rs",
    "projection_consumption/certification/audits/surfaces.rs",
    "projection_consumption/certification/bundle.rs",
    "projection_consumption/certification/bundle_outputs.rs",
    "projection_consumption/certification/fixtures.rs",
    "projection_consumption/certification/grouped_projection_contract.rs",
    "projection_consumption/certification/mod.rs",
    "projection_consumption/certification/oracle/comparison_terms.rs",
    "projection_consumption/certification/oracle/mod.rs",
    "projection_consumption/certification/oracle/report.rs",
    "projection_consumption/certification/oracle/value_terms.rs",
    "projection_consumption/certification/proof_artifacts.rs",
    "projection_consumption/certification/seeded.rs",
    "projection_consumption/certification/slopes.rs",
    "projection_consumption/consumed/facts.rs",
    "projection_consumption/consumed/mod.rs",
    "projection_consumption/consumed/set.rs",
    "projection_consumption/contracts.rs",
    "projection_consumption/declaration.rs",
    "projection_consumption/declaration_authoring.rs",
    "projection_consumption/dx.rs",
    "projection_consumption/eligibility.rs",
    "projection_consumption/envelope.rs",
    "projection_consumption/extraction/grouped.rs",
    "projection_consumption/extraction/live_binding.rs",
    "projection_consumption/extraction/mod.rs",
    "projection_consumption/extraction/query_context.rs",
    "projection_consumption/extraction/retained_binding.rs",
    "projection_consumption/extraction/row_like.rs",
    "projection_consumption/extraction/write_receipt.rs",
    "projection_consumption/facts.rs",
    "projection_consumption/identity/certification.rs",
    "projection_consumption/identity/certification_closeout.rs",
    "projection_consumption/identity/certification_oracle.rs",
    "projection_consumption/identity/certification_seeded.rs",
    "projection_consumption/identity/core/contract.rs",
    "projection_consumption/identity/core/counters.rs",
    "projection_consumption/identity/core/declaration.rs",
    "projection_consumption/identity/core/eligibility.rs",
    "projection_consumption/identity/core/entries.rs",
    "projection_consumption/identity/core/envelope.rs",
    "projection_consumption/identity/core/mod.rs",
    "projection_consumption/identity/core/receipt.rs",
    "projection_consumption/identity/core/transitions.rs",
    "projection_consumption/identity/extraction.rs",
    "projection_consumption/identity/fact_set.rs",
    "projection_consumption/identity/mod.rs",
    "projection_consumption/identity/scope.rs",
    "projection_consumption/mod.rs",
    "projection_consumption/receipt.rs",
    "projection_consumption/receipt_transitions.rs",
    "projection_consumption/source/constructors.rs",
    "projection_consumption/source/mod.rs",
    "projection_consumption/source_reference_identity.rs",
    "projection_consumption/support.rs",
    "workflow/foundation.rs",
    "workflow/identity/labels.rs",
    "workflow/identity/mod.rs",
    "workflow/inspection.rs",
    "workflow/inspection/identities.rs",
    "workflow/inspection/operations.rs",
    "workflow/inspection_projection.rs",
    "workflow/lowering/counters.rs",
    "workflow/lowering/errors.rs",
    "workflow/lowering/merge.rs",
    "workflow/lowering/mod.rs",
    "workflow/lowering/mutation.rs",
    "workflow/lowering/terms.rs",
    "workflow/lowering/writeback.rs",
    "workflow/mod.rs",
    "workflow/performance.rs",
    "intent_admission/handoffs/bindings/mod.rs",
    "intent_admission/handoffs/bindings/read.rs",
    "intent_admission/handoffs/bindings/inspection.rs",
    "intent_admission/handoffs/bindings/routing.rs",
    "intent_admission/handoffs/bindings/unified_inspection.rs",
    "intent_admission/eligibility/seeds/generic_inspection.rs",
    "runtime/support/authority_artifacts.rs",
    "runtime/support/authority_artifacts/basis_admission.rs",
    "runtime/support/authority_artifacts/bridge_imports.rs",
    "runtime/support/bridge_artifact_identity.rs",
    "runtime/backend/receipts.rs",
    "runtime/backend/signal_routing_receipt.rs",
    "runtime/backend/mutation_authority.rs",
    "runtime/runtime_writes.rs",
    "runtime/runtime_helpers.rs",
    "runtime/runtime_read_intents.rs",
    "runtime/runtime_unified_inspection_intents.rs",
    "runtime/runtime_batch_writes.rs",
    "runtime/runtime_probe_routing_intents.rs",
    "runtime/runtime_inspection_materialization_intents.rs",
    "runtime/runtime_sessions.rs",
    "runtime/preview/evidence/closeout.rs",
    "runtime/preview/evidence/promotion.rs",
    "runtime/preview/evidence/execution.rs",
    "runtime/preview/mod.rs",
    "runtime/preview/basics.rs",
    "runtime/preview/mutation_ops.rs",
    "runtime/preview/session_execution.rs",
    "runtime/preview/workflow_ops.rs",
    "runtime/live_subscription.rs",
    "runtime/inspection/unified/write_receipt/digest.rs",
    "runtime/inspection/unified/write_receipt/digest_components.rs",
    "runtime/inspection/unified/write_receipt/digest_mutation_evidence.rs",
    "runtime/inspection/unified/write_receipt.rs",
    "runtime/inspection/unified/batch_write_digest.rs",
    "runtime/inspection/unified/batch_write_digest_components.rs",
    "runtime/inspection/feedback.rs",
    "runtime/inspection/feedback_identity.rs",
    "runtime/inspection/intent_identity.rs",
    "runtime/inspection/intent.rs",
    "runtime/inspection/intent_denial.rs",
    "runtime/inspection/intent_delivery_counters.rs",
    "runtime/inspection/preview/intent_receipt.rs",
    "runtime/inspection/preview/intent_receipt_identity.rs",
    "runtime/inspection/causal/request.rs",
    "runtime/inspection/causal/admission_decision.rs",
    "runtime/inspection/causal/admission_trace.rs",
    "runtime/inspection/causal/admission.rs",
    "runtime/inspection/causal/builder_bridge.rs",
    "runtime/inspection/causal/receipt.rs",
    "runtime/inspection/causal/receipt_helpers.rs",
    "runtime/inspection/causal/observation_identity.rs",
    "runtime/inspection/causal/identity.rs",
    "runtime/inspection/causal/materialization/mod.rs",
    "runtime/inspection/causal/materialization/policy.rs",
    "runtime/inspection/causal/materialization/bridge_denial.rs",
    "runtime/inspection/causal/materialization/performance.rs",
    "runtime/inspection/causal/materialization/receipt.rs",
    "runtime/inspection/causal/materialization/proof.rs",
    "runtime/inspection/causal/materialization/artifacts/denied.rs",
    "runtime/inspection/causal/materialization/artifacts/bridge_backed.rs",
    "runtime/inspection/causal/certification/artifacts/performance.rs",
    "runtime/mutation/graph_composition/domain_invariant_denial.rs",
    "runtime/mutation/graph_composition/denial.rs",
    "runtime/mutation/graph_composition/hooks.rs",
    "runtime/mutation/assertion.rs",
    "runtime/mutation/binding/existing_truth.rs",
    "runtime/mutation/binding/symbolic_reference.rs",
    "runtime/mutation/continuity.rs",
    "runtime/mutation/lowering.rs",
    "runtime/mutation/naming.rs",
    "runtime/mutation/probe.rs",
    "runtime/bridge_mutation_lowering.rs",
    "runtime/runtime_batch_write_bridge_refs.rs",
    "runtime/runtime_batch_write_receipt_context.rs",
    "runtime/read_composition_runtime.rs",
    "runtime/delivery.rs",
    "runtime/runtime_intents.rs",
    "runtime/runtime_write_intents.rs",
    "runtime/computed/surface.rs",
    "runtime/inspection/causal/reference_index.rs",
    "subscription/maintenance_delta.rs",
    "subscription/delivery_window.rs",
    "subscription/delivery_work_packet.rs",
    "subscription/bridge_parity/support.rs",
    "subscription/closeout.rs",
    "subscription/active_error.rs",
    "subscription/delivery_error.rs",
    "subscription/attachment_error.rs",
    "subscription/continuation_error.rs",
    "subscription/preview_isolation_error.rs",
    "subscription/basis_request.rs",
    "subscription/counters.rs",
    "subscription/active_counters.rs",
    "subscription/admission_diagnostics.rs",
    "subscription/signal_strategy.rs",
    "subscription/diagnostic/stage.rs",
    "subscription/diagnostic/trace.rs",
    "subscription/diagnostic/bundle.rs",
    "subscription/diagnostic/context.rs",
    "subscription/support/profile.rs",
    "subscription/input.rs",
    "subscription/runtime_certification/error.rs",
    "subscription/runtime_certification/bundle.rs",
    "subscription/runtime_certification/coverage/row.rs",
    "subscription/certification.rs",
    "subscription/declaration.rs",
    "subscription/bridge_parity/validation.rs",
    "subscription/bridge_parity/witness.rs",
    "subscription/attachment.rs",
    "subscription/active_lane.rs",
    "subscription/active_handle.rs",
    "subscription/equivalence.rs",
    "subscription/declaration_digest.rs",
    "subscription/active_digest.rs",
    "subscription/attachment_digest.rs",
    "subscription/patch_group.rs",
    "subscription/acknowledgement.rs",
    "subscription/fanout.rs",
    "subscription/lane_attachment_accessors.rs",
    "subscription/runtime_certification/coverage/matrix.rs",
    "subscription/runtime_certification/coverage/variations.rs",
    "subscription/runtime_certification/scope.rs",
    "subscription/future_selection.rs",
    "domain_capabilities/aftermath/mod.rs",
    "domain_capabilities/authoring/admission.rs",
    "domain_capabilities/authoring/aftermath.rs",
    "domain_capabilities/authoring/continuity.rs",
    "domain_capabilities/authoring/continuity_correspondence.rs",
    "domain_capabilities/authoring/explanation.rs",
    "domain_capabilities/authoring/invariant_capability.rs",
    "domain_capabilities/authoring/mod.rs",
    "domain_capabilities/authoring/support.rs",
    "domain_capabilities/authoring/workflow.rs",
    "domain_capabilities/authoring/workflow_inspection.rs",
    "domain_capabilities/canonical_runtime/admission.rs",
    "domain_capabilities/canonical_runtime/aftermath.rs",
    "domain_capabilities/canonical_runtime/artifacts.rs",
    "domain_capabilities/canonical_runtime/continuity.rs",
    "domain_capabilities/canonical_runtime/continuity_correspondence.rs",
    "domain_capabilities/canonical_runtime/explanation.rs",
    "domain_capabilities/canonical_runtime/invariant_capability.rs",
    "domain_capabilities/canonical_runtime/mod.rs",
    "domain_capabilities/canonical_runtime/support.rs",
    "domain_capabilities/canonical_runtime/workflow/inspection.rs",
    "domain_capabilities/canonical_runtime/workflow/lowering.rs",
    "domain_capabilities/canonical_runtime/workflow/mod.rs",
    "domain_capabilities/canonical_runtime/workflow/preview.rs",
    "domain_capabilities/canonical_runtime/workflow/preview_identity.rs",
    "domain_capabilities/canonical_runtime/workflow/semantics.rs",
    "domain_capabilities/certification/boundaries.rs",
    "domain_capabilities/certification/bundle/mod.rs",
    "domain_capabilities/certification/bundle/outputs.rs",
    "domain_capabilities/certification/certification_surface.rs",
    "domain_capabilities/certification/mod.rs",
    "domain_capabilities/certification/output_manifest.rs",
    "domain_capabilities/certification/reports/fixtures.rs",
    "domain_capabilities/certification/reports/mod.rs",
    "domain_capabilities/certification/reports/representative.rs",
    "domain_capabilities/certification/reports/scaled.rs",
    "domain_capabilities/certification/reports/slopes.rs",
    "domain_capabilities/certification/surface/mod.rs",
    "domain_capabilities/certification/transcripts.rs",
    "domain_capabilities/continuity/mod.rs",
    "domain_capabilities/denials.rs",
    "domain_capabilities/dx/checked.rs",
    "domain_capabilities/dx/common.rs",
    "domain_capabilities/dx/common/admitted_plan.rs",
    "domain_capabilities/dx/common/aftermath.rs",
    "domain_capabilities/dx/common/intent.rs",
    "domain_capabilities/dx/common/intent_admission.rs",
    "domain_capabilities/dx/common/intent_workflow.rs",
    "domain_capabilities/dx/common/lower_runtime.rs",
    "domain_capabilities/dx/common/lower_runtime_explanation_request.rs",
    "domain_capabilities/dx/common/projection_contract_request.rs",
    "domain_capabilities/dx/common/root.rs",
    "domain_capabilities/dx/common/shared.rs",
    "domain_capabilities/dx/mod.rs",
    "domain_capabilities/eligibility/mod.rs",
    "domain_capabilities/eligibility/transitions.rs",
    "domain_capabilities/explanation/mod.rs",
    "domain_capabilities/foundational_integration/identity.rs",
    "domain_capabilities/foundational_integration/mod.rs",
    "domain_capabilities/foundational_integration/profiles.rs",
    "domain_capabilities/foundational_integration/provenance.rs",
    "domain_capabilities/foundational_integration/rows.rs",
    "domain_capabilities/identity/certification.rs",
    "domain_capabilities/identity/mod.rs",
    "domain_capabilities/identity/scope.rs",
    "domain_capabilities/materialization.rs",
    "domain_capabilities/mod.rs",
    "domain_capabilities/payloads/admission.rs",
    "domain_capabilities/payloads/aftermath.rs",
    "domain_capabilities/payloads/common.rs",
    "domain_capabilities/payloads/continuity.rs",
    "domain_capabilities/payloads/continuity_correspondence.rs",
    "domain_capabilities/payloads/explanation.rs",
    "domain_capabilities/payloads/invariant_capability.rs",
    "domain_capabilities/payloads/mod.rs",
    "domain_capabilities/payloads/support.rs",
    "domain_capabilities/payloads/workflow.rs",
    "domain_capabilities/payloads/workflow_semantics.rs",
    "domain_capabilities/proof_integration/artifacts.rs",
    "domain_capabilities/proof_integration/mod.rs",
    "domain_capabilities/proof_integration/phases.rs",
    "domain_capabilities/proof_integration/proofs.rs",
    "domain_capabilities/summary/artifacts.rs",
    "domain_capabilities/summary/materializers.rs",
    "domain_capabilities/summary/mod.rs",
    "domain_capabilities/support/artifacts.rs",
    "domain_capabilities/support/bundles.rs",
    "domain_capabilities/support/mod.rs",
    "domain_capabilities/support/reports.rs",
    "domain_capabilities/targets/core.rs",
    "domain_capabilities/targets/mod.rs",
    "domain_capabilities/targets/wrappers.rs",
    "domain_capabilities/trace/artifacts.rs",
    "domain_capabilities/trace/materializers.rs",
    "domain_capabilities/trace/mod.rs",
    "domain_capabilities/workflow/mod.rs",
    "intent_admission/eligibility/seeds/mutation.rs",
    "intent_admission/handoffs/mutation.rs",
    "runtime/read_composition_hooks.rs",
    "lower_runtime_routing/protocol.rs",
    "lower_runtime_routing/adapters/runtime_backend.rs",
    "lower_runtime_routing/adapters/runtime_backend/subject_digest.rs",
    "lower_runtime_routing/eligibility/mod.rs",
    "lower_runtime_routing/plans/mod.rs",
    "lower_runtime_routing/receipts/mod.rs",
    "lower_runtime_routing/envelopes.rs",
    "lower_runtime_routing/support.rs",
    "lower_runtime_routing/certification/surface/acceptance_cardinality.rs",
    "lower_runtime_routing/certification/surface/fixtures/bridge_fixture.rs",
    "runtime/surface/graph_composition_breadth.rs",
    "runtime/surface/graph_composition_program.rs",
    "runtime/surface/graph_composition_admission_trace.rs",
    "runtime/surface/graph_composition_domain_invariant_summary.rs",
    "runtime/surface/graph_composition_lifecycle_outcomes.rs",
    "runtime/surface/graph_composition_assumption_summary.rs",
    "runtime/surface/graph_composition_lineage_summary.rs",
    "runtime/surface/graph_composition_resolution_map.rs",
    "runtime/surface/graph_composition_evidence.rs",
    "runtime/surface/naming_mutation_evidence.rs",
    "runtime/surface/continuity_mutation_evidence.rs",
    "runtime/surface/symbolic_target_reference_evidence.rs",
    "runtime/surface/symbolic_aspect_resolution_evidence.rs",
    "runtime/surface/read_domain_invariant_summary.rs",
    "runtime/surface/verified_assumption_set.rs",
    "runtime/surface/mutation_evidence/binding.rs",
    "runtime/surface/mutation_evidence/causality.rs",
    "runtime/surface/mutation_evidence/provenance.rs",
    "runtime/surface/mutation_evidence/target.rs",
    "runtime/surface/mutation_evidence/batch.rs",
    "runtime/surface/mutation_evidence/batch_digest_helpers.rs",
    "runtime/surface/read_composition.rs",
    "runtime/surface/read_domain_invariant_denial.rs",
    "runtime/surface/mutation/batch_receipt.rs",
    "runtime/surface/mutation/write_receipt/helpers.rs",
    "runtime/effect/inspection.rs",
    "runtime/effect/declaration.rs",
    "runtime/effect/follow_on.rs",
    "runtime/effect/inspection_identity.rs",
    "effect_lifecycle/execution_bridge.rs",
    "effect_lifecycle/execution_relational_scalar.rs",
    "view_shape_live/grouped_execution.rs",
    "lower_runtime_routing/inventory/crossing_types.rs",
    "lower_runtime_routing/inventory/closeout_types.rs",
    "lower_runtime_routing/inventory/gap_types.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/mod.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/evidence_reference.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/binding.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/denial.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/counters.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/explanation_envelope.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/identity.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/receipt.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/assembly/request.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/assembly/mod.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/mod.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/digest_basis.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/retained_artifact_digest/planning_checkpoint.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/retained_artifact_digest/route_history_preview.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/retained_artifact_digest/source_structural_stream.rs",
    "worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/retained_artifact_digest/writeback.rs",
];

#[allow(dead_code)]
pub const LOWER_RUNTIME_IDENTITY_SHIM_PATHS: &[&str] = &[
    "lower_runtime_routing/protocol.rs",
    "lower_runtime_routing/adapters/runtime_backend.rs",
    "lower_runtime_routing/adapters/runtime_backend/subject_digest.rs",
    "lower_runtime_routing/eligibility/mod.rs",
    "lower_runtime_routing/plans/mod.rs",
    "lower_runtime_routing/receipts/mod.rs",
    "lower_runtime_routing/envelopes.rs",
    "lower_runtime_routing/support.rs",
    "lower_runtime_routing/inventory/crossing_types.rs",
    "lower_runtime_routing/inventory/closeout_types.rs",
    "lower_runtime_routing/inventory/gap_types.rs",
    "lower_runtime_routing/certification/surface/acceptance_cardinality.rs",
    "lower_runtime_routing/certification/surface/fixtures/bridge_fixture.rs",
    "lower_runtime_routing/certification/surface/fixtures/core.rs",
    "lower_runtime_routing/certification/surface/fixtures/phase_six/mod.rs",
    "lower_runtime_routing/certification/surface/fixtures/phase_six/causal_signal.rs",
    "lower_runtime_routing/certification/surface/fixtures/phase_six/effect.rs",
    "lower_runtime_routing/certification/surface/fixtures/phase_six/effect_support.rs",
    "lower_runtime_routing/certification/surface/fixtures/phase_six/historical.rs",
    "lower_runtime_routing/certification/surface/fixtures/phase_six/intent.rs",
    "lower_runtime_routing/certification/surface/fixtures/phase_six/live_aggregate.rs",
    "lower_runtime_routing/certification/surface/fixtures/phase_six/projection.rs",
    "lower_runtime_routing/certification/surface/fixtures/phase_six/projection_bridge_runtime.rs",
    "lower_runtime_routing/certification/surface/fixtures/phase_six/read_execution.rs",
    "lower_runtime_routing/certification/surface/fixtures/phase_six/readmission.rs",
    "lower_runtime_routing/certification/surface/fixtures/phase_six/readmission_support.rs",
    "lower_runtime_routing/certification/surface/fixtures/phase_six/subscription.rs",
];

pub const EXACT_ZERO_STRING_MATCHING_PATHS: &[&str] =
    &["runtime/tests/stop_class/consumer_support/routing.rs"];

pub const EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS: &[&str] =
    &["runtime/runtime_sessions.rs", "runtime/workspace.rs"];

pub const EXACT_ZERO_STRING_CARRIED_SESSION_IDENTITY_PATHS: &[&str] = &[
    "runtime/error.rs",
    "runtime/preview/workflow_ops.rs",
    "runtime/preview/binding.rs",
    "runtime/preview/session_execution.rs",
    "runtime/preview/mutation_ops.rs",
    "runtime/surface/mutation/write_receipt/preview.rs",
    "runtime/inspection/preview/binding.rs",
    "runtime/inspection/preview/outcome.rs",
];

/// Paths that retain pre-9.6 joined-string digest folklore by explicit milestone scope.
/// See `EXCLUDED_FOLKLORE_DEFERRALS` for owner milestones referenced in closeout evidence.
#[allow(dead_code)]
pub const EXCLUDED_FOLKLORE_PATHS: &[&str] = &[
    "harness/milestone_nine_five_",
    "runtime/intent/declaration.rs",
];

/// Named deferrals for same-class folklore outside the Milestone 9.6 ordinary-path contract.
#[allow(dead_code)]
pub const EXCLUDED_FOLKLORE_DEFERRALS: &[(&str, &str)] = &[
    (
        "harness/milestone_nine_five_",
        "Milestone 9.5 harness-only fixtures; not ordinary-path production",
    ),
    (
        "runtime/intent/declaration.rs",
        "Milestone 9.8 â€” intent declaration identity lowering track",
    ),
];

const FORBIDDEN_DIGEST_FOLKLORE_PATTERNS: &[&str] = &[
    "hash_parts(",
    "causal_envelope_digest",
    "retained_mapping_digest",
    "derive_causal_envelope_identity",
    "derive_retained_mapping_identity",
    "format!(\"{digest_domain}:sha256:",
    "from_external_authority(format!",
    "canonical.push('|')",
    "digest_owned_parts(",
    ".join(\"|\")",
    "format!(\"{}|",
    "format!(\"{:?}\"",
    "format!(\"{:?}|",
    "format!(\"{:#?}\"",
    "format!(\"{:#?}|",
    "optional_identity(",
    "bridge_digest",
    "query_observation_for_reporting: &str",
    "reference_digest: &str",
    "source_identity: impl AsRef<str>",
    "performance_digest: String",
    "reference_digest().as_str()",
    "field_identity(WorthQueryEvidenceTag::new(\"artifact\")",
    "field_identity(WorthQueryEvidenceTag::new(\"causality\")",
    "field_identity(WorthQueryEvidenceTag::new(\"performance\")",
    "field_identity(WorthQueryEvidenceTag::new(\"source\")",
    "field_identity(WorthQueryEvidenceTag::new(\"write_adjacent_trigger\")",
    "reference_identity: Arc<str>",
    "identity: Arc<str>",
    "CausalResultShapeContextHandle::from_rendered(\n                inspection.",
    "CausalObservationTargetHandle::from_rendered(\n                inspection.",
    ".terminal_projection_for_reporting()",
    "evidence_identity.as_str()",
    "posture_detail_identity.as_str()",
    "WorthQueryLowerRuntimeSubjectIdentity::from_digest(",
    "WorthQueryLowerRuntimeRouteSubjectIdentity::from_digest(",
    "WorthQueryLowerRuntimeCapabilityEligibility::admitted(",
    "source_digest: String",
    "failure_for_reporting: String",
    "source_for_reporting: String",
    "equivalence_for_reporting: String",
    "labels_for_reporting: String",
    "assembly_receipt_for_reporting: String",
    "bundle_width_for_reporting: String",
    "plan_digest: String",
];

#[allow(dead_code)]
const FORBIDDEN_LOWER_RUNTIME_IDENTITY_SHIM_PATTERNS: &[&str] = &[
    "hash_parts(",
    "from_bridge_harness_label",
    "WorthQueryLowerRuntimeSubjectIdentity::from_digest(",
    "WorthQueryLowerRuntimeRouteSubjectIdentity::from_digest(",
    "WorthQueryLowerRuntimeCapabilityEligibility::admitted(",
    "pub(crate) fn from_digest(",
    "pub(crate) fn admitted(",
];

#[allow(dead_code)]
const REQUIRED_TYPED_SESSION_LABEL_SIGNATURES: &[&str] = &[
    "pub fn preview<'a>(\n        &'a mut self,\n        label: WorthQuerySessionLabel,",
    "pub fn branch<'a>(\n        &'a mut self,\n        label: WorthQuerySessionLabel,",
    "pub fn preview_with_options<'a>(\n        &'a mut self,\n        label: WorthQuerySessionLabel,",
    "pub fn branch_with_options<'a>(\n        &'a mut self,\n        label: WorthQuerySessionLabel,",
    "pub fn try_preview<'a>(\n        &'a mut self,\n        label: WorthQuerySessionLabel,",
    "pub fn try_branch<'a>(\n        &'a mut self,\n        label: WorthQuerySessionLabel,",
    "pub fn try_preview_with_options<'a>(\n        &'a mut self,\n        label: WorthQuerySessionLabel,",
    "pub fn try_branch_with_options<'a>(\n        &'a mut self,\n        label: WorthQuerySessionLabel,",
];

#[allow(dead_code)]
const FORBIDDEN_RAW_SESSION_LABEL_SIGNATURES: &[&str] = &[
    "pub fn preview<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn branch<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn preview_with_options<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn branch_with_options<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn try_preview<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn try_branch<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn try_preview_with_options<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    "pub fn try_branch_with_options<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
];

pub fn normalize_source_text(source: &str) -> String {
    source.replace("\r\n", "\n")
}

pub fn format_digest_folklore_pattern_in(source: &str) -> Option<&'static str> {
    let normalized = normalize_source_text(source);
    FORBIDDEN_DIGEST_FOLKLORE_PATTERNS
        .iter()
        .copied()
        .find(|pattern| normalized.contains(pattern))
}

pub fn scan_format_digest_residue_paths() -> Vec<&'static str> {
    let mut remaining = Vec::new();
    for &path in EXACT_ZERO_FORMAT_DIGEST_PATHS {
        let Some(source) = source_for_format_digest_path(path) else {
            remaining.push(path);
            continue;
        };
        if format_digest_folklore_pattern_in(source).is_some() {
            remaining.push(path);
        }
    }
    remaining
}

#[cfg(test)]
pub fn scan_format_digest_residue_path_patterns() -> Vec<(&'static str, &'static str)> {
    let mut remaining = Vec::new();
    for &path in EXACT_ZERO_FORMAT_DIGEST_PATHS {
        let Some(source) = source_for_format_digest_path(path) else {
            remaining.push((path, "<missing-source>"));
            continue;
        };
        if let Some(pattern) = format_digest_folklore_pattern_in(source) {
            remaining.push((path, pattern));
        }
    }
    remaining
}

#[allow(dead_code)]
pub fn scan_lower_runtime_identity_shim_paths() -> Vec<&'static str> {
    let mut remaining = Vec::new();
    for &path in LOWER_RUNTIME_IDENTITY_SHIM_PATHS {
        let Some(source) = source_for_format_digest_path(path) else {
            remaining.push(path);
            continue;
        };
        if FORBIDDEN_LOWER_RUNTIME_IDENTITY_SHIM_PATTERNS
            .iter()
            .any(|pattern| source.contains(pattern))
        {
            remaining.push(path);
        }
    }
    remaining
}

pub fn scan_string_matching_residue_paths() -> Vec<&'static str> {
    let mut remaining = Vec::new();
    for &path in EXACT_ZERO_STRING_MATCHING_PATHS {
        let Some(source) = source_for_string_matching_path(path) else {
            remaining.push(path);
            continue;
        };
        if source.contains("to_string().contains(")
            || source.contains("message.contains")
            || source.contains("error_message.contains")
        {
            remaining.push(path);
        }
    }
    remaining
}

pub fn scan_raw_session_admission_residue_paths() -> Vec<&'static str> {
    let mut remaining = Vec::new();
    for &path in EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS {
        let Some(source) = source_for_session_admission_path(path) else {
            remaining.push(path);
            continue;
        };
        let normalized = normalize_source_text(source);
        if normalized.contains("label: impl Into<String>") {
            remaining.push(path);
            continue;
        }
        if !normalized.contains("label: WorthQuerySessionLabel") {
            remaining.push(path);
        }
    }
    remaining
}

pub fn scan_string_carried_session_identity_residue_paths() -> Vec<&'static str> {
    let mut remaining = Vec::new();
    for &path in EXACT_ZERO_STRING_CARRIED_SESSION_IDENTITY_PATHS {
        let Some(source) = source_for_string_carried_session_identity_path(path) else {
            remaining.push(path);
            continue;
        };
        let normalized = normalize_source_text(source);
        let carries_string_identity = normalized.contains("label: String")
            || normalized.contains("label: &str")
            || normalized.contains("self.label.to_string()")
            || normalized.contains("label.to_string(),")
            || normalized.contains("self.label.display(),")
            || normalized.contains("format!(\"preview:{label}:{sequence}\")");
        let preserves_typed_identity = normalized.contains("label: WorthQuerySessionLabel")
            || normalized.contains("label: &WorthQuerySessionLabel")
            || normalized.contains("&self.label")
            || normalized.contains("self.label.clone()");
        if carries_string_identity || !preserves_typed_identity {
            remaining.push(path);
        }
    }
    remaining
}

#[allow(dead_code)]
pub fn ordinary_session_entrypoint_audit_violations(
    runtime_sessions: &str,
    workspace: &str,
) -> Vec<String> {
    let runtime_sessions = normalize_source_text(runtime_sessions);
    let workspace = normalize_source_text(workspace);
    let mut violations = Vec::new();

    for required in REQUIRED_TYPED_SESSION_LABEL_SIGNATURES {
        if !runtime_sessions.contains(required) && !workspace.contains(required) {
            violations.push(format!("missing typed entrypoint signature: {required}"));
        }
    }
    for forbidden in FORBIDDEN_RAW_SESSION_LABEL_SIGNATURES {
        if runtime_sessions.contains(forbidden) || workspace.contains(forbidden) {
            violations.push(format!("raw-string entrypoint survived: {forbidden}"));
        }
    }
    violations
}

#[cfg(test)]
#[path = "identity_boundary_inventory_tests.rs"]
mod tests;
