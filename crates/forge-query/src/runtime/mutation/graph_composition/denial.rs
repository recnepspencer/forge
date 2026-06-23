use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryGraphCompositionAdmissionTrace, ForgeQueryGraphCompositionAdmissionTraceStage,
    ForgeQueryMutationTargetCollectionIdentity, ForgeQueryRuntimeError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphCompositionDenialKind {
    EmptyComposition,
    DuplicateSymbolDeclaration,
    UnresolvedSymbolicReference,
    SymbolicCollectionMismatch,
    ExistingTargetBindingUnsupported,
    ExistingTargetResolvedTargetMissing,
    ExistingTargetCollectionMismatch,
    ExistingTargetRetargetUnsupported,
    ExistingTargetIdentityPreservationUnavailable,
    ExistingTargetSupersessionUnsupported,
    ExistingTargetBackendVerificationUnsupported,
    ExistingTargetClearAssertionUnsupported,
    ExistingTargetMissingAssertedAspect,
    ExistingTargetAssertedValueMismatch,
}

impl ForgeQueryGraphCompositionDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyComposition => "empty-composition",
            Self::DuplicateSymbolDeclaration => "duplicate-symbol-declaration",
            Self::UnresolvedSymbolicReference => "unresolved-symbolic-reference",
            Self::SymbolicCollectionMismatch => "symbolic-collection-mismatch",
            Self::ExistingTargetBindingUnsupported => "existing-target-binding-unsupported",
            Self::ExistingTargetResolvedTargetMissing => "existing-target-resolved-target-missing",
            Self::ExistingTargetCollectionMismatch => "existing-target-collection-mismatch",
            Self::ExistingTargetRetargetUnsupported => "existing-target-retarget-unsupported",
            Self::ExistingTargetIdentityPreservationUnavailable => {
                "existing-target-identity-preservation-unavailable"
            }
            Self::ExistingTargetSupersessionUnsupported => {
                "existing-target-supersession-unsupported"
            }
            Self::ExistingTargetBackendVerificationUnsupported => {
                "existing-target-backend-verification-unsupported"
            }
            Self::ExistingTargetClearAssertionUnsupported => {
                "existing-target-clear-assertion-unsupported"
            }
            Self::ExistingTargetMissingAssertedAspect => "existing-target-missing-asserted-aspect",
            Self::ExistingTargetAssertedValueMismatch => "existing-target-asserted-value-mismatch",
        }
    }
}

impl std::fmt::Display for ForgeQueryGraphCompositionDenialKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionDenial {
    kind: ForgeQueryGraphCompositionDenialKind,
    symbol: Option<String>,
    target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
    admission_trace: ForgeQueryGraphCompositionAdmissionTrace,
    message: String,
    denial_digest: String,
}

impl ForgeQueryGraphCompositionDenial {
    pub(crate) fn new(
        kind: ForgeQueryGraphCompositionDenialKind,
        symbol: Option<String>,
        target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let admission_trace = default_admission_trace(kind);
        let denial_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "graph-composition-denial",
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
                .optional_value(ForgeQueryEvidenceTag::new("symbol"), symbol.as_deref())
                .optional_value(
                    ForgeQueryEvidenceTag::new("target_collection"),
                    target_collection
                        .as_ref()
                        .map(ForgeQueryMutationTargetCollectionIdentity::as_str),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("trace"),
                    admission_trace.admission_trace_digest(),
                )
                .field_value(ForgeQueryEvidenceTag::new("message"), &message)
                .seal()
                .as_str()
                .to_string();
        Self {
            kind,
            symbol,
            target_collection,
            admission_trace,
            message,
            denial_digest,
        }
    }

    pub fn kind(&self) -> ForgeQueryGraphCompositionDenialKind {
        self.kind
    }

    pub fn symbol(&self) -> Option<&str> {
        self.symbol.as_deref()
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection
            .as_ref()
            .map(ForgeQueryMutationTargetCollectionIdentity::as_str)
    }

    pub fn target_collection_identity(
        &self,
    ) -> Option<&ForgeQueryMutationTargetCollectionIdentity> {
        self.target_collection.as_ref()
    }

    pub fn admission_trace(&self) -> &ForgeQueryGraphCompositionAdmissionTrace {
        &self.admission_trace
    }

    pub fn failure_stage(&self) -> ForgeQueryGraphCompositionAdmissionTraceStage {
        self.admission_trace.failure_stage()
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

impl std::fmt::Display for ForgeQueryGraphCompositionDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.symbol() {
            Some(symbol) => write!(
                f,
                "graph composition denied for {} on symbol `{}`: {}",
                self.kind, symbol, self.message
            ),
            None => write!(
                f,
                "graph composition denied for {}: {}",
                self.kind, self.message
            ),
        }
    }
}

pub(crate) fn graph_composition_error(
    kind: ForgeQueryGraphCompositionDenialKind,
    symbol: Option<String>,
    target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
    message: impl Into<String>,
) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::GraphCompositionDenied(ForgeQueryGraphCompositionDenial::new(
        kind,
        symbol,
        target_collection,
        message,
    ))
}

fn default_admission_trace(
    kind: ForgeQueryGraphCompositionDenialKind,
) -> ForgeQueryGraphCompositionAdmissionTrace {
    use ForgeQueryGraphCompositionAdmissionTraceStage as Stage;

    match kind {
        ForgeQueryGraphCompositionDenialKind::EmptyComposition => {
            ForgeQueryGraphCompositionAdmissionTrace::new(
                vec![Stage::ProgramParsed, Stage::DeniedBeforeExecution],
                Stage::ProgramParsed,
            )
        }
        ForgeQueryGraphCompositionDenialKind::DuplicateSymbolDeclaration => {
            ForgeQueryGraphCompositionAdmissionTrace::new(
                vec![
                    Stage::ProgramParsed,
                    Stage::SymbolsValidated,
                    Stage::DeniedBeforeExecution,
                ],
                Stage::SymbolsValidated,
            )
        }
        ForgeQueryGraphCompositionDenialKind::UnresolvedSymbolicReference
        | ForgeQueryGraphCompositionDenialKind::SymbolicCollectionMismatch
        | ForgeQueryGraphCompositionDenialKind::ExistingTargetBindingUnsupported
        | ForgeQueryGraphCompositionDenialKind::ExistingTargetResolvedTargetMissing
        | ForgeQueryGraphCompositionDenialKind::ExistingTargetCollectionMismatch
        | ForgeQueryGraphCompositionDenialKind::ExistingTargetRetargetUnsupported
        | ForgeQueryGraphCompositionDenialKind::ExistingTargetIdentityPreservationUnavailable
        | ForgeQueryGraphCompositionDenialKind::ExistingTargetSupersessionUnsupported => {
            ForgeQueryGraphCompositionAdmissionTrace::new(
                vec![
                    Stage::ProgramParsed,
                    Stage::SymbolsValidated,
                    Stage::LoweringValidated,
                    Stage::DeniedBeforeExecution,
                ],
                Stage::LoweringValidated,
            )
        }
        ForgeQueryGraphCompositionDenialKind::ExistingTargetBackendVerificationUnsupported
        | ForgeQueryGraphCompositionDenialKind::ExistingTargetClearAssertionUnsupported
        | ForgeQueryGraphCompositionDenialKind::ExistingTargetMissingAssertedAspect
        | ForgeQueryGraphCompositionDenialKind::ExistingTargetAssertedValueMismatch => {
            ForgeQueryGraphCompositionAdmissionTrace::new(
                vec![
                    Stage::ProgramParsed,
                    Stage::SymbolsValidated,
                    Stage::LoweringValidated,
                    Stage::VerificationSubstrateEvaluated,
                    Stage::DeniedBeforeExecution,
                ],
                Stage::VerificationSubstrateEvaluated,
            )
        }
    }
}
