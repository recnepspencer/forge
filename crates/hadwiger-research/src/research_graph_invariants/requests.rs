use forge_query::facade::runtime::{
    ForgeQueryLowerRuntimeBoundaryEnvelope, ForgeQueryLowerRuntimeBoundaryEnvelopeSource,
};

use crate::discovery_loop::{ExperimentBatch, ResearchEvidenceCorpus};

use super::catalog::HadwigerResearchInvariantCatalog;
use super::violations::ResearchGraphInvariantViolation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchGraphInvariantCheckRequest {
    catalog: HadwigerResearchInvariantCatalog,
    experiment_batch: ExperimentBatch,
    corpus: Option<ResearchEvidenceCorpus>,
}

impl ResearchGraphInvariantCheckRequest {
    pub fn for_experiment_batch(
        catalog: &HadwigerResearchInvariantCatalog,
        experiment_batch: ExperimentBatch,
    ) -> Self {
        Self {
            catalog: catalog.clone(),
            experiment_batch,
            corpus: None,
        }
    }

    pub fn with_corpus(mut self, corpus: &ResearchEvidenceCorpus) -> Self {
        self.corpus = Some(corpus.clone());
        self
    }

    pub(crate) fn catalog(&self) -> &HadwigerResearchInvariantCatalog {
        &self.catalog
    }

    pub(crate) fn experiment_batch(&self) -> &ExperimentBatch {
        &self.experiment_batch
    }

    pub(crate) fn corpus(&self) -> Option<&ResearchEvidenceCorpus> {
        self.corpus.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchGraphInvariantDenialRequest {
    catalog: HadwigerResearchInvariantCatalog,
    violation: ResearchGraphInvariantViolation,
    lower_runtime_boundary_source: Option<ResearchGraphInvariantBoundarySource>,
}

impl ResearchGraphInvariantDenialRequest {
    pub fn from_violation(
        catalog: &HadwigerResearchInvariantCatalog,
        violation: &ResearchGraphInvariantViolation,
    ) -> Self {
        Self {
            catalog: catalog.clone(),
            violation: violation.clone(),
            lower_runtime_boundary_source: None,
        }
    }

    pub fn for_lower_runtime_boundary_envelope(
        self,
        envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        self.for_lower_runtime_boundary_source(envelope)
    }

    pub fn for_lower_runtime_boundary_source<S>(mut self, source: &S) -> Self
    where
        S: ForgeQueryLowerRuntimeBoundaryEnvelopeSource + ?Sized,
    {
        self.lower_runtime_boundary_source =
            Some(ResearchGraphInvariantBoundarySource::from_source(source));
        self
    }

    pub(crate) fn catalog(&self) -> &HadwigerResearchInvariantCatalog {
        &self.catalog
    }

    pub(crate) fn violation(&self) -> &ResearchGraphInvariantViolation {
        &self.violation
    }

    pub(crate) fn lower_runtime_boundary_source(
        &self,
    ) -> Option<&ResearchGraphInvariantBoundarySource> {
        self.lower_runtime_boundary_source.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResearchGraphInvariantBoundarySource {
    envelope: ForgeQueryLowerRuntimeBoundaryEnvelope,
    source_kind: &'static str,
    source_digest: String,
}

impl ResearchGraphInvariantBoundarySource {
    fn from_source<S>(source: &S) -> Self
    where
        S: ForgeQueryLowerRuntimeBoundaryEnvelopeSource + ?Sized,
    {
        Self {
            envelope: source.lower_runtime_boundary_envelope().clone(),
            source_kind: source.lower_runtime_boundary_source_kind(),
            source_digest: source
                .lower_runtime_boundary_source_identity()
                .as_str()
                .to_string(),
        }
    }

    pub(crate) fn envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope {
        &self.envelope
    }

    pub(crate) fn source_kind(&self) -> &'static str {
        self.source_kind
    }

    pub(crate) fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub(crate) fn envelope_digest(&self) -> &str {
        self.envelope.envelope_identity().as_str()
    }
}
