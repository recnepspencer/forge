use super::super::super::support::*;

pub(in crate::runtime::tests::stop_class) fn runtime_error_variant_key(
    error: &ForgeQueryRuntimeError,
) -> &'static str {
    match error {
        ForgeQueryRuntimeError::MissingBackend => "missing_backend",
        ForgeQueryRuntimeError::MissingRuntimeBridge => "missing_runtime_bridge",
        ForgeQueryRuntimeError::MissingSchemaAdapter => "missing_schema_adapter",
        ForgeQueryRuntimeError::MissingSnapshotIdentityAdapter => {
            "missing_snapshot_identity_adapter"
        }
        ForgeQueryRuntimeError::MissingSourceAdapter => "missing_source_adapter",
        ForgeQueryRuntimeError::MissingWriteAuthority => "missing_write_authority",
        ForgeQueryRuntimeError::MissingSignalSink => "missing_signal_sink",
        ForgeQueryRuntimeError::MissingSubscriptionActivation => "missing_subscription_activation",
        ForgeQueryRuntimeError::MissingPreviewBasis => "missing_preview_basis",
        ForgeQueryRuntimeError::MissingInspectorEvidence => "missing_inspector_evidence",
        ForgeQueryRuntimeError::MissingIntentAuthority => "missing_intent_authority",
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(_) => {
            "existing_truth_assertion_denied"
        }
        ForgeQueryRuntimeError::ExistingTruthProbeDenied(_) => "existing_truth_probe_denied",
        ForgeQueryRuntimeError::MutationBindingDenied(_) => "mutation_binding_denied",
        ForgeQueryRuntimeError::MutationContinuityDenied(_) => "mutation_continuity_denied",
        ForgeQueryRuntimeError::GraphObligationTouchDescriptorDenied(_) => {
            "graph_obligation_touch_descriptor_denied"
        }
        ForgeQueryRuntimeError::GraphObligationEffectTouchDescriptorMissing { .. } => {
            "graph_obligation_effect_touch_descriptor_missing"
        }
        ForgeQueryRuntimeError::GraphObligationIntentTouchDescriptorMissing { .. } => {
            "graph_obligation_intent_touch_descriptor_missing"
        }
        ForgeQueryRuntimeError::GraphMutationPolicyContextDenied { .. } => {
            "graph_mutation_policy_context_denied"
        }
        ForgeQueryRuntimeError::GraphMutationPolicyGateDenied(_) => {
            "graph_mutation_policy_gate_denied"
        }
        ForgeQueryRuntimeError::GraphObligationDenied(_) => "graph_obligation_denied",
        ForgeQueryRuntimeError::GraphCompositionDenied(_) => "graph_composition_denied",
        ForgeQueryRuntimeError::GraphCompositionDomainInvariantDenied(_) => {
            "graph_composition_domain_invariant_denied"
        }
        ForgeQueryRuntimeError::MutationNamingDenied(_) => "mutation_naming_denied",
        ForgeQueryRuntimeError::MutationTargetReferenceDenied(_) => {
            "mutation_target_reference_denied"
        }
        ForgeQueryRuntimeError::ReadCompositionDenied(_) => "read_composition_denied",
        ForgeQueryRuntimeError::ReadCompositionDomainInvariantDenied(_) => {
            "read_composition_domain_invariant_denied"
        }
        ForgeQueryRuntimeError::Workspace(_) => "workspace",
        ForgeQueryRuntimeError::Program(_) => "program",
        ForgeQueryRuntimeError::UnknownProgram(_) => "unknown_program",
        ForgeQueryRuntimeError::UnknownOperation { .. } => "unknown_operation",
        ForgeQueryRuntimeError::MissingLiveView(_) => "missing_live_view",
        ForgeQueryRuntimeError::MissingLiveSubscription(_) => "missing_live_subscription",
        ForgeQueryRuntimeError::MissingDerivedView(_) => "missing_derived_view",
        ForgeQueryRuntimeError::MissingEffect(_) => "missing_effect",
        ForgeQueryRuntimeError::MissingPendingWriteIntent(_) => "missing_pending_write_intent",
        ForgeQueryRuntimeError::RetainedRowDecode { .. } => "retained_row_decode",
        ForgeQueryRuntimeError::ComputedDeclaration { .. } => "computed_declaration",
        ForgeQueryRuntimeError::EffectDeclaration { .. } => "effect_declaration",
        ForgeQueryRuntimeError::LiveSubscriptionInstallation { .. } => {
            "live_subscription_installation"
        }
        ForgeQueryRuntimeError::UnsupportedAuthorityRequirement(_) => {
            "unsupported_authority_requirement"
        }
        ForgeQueryRuntimeError::ExistingTruthAssertionRequiresAuthorityLane { .. } => {
            "existing_truth_assertion_requires_authority_lane"
        }
        ForgeQueryRuntimeError::IntentCommitDenied { .. } => "intent_commit_denied",
        ForgeQueryRuntimeError::IntentExecutionRoutingFailed { .. } => {
            "intent_execution_routing_failed"
        }
        ForgeQueryRuntimeError::EffectPolicyDenied(_) => "effect_policy_denied",
        ForgeQueryRuntimeError::PreviewPromotionStaleBasis(_) => "preview_promotion_stale_basis",
        ForgeQueryRuntimeError::SharedReadStaleBasis { .. } => "shared_read_stale_basis",
        ForgeQueryRuntimeError::JournalReplayDenied(_) => "journal_replay_denied",
        ForgeQueryRuntimeError::PreviewPromotionAtomicBatchUnsupported(_) => {
            "preview_promotion_atomic_batch_unsupported"
        }
        ForgeQueryRuntimeError::PreviewPromotionRebindingRequired(_) => {
            "preview_promotion_rebinding_required"
        }
        ForgeQueryRuntimeError::PreviewPromotionWriteFailed { .. } => {
            "preview_promotion_write_failed"
        }
        ForgeQueryRuntimeError::InvariantRegistration { .. } => "invariant_registration",
        ForgeQueryRuntimeError::SessionLabelCollision { .. } => "session_label_collision",
        ForgeQueryRuntimeError::PreviewOperationEffectDenied { .. } => {
            "preview_operation_effect_denied"
        }
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(_) => "unsupported_facade_family",
    }
}

pub(in crate::runtime::tests::stop_class) fn stop_class_variant_key(
    stop_class: ForgeQueryStopClass<'_>,
) -> &'static str {
    match stop_class {
        ForgeQueryStopClass::MissingRuntimeComponent { .. } => "missing_runtime_component",
        ForgeQueryStopClass::ExistingTruthAssertionDenied { .. } => {
            "existing_truth_assertion_denied"
        }
        ForgeQueryStopClass::ExistingTruthProbeDenied { .. } => "existing_truth_probe_denied",
        ForgeQueryStopClass::MutationBindingDenied { .. } => "mutation_binding_denied",
        ForgeQueryStopClass::MutationContinuityDenied { .. } => "mutation_continuity_denied",
        ForgeQueryStopClass::GraphObligationTouchDescriptorDenied { .. } => {
            "graph_obligation_touch_descriptor_denied"
        }
        ForgeQueryStopClass::GraphObligationEffectTouchDescriptorMissing { .. } => {
            "graph_obligation_effect_touch_descriptor_missing"
        }
        ForgeQueryStopClass::GraphObligationIntentTouchDescriptorMissing { .. } => {
            "graph_obligation_intent_touch_descriptor_missing"
        }
        ForgeQueryStopClass::GraphMutationPolicyContextDenied { .. } => {
            "graph_mutation_policy_context_denied"
        }
        ForgeQueryStopClass::GraphMutationPolicyGateDenied { .. } => {
            "graph_mutation_policy_gate_denied"
        }
        ForgeQueryStopClass::GraphObligationDenied { .. } => "graph_obligation_denied",
        ForgeQueryStopClass::GraphCompositionDenied { .. } => "graph_composition_denied",
        ForgeQueryStopClass::GraphCompositionDomainInvariantDenied { .. } => {
            "graph_composition_domain_invariant_denied"
        }
        ForgeQueryStopClass::MutationNamingDenied { .. } => "mutation_naming_denied",
        ForgeQueryStopClass::MutationTargetReferenceDenied { .. } => {
            "mutation_target_reference_denied"
        }
        ForgeQueryStopClass::ReadCompositionDenied { .. } => "read_composition_denied",
        ForgeQueryStopClass::ReadCompositionDomainInvariantDenied { .. } => {
            "read_composition_domain_invariant_denied"
        }
        ForgeQueryStopClass::Workspace { .. } => "workspace",
        ForgeQueryStopClass::Program { .. } => "program",
        ForgeQueryStopClass::RuntimeLookupFailed { .. } => "runtime_lookup_failed",
        ForgeQueryStopClass::MissingRuntimeArtifact { .. } => "missing_runtime_artifact",
        ForgeQueryStopClass::SharedReadStaleBasis { .. } => "shared_read_stale_basis",
        ForgeQueryStopClass::JournalReplayDenied { .. } => "journal_replay_denied",
        ForgeQueryStopClass::RuntimeDeclarationFailed { .. } => "runtime_declaration_failed",
        ForgeQueryStopClass::PreviewOperationEffectDenied { .. } => {
            "preview_operation_effect_denied"
        }
        ForgeQueryStopClass::SessionLabelCollision { .. } => "session_label_collision",
        ForgeQueryStopClass::UnsupportedAuthorityRequirement { .. } => {
            "unsupported_authority_requirement"
        }
        ForgeQueryStopClass::ExistingTruthAssertionRequiresAuthorityLane { .. } => {
            "existing_truth_assertion_requires_authority_lane"
        }
        ForgeQueryStopClass::IntentCommitDenied { .. } => "intent_commit_denied",
        ForgeQueryStopClass::IntentExecutionRoutingFailed { .. } => {
            "intent_execution_routing_failed"
        }
        ForgeQueryStopClass::EffectPolicyDenied { .. } => "effect_policy_denied",
        ForgeQueryStopClass::PreviewPromotionDenied { .. } => "preview_promotion_denied",
        ForgeQueryStopClass::FamilyAdmissionDenied { .. } => "family_admission_denied",
    }
}
