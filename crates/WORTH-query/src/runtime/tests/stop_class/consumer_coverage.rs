use super::super::support::*;
use super::completeness_support::{existing_binding, status_value_touch};
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
    let status_touch = status_value_touch();
    let continuity_intent = WorthQueryContinuityMutationIntent::rebind_existing_target(
        crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_prior_authority(
            crate::runtime::WorthQueryContinuityPriorAuthorityLabel::new("authority:task-1")
                .expect("continuity prior authority label"),
        )
        .expect("continuity prior authority identity"),
        crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_successor_authority(
            crate::runtime::WorthQueryContinuitySuccessorAuthorityLabel::new("authority:task-1")
                .expect("continuity successor authority label"),
        )
        .expect("continuity successor authority identity"),
    )
    .expect("continuity intent should build");
    let target_reference =
        WorthQuerySymbolicTargetReference::new("task_symbol").expect("reference should build");
    let effect_policy_denial = WorthQueryEffectPolicy::DeriveOnly
        .admit(
            WorthQueryEffectAction::WriteIntent,
            WorthQueryAuthorityLane::AuthoritativeTruth,
        )
        .expect_err("derive-only write intent should deny");

    let manual_cases = [
        (
            WorthQueryRuntimeError::MissingBackend,
            ConsumerStopRoute::MissingRuntimeComponent(WorthQueryRuntimeMissingComponent::Backend),
        ),
        (
            WorthQueryRuntimeError::ExistingTruthAssertionDenied(
                WorthQueryExistingTruthAssertionDenial::new(
                    &binding,
                    WorthQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                    Some(status_touch.clone()),
                    Some("\"open\"".to_string()),
                    None,
                    "missing asserted aspect",
                ),
            ),
            ConsumerStopRoute::ExistingTruthAssertionDenied(
                WorthQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
            ),
        ),
        (
            WorthQueryRuntimeError::ExistingTruthProbeDenied(
                WorthQueryExistingTruthProbeDenial::new(
                    &binding,
                    WorthQueryExistingTruthProbeDenialKind::MissingProbedAspect,
                    Some(status_touch),
                    "missing probed aspect",
                ),
            ),
            ConsumerStopRoute::ExistingTruthProbeDenied(
                WorthQueryExistingTruthProbeDenialKind::MissingProbedAspect,
            ),
        ),
        (
            WorthQueryRuntimeError::MutationBindingDenied(
                WorthQueryExistingTruthBindingDenial::new(
                    &binding,
                    WorthQueryExistingTruthBindingDenialKind::CollectionMismatch,
                    "wrong collection",
                ),
            ),
            ConsumerStopRoute::MutationBindingDenied(
                WorthQueryExistingTruthBindingDenialKind::CollectionMismatch,
            ),
        ),
        (
            WorthQueryRuntimeError::MutationContinuityDenied(
                WorthQueryContinuityMutationDenial::new(
                    &continuity_intent,
                    Some(&binding),
                    WorthQueryContinuityMutationDenialKind::RequiresExistingTruthBinding,
                    "continuity requires binding",
                ),
            ),
            ConsumerStopRoute::MutationContinuityDenied(
                WorthQueryContinuityMutationDenialKind::RequiresExistingTruthBinding,
            ),
        ),
        (
            WorthQueryRuntimeError::GraphCompositionDenied(WorthQueryGraphCompositionDenial::new(
                WorthQueryGraphCompositionDenialKind::DuplicateSymbolDeclaration,
                Some("task_symbol".to_string()),
                Some(
                    crate::runtime::WorthQueryMutationTargetCollectionIdentity::new(
                        "graph-composition-test",
                        "Task",
                    ),
                ),
                "duplicate symbol",
            )),
            ConsumerStopRoute::GraphCompositionDenied(
                WorthQueryGraphCompositionDenialKind::DuplicateSymbolDeclaration,
            ),
        ),
        (
            WorthQueryRuntimeError::GraphCompositionDomainInvariantDenied(
                WorthQueryGraphCompositionDomainInvariantDenial::from_contributed(
                    "graph.family",
                    "graph domain invariant failed",
                    WorthQueryGraphCompositionDomainInvariantSummary::from_parts(
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
            WorthQueryRuntimeError::MutationNamingDenied(WorthQueryNamingMutationDenial::new(
                &WorthQueryNamingMutationIntent::attach_new_target(
                    crate::runtime::WorthQueryMutationAuthorityIdentity::naming_attachment(
                        crate::runtime::WorthQueryNamingAttachmentAuthorityLabel::new(
                            "attachment-1",
                        )
                        .expect("naming attachment authority label"),
                    )
                    .expect("naming attachment identity"),
                ),
                WorthQueryNamingMutationDenialKind::RequiresSameBatchTargetReference,
                "naming needs a same-batch target",
            )),
            ConsumerStopRoute::MutationNamingDenied(
                WorthQueryNamingMutationDenialKind::RequiresSameBatchTargetReference,
            ),
        ),
        (
            WorthQueryRuntimeError::MutationTargetReferenceDenied(
                WorthQuerySymbolicTargetReferenceDenial::new(
                    &target_reference,
                    WorthQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget,
                    "same-batch target unresolved",
                ),
            ),
            ConsumerStopRoute::MutationTargetReferenceDenied(
                WorthQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget,
            ),
        ),
        (
            WorthQueryRuntimeError::ReadCompositionDenied(WorthQueryReadDenial::new(
                WorthQueryReadDenialKind::ValidationDenied,
                "read validation failed",
            )),
            ConsumerStopRoute::ReadCompositionDenied(WorthQueryReadDenialKind::ValidationDenied),
        ),
        (
            WorthQueryRuntimeError::Workspace(WorthQueryWorkspaceError::new("workspace failed")),
            ConsumerStopRoute::WorkspaceDenied,
        ),
        (
            WorthQueryRuntimeError::Program(WorthQueryProgramError::new("program failed")),
            ConsumerStopRoute::ProgramDenied,
        ),
        (
            WorthQueryRuntimeError::UnknownProgram("missing.program".to_string()),
            ConsumerStopRoute::RuntimeLookupDenied(
                WorthQueryRuntimeLookupFailureKind::UnknownProgram,
            ),
        ),
        (
            WorthQueryRuntimeError::MissingLiveView("view.live".to_string()),
            ConsumerStopRoute::MissingRuntimeArtifact(
                WorthQueryRuntimeMissingArtifactKind::LiveView,
            ),
        ),
        (
            WorthQueryRuntimeError::SharedReadStaleBasis {
                snapshot_identity: crate::memory_workspace::admit_external_snapshot_label(
                    "shared-read-stale",
                ),
            },
            ConsumerStopRoute::SharedReadStaleBasis,
        ),
        (
            WorthQueryRuntimeError::ComputedDeclaration {
                view_name: "view.computed".to_string(),
                stage: "declare",
                message: "computed failed".to_string(),
            },
            ConsumerStopRoute::RuntimeDeclarationDenied(
                WorthQueryRuntimeDeclarationFailureKind::ComputedDeclaration,
            ),
        ),
        (
            WorthQueryRuntimeError::UnsupportedAuthorityRequirement(
                WorthQueryAuthorityRequirement::Merge,
            ),
            ConsumerStopRoute::UnsupportedAuthorityRequirement(
                WorthQueryAuthorityRequirement::Merge,
            ),
        ),
        (
            WorthQueryRuntimeError::ExistingTruthAssertionRequiresAuthorityLane {
                required_lane: WorthQueryAuthorityLane::AuthoritativeTruth,
            },
            ConsumerStopRoute::ExistingTruthAssertionRequiresAuthorityLane(
                WorthQueryAuthorityLane::AuthoritativeTruth,
            ),
        ),
        (
            WorthQueryRuntimeError::EffectPolicyDenied(effect_policy_denial),
            ConsumerStopRoute::EffectPolicyDenied,
        ),
        (
            WorthQueryRuntimeError::SessionLabelCollision {
                authority_lane: WorthQueryAuthorityLane::PreviewTruth,
                label: test_session_label("consumer-stop-class-collision"),
            },
            ConsumerStopRoute::SessionLabelCollision(WorthQueryAuthorityLane::PreviewTruth),
        ),
        (
            WorthQueryRuntimeError::UnsupportedFacadeFamily(WorthQueryRuntimeSupportDenial::new(
                WorthQueryRuntimeFacadeFamily::Temporal,
                WorthQueryRuntimeFamilySupportStatus::Unsupported,
                Some(WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
                "temporal gate is closed",
            )),
            ConsumerStopRoute::FamilyAdmissionDenied {
                family: WorthQueryRuntimeFacadeFamily::Temporal,
                status: WorthQueryRuntimeFamilySupportStatus::Unsupported,
                teaching_posture: Some(WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            },
        ),
    ];

    for (error, expected) in manual_cases {
        assert_eq!(route_consumer_stop_class(&error), expected);
    }
}

fn graph_domain_fixture_digest(
    role: &'static str,
) -> crate::evidence_identity::WorthQueryEvidenceIdentity {
    crate::evidence_identity::worth_query_evidence_identity(
        crate::evidence_identity::WorthQueryEvidenceScope::MutationEvidenceAggregateDigest,
    )
    .field_shape(
        crate::evidence_identity::WorthQueryEvidenceTag::new("role"),
        "consumer-graph-domain-fixture",
    )
    .field_shape(
        crate::evidence_identity::WorthQueryEvidenceTag::new("fixture"),
        role,
    )
    .seal()
}
