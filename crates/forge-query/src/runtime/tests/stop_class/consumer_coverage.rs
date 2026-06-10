use super::super::support::*;
use super::completeness_support::existing_binding;
use super::consumer_support::routing::{route_consumer_stop_class, ConsumerStopRoute};

#[test]
fn consumer_router_handles_manually_constructed_stop_classes_without_string_matching() {
    let binding = existing_binding();
    let continuity_intent =
        ForgeQueryContinuityMutationIntent::rebind_existing_target("authority:task-1", "Task:2")
            .expect("continuity intent should build");
    let target_reference =
        ForgeQuerySymbolicTargetReference::new("task_symbol").expect("reference should build");
    let effect_policy_denial = ForgeQueryEffectPolicy::DeriveOnly
        .admit(
            ForgeQueryEffectAction::WriteIntent,
            ForgeQueryAuthorityLane::AuthoritativeTruth,
        )
        .expect_err("derive-only write intent should deny");

    let manual_cases = [
        (
            ForgeQueryRuntimeError::MissingBackend,
            ConsumerStopRoute::MissingRuntimeComponent(ForgeQueryRuntimeMissingComponent::Backend),
        ),
        (
            ForgeQueryRuntimeError::ExistingTruthAssertionDenied(
                ForgeQueryExistingTruthAssertionDenial::new(
                    &binding,
                    ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                    Some("status.value".to_string()),
                    Some("\"open\"".to_string()),
                    None,
                    "missing asserted aspect",
                ),
            ),
            ConsumerStopRoute::ExistingTruthAssertionDenied(
                ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
            ),
        ),
        (
            ForgeQueryRuntimeError::ExistingTruthProbeDenied(
                ForgeQueryExistingTruthProbeDenial::new(
                    &binding,
                    ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
                    Some("status.value".to_string()),
                    "missing probed aspect",
                ),
            ),
            ConsumerStopRoute::ExistingTruthProbeDenied(
                ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
            ),
        ),
        (
            ForgeQueryRuntimeError::MutationBindingDenied(
                ForgeQueryExistingTruthBindingDenial::new(
                    &binding,
                    ForgeQueryExistingTruthBindingDenialKind::CollectionMismatch,
                    "wrong collection",
                ),
            ),
            ConsumerStopRoute::MutationBindingDenied(
                ForgeQueryExistingTruthBindingDenialKind::CollectionMismatch,
            ),
        ),
        (
            ForgeQueryRuntimeError::MutationContinuityDenied(
                ForgeQueryContinuityMutationDenial::new(
                    &continuity_intent,
                    Some(&binding),
                    ForgeQueryContinuityMutationDenialKind::RequiresExistingTruthBinding,
                    "continuity requires binding",
                ),
            ),
            ConsumerStopRoute::MutationContinuityDenied(
                ForgeQueryContinuityMutationDenialKind::RequiresExistingTruthBinding,
            ),
        ),
        (
            ForgeQueryRuntimeError::GraphCompositionDenied(ForgeQueryGraphCompositionDenial::new(
                ForgeQueryGraphCompositionDenialKind::DuplicateSymbolDeclaration,
                Some("task_symbol".to_string()),
                Some("Task".to_string()),
                "duplicate symbol",
            )),
            ConsumerStopRoute::GraphCompositionDenied(
                ForgeQueryGraphCompositionDenialKind::DuplicateSymbolDeclaration,
            ),
        ),
        (
            ForgeQueryRuntimeError::GraphCompositionDomainInvariantDenied(
                ForgeQueryGraphCompositionDomainInvariantDenial::from_contributed(
                    "graph.family",
                    "graph domain invariant failed",
                    ForgeQueryGraphCompositionDomainInvariantSummary::from_parts(
                        vec!["Task".to_string()],
                        vec!["task_symbol".to_string()],
                        vec!["same_batch_entity_relation_identity_edges".to_string()],
                        vec!["mixed_existing_target_followup_mutation".to_string()],
                        "program-digest".to_string(),
                        "breadth-digest".to_string(),
                        "components=1".to_string(),
                    ),
                ),
            ),
            ConsumerStopRoute::GraphCompositionDomainInvariantDenied,
        ),
        (
            ForgeQueryRuntimeError::MutationNamingDenied(ForgeQueryNamingMutationDenial::new(
                &ForgeQueryNamingMutationIntent::attach_new_target("attachment-1").expect("intent"),
                ForgeQueryNamingMutationDenialKind::RequiresSameBatchTargetReference,
                "naming needs a same-batch target",
            )),
            ConsumerStopRoute::MutationNamingDenied(
                ForgeQueryNamingMutationDenialKind::RequiresSameBatchTargetReference,
            ),
        ),
        (
            ForgeQueryRuntimeError::MutationTargetReferenceDenied(
                ForgeQuerySymbolicTargetReferenceDenial::new(
                    &target_reference,
                    ForgeQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget,
                    "same-batch target unresolved",
                ),
            ),
            ConsumerStopRoute::MutationTargetReferenceDenied(
                ForgeQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget,
            ),
        ),
        (
            ForgeQueryRuntimeError::ReadCompositionDenied(ForgeQueryReadDenial::new(
                ForgeQueryReadDenialKind::ValidationDenied,
                "read validation failed",
            )),
            ConsumerStopRoute::ReadCompositionDenied(ForgeQueryReadDenialKind::ValidationDenied),
        ),
        (
            ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new("workspace failed")),
            ConsumerStopRoute::WorkspaceDenied,
        ),
        (
            ForgeQueryRuntimeError::Program(ForgeQueryProgramError::new("program failed")),
            ConsumerStopRoute::ProgramDenied,
        ),
        (
            ForgeQueryRuntimeError::UnknownProgram("missing.program".to_string()),
            ConsumerStopRoute::RuntimeLookupDenied(
                ForgeQueryRuntimeLookupFailureKind::UnknownProgram,
            ),
        ),
        (
            ForgeQueryRuntimeError::MissingLiveView("view.live".to_string()),
            ConsumerStopRoute::MissingRuntimeArtifact(
                ForgeQueryRuntimeMissingArtifactKind::LiveView,
            ),
        ),
        (
            ForgeQueryRuntimeError::SharedReadStaleBasis {
                snapshot_token: "shared-read-stale".to_string(),
            },
            ConsumerStopRoute::SharedReadStaleBasis,
        ),
        (
            ForgeQueryRuntimeError::ComputedDeclaration {
                view_name: "view.computed".to_string(),
                stage: "declare",
                message: "computed failed".to_string(),
            },
            ConsumerStopRoute::RuntimeDeclarationDenied(
                ForgeQueryRuntimeDeclarationFailureKind::ComputedDeclaration,
            ),
        ),
        (
            ForgeQueryRuntimeError::UnsupportedAuthority("authoritative-lane".to_string()),
            ConsumerStopRoute::UnsupportedAuthority,
        ),
        (
            ForgeQueryRuntimeError::EffectPolicyDenied(effect_policy_denial),
            ConsumerStopRoute::EffectPolicyDenied,
        ),
        (
            ForgeQueryRuntimeError::SessionLabelCollision {
                authority_lane: ForgeQueryAuthorityLane::PreviewTruth,
                label: test_session_label("consumer-stop-class-collision"),
            },
            ConsumerStopRoute::SessionLabelCollision(ForgeQueryAuthorityLane::PreviewTruth),
        ),
        (
            ForgeQueryRuntimeError::UnsupportedFacadeFamily(ForgeQueryRuntimeSupportDenial::new(
                ForgeQueryRuntimeFacadeFamily::Temporal,
                ForgeQueryRuntimeFamilySupportStatus::Unsupported,
                Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
                "temporal gate is closed",
            )),
            ConsumerStopRoute::FamilyAdmissionDenied {
                family: ForgeQueryRuntimeFacadeFamily::Temporal,
                status: ForgeQueryRuntimeFamilySupportStatus::Unsupported,
                teaching_posture: Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            },
        ),
    ];

    for (error, expected) in manual_cases {
        assert_eq!(route_consumer_stop_class(&error), expected);
    }
}
