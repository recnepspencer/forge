use super::stop_class::{
    ForgeQueryRuntimeDeclarationFailureKind, ForgeQueryRuntimeLookupFailureKind,
    ForgeQueryRuntimeMissingArtifactKind, ForgeQueryRuntimeMissingComponent, ForgeQueryStopClass,
};
use super::*;

pub(super) fn classify_stop_class(error: &ForgeQueryRuntimeError) -> ForgeQueryStopClass<'_> {
    match error {
        ForgeQueryRuntimeError::MissingBackend => ForgeQueryStopClass::MissingRuntimeComponent {
            component: ForgeQueryRuntimeMissingComponent::Backend,
        },
        ForgeQueryRuntimeError::MissingRuntimeBridge => {
            ForgeQueryStopClass::MissingRuntimeComponent {
                component: ForgeQueryRuntimeMissingComponent::RuntimeBridge,
            }
        }
        ForgeQueryRuntimeError::MissingSchemaAdapter => {
            ForgeQueryStopClass::MissingRuntimeComponent {
                component: ForgeQueryRuntimeMissingComponent::SchemaAdapter,
            }
        }
        ForgeQueryRuntimeError::MissingSourceAdapter => {
            ForgeQueryStopClass::MissingRuntimeComponent {
                component: ForgeQueryRuntimeMissingComponent::SourceAdapter,
            }
        }
        ForgeQueryRuntimeError::MissingSnapshotIdentityAdapter => {
            ForgeQueryStopClass::MissingRuntimeComponent {
                component: ForgeQueryRuntimeMissingComponent::SnapshotIdentityAdapter,
            }
        }
        ForgeQueryRuntimeError::MissingWriteAuthority => {
            ForgeQueryStopClass::MissingRuntimeComponent {
                component: ForgeQueryRuntimeMissingComponent::WriteAuthority,
            }
        }
        ForgeQueryRuntimeError::MissingSignalSink => ForgeQueryStopClass::MissingRuntimeComponent {
            component: ForgeQueryRuntimeMissingComponent::SignalSink,
        },
        ForgeQueryRuntimeError::MissingSubscriptionActivation => {
            ForgeQueryStopClass::MissingRuntimeComponent {
                component: ForgeQueryRuntimeMissingComponent::SubscriptionActivation,
            }
        }
        ForgeQueryRuntimeError::MissingPreviewBasis => {
            ForgeQueryStopClass::MissingRuntimeComponent {
                component: ForgeQueryRuntimeMissingComponent::PreviewBasis,
            }
        }
        ForgeQueryRuntimeError::MissingInspectorEvidence => {
            ForgeQueryStopClass::MissingRuntimeComponent {
                component: ForgeQueryRuntimeMissingComponent::InspectorEvidence,
            }
        }
        ForgeQueryRuntimeError::MissingIntentAuthority => {
            ForgeQueryStopClass::MissingRuntimeComponent {
                component: ForgeQueryRuntimeMissingComponent::IntentAuthority,
            }
        }
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial) => {
            ForgeQueryStopClass::ExistingTruthAssertionDenied { denial }
        }
        ForgeQueryRuntimeError::ExistingTruthProbeDenied(denial) => {
            ForgeQueryStopClass::ExistingTruthProbeDenied { denial }
        }
        ForgeQueryRuntimeError::MutationBindingDenied(denial) => {
            ForgeQueryStopClass::MutationBindingDenied { denial }
        }
        ForgeQueryRuntimeError::MutationContinuityDenied(denial) => {
            ForgeQueryStopClass::MutationContinuityDenied { denial }
        }
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => {
            ForgeQueryStopClass::GraphCompositionDenied { denial }
        }
        ForgeQueryRuntimeError::GraphCompositionDomainInvariantDenied(denial) => {
            ForgeQueryStopClass::GraphCompositionDomainInvariantDenied { denial }
        }
        ForgeQueryRuntimeError::MutationNamingDenied(denial) => {
            ForgeQueryStopClass::MutationNamingDenied { denial }
        }
        ForgeQueryRuntimeError::MutationTargetReferenceDenied(denial) => {
            ForgeQueryStopClass::MutationTargetReferenceDenied { denial }
        }
        ForgeQueryRuntimeError::ReadCompositionDenied(denial) => {
            ForgeQueryStopClass::ReadCompositionDenied { denial }
        }
        ForgeQueryRuntimeError::ReadCompositionDomainInvariantDenied(denial) => {
            ForgeQueryStopClass::ReadCompositionDomainInvariantDenied { denial }
        }
        ForgeQueryRuntimeError::Workspace(inner) => ForgeQueryStopClass::Workspace { error: inner },
        ForgeQueryRuntimeError::Program(inner) => ForgeQueryStopClass::Program { error: inner },
        ForgeQueryRuntimeError::UnknownProgram(program_id) => {
            ForgeQueryStopClass::RuntimeLookupFailed {
                kind: ForgeQueryRuntimeLookupFailureKind::UnknownProgram,
                program_id,
                operation_id: None,
            }
        }
        ForgeQueryRuntimeError::UnknownOperation {
            program_id,
            operation_id,
        } => ForgeQueryStopClass::RuntimeLookupFailed {
            kind: ForgeQueryRuntimeLookupFailureKind::UnknownOperation,
            program_id,
            operation_id: Some(operation_id),
        },
        ForgeQueryRuntimeError::MissingLiveView(view_name) => {
            ForgeQueryStopClass::MissingRuntimeArtifact {
                kind: ForgeQueryRuntimeMissingArtifactKind::LiveView,
                name: view_name,
            }
        }
        ForgeQueryRuntimeError::MissingLiveSubscription(view_name) => {
            ForgeQueryStopClass::MissingRuntimeArtifact {
                kind: ForgeQueryRuntimeMissingArtifactKind::LiveSubscription,
                name: view_name,
            }
        }
        ForgeQueryRuntimeError::MissingDerivedView(view_name) => {
            ForgeQueryStopClass::MissingRuntimeArtifact {
                kind: ForgeQueryRuntimeMissingArtifactKind::DerivedView,
                name: view_name,
            }
        }
        ForgeQueryRuntimeError::SharedReadStaleBasis { snapshot_identity } => {
            ForgeQueryStopClass::SharedReadStaleBasis { snapshot_identity }
        }
        ForgeQueryRuntimeError::JournalReplayDenied(denial) => {
            ForgeQueryStopClass::JournalReplayDenied { denial }
        }
        ForgeQueryRuntimeError::MissingEffect(effect_name) => {
            ForgeQueryStopClass::MissingRuntimeArtifact {
                kind: ForgeQueryRuntimeMissingArtifactKind::Effect,
                name: effect_name,
            }
        }
        ForgeQueryRuntimeError::MissingPendingWriteIntent(effect_name) => {
            ForgeQueryStopClass::MissingRuntimeArtifact {
                kind: ForgeQueryRuntimeMissingArtifactKind::PendingWriteIntent,
                name: effect_name,
            }
        }
        ForgeQueryRuntimeError::RetainedRowDecode {
            view_name,
            stage,
            message,
        } => ForgeQueryStopClass::RuntimeDeclarationFailed {
            kind: ForgeQueryRuntimeDeclarationFailureKind::RetainedRowDecode,
            name: view_name,
            stage,
            message,
        },
        ForgeQueryRuntimeError::ComputedDeclaration {
            view_name,
            stage,
            message,
        } => ForgeQueryStopClass::RuntimeDeclarationFailed {
            kind: ForgeQueryRuntimeDeclarationFailureKind::ComputedDeclaration,
            name: view_name,
            stage,
            message,
        },
        ForgeQueryRuntimeError::EffectDeclaration {
            effect_name,
            stage,
            message,
        } => ForgeQueryStopClass::RuntimeDeclarationFailed {
            kind: ForgeQueryRuntimeDeclarationFailureKind::EffectDeclaration,
            name: effect_name,
            stage,
            message,
        },
        ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name,
            stage,
            message,
        } => ForgeQueryStopClass::RuntimeDeclarationFailed {
            kind: ForgeQueryRuntimeDeclarationFailureKind::LiveSubscriptionInstallation,
            name: view_name,
            stage,
            message,
        },
        ForgeQueryRuntimeError::SessionLabelCollision {
            authority_lane,
            label,
        } => ForgeQueryStopClass::SessionLabelCollision {
            authority_lane: *authority_lane,
            label,
        },
        ForgeQueryRuntimeError::UnsupportedAuthorityRequirement(requirement) => {
            ForgeQueryStopClass::UnsupportedAuthorityRequirement { requirement }
        }
        ForgeQueryRuntimeError::ExistingTruthAssertionRequiresAuthorityLane { required_lane } => {
            ForgeQueryStopClass::ExistingTruthAssertionRequiresAuthorityLane {
                required_lane: *required_lane,
            }
        }
        ForgeQueryRuntimeError::IntentCommitDenied {
            intent_name,
            stage,
            message,
            evidence,
        } => ForgeQueryStopClass::IntentCommitDenied {
            intent_name,
            stage,
            message,
            evidence,
        },
        ForgeQueryRuntimeError::IntentExecutionRoutingFailed {
            intent_name,
            stage,
            message,
            evidence,
            source,
        } => ForgeQueryStopClass::IntentExecutionRoutingFailed {
            intent_name,
            stage,
            message,
            evidence,
            source,
        },
        ForgeQueryRuntimeError::EffectPolicyDenied(denial) => {
            ForgeQueryStopClass::EffectPolicyDenied { denial: *denial }
        }
        ForgeQueryRuntimeError::PreviewPromotionStaleBasis(evidence) => {
            ForgeQueryStopClass::PreviewPromotionDenied {
                kind: ForgeQueryPreviewPromotionDenialKind::StaleBasis,
                evidence,
            }
        }
        ForgeQueryRuntimeError::PreviewPromotionAtomicBatchUnsupported(evidence) => {
            ForgeQueryStopClass::PreviewPromotionDenied {
                kind: ForgeQueryPreviewPromotionDenialKind::AtomicBatchUnsupported,
                evidence,
            }
        }
        ForgeQueryRuntimeError::PreviewPromotionRebindingRequired(evidence) => {
            ForgeQueryStopClass::PreviewPromotionDenied {
                kind: ForgeQueryPreviewPromotionDenialKind::RebindingRequired,
                evidence,
            }
        }
        ForgeQueryRuntimeError::PreviewPromotionWriteFailed { evidence } => {
            ForgeQueryStopClass::PreviewPromotionDenied {
                kind: ForgeQueryPreviewPromotionDenialKind::WriteFailed,
                evidence,
            }
        }
        ForgeQueryRuntimeError::InvariantRegistration { stage, message } => {
            ForgeQueryStopClass::RuntimeDeclarationFailed {
                kind: ForgeQueryRuntimeDeclarationFailureKind::InvariantRegistration,
                name: "runtime-invariant-registration",
                stage,
                message,
            }
        }
        ForgeQueryRuntimeError::PreviewOperationEffectDenied {
            label,
            stage,
            message,
        } => ForgeQueryStopClass::PreviewOperationEffectDenied {
            label,
            stage,
            message,
        },
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            ForgeQueryStopClass::FamilyAdmissionDenied {
                family: denial.family(),
                status: denial.status(),
                teaching_posture: denial.teaching_posture(),
                reason: denial.reason(),
            }
        }
    }
}
