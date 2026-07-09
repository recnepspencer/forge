use super::super::super::support::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in super::super) enum ConsumerStopRoute<'a> {
    MissingRuntimeComponent(WorthQueryRuntimeMissingComponent),
    ExistingTruthAssertionDenied(WorthQueryExistingTruthAssertionDenialKind),
    ExistingTruthProbeDenied(WorthQueryExistingTruthProbeDenialKind),
    MutationBindingDenied(WorthQueryExistingTruthBindingDenialKind),
    MutationContinuityDenied(WorthQueryContinuityMutationDenialKind),
    GraphObligationTouchDescriptorDenied(WorthQueryGraphTouchDescriptorDenialKind),
    GraphObligationEffectTouchDescriptorMissing,
    GraphObligationIntentTouchDescriptorMissing,
    GraphMutationPolicyContextDenied {
        expected: crate::policy_basis::PolicyExecutionModeRequest,
        actual: crate::policy_basis::PolicyExecutionModeRequest,
    },
    GraphMutationPolicyGateDenied {
        verdict: WorthQueryGraphMutationPolicyGateVerdict,
    },
    GraphObligationDenied {
        blocking_count: usize,
    },
    GraphCompositionDenied(WorthQueryGraphCompositionDenialKind),
    GraphCompositionDomainInvariantDenied {
        hook_family: &'a str,
        invariant_family: &'a str,
    },
    MutationNamingDenied(WorthQueryNamingMutationDenialKind),
    MutationTargetReferenceDenied(WorthQuerySymbolicTargetReferenceDenialKind),
    ReadCompositionDenied(WorthQueryReadDenialKind),
    ReadCompositionDomainInvariantDenied {
        hook_family: &'a str,
        invariant_family: &'a str,
    },
    WorkspaceDenied,
    ProgramDenied,
    RuntimeLookupDenied(WorthQueryRuntimeLookupFailureKind),
    MissingRuntimeArtifact(WorthQueryRuntimeMissingArtifactKind),
    SharedReadStaleBasis,
    JournalReplayDenied(WorthQueryJournalReplayDenialKind),
    RuntimeDeclarationDenied(WorthQueryRuntimeDeclarationFailureKind),
    PreviewOperationEffectDenied(crate::WorthQueryEvidenceIdentity),
    UnsupportedAuthorityRequirement(WorthQueryAuthorityRequirement),
    ExistingTruthAssertionRequiresAuthorityLane(WorthQueryAuthorityLane),
    IntentCommitDenied,
    IntentExecutionRoutingFailed(WorthQueryRuntimeDeclarationFailureKind),
    EffectPolicyDenied,
    PreviewPromotionDenied(WorthQueryPreviewPromotionDenialKind),
    FamilyAdmissionDenied {
        family: WorthQueryRuntimeFacadeFamily,
        status: WorthQueryRuntimeFamilySupportStatus,
        teaching_posture: Option<WorthQueryRuntimeFamilyTeachingPosture>,
    },
    SessionLabelCollision(WorthQueryAuthorityLane),
}

pub(in super::super) fn route_consumer_stop_class(
    error: &WorthQueryRuntimeError,
) -> ConsumerStopRoute<'_> {
    match error.stop_class() {
        WorthQueryStopClass::MissingRuntimeComponent { component } => {
            ConsumerStopRoute::MissingRuntimeComponent(component)
        }
        WorthQueryStopClass::ExistingTruthAssertionDenied { denial } => {
            ConsumerStopRoute::ExistingTruthAssertionDenied(denial.kind())
        }
        WorthQueryStopClass::ExistingTruthProbeDenied { denial } => {
            ConsumerStopRoute::ExistingTruthProbeDenied(denial.kind())
        }
        WorthQueryStopClass::MutationBindingDenied { denial } => {
            ConsumerStopRoute::MutationBindingDenied(denial.kind())
        }
        WorthQueryStopClass::MutationContinuityDenied { denial } => {
            ConsumerStopRoute::MutationContinuityDenied(denial.kind())
        }
        WorthQueryStopClass::GraphObligationTouchDescriptorDenied { denial } => {
            ConsumerStopRoute::GraphObligationTouchDescriptorDenied(denial.kind())
        }
        WorthQueryStopClass::GraphObligationEffectTouchDescriptorMissing { .. } => {
            ConsumerStopRoute::GraphObligationEffectTouchDescriptorMissing
        }
        WorthQueryStopClass::GraphObligationIntentTouchDescriptorMissing { .. } => {
            ConsumerStopRoute::GraphObligationIntentTouchDescriptorMissing
        }
        WorthQueryStopClass::GraphMutationPolicyContextDenied {
            expected, actual, ..
        } => ConsumerStopRoute::GraphMutationPolicyContextDenied { expected, actual },
        WorthQueryStopClass::GraphMutationPolicyGateDenied { evidence } => {
            ConsumerStopRoute::GraphMutationPolicyGateDenied {
                verdict: evidence.verdict(),
            }
        }
        WorthQueryStopClass::GraphObligationDenied { denial } => {
            ConsumerStopRoute::GraphObligationDenied {
                blocking_count: denial.blocking_count(),
            }
        }
        WorthQueryStopClass::GraphCompositionDenied { denial } => {
            ConsumerStopRoute::GraphCompositionDenied(denial.kind())
        }
        WorthQueryStopClass::GraphCompositionDomainInvariantDenied { denial } => {
            ConsumerStopRoute::GraphCompositionDomainInvariantDenied {
                hook_family: denial.hook_family(),
                invariant_family: denial.invariant_family(),
            }
        }
        WorthQueryStopClass::MutationNamingDenied { denial } => {
            ConsumerStopRoute::MutationNamingDenied(denial.kind())
        }
        WorthQueryStopClass::MutationTargetReferenceDenied { denial } => {
            ConsumerStopRoute::MutationTargetReferenceDenied(denial.kind())
        }
        WorthQueryStopClass::ReadCompositionDenied { denial } => {
            ConsumerStopRoute::ReadCompositionDenied(denial.kind().clone())
        }
        WorthQueryStopClass::ReadCompositionDomainInvariantDenied { denial } => {
            ConsumerStopRoute::ReadCompositionDomainInvariantDenied {
                hook_family: denial.hook_family(),
                invariant_family: denial.invariant_family(),
            }
        }
        WorthQueryStopClass::Workspace { .. } => ConsumerStopRoute::WorkspaceDenied,
        WorthQueryStopClass::Program { .. } => ConsumerStopRoute::ProgramDenied,
        WorthQueryStopClass::RuntimeLookupFailed { kind, .. } => {
            ConsumerStopRoute::RuntimeLookupDenied(kind)
        }
        WorthQueryStopClass::MissingRuntimeArtifact { kind, .. } => {
            ConsumerStopRoute::MissingRuntimeArtifact(kind)
        }
        WorthQueryStopClass::SharedReadStaleBasis { .. } => ConsumerStopRoute::SharedReadStaleBasis,
        WorthQueryStopClass::JournalReplayDenied { denial } => {
            ConsumerStopRoute::JournalReplayDenied(denial.kind())
        }
        WorthQueryStopClass::RuntimeDeclarationFailed { kind, .. } => {
            ConsumerStopRoute::RuntimeDeclarationDenied(kind)
        }
        WorthQueryStopClass::PreviewOperationEffectDenied { label, .. } => {
            ConsumerStopRoute::PreviewOperationEffectDenied(label.identity_digest().clone())
        }
        WorthQueryStopClass::UnsupportedAuthorityRequirement { requirement } => {
            ConsumerStopRoute::UnsupportedAuthorityRequirement(requirement.clone())
        }
        WorthQueryStopClass::ExistingTruthAssertionRequiresAuthorityLane { required_lane } => {
            ConsumerStopRoute::ExistingTruthAssertionRequiresAuthorityLane(required_lane)
        }
        WorthQueryStopClass::IntentCommitDenied { .. } => ConsumerStopRoute::IntentCommitDenied,
        WorthQueryStopClass::IntentExecutionRoutingFailed { source, .. } => {
            match source.stop_class() {
                WorthQueryStopClass::RuntimeDeclarationFailed { kind, .. } => {
                    ConsumerStopRoute::IntentExecutionRoutingFailed(kind)
                }
                other => panic!("consumer source route expected declaration stop, got {other:?}"),
            }
        }
        WorthQueryStopClass::EffectPolicyDenied { .. } => ConsumerStopRoute::EffectPolicyDenied,
        WorthQueryStopClass::PreviewPromotionDenied { kind, .. } => {
            ConsumerStopRoute::PreviewPromotionDenied(kind)
        }
        WorthQueryStopClass::FamilyAdmissionDenied {
            family,
            status,
            teaching_posture,
            ..
        } => ConsumerStopRoute::FamilyAdmissionDenied {
            family,
            status,
            teaching_posture,
        },
        WorthQueryStopClass::SessionLabelCollision { authority_lane, .. } => {
            ConsumerStopRoute::SessionLabelCollision(authority_lane)
        }
    }
}
