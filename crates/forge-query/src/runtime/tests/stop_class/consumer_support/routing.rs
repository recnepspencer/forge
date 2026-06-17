use super::super::super::support::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in super::super) enum ConsumerStopRoute<'a> {
    MissingRuntimeComponent(ForgeQueryRuntimeMissingComponent),
    ExistingTruthAssertionDenied(ForgeQueryExistingTruthAssertionDenialKind),
    ExistingTruthProbeDenied(ForgeQueryExistingTruthProbeDenialKind),
    MutationBindingDenied(ForgeQueryExistingTruthBindingDenialKind),
    MutationContinuityDenied(ForgeQueryContinuityMutationDenialKind),
    GraphCompositionDenied(ForgeQueryGraphCompositionDenialKind),
    GraphCompositionDomainInvariantDenied {
        hook_family: &'a str,
        invariant_family: &'a str,
    },
    MutationNamingDenied(ForgeQueryNamingMutationDenialKind),
    MutationTargetReferenceDenied(ForgeQuerySymbolicTargetReferenceDenialKind),
    ReadCompositionDenied(ForgeQueryReadDenialKind),
    ReadCompositionDomainInvariantDenied {
        hook_family: &'a str,
        invariant_family: &'a str,
    },
    WorkspaceDenied,
    ProgramDenied,
    RuntimeLookupDenied(ForgeQueryRuntimeLookupFailureKind),
    MissingRuntimeArtifact(ForgeQueryRuntimeMissingArtifactKind),
    SharedReadStaleBasis,
    JournalReplayDenied(ForgeQueryJournalReplayDenialKind),
    RuntimeDeclarationDenied(ForgeQueryRuntimeDeclarationFailureKind),
    PreviewOperationEffectDenied(crate::ForgeQueryEvidenceIdentity),
    UnsupportedAuthorityRequirement(ForgeQueryAuthorityRequirement),
    ExistingTruthAssertionRequiresAuthorityLane(ForgeQueryAuthorityLane),
    IntentCommitDenied,
    IntentExecutionRoutingFailed(ForgeQueryRuntimeDeclarationFailureKind),
    EffectPolicyDenied,
    PreviewPromotionDenied(ForgeQueryPreviewPromotionDenialKind),
    FamilyAdmissionDenied {
        family: ForgeQueryRuntimeFacadeFamily,
        status: ForgeQueryRuntimeFamilySupportStatus,
        teaching_posture: Option<ForgeQueryRuntimeFamilyTeachingPosture>,
    },
    SessionLabelCollision(ForgeQueryAuthorityLane),
}

pub(in super::super) fn route_consumer_stop_class(
    error: &ForgeQueryRuntimeError,
) -> ConsumerStopRoute<'_> {
    match error.stop_class() {
        ForgeQueryStopClass::MissingRuntimeComponent { component } => {
            ConsumerStopRoute::MissingRuntimeComponent(component)
        }
        ForgeQueryStopClass::ExistingTruthAssertionDenied { denial } => {
            ConsumerStopRoute::ExistingTruthAssertionDenied(denial.kind())
        }
        ForgeQueryStopClass::ExistingTruthProbeDenied { denial } => {
            ConsumerStopRoute::ExistingTruthProbeDenied(denial.kind())
        }
        ForgeQueryStopClass::MutationBindingDenied { denial } => {
            ConsumerStopRoute::MutationBindingDenied(denial.kind())
        }
        ForgeQueryStopClass::MutationContinuityDenied { denial } => {
            ConsumerStopRoute::MutationContinuityDenied(denial.kind())
        }
        ForgeQueryStopClass::GraphCompositionDenied { denial } => {
            ConsumerStopRoute::GraphCompositionDenied(denial.kind())
        }
        ForgeQueryStopClass::GraphCompositionDomainInvariantDenied { denial } => {
            ConsumerStopRoute::GraphCompositionDomainInvariantDenied {
                hook_family: denial.hook_family(),
                invariant_family: denial.invariant_family(),
            }
        }
        ForgeQueryStopClass::MutationNamingDenied { denial } => {
            ConsumerStopRoute::MutationNamingDenied(denial.kind())
        }
        ForgeQueryStopClass::MutationTargetReferenceDenied { denial } => {
            ConsumerStopRoute::MutationTargetReferenceDenied(denial.kind())
        }
        ForgeQueryStopClass::ReadCompositionDenied { denial } => {
            ConsumerStopRoute::ReadCompositionDenied(denial.kind().clone())
        }
        ForgeQueryStopClass::ReadCompositionDomainInvariantDenied { denial } => {
            ConsumerStopRoute::ReadCompositionDomainInvariantDenied {
                hook_family: denial.hook_family(),
                invariant_family: denial.invariant_family(),
            }
        }
        ForgeQueryStopClass::Workspace { .. } => ConsumerStopRoute::WorkspaceDenied,
        ForgeQueryStopClass::Program { .. } => ConsumerStopRoute::ProgramDenied,
        ForgeQueryStopClass::RuntimeLookupFailed { kind, .. } => {
            ConsumerStopRoute::RuntimeLookupDenied(kind)
        }
        ForgeQueryStopClass::MissingRuntimeArtifact { kind, .. } => {
            ConsumerStopRoute::MissingRuntimeArtifact(kind)
        }
        ForgeQueryStopClass::SharedReadStaleBasis { .. } => ConsumerStopRoute::SharedReadStaleBasis,
        ForgeQueryStopClass::JournalReplayDenied { denial } => {
            ConsumerStopRoute::JournalReplayDenied(denial.kind())
        }
        ForgeQueryStopClass::RuntimeDeclarationFailed { kind, .. } => {
            ConsumerStopRoute::RuntimeDeclarationDenied(kind)
        }
        ForgeQueryStopClass::PreviewOperationEffectDenied { label, .. } => {
            ConsumerStopRoute::PreviewOperationEffectDenied(label.identity_digest().clone())
        }
        ForgeQueryStopClass::UnsupportedAuthorityRequirement { requirement } => {
            ConsumerStopRoute::UnsupportedAuthorityRequirement(requirement.clone())
        }
        ForgeQueryStopClass::ExistingTruthAssertionRequiresAuthorityLane { required_lane } => {
            ConsumerStopRoute::ExistingTruthAssertionRequiresAuthorityLane(required_lane)
        }
        ForgeQueryStopClass::IntentCommitDenied { .. } => ConsumerStopRoute::IntentCommitDenied,
        ForgeQueryStopClass::IntentExecutionRoutingFailed { source, .. } => {
            match source.stop_class() {
                ForgeQueryStopClass::RuntimeDeclarationFailed { kind, .. } => {
                    ConsumerStopRoute::IntentExecutionRoutingFailed(kind)
                }
                other => panic!("consumer source route expected declaration stop, got {other:?}"),
            }
        }
        ForgeQueryStopClass::EffectPolicyDenied { .. } => ConsumerStopRoute::EffectPolicyDenied,
        ForgeQueryStopClass::PreviewPromotionDenied { kind, .. } => {
            ConsumerStopRoute::PreviewPromotionDenied(kind)
        }
        ForgeQueryStopClass::FamilyAdmissionDenied {
            family,
            status,
            teaching_posture,
            ..
        } => ConsumerStopRoute::FamilyAdmissionDenied {
            family,
            status,
            teaching_posture,
        },
        ForgeQueryStopClass::SessionLabelCollision { authority_lane, .. } => {
            ConsumerStopRoute::SessionLabelCollision(authority_lane)
        }
    }
}
