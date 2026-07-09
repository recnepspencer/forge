use worth_query::facade::runtime::WORTHQueryGraphCompositionDomainInvariantDenial;

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::catalog::{HadwigerResearchInvariantCatalog, ResearchGraphInvariantCounters};
use super::requests::ResearchGraphInvariantBoundarySource;
use super::violations::ResearchGraphInvariantViolation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchGraphInvariantDenial {
    core: HadwigerArtifactCore,
    invariant_family: String,
    lower_runtime_source_kind: &'static str,
    lower_runtime_source_digest: String,
    lower_runtime_envelope_digest: String,
    query_denial: WORTHQueryGraphCompositionDomainInvariantDenial,
    counters: ResearchGraphInvariantCounters,
}

impl ResearchGraphInvariantDenial {
    pub(crate) fn new(
        catalog: &HadwigerResearchInvariantCatalog,
        violation: &ResearchGraphInvariantViolation,
        boundary_source: &ResearchGraphInvariantBoundarySource,
        query_denial: WORTHQueryGraphCompositionDomainInvariantDenial,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let counters = ResearchGraphInvariantCounters::new(
            catalog.rules().len(),
            1,
            1,
            catalog.rules().len(),
            violation.counters().breadth_inspected(),
        );
        let invariant_family = query_denial.invariant_family().to_string();
        let lower_runtime_source_digest = boundary_source.source_digest().to_string();
        let lower_runtime_envelope_digest = boundary_source.envelope_digest().to_string();
        let core = artifact_core(
            HadwigerArtifactKind::ResearchGraphInvariantDenial,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "research_graph_invariant_denial".to_string(),
            },
            vec![catalog.reference(), violation.reference()],
            vec![
                HadwigerArtifactPayloadEntry::text("schema", "WORTH.hadwiger.denial.v1"),
                HadwigerArtifactPayloadEntry::text(
                    "catalog",
                    catalog.artifact_digest().stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text("invariant_family", invariant_family.clone()),
                HadwigerArtifactPayloadEntry::text(
                    "violation",
                    violation.reference().stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "violation_digest",
                    violation.artifact_digest().stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "lower_runtime_source_kind",
                    boundary_source.source_kind(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "lower_runtime_source_digest",
                    lower_runtime_source_digest.clone(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "lower_runtime_envelope_digest",
                    lower_runtime_envelope_digest.clone(),
                ),
                HadwigerArtifactPayloadEntry::text("query_denial", query_denial.denial_digest()),
                HadwigerArtifactPayloadEntry::text("counters", counters.stable_token()),
            ],
        )?;
        Ok(Self {
            core,
            invariant_family,
            lower_runtime_source_kind: boundary_source.source_kind(),
            lower_runtime_source_digest,
            lower_runtime_envelope_digest,
            query_denial,
            counters,
        })
    }

    pub fn invariant_family(&self) -> &str {
        &self.invariant_family
    }

    pub fn query_denial(&self) -> Option<&WORTHQueryGraphCompositionDomainInvariantDenial> {
        Some(&self.query_denial)
    }

    pub fn lower_runtime_source_kind(&self) -> &'static str {
        self.lower_runtime_source_kind
    }

    pub fn lower_runtime_source_digest(&self) -> &str {
        &self.lower_runtime_source_digest
    }

    pub fn lower_runtime_envelope_digest(&self) -> &str {
        &self.lower_runtime_envelope_digest
    }

    pub fn counters(&self) -> &ResearchGraphInvariantCounters {
        &self.counters
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(ResearchGraphInvariantDenial, core);
