use super::super::support::*;
use super::completeness_support::existing_binding;
use super::consumer_support::routing::{route_consumer_stop_class, ConsumerStopRoute};

#[test]
fn consumer_router_handles_all_representative_runtime_stop_errors() {
    use super::completeness_support::representative_runtime_stop_errors;

    for error in representative_runtime_stop_errors() {
        let _route = route_consumer_stop_class(&error);
    }
}

#[test]
fn consumer_router_handles_manually_constructed_stop_classes_without_string_matching() {
    let binding = existing_binding();
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
                        graph_domain_fixture_digest("program"),
                        graph_domain_fixture_digest("breadth"),
                        "components=1".to_string(),
                    ),
                ),
            ),
            ConsumerStopRoute::GraphCompositionDomainInvariantDenied {
                hook_family: "domain_invariant_pack_hook",
                invariant_family: "graph.family",
            },
        ),
        (
            ForgeQueryRuntimeError::MutationNamingDenied(ForgeQueryNamingMutationDenial::new(
                &ForgeQueryNamingMutationIntent::attach_new_target(crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_attachment(crate::runtime::ForgeQueryNamingAttachmentAuthorityLabel::new("attachment-1").expect("naming attachment authority label")).expect("naming attachment identity")),
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
                snapshot_identity: crate::memory_workspace::ForgeQuerySnapshotIdentity::from_external_authority_label("shared-read-stale"),
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
            ForgeQueryRuntimeError::UnsupportedAuthorityRequirement(
                ForgeQueryAuthorityRequirement::Merge,
            ),
            ConsumerStopRoute::UnsupportedAuthorityRequirement(
                ForgeQueryAuthorityRequirement::Merge,
            ),
        ),
        (
            ForgeQueryRuntimeError::ExistingTruthAssertionRequiresAuthorityLane {
                required_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            },
            ConsumerStopRoute::ExistingTruthAssertionRequiresAuthorityLane(
                ForgeQueryAuthorityLane::AuthoritativeTruth,
            ),
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

fn graph_domain_fixture_digest(
    role: &'static str,
) -> crate::evidence_identity::ForgeQueryEvidenceIdentity {
    crate::evidence_identity::forge_query_evidence_identity(
        crate::evidence_identity::ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
    )
    .field_shape(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("role"),
        "consumer-graph-domain-fixture",
    )
    .field_shape(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("fixture"),
        role,
    )
    .seal()
}
