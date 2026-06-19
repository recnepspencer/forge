use super::super::super::digest::digest_owned_parts;
use super::super::super::intent::PrimitiveConstructionIntent;
use super::super::super::outcome::{
    rejected_outcome, GeometryRecoveryActionFactReceipt, PrimitiveConstructionRejectedOutcome,
};
use super::super::super::request::PrimitiveConstructionFamily;
use super::super::super::result::{
    prepare_primitive_construction_executed_result, prepare_primitive_construction_result,
    PreparedPrimitiveConstructionResult,
};
use topology::facade::{
    TopologyConstructionQueryFactProvenance, TopologyConstructionQueryReadSurface,
};
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass, PrimitiveSupportNormalClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionAcceptedOutcome {
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
    fn from_prepared_result(
        family: PrimitiveConstructionFamily,
        prepared: &PreparedPrimitiveConstructionResult,
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
            prepared
                .topology_compose_evidence()
                .map(|evidence| evidence.evidence_digest().to_string())
                .unwrap_or_else(|| "handoff-only-no-compose-evidence".to_string()),
            prepared
                .topology_compose_evidence()
                .map(|evidence| evidence.graph_obligation_selected_count().to_string())
                .unwrap_or_else(|| "0".to_string()),
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
            topology_compose_evidence_digest: prepared
                .topology_compose_evidence()
                .map(|evidence| evidence.evidence_digest().to_string()),
            graph_obligation_envelope_digest: prepared
                .topology_compose_evidence()
                .map(|evidence| evidence.graph_obligation_envelope_digest().to_string()),
            graph_obligation_selected_count: prepared
                .topology_compose_evidence()
                .map(|evidence| evidence.graph_obligation_selected_count())
                .unwrap_or(0),
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

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub fn canonical_artifact_digest(&self) -> &str {
        &self.canonical_artifact_digest
    }

    pub fn topology_compose_evidence_digest(&self) -> Option<&str> {
        self.topology_compose_evidence_digest.as_deref()
    }

    pub fn graph_obligation_envelope_digest(&self) -> Option<&str> {
        self.graph_obligation_envelope_digest.as_deref()
    }

    pub fn graph_obligation_selected_count(&self) -> usize {
        self.graph_obligation_selected_count
    }

    pub fn outcome_digest(&self) -> &str {
        &self.outcome_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPreparedOutcome {
    Accepted(PrimitiveConstructionAcceptedOutcome),
    Rejected(PrimitiveConstructionRejectedOutcome),
}

impl PrimitiveConstructionPreparedOutcome {
    pub fn family(&self) -> PrimitiveConstructionFamily {
        match self {
            Self::Accepted(outcome) => outcome.family(),
            Self::Rejected(outcome) => outcome.family(),
        }
    }

    pub fn recovery_fact_receipts(&self) -> &[GeometryRecoveryActionFactReceipt] {
        match self {
            Self::Accepted(_) => &[],
            Self::Rejected(outcome) => outcome.recovery_fact_receipts(),
        }
    }
}

pub fn prepare_primitive_construction_executed_outcome<I: Into<PrimitiveConstructionIntent>>(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    intent: I,
) -> PrimitiveConstructionPreparedOutcome {
    let intent = intent.into();
    let family = intent.family();
    match prepare_primitive_construction_executed_result(workspace, intent) {
        Ok(prepared) => PrimitiveConstructionPreparedOutcome::Accepted(
            PrimitiveConstructionAcceptedOutcome::from_prepared_result(family, &prepared),
        ),
        Err(error) => {
            PrimitiveConstructionPreparedOutcome::Rejected(rejected_outcome(family, &error))
        }
    }
}

pub fn prepare_primitive_construction_outcome<I: Into<PrimitiveConstructionIntent>>(
    intent: I,
) -> PrimitiveConstructionPreparedOutcome {
    let intent = intent.into();
    let family = intent.family();
    match prepare_primitive_construction_result(intent) {
        Ok(prepared) => PrimitiveConstructionPreparedOutcome::Accepted(
            PrimitiveConstructionAcceptedOutcome::from_prepared_result(family, &prepared),
        ),
        Err(error) => {
            PrimitiveConstructionPreparedOutcome::Rejected(rejected_outcome(family, &error))
        }
    }
}
