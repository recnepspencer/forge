use super::super::support::*;

pub(super) mod runtime_paths;
mod variant_keys;

use runtime_paths::{
    intent_commit_denied_error, intent_execution_routing_failed_error,
    preview_promotion_atomic_batch_unsupported_error, preview_promotion_rebinding_required_error,
    preview_promotion_stale_basis_error, preview_promotion_write_failed_error,
    read_domain_invariant_denied_error,
};
pub(super) use variant_keys::{runtime_error_variant_key, stop_class_variant_key};

pub(super) fn representative_runtime_stop_errors() -> Vec<ForgeQueryRuntimeError> {
    let binding = existing_binding();
    let status_touch = status_value_touch();
    let assertion_denial = ForgeQueryExistingTruthAssertionDenial::new(
        &binding,
        ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
        Some(status_touch.clone()),
        Some("\"open\"".to_string()),
        None,
        "missing asserted aspect",
    );
    let probe_denial = ForgeQueryExistingTruthProbeDenial::new(
        &binding,
        ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
        Some(status_touch),
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
        Some(
            crate::runtime::ForgeQueryMutationTargetCollectionIdentity::new(
                "graph-composition-test",
                "Task",
            ),
        ),
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
    let replay_denial = ForgeQueryJournalReplayDenial::new(
        ForgeQueryJournalReplayDenialKind::UnknownSegmentIdentity,
        "segment is not retained by replay registry",
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
        ForgeQueryRuntimeError::GraphObligationTouchDescriptorDenied(
            ForgeQueryGraphTouchDescriptorDenial::new(
                ForgeQueryGraphTouchDescriptorDenialKind::ProgramComponentCountMismatch,
                "program and commands disagree",
            ),
        ),
        ForgeQueryRuntimeError::GraphObligationEffectTouchDescriptorMissing {
            effect_name: "effect.graph-obligation".to_string(),
        },
        ForgeQueryRuntimeError::GraphObligationIntentTouchDescriptorMissing {
            intent_name: "intent.graph-obligation".to_string(),
        },
        ForgeQueryRuntimeError::GraphMutationPolicyContextDenied {
            expected: crate::policy_basis::PolicyExecutionModeRequest::GraphMutation,
            actual: crate::policy_basis::PolicyExecutionModeRequest::CurrentRead,
            policy_tenant_admission_digest: "policy-admission:wrong-mode".to_string(),
        },
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
            snapshot_identity: crate::memory_workspace::admit_external_snapshot_label(
                "snapshot.stale",
            ),
        },
        ForgeQueryRuntimeError::JournalReplayDenied(replay_denial),
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

pub(super) fn status_value_touch() -> ForgeQueryAspectTouch {
    test_aspect_touch("status.value")
}
