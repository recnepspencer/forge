use super::*;

#[derive(Debug)]
#[non_exhaustive]
pub enum ForgeQueryRuntimeError {
    MissingBackend,
    MissingRuntimeBridge,
    MissingSchemaAdapter,
    MissingSourceAdapter,
    MissingWriteAuthority,
    MissingSignalSink,
    MissingSubscriptionActivation,
    MissingPreviewBasis,
    MissingInspectorEvidence,
    MissingIntentAuthority,
    ExistingTruthAssertionDenied(ForgeQueryExistingTruthAssertionDenial),
    ExistingTruthProbeDenied(ForgeQueryExistingTruthProbeDenial),
    MutationBindingDenied(ForgeQueryExistingTruthBindingDenial),
    MutationContinuityDenied(ForgeQueryContinuityMutationDenial),
    GraphCompositionDenied(ForgeQueryGraphCompositionDenial),
    GraphCompositionDomainInvariantDenied(ForgeQueryGraphCompositionDomainInvariantDenial),
    MutationNamingDenied(ForgeQueryNamingMutationDenial),
    MutationTargetReferenceDenied(ForgeQuerySymbolicTargetReferenceDenial),
    ReadCompositionDenied(ForgeQueryReadDenial),
    ReadCompositionDomainInvariantDenied(ForgeQueryReadDomainInvariantDenial),
    Workspace(ForgeQueryWorkspaceError),
    Program(ForgeQueryProgramError),
    UnknownProgram(String),
    UnknownOperation {
        program_id: String,
        operation_id: String,
    },
    MissingLiveView(String),
    MissingLiveSubscription(String),
    MissingDerivedView(String),
    MissingEffect(String),
    MissingPendingWriteIntent(String),
    ComputedDeclaration {
        view_name: String,
        stage: &'static str,
        message: String,
    },
    EffectDeclaration {
        effect_name: String,
        stage: &'static str,
        message: String,
    },
    LiveSubscriptionInstallation {
        view_name: String,
        stage: &'static str,
        message: String,
    },
    UnsupportedAuthority(String),
    IntentCommitDenied {
        intent_name: String,
        stage: &'static str,
        message: String,
        evidence: ForgeQueryIntentDenialEvidence,
    },
    IntentExecutionRoutingFailed {
        intent_name: String,
        stage: &'static str,
        message: String,
        evidence: ForgeQueryIntentExecutionFailureEvidence,
        source: Box<ForgeQueryRuntimeError>,
    },
    EffectPolicyDenied(ForgeQueryEffectPolicyDenial),
    PreviewPromotionStaleBasis(ForgeQueryPreviewPromotionDenialEvidence),
    PreviewPromotionAtomicBatchUnsupported(ForgeQueryPreviewPromotionDenialEvidence),
    PreviewPromotionWriteFailed {
        evidence: ForgeQueryPreviewPromotionDenialEvidence,
    },
    InvariantRegistration {
        stage: &'static str,
        message: String,
    },
    PreviewOperationEffectDenied {
        label: String,
        stage: &'static str,
        message: String,
    },
    UnsupportedFacadeFamily(ForgeQueryRuntimeSupportDenial),
}

impl std::fmt::Display for ForgeQueryRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingBackend => {
                write!(
                    f,
                    "forge query runtime builder requires a backend; prefer runtime_bridge(...)+adapters+build_backend_from_parts() for ordinary bridge-backed runtimes, and use backend(...) only for explicit internal test or scaffold seams"
                )
            }
            Self::MissingRuntimeBridge => write!(
                f,
                "forge query runtime backend parts require a RuntimeBridge"
            ),
            Self::MissingSchemaAdapter => write!(
                f,
                "forge query runtime backend parts require a schema adapter"
            ),
            Self::MissingSourceAdapter => write!(
                f,
                "forge query runtime backend parts require a source adapter"
            ),
            Self::MissingWriteAuthority => write!(
                f,
                "forge query runtime backend parts require a write authority adapter"
            ),
            Self::MissingSignalSink => write!(
                f,
                "forge query runtime backend parts require a signal sink adapter"
            ),
            Self::MissingSubscriptionActivation => write!(
                f,
                "forge query runtime backend parts require a subscription activation adapter"
            ),
            Self::MissingPreviewBasis => write!(
                f,
                "forge query runtime backend parts require a preview basis adapter"
            ),
            Self::MissingInspectorEvidence => write!(
                f,
                "forge query runtime backend parts require an inspector evidence adapter"
            ),
            Self::MissingIntentAuthority => write!(
                f,
                "forge query runtime backend parts that claim intent support require an intent authority adapter"
            ),
            Self::ExistingTruthAssertionDenied(denial) => write!(f, "{denial}"),
            Self::ExistingTruthProbeDenied(denial) => write!(f, "{denial}"),
            Self::MutationBindingDenied(denial) => write!(f, "{denial}"),
            Self::MutationContinuityDenied(denial) => write!(f, "{denial}"),
            Self::GraphCompositionDenied(denial) => write!(f, "{denial}"),
            Self::GraphCompositionDomainInvariantDenied(denial) => write!(f, "{denial}"),
            Self::MutationNamingDenied(denial) => write!(f, "{denial}"),
            Self::MutationTargetReferenceDenied(denial) => write!(f, "{denial}"),
            Self::ReadCompositionDenied(denial) => write!(f, "{denial}"),
            Self::ReadCompositionDomainInvariantDenied(denial) => write!(f, "{denial}"),
            Self::Workspace(error) => write!(f, "{error}"),
            Self::Program(error) => write!(f, "{error}"),
            Self::UnknownProgram(program) => write!(f, "unknown query program `{program}`"),
            Self::UnknownOperation {
                program_id,
                operation_id,
            } => write!(
                f,
                "unknown query operation `{operation_id}` in program `{program_id}`"
            ),
            Self::MissingLiveView(view) => write!(f, "unknown live view `{view}`"),
            Self::MissingLiveSubscription(view) => {
                write!(
                    f,
                    "live view `{view}` has no retained subscription installation"
                )
            }
            Self::MissingDerivedView(view) => write!(f, "unknown computed view `{view}`"),
            Self::MissingEffect(effect) => write!(f, "unknown effect `{effect}`"),
            Self::MissingPendingWriteIntent(effect) => {
                write!(f, "effect `{effect}` has no pending write intent delivery")
            }
            Self::ComputedDeclaration {
                view_name,
                stage,
                message,
            } => write!(
                f,
                "computed declaration `{view_name}` failed during {stage}: {message}"
            ),
            Self::EffectDeclaration {
                effect_name,
                stage,
                message,
            } => write!(
                f,
                "effect declaration `{effect_name}` failed during {stage}: {message}"
            ),
            Self::LiveSubscriptionInstallation {
                view_name,
                stage,
                message,
            } => write!(
                f,
                "live view `{view_name}` subscription installation failed during {stage}: {message}"
            ),
            Self::UnsupportedAuthority(authority) => {
                write!(
                    f,
                    "authority requirement `{authority}` is not admitted by this runtime"
                )
            }
            Self::IntentCommitDenied {
                intent_name,
                stage,
                message,
                evidence: _,
            } => write!(
                f,
                "intent `{intent_name}` commit failed during {stage}: {message}"
            ),
            Self::IntentExecutionRoutingFailed {
                intent_name,
                stage,
                message,
                evidence: _,
                source: _,
            } => write!(
                f,
                "intent `{intent_name}` execution failed during {stage}: {message}"
            ),
            Self::EffectPolicyDenied(denial) => write!(f, "{denial}"),
            Self::PreviewPromotionStaleBasis(evidence) => write!(
                f,
                "preview promotion `{}` failed during {}: {}",
                evidence.label(),
                evidence.kind().as_str(),
                evidence.reason()
            ),
            Self::PreviewPromotionAtomicBatchUnsupported(evidence) => write!(
                f,
                "preview promotion `{}` failed during {}: {}",
                evidence.label(),
                evidence.kind().as_str(),
                evidence.reason()
            ),
            Self::PreviewPromotionWriteFailed { evidence } => write!(
                f,
                "preview promotion `{}` failed during {}: {}",
                evidence.label(),
                evidence.kind().as_str(),
                evidence.reason()
            ),
            Self::InvariantRegistration { stage, message } => write!(
                f,
                "runtime invariant registration failed during {stage}: {message}"
            ),
            Self::PreviewOperationEffectDenied {
                label,
                stage,
                message,
            } => write!(
                f,
                "preview operation `{label}` failed during {stage}: {message}"
            ),
            Self::UnsupportedFacadeFamily(denial) => write!(f, "{denial}"),
        }
    }
}

impl std::error::Error for ForgeQueryRuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IntentExecutionRoutingFailed { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl From<ForgeQueryWorkspaceError> for ForgeQueryRuntimeError {
    fn from(value: ForgeQueryWorkspaceError) -> Self {
        Self::Workspace(value)
    }
}

impl From<ForgeQueryProgramError> for ForgeQueryRuntimeError {
    fn from(value: ForgeQueryProgramError) -> Self {
        Self::Program(value)
    }
}
