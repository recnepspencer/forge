#[path = "geometry_recovery.rs"]
mod geometry_recovery;

#[path = "rejection_facts.rs"]
mod rejection_facts;

#[path = "outcome_rejection.rs"]
mod outcome_rejection;

use super::digest::digest_owned_parts;
use super::intent::PrimitiveConstructionIntent;
use super::request::PrimitiveConstructionFamily;
use super::result::{
    prepare_primitive_construction_executed_result, prepare_primitive_construction_result,
    ExecutedPrimitiveConstructionGraphAuthorityResult, PreparedPrimitiveConstructionHandoffResult,
};
use topology::facade::{
    TopologyConstructionQueryFactProvenance, TopologyConstructionQueryReadSurface,
};
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass, PrimitiveSupportNormalClass,
};

pub(crate) use geometry_recovery::GeometryRecoveryActionFactReceipt;
#[cfg(test)]
pub(crate) use geometry_recovery::PrimitiveConstructionRecoveryAction;
#[cfg(test)]
pub(crate) use geometry_recovery::{
    GeometryRecoveryAction, GeometryRecoverySourcePosture, GeometryRecoveryTargetScope,
};
pub(crate) use outcome_rejection::PrimitiveConstructionRejectedOutcome;
#[cfg(test)]
pub(crate) use outcome_rejection::{
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
#[cfg(test)]
pub(crate) use rejection_facts::prepare_primitive_construction_rejected_facts;
#[allow(unused_imports)]
pub(crate) use rejection_facts::PrimitiveConstructionRejectedFacts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionAcceptedOutcome {
    family: PrimitiveConstructionFamily,
    topology_birth_class: String,
    canonical_artifact_digest: String,
    result_digest: String,
    birth_truth_digest: String,
    birth_completeness_digest: String,
    topology_fact_digest: String,
    projection_receipt_digest: String,
    topology_compose_evidence_digest: Option<String>,
    graph_obligation_envelope_digest: Option<String>,
    graph_obligation_selected_count: usize,
    read_surface: TopologyConstructionQueryReadSurface,
    fact_provenance: TopologyConstructionQueryFactProvenance,
    realization_strategy: PrimitiveRealizationStrategy,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: PrimitiveStabilityClass,
    feature_conditioning_class: PrimitiveFeatureConditioningClass,
    support_normal_class: PrimitiveSupportNormalClass,
    normalization_disposition: PrimitiveNormalizationDisposition,
    outcome_digest: String,
}

impl PrimitiveConstructionAcceptedOutcome {
    fn from_handoff_result(
        family: PrimitiveConstructionFamily,
        prepared: &PreparedPrimitiveConstructionHandoffResult,
    ) -> Self {
        let outcome_digest = digest_owned_parts(&[
            family.as_str().to_string(),
            prepared.topology_birth_class().to_string(),
            prepared.artifact_digest().to_string(),
            prepared.result_digest().to_string(),
            prepared.realization_strategy().as_str().to_string(),
            prepared
                .attempted_realization_strategies()
                .iter()
                .map(|strategy: &PrimitiveRealizationStrategy| strategy.as_str())
                .collect::<Vec<_>>()
                .join("->"),
            prepared.stability_class().as_str().to_string(),
            prepared.feature_conditioning_class().as_str().to_string(),
            prepared.support_normal_class().as_str().to_string(),
            prepared.normalization_disposition().as_str().to_string(),
            "handoff-only-no-compose-evidence".to_string(),
            "0".to_string(),
        ]);
        Self {
            family,
            topology_birth_class: prepared.topology_birth_class().to_string(),
            canonical_artifact_digest: prepared.artifact_digest().to_string(),
            result_digest: prepared.result_digest().to_string(),
            birth_truth_digest: prepared.birth_truth_digest().to_string(),
            birth_completeness_digest: prepared.birth_completeness_digest().to_string(),
            topology_fact_digest: prepared.topology_fact_digest().to_string(),
            projection_receipt_digest: prepared.projection_receipt_digest().to_string(),
            topology_compose_evidence_digest: None,
            graph_obligation_envelope_digest: None,
            graph_obligation_selected_count: 0,
            read_surface: prepared.read_surface(),
            fact_provenance: prepared.fact_provenance(),
            realization_strategy: prepared.realization_strategy(),
            attempted_realization_strategies: prepared.attempted_realization_strategies().to_vec(),
            stability_class: prepared.stability_class(),
            feature_conditioning_class: prepared.feature_conditioning_class(),
            support_normal_class: prepared.support_normal_class(),
            normalization_disposition: prepared.normalization_disposition(),
            outcome_digest,
        }
    }

    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub(crate) fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub(crate) fn canonical_artifact_digest(&self) -> &str {
        &self.canonical_artifact_digest
    }

    pub(crate) fn outcome_digest(&self) -> &str {
        &self.outcome_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ExecutedPrimitiveConstructionGraphAuthorityOutcome {
    family: PrimitiveConstructionFamily,
    canonical_artifact_digest: String,
    result_digest: String,
    topology_compose_evidence_digest: String,
    graph_obligation_envelope_digest: String,
    graph_obligation_selected_count: usize,
    outcome_digest: String,
}

impl ExecutedPrimitiveConstructionGraphAuthorityOutcome {
    fn from_executed_result(
        family: PrimitiveConstructionFamily,
        prepared: &ExecutedPrimitiveConstructionGraphAuthorityResult,
    ) -> Self {
        let evidence = prepared.topology_compose_evidence();
        let outcome_digest = digest_owned_parts(&[
            family.as_str().to_string(),
            prepared.topology_birth_class().to_string(),
            prepared.artifact_digest().to_string(),
            prepared.result_digest().to_string(),
            prepared.realization_strategy().as_str().to_string(),
            prepared
                .attempted_realization_strategies()
                .iter()
                .map(|strategy: &PrimitiveRealizationStrategy| strategy.as_str())
                .collect::<Vec<_>>()
                .join("->"),
            prepared.stability_class().as_str().to_string(),
            prepared.feature_conditioning_class().as_str().to_string(),
            prepared.support_normal_class().as_str().to_string(),
            prepared.normalization_disposition().as_str().to_string(),
            evidence.evidence_digest().to_string(),
            evidence.graph_obligation_selected_count().to_string(),
        ]);
        Self {
            family,
            canonical_artifact_digest: prepared.artifact_digest().to_string(),
            result_digest: prepared.result_digest().to_string(),
            topology_compose_evidence_digest: evidence.evidence_digest().to_string(),
            graph_obligation_envelope_digest: evidence
                .graph_obligation_envelope_digest()
                .to_string(),
            graph_obligation_selected_count: evidence.graph_obligation_selected_count(),
            outcome_digest,
        }
    }

    pub(crate) fn topology_compose_evidence_digest(&self) -> &str {
        &self.topology_compose_evidence_digest
    }

    pub(crate) fn graph_obligation_envelope_digest(&self) -> &str {
        &self.graph_obligation_envelope_digest
    }

    pub(crate) fn graph_obligation_selected_count(&self) -> usize {
        self.graph_obligation_selected_count
    }

    pub(crate) fn outcome_digest(&self) -> &str {
        &self.outcome_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionPreparedOutcome {
    Accepted(PrimitiveConstructionAcceptedOutcome),
    Rejected(PrimitiveConstructionRejectedOutcome),
}

impl PrimitiveConstructionPreparedOutcome {
    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        match self {
            Self::Accepted(outcome) => outcome.family(),
            Self::Rejected(outcome) => outcome.family(),
        }
    }

    pub(crate) fn recovery_fact_receipts(&self) -> &[GeometryRecoveryActionFactReceipt] {
        match self {
            Self::Accepted(_) => &[],
            Self::Rejected(outcome) => outcome.recovery_fact_receipts(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionExecutedPreparedOutcome {
    Accepted(ExecutedPrimitiveConstructionGraphAuthorityOutcome),
    Rejected(PrimitiveConstructionRejectedOutcome),
}

pub(crate) fn prepare_primitive_construction_executed_outcome<
    I: Into<PrimitiveConstructionIntent>,
>(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    intent: I,
) -> PrimitiveConstructionExecutedPreparedOutcome {
    let intent = intent.into();
    let family = intent.family();
    match prepare_primitive_construction_executed_result(workspace, intent) {
        Ok(prepared) => PrimitiveConstructionExecutedPreparedOutcome::Accepted(
            ExecutedPrimitiveConstructionGraphAuthorityOutcome::from_executed_result(
                family, &prepared,
            ),
        ),
        Err(error) => {
            PrimitiveConstructionExecutedPreparedOutcome::Rejected(rejected_outcome(family, &error))
        }
    }
}

pub(crate) fn prepare_primitive_construction_outcome<I: Into<PrimitiveConstructionIntent>>(
    intent: I,
) -> PrimitiveConstructionPreparedOutcome {
    let intent = intent.into();
    let family = intent.family();
    match prepare_primitive_construction_result(intent) {
        Ok(prepared) => PrimitiveConstructionPreparedOutcome::Accepted(
            PrimitiveConstructionAcceptedOutcome::from_handoff_result(family, &prepared),
        ),
        Err(error) => {
            PrimitiveConstructionPreparedOutcome::Rejected(rejected_outcome(family, &error))
        }
    }
}

pub(crate) fn rejected_outcome(
    family: super::request::PrimitiveConstructionFamily,
    error: &super::result::PrimitiveConstructionResultError,
) -> outcome_rejection::PrimitiveConstructionRejectedOutcome {
    outcome_rejection::rejected_outcome(family, error)
}

#[cfg(test)]
#[path = "../tests/outcome.rs"]
mod tests;
