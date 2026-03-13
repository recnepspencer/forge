use crate::data::output::{ChangedRegion, MemoizedResultOrigin, OutputIdentity};
use crate::data::trace::CausalityMetadata;
use crate::logic::prepared::PreparedKeyedContext;

use super::{DiagnosticEnvelope, OperationalEffect};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EvaluationEffect {
    pub operational: OperationalEffect,
    pub diagnostics: Option<DiagnosticEnvelope>,
    pub runtime_metadata: EffectRuntimeMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct EffectRuntimeMetadata {
    pub recomputed: bool,
    pub keyed_context: Option<PreparedKeyedContext>,
    pub causality: Option<CausalityMetadata>,
}

impl EvaluationEffect {
    pub(crate) fn output_identity(&self) -> Option<&OutputIdentity> {
        self.diagnostics
            .as_ref()
            .and_then(DiagnosticEnvelope::output_identity)
    }

    pub(crate) fn continuity_token(&self) -> Option<&crate::data::output::ArtifactContinuityToken> {
        self.diagnostics
            .as_ref()
            .and_then(DiagnosticEnvelope::continuity_token)
    }

    pub(crate) fn changed_regions(&self) -> &[ChangedRegion] {
        self.diagnostics
            .as_ref()
            .map(DiagnosticEnvelope::changed_regions)
            .unwrap_or(&[])
    }

    pub(crate) fn labels(&self) -> &[String] {
        self.diagnostics
            .as_ref()
            .map(DiagnosticEnvelope::labels)
            .unwrap_or(&[])
    }

    pub(crate) fn memoized_origin(&self) -> MemoizedResultOrigin {
        self.diagnostics
            .as_ref()
            .map(DiagnosticEnvelope::memoized_origin)
            .unwrap_or(MemoizedResultOrigin::DirectCompute)
    }

    pub(crate) fn recomputed(&self) -> bool {
        self.runtime_metadata.recomputed
    }

    pub(crate) fn keyed_context(&self) -> Option<&PreparedKeyedContext> {
        self.runtime_metadata.keyed_context.as_ref()
    }

    pub(crate) fn take_causality(&mut self) -> Option<CausalityMetadata> {
        self.runtime_metadata.causality.take()
    }
}
