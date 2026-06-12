use super::super::support::*;

pub(super) mod runtime_paths;

use runtime_paths::{
    intent_commit_denied_error, intent_execution_routing_failed_error,
    preview_promotion_atomic_batch_unsupported_error, preview_promotion_rebinding_required_error,
    preview_promotion_stale_basis_error, preview_promotion_write_failed_error,
    read_domain_invariant_denied_error,
};

pub(super) fn representative_runtime_stop_errors() -> Vec<ForgeQueryRuntimeError> {
    let binding = existing_binding();
    let assertion_denial = ForgeQueryExistingTruthAssertionDenial::new(
        &binding,
        ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
        Some("status.value".to_string()),
        Some("\"open\"".to_string()),
        None,
        "missing asserted aspect",
    );
    let probe_denial = ForgeQueryExistingTruthProbeDenial::new(
        &binding,
        ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
        Some("status.value".to_string()),
        "missing probed aspect",
    );
    let binding_denial = ForgeQueryExistingTruthBindingDenial::new(
        &binding,
        ForgeQueryExistingTruthBindingDenialKind::CollectionMismatch,
        "wrong collection",
    );
    let continuity_intent = ForgeQueryContinuityMutationIntent::rebind_existing_target(
        crate::runtime::ForgeQueryMutationAuthorityIdentity::continuity_prior_authority(
            crate::runtime::ForgeQueryContinuityPriorAuthorityLabel::new("authority:task-1")
                .expect("continuity prior authority label"),
        )
        .expect("continuity prior authority identity"),
        crate::runtime::ForgeQueryMutationAuthorityIdentity::continuity_successor_authority(
            crate::runtime::ForgeQueryContinuitySuccessorAuthorityLabel::new("authority:task-1")
                .expect("continuity successor authority label"),
        )
        .expect("continuity successor authority identity"),
    )
    .expect("continuity intent should build");
    let continuity_denial = ForgeQueryContinuityMutationDenial::new(
        &continuity_intent,
        Some(&binding),
        ForgeQueryContinuityMutationDenialKind::RequiresExistingTruthBinding,
        "continuity requires binding",
    );
    let graph_denial = ForgeQueryGraphCompositionDenial::new(
        ForgeQueryGraphCompositionDenialKind::DuplicateSymbolDeclaration,
        Some("task_symbol".to_string()),
        Some("Task".to_string()),
        "duplicate symbol",
    );
    let graph_domain_denial = ForgeQueryGraphCompositionDomainInvariantDenial::from_contributed(
        "graph.family",
        "graph domain invariant failed",
        ForgeQueryGraphCompositionDomainInvariantSummary::from_parts(
            vec!["Task".to_string()],
            vec!["task_symbol".to_string()],
            vec!["same_batch_entity_relation_identity_edges".to_string()],
            vec!["mixed_existing_target_followup_mutation".to_string()],
            graph_domain_fixture_digest("program"),
            graph_domain_fixture_digest("breadth"),
            "components=1".to_string(),
        ),
    );
    let naming_intent = ForgeQueryNamingMutationIntent::attach_new_target(
        crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_attachment(
            crate::runtime::ForgeQueryNamingAttachmentAuthorityLabel::new("attachment-1")
                .expect("naming attachment authority label"),
        )
        .expect("naming attachment identity"),
    );
    let naming_denial = ForgeQueryNamingMutationDenial::new(
        &naming_intent,
        ForgeQueryNamingMutationDenialKind::RequiresSameBatchTargetReference,
        "naming needs a same-batch target",
    );
    let target_reference =
        ForgeQuerySymbolicTargetReference::new("task_symbol").expect("reference should build");
    let symbolic_denial = ForgeQuerySymbolicTargetReferenceDenial::new(
        &target_reference,
        ForgeQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget,
        "same-batch target unresolved",
    );
    let read_denial = ForgeQueryReadDenial::new(
        ForgeQueryReadDenialKind::ValidationDenied,
        "read validation failed",
    );
    let effect_policy_denial = ForgeQueryEffectPolicy::DeriveOnly
        .admit(
            ForgeQueryEffectAction::WriteIntent,
            ForgeQueryAuthorityLane::AuthoritativeTruth,
        )
        .expect_err("derive-only write intent should deny");
    let support_denial = ForgeQueryRuntimeSupportDenial::new(
        ForgeQueryRuntimeFacadeFamily::Temporal,
        ForgeQueryRuntimeFamilySupportStatus::Unsupported,
        Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
        "temporal gate is closed",
    );

    vec![
        ForgeQueryRuntimeError::MissingBackend,
        ForgeQueryRuntimeError::MissingRuntimeBridge,
        ForgeQueryRuntimeError::MissingSchemaAdapter,
        ForgeQueryRuntimeError::MissingSourceAdapter,
        ForgeQueryRuntimeError::MissingWriteAuthority,
        ForgeQueryRuntimeError::MissingSignalSink,
        ForgeQueryRuntimeError::MissingSubscriptionActivation,
        ForgeQueryRuntimeError::MissingPreviewBasis,
        ForgeQueryRuntimeError::MissingInspectorEvidence,
        ForgeQueryRuntimeError::MissingIntentAuthority,
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(assertion_denial),
        ForgeQueryRuntimeError::ExistingTruthProbeDenied(probe_denial),
        ForgeQueryRuntimeError::MutationBindingDenied(binding_denial),
        ForgeQueryRuntimeError::MutationContinuityDenied(continuity_denial),
        ForgeQueryRuntimeError::GraphCompositionDenied(graph_denial),
        ForgeQueryRuntimeError::GraphCompositionDomainInvariantDenied(graph_domain_denial),
        ForgeQueryRuntimeError::MutationNamingDenied(naming_denial),
        ForgeQueryRuntimeError::MutationTargetReferenceDenied(symbolic_denial),
        ForgeQueryRuntimeError::ReadCompositionDenied(read_denial),
        read_domain_invariant_denied_error(),
        ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new("workspace failed")),
        ForgeQueryRuntimeError::Program(ForgeQueryProgramError::new("program failed")),
        ForgeQueryRuntimeError::UnknownProgram("missing.program".to_string()),
        ForgeQueryRuntimeError::UnknownOperation {
            program_id: "program.id".to_string(),
            operation_id: "operation.id".to_string(),
        },
        ForgeQueryRuntimeError::MissingLiveView("view.live".to_string()),
        ForgeQueryRuntimeError::MissingLiveSubscription("sub.live".to_string()),
        ForgeQueryRuntimeError::MissingDerivedView("view.derived".to_string()),
        ForgeQueryRuntimeError::SharedReadStaleBasis {
            snapshot_identity:
                crate::memory_workspace::ForgeQuerySnapshotIdentity::from_external_authority_label(
                    "snapshot.stale",
                ),
        },
        ForgeQueryRuntimeError::MissingEffect("effect.name".to_string()),
        ForgeQueryRuntimeError::MissingPendingWriteIntent("effect.name".to_string()),
        ForgeQueryRuntimeError::RetainedRowDecode {
            view_name: "view.retained".to_string(),
            stage: "decode",
            message: "decode failed".to_string(),
        },
        ForgeQueryRuntimeError::ComputedDeclaration {
            view_name: "view.computed".to_string(),
            stage: "declare",
            message: "computed failed".to_string(),
        },
        ForgeQueryRuntimeError::EffectDeclaration {
            effect_name: "effect.declare".to_string(),
            stage: "declare",
            message: "effect failed".to_string(),
        },
        ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: "view.live".to_string(),
            stage: "install",
            message: "install failed".to_string(),
        },
        ForgeQueryRuntimeError::UnsupportedAuthorityRequirement(
            ForgeQueryAuthorityRequirement::Merge,
        ),
        ForgeQueryRuntimeError::ExistingTruthAssertionRequiresAuthorityLane {
            required_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        },
        intent_commit_denied_error(),
        intent_execution_routing_failed_error(),
        ForgeQueryRuntimeError::EffectPolicyDenied(effect_policy_denial),
        preview_promotion_stale_basis_error(),
        preview_promotion_atomic_batch_unsupported_error(),
        preview_promotion_rebinding_required_error(),
        preview_promotion_write_failed_error(),
        ForgeQueryRuntimeError::InvariantRegistration {
            stage: "registration",
            message: "registration failed".to_string(),
        },
        ForgeQueryRuntimeError::SessionLabelCollision {
            authority_lane: ForgeQueryAuthorityLane::PreviewTruth,
            label: test_session_label("stop-class-collision"),
        },
        ForgeQueryRuntimeError::PreviewOperationEffectDenied {
            label: test_session_label("preview-label"),
            stage: "effect-admission",
            message: "preview declaration denied".to_string(),
        },
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(support_denial),
    ]
}

fn graph_domain_fixture_digest(
    role: &'static str,
) -> crate::evidence_identity::ForgeQueryEvidenceIdentity {
    crate::evidence_identity::forge_query_evidence_identity(
        crate::evidence_identity::ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
    )
    .field_shape(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("role"),
        "stop-class-graph-domain-fixture",
    )
    .field_shape(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("fixture"),
        role,
    )
    .seal()
}

pub(super) fn representative_runtime_generated_stop_errors() -> Vec<ForgeQueryRuntimeError> {
    vec![
        read_domain_invariant_denied_error(),
        intent_commit_denied_error(),
        intent_execution_routing_failed_error(),
        preview_promotion_stale_basis_error(),
        preview_promotion_atomic_batch_unsupported_error(),
        preview_promotion_rebinding_required_error(),
        preview_promotion_write_failed_error(),
    ]
}

pub(super) fn runtime_error_variant_key(error: &ForgeQueryRuntimeError) -> &'static str {
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

pub(super) fn stop_class_variant_key(stop_class: ForgeQueryStopClass<'_>) -> &'static str {
    match stop_class {
        ForgeQueryStopClass::MissingRuntimeComponent { .. } => "missing_runtime_component",
        ForgeQueryStopClass::ExistingTruthAssertionDenied { .. } => {
            "existing_truth_assertion_denied"
        }
        ForgeQueryStopClass::ExistingTruthProbeDenied { .. } => "existing_truth_probe_denied",
        ForgeQueryStopClass::MutationBindingDenied { .. } => "mutation_binding_denied",
        ForgeQueryStopClass::MutationContinuityDenied { .. } => "mutation_continuity_denied",
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

pub(super) fn existing_binding() -> ForgeQueryExistingTruthTargetBinding {
    ForgeQueryExistingTruthTargetBinding::direct_entity(
        crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity"),
        test_entity_identity("Task:1"),
    )
    .expect("binding should build")
    .in_target_collection("Task")
    .expect("collection should build")
}
