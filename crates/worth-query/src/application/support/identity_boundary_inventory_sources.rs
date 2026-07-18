pub fn source_for_format_digest_path(path: &str) -> Option<&'static str> {
    match path {
        "application/support/report.rs" => Some(include_str!("report.rs")),
        "runtime/support_matrix.rs" => Some(include_str!("../../runtime/support_matrix.rs")),
        "runtime/state_snapshot.rs" => Some(include_str!("../../runtime/state_snapshot.rs")),
        "runtime/public_api_transcript.rs" => {
            Some(include_str!("../../runtime/public_api_transcript.rs"))
        }
        "runtime/public_api.rs" => Some(include_str!("../../runtime/public_api.rs")),
        "runtime/support/profile.rs" => Some(include_str!("../../runtime/support/profile.rs")),
        "runtime/public_api_naming.rs" => Some(include_str!("../../runtime/public_api_naming.rs")),
        "application/declaration_bridge_routing/digest.rs" => {
            Some(include_str!("../declaration_bridge_routing/digest.rs"))
        }
        "application/declaration_bridge_routing/lower.rs" => {
            Some(include_str!("../declaration_bridge_routing/lower.rs"))
        }
        "application/declaration_bridge_routing/lower_identity.rs" => Some(include_str!(
            "../declaration_bridge_routing/lower_identity.rs"
        )),
        "application/support/registry.rs" => Some(include_str!("registry.rs")),
        "continuation_pipeline/execution/execute.rs" => Some(include_str!(
            "../../continuation_pipeline/execution/execute.rs"
        )),
        "continuation_pipeline/execution/readmission.rs" => Some(include_str!(
            "../../continuation_pipeline/execution/readmission.rs"
        )),
        "continuation_pipeline/execution/support.rs" => Some(include_str!(
            "../../continuation_pipeline/execution/support.rs"
        )),
        "runtime/intent/preview.rs" => Some(include_str!("../../runtime/intent/preview.rs")),
        "runtime/intent/preview_receipt_identity.rs" => Some(include_str!(
            "../../runtime/intent/preview_receipt_identity.rs"
        )),
        "runtime/intent/receipt.rs" => Some(include_str!("../../runtime/intent/receipt.rs")),
        "runtime/intent/receipt_identity.rs" => {
            Some(include_str!("../../runtime/intent/receipt_identity.rs"))
        }
        "runtime/intent/effect_triggered.rs" => {
            Some(include_str!("../../runtime/intent/effect_triggered.rs"))
        }
        "runtime/intent/provenance.rs" => Some(include_str!("../../runtime/intent/provenance.rs")),
        "runtime/intent/provenance_identity.rs" => {
            Some(include_str!("../../runtime/intent/provenance_identity.rs"))
        }
        "runtime/intent/denial.rs" => Some(include_str!("../../runtime/intent/denial.rs")),
        "runtime/intent/failure.rs" => Some(include_str!("../../runtime/intent/failure.rs")),
        "runtime/intent/branch.rs" => Some(include_str!("../../runtime/intent/branch.rs")),
        "runtime/branch.rs" => Some(include_str!("../../runtime/branch.rs")),
        "preview/scoped.rs" => Some(include_str!("../../preview/scoped.rs")),
        "preview/mod.rs" => Some(include_str!("../../preview/mod.rs")),
        "query_context/basis.rs" => Some(include_str!("../../query_context/basis.rs")),
        "query_context/execution.rs" => Some(include_str!("../../query_context/execution.rs")),
        "query_context/metadata.rs" => Some(include_str!("../../query_context/metadata.rs")),
        "query_context/support.rs" => Some(include_str!("../../query_context/support.rs")),
        "projection_consumption/certification/audits/boundary.rs" => Some(include_str!(
            "../../projection_consumption/certification/audits/boundary.rs"
        )),
        "projection_consumption/certification/audits/forbidden_fallback.rs" => Some(
            include_str!("../../projection_consumption/certification/audits/forbidden_fallback.rs"),
        ),
        "projection_consumption/certification/audits/lane_local_hostile.rs" => Some(
            include_str!("../../projection_consumption/certification/audits/lane_local_hostile.rs"),
        ),
        "projection_consumption/certification/audits/mod.rs" => Some(include_str!(
            "../../projection_consumption/certification/audits/mod.rs"
        )),
        "projection_consumption/certification/audits/proof_shape.rs" => Some(include_str!(
            "../../projection_consumption/certification/audits/proof_shape.rs"
        )),
        "projection_consumption/certification/audits/support_matrix.rs" => Some(include_str!(
            "../../projection_consumption/certification/audits/support_matrix.rs"
        )),
        "projection_consumption/certification/audits/surfaces.rs" => Some(include_str!(
            "../../projection_consumption/certification/audits/surfaces.rs"
        )),
        "projection_consumption/certification/bundle.rs" => {
            Some(include_str!("../../projection_consumption/certification/bundle.rs"))
        }
        "projection_consumption/certification/bundle_outputs.rs" => Some(include_str!(
            "../../projection_consumption/certification/bundle_outputs.rs"
        )),
        "projection_consumption/certification/fixtures.rs" => {
            Some(include_str!("../../projection_consumption/certification/fixtures.rs"))
        }
        "projection_consumption/certification/grouped_projection_contract.rs" => Some(
            include_str!("../../projection_consumption/certification/grouped_projection_contract.rs"),
        ),
        "projection_consumption/certification/mod.rs" => {
            Some(include_str!("../../projection_consumption/certification/mod.rs"))
        }
        "projection_consumption/certification/oracle/comparison_terms.rs" => Some(
            include_str!("../../projection_consumption/certification/oracle/comparison_terms.rs"),
        ),
        "projection_consumption/certification/oracle/mod.rs" => Some(include_str!(
            "../../projection_consumption/certification/oracle/mod.rs"
        )),
        "projection_consumption/certification/oracle/report.rs" => Some(include_str!(
            "../../projection_consumption/certification/oracle/report.rs"
        )),
        "projection_consumption/certification/oracle/value_terms.rs" => Some(include_str!(
            "../../projection_consumption/certification/oracle/value_terms.rs"
        )),
        "projection_consumption/certification/proof_artifacts.rs" => Some(include_str!(
            "../../projection_consumption/certification/proof_artifacts.rs"
        )),
        "projection_consumption/certification/seeded.rs" => {
            Some(include_str!("../../projection_consumption/certification/seeded.rs"))
        }
        "projection_consumption/certification/slopes.rs" => {
            Some(include_str!("../../projection_consumption/certification/slopes.rs"))
        }
        "projection_consumption/consumed/facts.rs" => {
            Some(include_str!("../../projection_consumption/consumed/facts.rs"))
        }
        "projection_consumption/consumed/mod.rs" => {
            Some(include_str!("../../projection_consumption/consumed/mod.rs"))
        }
        "projection_consumption/consumed/set.rs" => {
            Some(include_str!("../../projection_consumption/consumed/set.rs"))
        }
        "projection_consumption/contracts.rs" => {
            Some(include_str!("../../projection_consumption/contracts.rs"))
        }
        "projection_consumption/declaration.rs" => {
            Some(include_str!("../../projection_consumption/declaration.rs"))
        }
        "projection_consumption/declaration_authoring.rs" => Some(include_str!(
            "../../projection_consumption/declaration_authoring.rs"
        )),
        "projection_consumption/dx.rs" => Some(include_str!("../../projection_consumption/dx.rs")),
        "projection_consumption/eligibility.rs" => {
            Some(include_str!("../../projection_consumption/eligibility.rs"))
        }
        "projection_consumption/envelope.rs" => {
            Some(include_str!("../../projection_consumption/envelope.rs"))
        }
        "projection_consumption/extraction/grouped.rs" => Some(include_str!(
            "../../projection_consumption/extraction/grouped.rs"
        )),
        "projection_consumption/extraction/live_binding.rs" => Some(include_str!(
            "../../projection_consumption/extraction/live_binding.rs"
        )),
        "projection_consumption/extraction/mod.rs" => {
            Some(include_str!("../../projection_consumption/extraction/mod.rs"))
        }
        "projection_consumption/extraction/query_context.rs" => Some(include_str!(
            "../../projection_consumption/extraction/query_context.rs"
        )),
        "projection_consumption/extraction/retained_binding.rs" => Some(include_str!(
            "../../projection_consumption/extraction/retained_binding.rs"
        )),
        "projection_consumption/extraction/row_like.rs" => Some(include_str!(
            "../../projection_consumption/extraction/row_like.rs"
        )),
        "projection_consumption/extraction/write_receipt.rs" => Some(include_str!(
            "../../projection_consumption/extraction/write_receipt.rs"
        )),
        "projection_consumption/facts.rs" => Some(include_str!("../../projection_consumption/facts.rs")),
        "projection_consumption/identity/certification.rs" => Some(include_str!(
            "../../projection_consumption/identity/certification.rs"
        )),
        "projection_consumption/identity/certification_closeout.rs" => Some(include_str!(
            "../../projection_consumption/identity/certification_closeout.rs"
        )),
        "projection_consumption/identity/certification_oracle.rs" => Some(include_str!(
            "../../projection_consumption/identity/certification_oracle.rs"
        )),
        "projection_consumption/identity/certification_seeded.rs" => Some(include_str!(
            "../../projection_consumption/identity/certification_seeded.rs"
        )),
        "projection_consumption/identity/core/contract.rs" => Some(include_str!(
            "../../projection_consumption/identity/core/contract.rs"
        )),
        "projection_consumption/identity/core/counters.rs" => Some(include_str!(
            "../../projection_consumption/identity/core/counters.rs"
        )),
        "projection_consumption/identity/core/declaration.rs" => Some(include_str!(
            "../../projection_consumption/identity/core/declaration.rs"
        )),
        "projection_consumption/identity/core/eligibility.rs" => Some(include_str!(
            "../../projection_consumption/identity/core/eligibility.rs"
        )),
        "projection_consumption/identity/core/entries.rs" => Some(include_str!(
            "../../projection_consumption/identity/core/entries.rs"
        )),
        "projection_consumption/identity/core/envelope.rs" => Some(include_str!(
            "../../projection_consumption/identity/core/envelope.rs"
        )),
        "projection_consumption/identity/core/mod.rs" => {
            Some(include_str!("../../projection_consumption/identity/core/mod.rs"))
        }
        "projection_consumption/identity/core/receipt.rs" => Some(include_str!(
            "../../projection_consumption/identity/core/receipt.rs"
        )),
        "projection_consumption/identity/core/transitions.rs" => Some(include_str!(
            "../../projection_consumption/identity/core/transitions.rs"
        )),
        "projection_consumption/identity/extraction.rs" => Some(include_str!(
            "../../projection_consumption/identity/extraction.rs"
        )),
        "projection_consumption/identity/fact_set.rs" => Some(include_str!(
            "../../projection_consumption/identity/fact_set.rs"
        )),
        "projection_consumption/identity/mod.rs" => {
            Some(include_str!("../../projection_consumption/identity/mod.rs"))
        }
        "projection_consumption/identity/scope.rs" => {
            Some(include_str!("../../projection_consumption/identity/scope.rs"))
        }
        "projection_consumption/mod.rs" => Some(include_str!("../../projection_consumption/mod.rs")),
        "projection_consumption/receipt.rs" => {
            Some(include_str!("../../projection_consumption/receipt.rs"))
        }
        "projection_consumption/receipt_transitions.rs" => Some(include_str!(
            "../../projection_consumption/receipt_transitions.rs"
        )),
        "projection_consumption/source/constructors.rs" => Some(include_str!(
            "../../projection_consumption/source/constructors.rs"
        )),
        "projection_consumption/source/mod.rs" => {
            Some(include_str!("../../projection_consumption/source/mod.rs"))
        }
        "projection_consumption/source_reference_identity.rs" => Some(include_str!(
            "../../projection_consumption/source_reference_identity.rs"
        )),
        "projection_consumption/support.rs" => {
            Some(include_str!("../../projection_consumption/support.rs"))
        }
        "workflow/foundation.rs" => Some(include_str!("../../workflow/foundation.rs")),
        "workflow/identity/labels.rs" => Some(include_str!("../../workflow/identity/labels.rs")),
        "workflow/identity/mod.rs" => Some(include_str!("../../workflow/identity/mod.rs")),
        "workflow/inspection.rs" => Some(include_str!("../../workflow/inspection.rs")),
        "workflow/inspection/identities.rs" => {
            Some(include_str!("../../workflow/inspection/identities.rs"))
        }
        "workflow/inspection/operations.rs" => {
            Some(include_str!("../../workflow/inspection/operations.rs"))
        }
        "workflow/inspection_projection.rs" => {
            Some(include_str!("../../workflow/inspection_projection.rs"))
        }
        "workflow/lowering/counters.rs" => Some(include_str!("../../workflow/lowering/counters.rs")),
        "workflow/lowering/errors.rs" => Some(include_str!("../../workflow/lowering/errors.rs")),
        "workflow/lowering/merge.rs" => Some(include_str!("../../workflow/lowering/merge.rs")),
        "workflow/lowering/mod.rs" => Some(include_str!("../../workflow/lowering/mod.rs")),
        "workflow/lowering/mutation.rs" => Some(include_str!("../../workflow/lowering/mutation.rs")),
        "workflow/lowering/terms.rs" => Some(include_str!("../../workflow/lowering/terms.rs")),
        "workflow/lowering/writeback.rs" => Some(include_str!("../../workflow/lowering/writeback.rs")),
        "workflow/mod.rs" => Some(include_str!("../../workflow/mod.rs")),
        "workflow/performance.rs" => Some(include_str!("../../workflow/performance.rs")),
        "intent_admission/handoffs/bindings/mod.rs" => Some(include_str!(
            "../../intent_admission/handoffs/bindings/mod.rs"
        )),
        "intent_admission/handoffs/bindings/read.rs" => Some(include_str!(
            "../../intent_admission/handoffs/bindings/read.rs"
        )),
        "intent_admission/handoffs/bindings/inspection.rs" => Some(include_str!(
            "../../intent_admission/handoffs/bindings/inspection.rs"
        )),
        "intent_admission/handoffs/bindings/routing.rs" => Some(include_str!(
            "../../intent_admission/handoffs/bindings/routing.rs"
        )),
        "intent_admission/handoffs/bindings/unified_inspection.rs" => Some(include_str!(
            "../../intent_admission/handoffs/bindings/unified_inspection.rs"
        )),
        "intent_admission/eligibility/seeds/generic_inspection.rs" => Some(include_str!(
            "../../intent_admission/eligibility/seeds/generic_inspection.rs"
        )),
        "intent_admission/eligibility/seeds/mutation.rs" => Some(include_str!(
            "../../intent_admission/eligibility/seeds/mutation.rs"
        )),
        "intent_admission/handoffs/mutation.rs" => {
            Some(include_str!("../../intent_admission/handoffs/mutation.rs"))
        }
        "runtime/support/authority_artifacts.rs" => {
            Some(include_str!("../../runtime/support/authority_artifacts.rs"))
        }
        "runtime/support/authority_artifacts/basis_admission.rs" => Some(include_str!(
            "../../runtime/support/authority_artifacts/basis_admission.rs"
        )),
        "runtime/support/authority_artifacts/bridge_imports.rs" => Some(include_str!(
            "../../runtime/support/authority_artifacts/bridge_imports.rs"
        )),
        "runtime/support/bridge_artifact_identity.rs" => Some(include_str!(
            "../../runtime/support/bridge_artifact_identity.rs"
        )),
        "runtime/backend/receipts.rs" => Some(include_str!("../../runtime/backend/receipts.rs")),
        "runtime/backend/signal_routing_receipt.rs" => Some(include_str!(
            "../../runtime/backend/signal_routing_receipt.rs"
        )),
        "runtime/backend/mutation_authority.rs" => {
            Some(include_str!("../../runtime/backend/mutation_authority.rs"))
        }
        "runtime/runtime_writes.rs" => Some(include_str!("../../runtime/runtime_writes.rs")),
        "runtime/runtime_helpers.rs" => Some(include_str!("../../runtime/runtime_helpers.rs")),
        "runtime/runtime_read_intents.rs" => {
            Some(include_str!("../../runtime/runtime_read_intents.rs"))
        }
        "runtime/runtime_unified_inspection_intents.rs" => Some(include_str!(
            "../../runtime/runtime_unified_inspection_intents.rs"
        )),
        "runtime/runtime_batch_writes.rs" => {
            Some(include_str!("../../runtime/runtime_batch_writes.rs"))
        }
        "runtime/runtime_probe_routing_intents.rs" => Some(include_str!(
            "../../runtime/runtime_probe_routing_intents.rs"
        )),
        "runtime/runtime_inspection_materialization_intents.rs" => Some(include_str!(
            "../../runtime/runtime_inspection_materialization_intents.rs"
        )),
        "runtime/runtime_sessions.rs" => Some(include_str!("../../runtime/runtime_sessions.rs")),
        "runtime/preview/evidence/closeout.rs" => {
            Some(include_str!("../../runtime/preview/evidence/closeout.rs"))
        }
        "runtime/preview/evidence/promotion.rs" => {
            Some(include_str!("../../runtime/preview/evidence/promotion.rs"))
        }
        "runtime/preview/evidence/execution.rs" => {
            Some(include_str!("../../runtime/preview/evidence/execution.rs"))
        }
        "runtime/preview/mod.rs" => Some(include_str!("../../runtime/preview/mod.rs")),
        "runtime/preview/basics.rs" => Some(include_str!("../../runtime/preview/basics.rs")),
        "runtime/preview/mutation_ops.rs" => {
            Some(include_str!("../../runtime/preview/mutation_ops.rs"))
        }
        "runtime/preview/session_execution.rs" => {
            Some(include_str!("../../runtime/preview/session_execution.rs"))
        }
        "runtime/preview/workflow_ops.rs" => {
            Some(include_str!("../../runtime/preview/workflow_ops.rs"))
        }
        "runtime/live_subscription.rs" => Some(include_str!("../../runtime/live_subscription.rs")),
        "runtime/inspection/unified/write_receipt/digest.rs" => Some(include_str!(
            "../../runtime/inspection/unified/write_receipt/digest.rs"
        )),
        "runtime/inspection/unified/write_receipt/digest_components.rs" => Some(include_str!(
            "../../runtime/inspection/unified/write_receipt/digest_components.rs"
        )),
        "runtime/inspection/unified/write_receipt/digest_mutation_evidence.rs" => {
            Some(include_str!(
                "../../runtime/inspection/unified/write_receipt/digest_mutation_evidence.rs"
            ))
        }
        "runtime/inspection/unified/write_receipt.rs" => Some(include_str!(
            "../../runtime/inspection/unified/write_receipt.rs"
        )),
        "runtime/inspection/unified/batch_write_digest.rs" => Some(include_str!(
            "../../runtime/inspection/unified/batch_write_digest.rs"
        )),
        "runtime/inspection/unified/batch_write_digest_components.rs" => Some(include_str!(
            "../../runtime/inspection/unified/batch_write_digest_components.rs"
        )),
        "runtime/inspection/feedback.rs" => {
            Some(include_str!("../../runtime/inspection/feedback.rs"))
        }
        "runtime/inspection/feedback_identity.rs" => Some(include_str!(
            "../../runtime/inspection/feedback_identity.rs"
        )),
        "runtime/inspection/intent_identity.rs" => {
            Some(include_str!("../../runtime/inspection/intent_identity.rs"))
        }
        "runtime/inspection/intent.rs" => Some(include_str!("../../runtime/inspection/intent.rs")),
        "runtime/inspection/intent_denial.rs" => {
            Some(include_str!("../../runtime/inspection/intent_denial.rs"))
        }
        "runtime/inspection/intent_delivery_counters.rs" => Some(include_str!(
            "../../runtime/inspection/intent_delivery_counters.rs"
        )),
        "runtime/inspection/preview/intent_receipt.rs" => Some(include_str!(
            "../../runtime/inspection/preview/intent_receipt.rs"
        )),
        "runtime/inspection/preview/intent_receipt_identity.rs" => Some(include_str!(
            "../../runtime/inspection/preview/intent_receipt_identity.rs"
        )),
        "runtime/inspection/causal/request.rs" => {
            Some(include_str!("../../runtime/inspection/causal/request.rs"))
        }
        "runtime/inspection/causal/admission_decision.rs" => Some(include_str!(
            "../../runtime/inspection/causal/admission_decision.rs"
        )),
        "runtime/inspection/causal/admission_trace.rs" => Some(include_str!(
            "../../runtime/inspection/causal/admission_trace.rs"
        )),
        "runtime/inspection/causal/admission.rs" => {
            Some(include_str!("../../runtime/inspection/causal/admission.rs"))
        }
        "runtime/inspection/causal/builder_bridge.rs" => Some(include_str!(
            "../../runtime/inspection/causal/builder_bridge.rs"
        )),
        "runtime/inspection/causal/receipt.rs" => {
            Some(include_str!("../../runtime/inspection/causal/receipt.rs"))
        }
        "runtime/inspection/causal/receipt_helpers.rs" => Some(include_str!(
            "../../runtime/inspection/causal/receipt_helpers.rs"
        )),
        "runtime/inspection/causal/observation_identity.rs" => Some(include_str!(
            "../../runtime/inspection/causal/observation_identity.rs"
        )),
        "runtime/inspection/causal/identity.rs" => {
            Some(include_str!("../../runtime/inspection/causal/identity.rs"))
        }
        "runtime/inspection/causal/materialization/mod.rs" => Some(include_str!(
            "../../runtime/inspection/causal/materialization/mod.rs"
        )),
        "runtime/inspection/causal/materialization/policy.rs" => Some(include_str!(
            "../../runtime/inspection/causal/materialization/policy.rs"
        )),
        "runtime/inspection/causal/materialization/bridge_denial.rs" => Some(include_str!(
            "../../runtime/inspection/causal/materialization/bridge_denial.rs"
        )),
        "runtime/inspection/causal/materialization/performance.rs" => Some(include_str!(
            "../../runtime/inspection/causal/materialization/performance.rs"
        )),
        "runtime/inspection/causal/materialization/receipt.rs" => Some(include_str!(
            "../../runtime/inspection/causal/materialization/receipt.rs"
        )),
        "runtime/inspection/causal/materialization/proof.rs" => Some(include_str!(
            "../../runtime/inspection/causal/materialization/proof.rs"
        )),
        "runtime/inspection/causal/materialization/artifacts/denied.rs" => Some(include_str!(
            "../../runtime/inspection/causal/materialization/artifacts/denied.rs"
        )),
        "runtime/inspection/causal/materialization/artifacts/bridge_backed.rs" => Some(
            include_str!("../../runtime/inspection/causal/materialization/artifacts/bridge_backed.rs"),
        ),
        "runtime/inspection/causal/certification/artifacts/performance.rs" => Some(include_str!(
            "../../runtime/inspection/causal/certification/artifacts/performance.rs"
        )),
        "runtime/mutation/graph_composition/domain_invariant_denial.rs" => Some(include_str!(
            "../../runtime/mutation/graph_composition/domain_invariant_denial.rs"
        )),
        "runtime/mutation/graph_composition/denial.rs" => Some(include_str!(
            "../../runtime/mutation/graph_composition/denial.rs"
        )),
        "runtime/mutation/graph_composition/hooks.rs" => Some(include_str!(
            "../../runtime/mutation/graph_composition/hooks.rs"
        )),
        "runtime/mutation/assertion.rs" => {
            Some(include_str!("../../runtime/mutation/assertion.rs"))
        }
        "runtime/mutation/binding/existing_truth.rs" => Some(include_str!(
            "../../runtime/mutation/binding/existing_truth.rs"
        )),
        "runtime/mutation/binding/symbolic_reference.rs" => Some(include_str!(
            "../../runtime/mutation/binding/symbolic_reference.rs"
        )),
        "runtime/mutation/continuity.rs" => {
            Some(include_str!("../../runtime/mutation/continuity.rs"))
        }
        "runtime/mutation/lowering.rs" => Some(include_str!("../../runtime/mutation/lowering.rs")),
        "runtime/mutation/naming.rs" => Some(include_str!("../../runtime/mutation/naming.rs")),
        "runtime/mutation/probe.rs" => Some(include_str!("../../runtime/mutation/probe.rs")),
        "runtime/bridge_mutation_lowering.rs" => {
            Some(include_str!("../../runtime/bridge_mutation_lowering.rs"))
        }
        "runtime/runtime_batch_write_bridge_refs.rs" => Some(include_str!(
            "../../runtime/runtime_batch_write_bridge_refs.rs"
        )),
        "runtime/runtime_batch_write_receipt_context.rs" => Some(include_str!(
            "../../runtime/runtime_batch_write_receipt_context.rs"
        )),
        "runtime/read_composition_runtime.rs" => {
            Some(include_str!("../../runtime/read_composition_runtime.rs"))
        }
        "runtime/delivery.rs" => Some(include_str!("../../runtime/delivery.rs")),
        "runtime/runtime_intents.rs" => Some(include_str!("../../runtime/runtime_intents.rs")),
        "runtime/runtime_write_intents.rs" => {
            Some(include_str!("../../runtime/runtime_write_intents.rs"))
        }
        "runtime/computed/surface.rs" => Some(include_str!("../../runtime/computed/surface.rs")),
        "runtime/inspection/causal/reference_index.rs" => Some(include_str!(
            "../../runtime/inspection/causal/reference_index.rs"
        )),
        "subscription/maintenance_delta.rs" => {
            Some(include_str!("../../subscription/maintenance_delta.rs"))
        }
        "subscription/diagnostic/stage.rs" => {
            Some(include_str!("../../subscription/diagnostic/stage.rs"))
        }
        "subscription/diagnostic/trace.rs" => {
            Some(include_str!("../../subscription/diagnostic/trace.rs"))
        }
        "subscription/diagnostic/bundle.rs" => {
            Some(include_str!("../../subscription/diagnostic/bundle.rs"))
        }
        "subscription/diagnostic/context.rs" => {
            Some(include_str!("../../subscription/diagnostic/context.rs"))
        }
        "subscription/support/profile.rs" => {
            Some(include_str!("../../subscription/support/profile.rs"))
        }
        "subscription/input.rs" => Some(include_str!("../../subscription/input.rs")),
        "subscription/runtime_certification/error.rs" => Some(include_str!(
            "../../subscription/runtime_certification/error.rs"
        )),
        "subscription/runtime_certification/bundle.rs" => Some(include_str!(
            "../../subscription/runtime_certification/bundle.rs"
        )),
        "subscription/runtime_certification/coverage/row.rs" => Some(include_str!(
            "../../subscription/runtime_certification/coverage/row.rs"
        )),
        "subscription/certification.rs" => Some(include_str!("../../subscription/certification.rs")),
        "subscription/declaration.rs" => Some(include_str!("../../subscription/declaration.rs")),
        "subscription/delivery_window.rs" => {
            Some(include_str!("../../subscription/delivery_window.rs"))
        }
        "subscription/delivery_work_packet.rs" => {
            Some(include_str!("../../subscription/delivery_work_packet.rs"))
        }
        "subscription/bridge_parity/support.rs" => {
            Some(include_str!("../../subscription/bridge_parity/support.rs"))
        }
        "subscription/closeout.rs" => Some(include_str!("../../subscription/closeout.rs")),
        "subscription/active_error.rs" => Some(include_str!("../../subscription/active_error.rs")),
        "subscription/delivery_error.rs" => {
            Some(include_str!("../../subscription/delivery_error.rs"))
        }
        "subscription/attachment_error.rs" => {
            Some(include_str!("../../subscription/attachment_error.rs"))
        }
        "subscription/continuation_error.rs" => {
            Some(include_str!("../../subscription/continuation_error.rs"))
        }
        "subscription/preview_isolation_error.rs" => Some(include_str!(
            "../../subscription/preview_isolation_error.rs"
        )),
        "subscription/basis_request.rs" => Some(include_str!("../../subscription/basis_request.rs")),
        "subscription/counters.rs" => Some(include_str!("../../subscription/counters.rs")),
        "subscription/active_counters.rs" => {
            Some(include_str!("../../subscription/active_counters.rs"))
        }
        "subscription/admission_diagnostics.rs" => Some(include_str!(
            "../../subscription/admission_diagnostics.rs"
        )),
        "subscription/signal_strategy.rs" => {
            Some(include_str!("../../subscription/signal_strategy.rs"))
        }
        "subscription/bridge_parity/validation.rs" => Some(include_str!(
            "../../subscription/bridge_parity/validation.rs"
        )),
        "subscription/bridge_parity/witness.rs" => {
            Some(include_str!("../../subscription/bridge_parity/witness.rs"))
        }
        "subscription/attachment.rs" => Some(include_str!("../../subscription/attachment.rs")),
        "subscription/active_lane.rs" => Some(include_str!("../../subscription/active_lane.rs")),
        "subscription/active_handle.rs" => Some(include_str!("../../subscription/active_handle.rs")),
        "subscription/equivalence.rs" => Some(include_str!("../../subscription/equivalence.rs")),
        "subscription/declaration_digest.rs" => {
            Some(include_str!("../../subscription/declaration_digest.rs"))
        }
        "subscription/active_digest.rs" => Some(include_str!("../../subscription/active_digest.rs")),
        "subscription/attachment_digest.rs" => {
            Some(include_str!("../../subscription/attachment_digest.rs"))
        }
        "subscription/patch_group.rs" => Some(include_str!("../../subscription/patch_group.rs")),
        "subscription/acknowledgement.rs" => {
            Some(include_str!("../../subscription/acknowledgement.rs"))
        }
        "subscription/fanout.rs" => Some(include_str!("../../subscription/fanout.rs")),
        "subscription/lane_attachment_accessors.rs" => Some(include_str!(
            "../../subscription/lane_attachment_accessors.rs"
        )),
        "subscription/runtime_certification/coverage/matrix.rs" => Some(include_str!(
            "../../subscription/runtime_certification/coverage/matrix.rs"
        )),
        "subscription/runtime_certification/coverage/variations.rs" => Some(include_str!(
            "../../subscription/runtime_certification/coverage/variations.rs"
        )),
        "subscription/runtime_certification/scope.rs" => Some(include_str!(
            "../../subscription/runtime_certification/scope.rs"
        )),
        "subscription/future_selection.rs" => {
            Some(include_str!("../../subscription/future_selection.rs"))
        }
        "domain_capabilities/aftermath/mod.rs" => Some(include_str!("../../domain_capabilities/aftermath/mod.rs")),
        "domain_capabilities/authoring/admission.rs" => Some(include_str!("../../domain_capabilities/authoring/admission.rs")),
        "domain_capabilities/authoring/aftermath.rs" => Some(include_str!("../../domain_capabilities/authoring/aftermath.rs")),
        "domain_capabilities/authoring/continuity.rs" => Some(include_str!("../../domain_capabilities/authoring/continuity.rs")),
        "domain_capabilities/authoring/continuity_correspondence.rs" => Some(include_str!("../../domain_capabilities/authoring/continuity_correspondence.rs")),
        "domain_capabilities/authoring/explanation.rs" => Some(include_str!("../../domain_capabilities/authoring/explanation.rs")),
        "domain_capabilities/authoring/invariant_capability.rs" => Some(include_str!("../../domain_capabilities/authoring/invariant_capability.rs")),
        "domain_capabilities/authoring/mod.rs" => Some(include_str!("../../domain_capabilities/authoring/mod.rs")),
        "domain_capabilities/authoring/support.rs" => Some(include_str!("../../domain_capabilities/authoring/support.rs")),
        "domain_capabilities/authoring/workflow.rs" => Some(include_str!("../../domain_capabilities/authoring/workflow.rs")),
        "domain_capabilities/authoring/workflow_inspection.rs" => Some(include_str!("../../domain_capabilities/authoring/workflow_inspection.rs")),
        "domain_capabilities/canonical_runtime/admission.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/admission.rs")),
        "domain_capabilities/canonical_runtime/aftermath.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/aftermath.rs")),
        "domain_capabilities/canonical_runtime/artifacts.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/artifacts.rs")),
        "domain_capabilities/canonical_runtime/continuity.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/continuity.rs")),
        "domain_capabilities/canonical_runtime/continuity_correspondence.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/continuity_correspondence.rs")),
        "domain_capabilities/canonical_runtime/explanation.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/explanation.rs")),
        "domain_capabilities/canonical_runtime/invariant_capability.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/invariant_capability.rs")),
        "domain_capabilities/canonical_runtime/mod.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/mod.rs")),
        "domain_capabilities/canonical_runtime/support.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/support.rs")),
        "domain_capabilities/canonical_runtime/workflow/inspection.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/workflow/inspection.rs")),
        "domain_capabilities/canonical_runtime/workflow/lowering.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/workflow/lowering.rs")),
        "domain_capabilities/canonical_runtime/workflow/mod.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/workflow/mod.rs")),
        "domain_capabilities/canonical_runtime/workflow/preview.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/workflow/preview.rs")),
        "domain_capabilities/canonical_runtime/workflow/preview_identity.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/workflow/preview_identity.rs")),
        "domain_capabilities/canonical_runtime/workflow/semantics.rs" => Some(include_str!("../../domain_capabilities/canonical_runtime/workflow/semantics.rs")),
        "domain_capabilities/certification/bundle/mod.rs" => Some(include_str!("../../domain_capabilities/certification/bundle/mod.rs")),
        "domain_capabilities/certification/bundle/outputs.rs" => Some(include_str!("../../domain_capabilities/certification/bundle/outputs.rs")),
        "domain_capabilities/certification/certification_surface.rs" => Some(include_str!("../../domain_capabilities/certification/certification_surface.rs")),
        "domain_capabilities/certification/mod.rs" => Some(include_str!("../../domain_capabilities/certification/mod.rs")),
        "domain_capabilities/certification/output_manifest.rs" => Some(include_str!("../../domain_capabilities/certification/output_manifest.rs")),
        "domain_capabilities/certification/reports/fixtures.rs" => Some(include_str!("../../domain_capabilities/certification/reports/fixtures.rs")),
        "domain_capabilities/certification/reports/mod.rs" => Some(include_str!("../../domain_capabilities/certification/reports/mod.rs")),
        "domain_capabilities/certification/reports/representative.rs" => Some(include_str!("../../domain_capabilities/certification/reports/representative.rs")),
        "domain_capabilities/certification/reports/scaled.rs" => Some(include_str!("../../domain_capabilities/certification/reports/scaled.rs")),
        "domain_capabilities/certification/reports/slopes.rs" => Some(include_str!("../../domain_capabilities/certification/reports/slopes.rs")),
        "domain_capabilities/certification/surface/mod.rs" => Some(include_str!("../../domain_capabilities/certification/surface/mod.rs")),
        "domain_capabilities/continuity/mod.rs" => Some(include_str!("../../domain_capabilities/continuity/mod.rs")),
        "domain_capabilities/denials.rs" => Some(include_str!("../../domain_capabilities/denials.rs")),
        "domain_capabilities/dx/checked.rs" => Some(include_str!("../../domain_capabilities/dx/checked.rs")),
        "domain_capabilities/dx/common.rs" => Some(include_str!("../../domain_capabilities/dx/common.rs")),
        "domain_capabilities/dx/common/admitted_plan.rs" => Some(include_str!("../../domain_capabilities/dx/common/admitted_plan.rs")),
        "domain_capabilities/dx/common/aftermath.rs" => Some(include_str!("../../domain_capabilities/dx/common/aftermath.rs")),
        "domain_capabilities/dx/common/intent.rs" => Some(include_str!("../../domain_capabilities/dx/common/intent.rs")),
        "domain_capabilities/dx/common/intent_admission.rs" => Some(include_str!("../../domain_capabilities/dx/common/intent_admission.rs")),
        "domain_capabilities/dx/common/intent_workflow.rs" => Some(include_str!("../../domain_capabilities/dx/common/intent_workflow.rs")),
        "domain_capabilities/dx/common/lower_runtime.rs" => Some(include_str!("../../domain_capabilities/dx/common/lower_runtime.rs")),
        "domain_capabilities/dx/common/lower_runtime_explanation_request.rs" => Some(include_str!("../../domain_capabilities/dx/common/lower_runtime_explanation_request.rs")),
        "domain_capabilities/dx/common/projection_contract_request.rs" => Some(include_str!("../../domain_capabilities/dx/common/projection_contract_request.rs")),
        "domain_capabilities/dx/common/root.rs" => Some(include_str!("../../domain_capabilities/dx/common/root.rs")),
        "domain_capabilities/dx/common/shared.rs" => Some(include_str!("../../domain_capabilities/dx/common/shared.rs")),
        "domain_capabilities/dx/mod.rs" => Some(include_str!("../../domain_capabilities/dx/mod.rs")),
        "domain_capabilities/eligibility/mod.rs" => Some(include_str!("../../domain_capabilities/eligibility/mod.rs")),
        "domain_capabilities/eligibility/transitions.rs" => Some(include_str!("../../domain_capabilities/eligibility/transitions.rs")),
        "domain_capabilities/explanation/mod.rs" => Some(include_str!("../../domain_capabilities/explanation/mod.rs")),
        "domain_capabilities/foundational_integration/identity.rs" => Some(include_str!("../../domain_capabilities/foundational_integration/identity.rs")),
        "domain_capabilities/foundational_integration/mod.rs" => Some(include_str!("../../domain_capabilities/foundational_integration/mod.rs")),
        "domain_capabilities/foundational_integration/profiles.rs" => Some(include_str!("../../domain_capabilities/foundational_integration/profiles.rs")),
        "domain_capabilities/foundational_integration/provenance.rs" => Some(include_str!("../../domain_capabilities/foundational_integration/provenance.rs")),
        "domain_capabilities/foundational_integration/rows.rs" => Some(include_str!("../../domain_capabilities/foundational_integration/rows.rs")),
        "domain_capabilities/identity/certification.rs" => Some(include_str!("../../domain_capabilities/identity/certification.rs")),
        "domain_capabilities/identity/mod.rs" => Some(include_str!("../../domain_capabilities/identity/mod.rs")),
        "domain_capabilities/identity/scope.rs" => Some(include_str!("../../domain_capabilities/identity/scope.rs")),
        "domain_capabilities/materialization.rs" => Some(include_str!("../../domain_capabilities/materialization.rs")),
        "domain_capabilities/mod.rs" => Some(include_str!("../../domain_capabilities/mod.rs")),
        "domain_capabilities/payloads/admission.rs" => Some(include_str!("../../domain_capabilities/payloads/admission.rs")),
        "domain_capabilities/payloads/aftermath.rs" => Some(include_str!("../../domain_capabilities/payloads/aftermath.rs")),
        "domain_capabilities/payloads/common.rs" => Some(include_str!("../../domain_capabilities/payloads/common.rs")),
        "domain_capabilities/payloads/continuity.rs" => Some(include_str!("../../domain_capabilities/payloads/continuity.rs")),
        "domain_capabilities/payloads/continuity_correspondence.rs" => Some(include_str!("../../domain_capabilities/payloads/continuity_correspondence.rs")),
        "domain_capabilities/payloads/explanation.rs" => Some(include_str!("../../domain_capabilities/payloads/explanation.rs")),
        "domain_capabilities/payloads/invariant_capability.rs" => Some(include_str!("../../domain_capabilities/payloads/invariant_capability.rs")),
        "domain_capabilities/payloads/mod.rs" => Some(include_str!("../../domain_capabilities/payloads/mod.rs")),
        "domain_capabilities/payloads/support.rs" => Some(include_str!("../../domain_capabilities/payloads/support.rs")),
        "domain_capabilities/payloads/workflow.rs" => Some(include_str!("../../domain_capabilities/payloads/workflow.rs")),
        "domain_capabilities/payloads/workflow_semantics.rs" => Some(include_str!("../../domain_capabilities/payloads/workflow_semantics.rs")),
        "domain_capabilities/proof_integration/artifacts.rs" => Some(include_str!("../../domain_capabilities/proof_integration/artifacts.rs")),
        "domain_capabilities/proof_integration/mod.rs" => Some(include_str!("../../domain_capabilities/proof_integration/mod.rs")),
        "domain_capabilities/proof_integration/phases.rs" => Some(include_str!("../../domain_capabilities/proof_integration/phases.rs")),
        "domain_capabilities/proof_integration/proofs.rs" => Some(include_str!("../../domain_capabilities/proof_integration/proofs.rs")),
        "domain_capabilities/summary/artifacts.rs" => Some(include_str!("../../domain_capabilities/summary/artifacts.rs")),
        "domain_capabilities/summary/materializers.rs" => Some(include_str!("../../domain_capabilities/summary/materializers.rs")),
        "domain_capabilities/summary/mod.rs" => Some(include_str!("../../domain_capabilities/summary/mod.rs")),
        "domain_capabilities/support/artifacts.rs" => Some(include_str!("../../domain_capabilities/support/artifacts.rs")),
        "domain_capabilities/support/bundles.rs" => Some(include_str!("../../domain_capabilities/support/bundles.rs")),
        "domain_capabilities/support/mod.rs" => Some(include_str!("../../domain_capabilities/support/mod.rs")),
        "domain_capabilities/support/reports.rs" => Some(include_str!("../../domain_capabilities/support/reports.rs")),
        "domain_capabilities/targets/core.rs" => Some(include_str!("../../domain_capabilities/targets/core.rs")),
        "domain_capabilities/targets/mod.rs" => Some(include_str!("../../domain_capabilities/targets/mod.rs")),
        "domain_capabilities/targets/wrappers.rs" => Some(include_str!("../../domain_capabilities/targets/wrappers.rs")),
        "domain_capabilities/trace/artifacts.rs" => Some(include_str!("../../domain_capabilities/trace/artifacts.rs")),
        "domain_capabilities/trace/materializers.rs" => Some(include_str!("../../domain_capabilities/trace/materializers.rs")),
        "domain_capabilities/trace/mod.rs" => Some(include_str!("../../domain_capabilities/trace/mod.rs")),
        "domain_capabilities/workflow/mod.rs" => Some(include_str!("../../domain_capabilities/workflow/mod.rs")),
        "runtime/read_composition_hooks.rs" => {
            Some(include_str!("../../runtime/read_composition_hooks.rs"))
        }
        "lower_runtime_routing/protocol.rs" => {
            Some(include_str!("../../lower_runtime_routing/protocol.rs"))
        }
        "lower_runtime_routing/adapters/runtime_backend.rs" => Some(include_str!(
            "../../lower_runtime_routing/adapters/runtime_backend.rs"
        )),
        "lower_runtime_routing/adapters/runtime_backend/subject_digest.rs" => Some(include_str!(
            "../../lower_runtime_routing/adapters/runtime_backend/subject_digest.rs"
        )),
        "lower_runtime_routing/eligibility/mod.rs" => Some(include_str!(
            "../../lower_runtime_routing/eligibility/mod.rs"
        )),
        "lower_runtime_routing/plans/mod.rs" => {
            Some(include_str!("../../lower_runtime_routing/plans/mod.rs"))
        }
        "lower_runtime_routing/receipts/mod.rs" => {
            Some(include_str!("../../lower_runtime_routing/receipts/mod.rs"))
        }
        "lower_runtime_routing/envelopes.rs" => {
            Some(include_str!("../../lower_runtime_routing/envelopes.rs"))
        }
        "lower_runtime_routing/support.rs" => {
            Some(include_str!("../../lower_runtime_routing/support.rs"))
        }
        "lower_runtime_routing/inventory/crossing_types.rs" => {
            Some(include_str!("../../lower_runtime_routing/inventory/crossing_types.rs"))
        }
        "lower_runtime_routing/inventory/closeout_types.rs" => {
            Some(include_str!("../../lower_runtime_routing/inventory/closeout_types.rs"))
        }
        "lower_runtime_routing/inventory/gap_types.rs" => {
            Some(include_str!("../../lower_runtime_routing/inventory/gap_types.rs"))
        }
        "worth-runtime-bridge/src/diagnostics/causal_envelope/mod.rs" => Some(include_str!(
            "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/mod.rs"
        )),
        "worth-runtime-bridge/src/diagnostics/causal_envelope/evidence_reference.rs" => Some(
            include_str!(
                "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/evidence_reference.rs"
            ),
        ),
        "worth-runtime-bridge/src/diagnostics/causal_envelope/binding.rs" => Some(include_str!(
            "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/binding.rs"
        )),
        "worth-runtime-bridge/src/diagnostics/causal_envelope/denial.rs" => Some(include_str!(
            "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/denial.rs"
        )),
        "worth-runtime-bridge/src/diagnostics/causal_envelope/counters.rs" => Some(include_str!(
            "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/counters.rs"
        )),
        "worth-runtime-bridge/src/diagnostics/causal_envelope/explanation_envelope.rs" => Some(
            include_str!(
                "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/explanation_envelope.rs"
            ),
        ),
        "worth-runtime-bridge/src/diagnostics/causal_envelope/identity.rs" => Some(include_str!(
            "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/identity.rs"
        )),
        "worth-runtime-bridge/src/diagnostics/causal_envelope/receipt.rs" => Some(include_str!(
            "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/receipt.rs"
        )),
        "worth-runtime-bridge/src/diagnostics/causal_envelope/assembly/request.rs" => Some(
            include_str!(
                "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/assembly/request.rs"
            ),
        ),
        "worth-runtime-bridge/src/diagnostics/causal_envelope/assembly/mod.rs" => Some(
            include_str!(
                "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/assembly/mod.rs"
            ),
        ),
        "worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/mod.rs" => Some(
            include_str!(
                "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/mod.rs"
            ),
        ),
        "worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/digest_basis.rs" => {
            Some(include_str!(
                "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/digest_basis.rs"
            ))
        }
        "worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/retained_artifact_digest/planning_checkpoint.rs" => {
            Some(include_str!(
                "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/retained_artifact_digest/planning_checkpoint.rs"
            ))
        }
        "worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/retained_artifact_digest/route_history_preview.rs" => {
            Some(include_str!(
                "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/retained_artifact_digest/route_history_preview.rs"
            ))
        }
        "worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/retained_artifact_digest/source_structural_stream.rs" => {
            Some(include_str!(
                "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/retained_artifact_digest/source_structural_stream.rs"
            ))
        }
        "worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/retained_artifact_digest/writeback.rs" => {
            Some(include_str!(
                "../../../../worth-runtime-bridge/src/diagnostics/causal_envelope/retained_mapping/retained_artifact_digest/writeback.rs"
            ))
        }
        "lower_runtime_routing/certification/surface/acceptance_cardinality.rs" => {
            Some(include_str!(
                "../../lower_runtime_routing/certification/surface/acceptance_cardinality.rs"
            ))
        }
        "lower_runtime_routing/certification/surface/fixtures/bridge_fixture.rs" => {
            Some(include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/bridge_fixture.rs"
            ))
        }
        "lower_runtime_routing/certification/surface/fixtures/core.rs" => Some(include_str!(
            "../../lower_runtime_routing/certification/surface/fixtures/core.rs"
        )),
        "lower_runtime_routing/certification/surface/fixtures/phase_six/mod.rs" => {
            Some(include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/phase_six/mod.rs"
            ))
        }
        "lower_runtime_routing/certification/surface/fixtures/phase_six/causal_signal.rs" => {
            Some(include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/phase_six/causal_signal.rs"
            ))
        }
        "lower_runtime_routing/certification/surface/fixtures/phase_six/effect.rs" => Some(
            include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/phase_six/effect.rs"
            ),
        ),
        "lower_runtime_routing/certification/surface/fixtures/phase_six/effect_support.rs" => {
            Some(include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/phase_six/effect_support.rs"
            ))
        }
        "lower_runtime_routing/certification/surface/fixtures/phase_six/historical.rs" => {
            Some(include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/phase_six/historical.rs"
            ))
        }
        "lower_runtime_routing/certification/surface/fixtures/phase_six/intent.rs" => Some(
            include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/phase_six/intent.rs"
            ),
        ),
        "lower_runtime_routing/certification/surface/fixtures/phase_six/live_aggregate.rs" => {
            Some(include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/phase_six/live_aggregate.rs"
            ))
        }
        "lower_runtime_routing/certification/surface/fixtures/phase_six/projection.rs" => {
            Some(include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/phase_six/projection.rs"
            ))
        }
        "lower_runtime_routing/certification/surface/fixtures/phase_six/projection_bridge_runtime.rs" => {
            Some(include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/phase_six/projection_bridge_runtime.rs"
            ))
        }
        "lower_runtime_routing/certification/surface/fixtures/phase_six/read_execution.rs" => {
            Some(include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/phase_six/read_execution.rs"
            ))
        }
        "lower_runtime_routing/certification/surface/fixtures/phase_six/readmission.rs" => {
            Some(include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/phase_six/readmission.rs"
            ))
        }
        "lower_runtime_routing/certification/surface/fixtures/phase_six/readmission_support.rs" => {
            Some(include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/phase_six/readmission_support.rs"
            ))
        }
        "lower_runtime_routing/certification/surface/fixtures/phase_six/subscription.rs" => {
            Some(include_str!(
                "../../lower_runtime_routing/certification/surface/fixtures/phase_six/subscription.rs"
            ))
        }
        "runtime/surface/graph_composition_breadth.rs" => Some(include_str!(
            "../../runtime/surface/graph_composition_breadth.rs"
        )),
        "runtime/surface/graph_composition_program.rs" => Some(include_str!(
            "../../runtime/surface/graph_composition_program.rs"
        )),
        "runtime/surface/graph_composition_admission_trace.rs" => Some(include_str!(
            "../../runtime/surface/graph_composition_admission_trace.rs"
        )),
        "runtime/surface/graph_composition_domain_invariant_summary.rs" => Some(include_str!(
            "../../runtime/surface/graph_composition_domain_invariant_summary.rs"
        )),
        "runtime/surface/graph_composition_lifecycle_outcomes.rs" => Some(include_str!(
            "../../runtime/surface/graph_composition_lifecycle_outcomes.rs"
        )),
        "runtime/surface/graph_composition_assumption_summary.rs" => Some(include_str!(
            "../../runtime/surface/graph_composition_assumption_summary.rs"
        )),
        "runtime/surface/graph_composition_lineage_summary.rs" => Some(include_str!(
            "../../runtime/surface/graph_composition_lineage_summary.rs"
        )),
        "runtime/surface/graph_composition_resolution_map.rs" => Some(include_str!(
            "../../runtime/surface/graph_composition_resolution_map.rs"
        )),
        "runtime/surface/graph_composition_evidence.rs" => Some(include_str!(
            "../../runtime/surface/graph_composition_evidence.rs"
        )),
        "runtime/surface/naming_mutation_evidence.rs" => Some(include_str!(
            "../../runtime/surface/naming_mutation_evidence.rs"
        )),
        "runtime/surface/continuity_mutation_evidence.rs" => Some(include_str!(
            "../../runtime/surface/continuity_mutation_evidence.rs"
        )),
        "runtime/surface/symbolic_target_reference_evidence.rs" => Some(include_str!(
            "../../runtime/surface/symbolic_target_reference_evidence.rs"
        )),
        "runtime/surface/symbolic_aspect_resolution_evidence.rs" => Some(include_str!(
            "../../runtime/surface/symbolic_aspect_resolution_evidence.rs"
        )),
        "runtime/surface/read_domain_invariant_summary.rs" => Some(include_str!(
            "../../runtime/surface/read_domain_invariant_summary.rs"
        )),
        "runtime/surface/verified_assumption_set.rs" => Some(include_str!(
            "../../runtime/surface/verified_assumption_set.rs"
        )),
        "runtime/surface/mutation_evidence/binding.rs" => Some(include_str!(
            "../../runtime/surface/mutation_evidence/binding.rs"
        )),
        "runtime/surface/mutation_evidence/causality.rs" => Some(include_str!(
            "../../runtime/surface/mutation_evidence/causality.rs"
        )),
        "runtime/surface/mutation_evidence/provenance.rs" => Some(include_str!(
            "../../runtime/surface/mutation_evidence/provenance.rs"
        )),
        "runtime/surface/mutation_evidence/target.rs" => Some(include_str!(
            "../../runtime/surface/mutation_evidence/target.rs"
        )),
        "runtime/surface/mutation_evidence/batch.rs" => Some(include_str!(
            "../../runtime/surface/mutation_evidence/batch.rs"
        )),
        "runtime/surface/mutation_evidence/batch_digest_helpers.rs" => Some(include_str!(
            "../../runtime/surface/mutation_evidence/batch_digest_helpers.rs"
        )),
        "runtime/surface/read_composition.rs" => {
            Some(include_str!("../../runtime/surface/read_composition.rs"))
        }
        "runtime/surface/read_domain_invariant_denial.rs" => Some(include_str!(
            "../../runtime/surface/read_domain_invariant_denial.rs"
        )),
        "runtime/surface/mutation/batch_receipt.rs" => Some(include_str!(
            "../../runtime/surface/mutation/batch_receipt.rs"
        )),
        "runtime/surface/mutation/write_receipt/helpers.rs" => Some(include_str!(
            "../../runtime/surface/mutation/write_receipt/helpers.rs"
        )),
        "runtime/effect/inspection.rs" => Some(include_str!("../../runtime/effect/inspection.rs")),
        "runtime/effect/declaration.rs" => {
            Some(include_str!("../../runtime/effect/declaration.rs"))
        }
        "runtime/effect/follow_on.rs" => Some(include_str!("../../runtime/effect/follow_on.rs")),
        "runtime/effect/inspection_identity.rs" => Some(include_str!(
            "../../runtime/effect/inspection_identity.rs"
        )),
        "effect_lifecycle/execution_bridge.rs" => {
            Some(include_str!("../../effect_lifecycle/execution_bridge.rs"))
        }
        "effect_lifecycle/execution_relational_scalar.rs" => Some(include_str!(
            "../../effect_lifecycle/execution_relational_scalar.rs"
        )),
        "view_shape_live/grouped_execution.rs" => {
            Some(include_str!("../../view_shape_live/grouped_execution.rs"))
        }
        _ => None,
    }
}

pub fn source_for_session_admission_path(path: &str) -> Option<&'static str> {
    match path {
        "runtime/runtime_sessions.rs" => Some(include_str!("../../runtime/runtime_sessions.rs")),
        "runtime/workspace.rs" => Some(include_str!("../../runtime/workspace.rs")),
        _ => None,
    }
}

pub fn source_for_string_carried_session_identity_path(path: &str) -> Option<&'static str> {
    match path {
        "runtime/error.rs" => Some(include_str!("../../runtime/error.rs")),
        "runtime/preview/workflow_ops.rs" => {
            Some(include_str!("../../runtime/preview/workflow_ops.rs"))
        }
        "runtime/preview/binding.rs" => Some(include_str!("../../runtime/preview/binding.rs")),
        "runtime/preview/session_execution.rs" => {
            Some(include_str!("../../runtime/preview/session_execution.rs"))
        }
        "runtime/preview/mutation_ops.rs" => {
            Some(include_str!("../../runtime/preview/mutation_ops.rs"))
        }
        "runtime/surface/mutation/write_receipt/preview.rs" => Some(include_str!(
            "../../runtime/surface/mutation/write_receipt/preview.rs"
        )),
        "runtime/inspection/preview/binding.rs" => {
            Some(include_str!("../../runtime/inspection/preview/binding.rs"))
        }
        "runtime/inspection/preview/outcome.rs" => {
            Some(include_str!("../../runtime/inspection/preview/outcome.rs"))
        }
        _ => None,
    }
}
