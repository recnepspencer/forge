use crate::construction::diagnostics::PrimitiveConstructionBlockingBoundary;
use crate::construction::digest::digest_owned_parts;
use crate::construction::outcome::{
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
use crate::construction::request::PrimitiveConstructionFamily;
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};

pub use super::ordering::PrimitiveConstructionCorpusAuthoringOrderRow;
pub use super::rejection_witnesses::PrimitiveConstructionCorpusRejectionWitnessRow;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionCorpusParameterRole {
    MinimalAdmitted,
    GenericAdmitted,
    StressAdmitted,
    ThresholdAdmitted,
    ThresholdRejected,
    ExplicitRejected,
}

impl PrimitiveConstructionCorpusParameterRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MinimalAdmitted => "minimal_admitted",
            Self::GenericAdmitted => "generic_admitted",
            Self::StressAdmitted => "stress_admitted",
            Self::ThresholdAdmitted => "threshold_admitted",
            Self::ThresholdRejected => "threshold_rejected",
            Self::ExplicitRejected => "explicit_rejected",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionCorpusOutcomeDisposition {
    Admitted,
    Rejected,
}

impl PrimitiveConstructionCorpusOutcomeDisposition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCorpusReplaySiegeRow {
    scenario_id: String,
    family: PrimitiveConstructionFamily,
    parameter_role: PrimitiveConstructionCorpusParameterRole,
    outcome_disposition: PrimitiveConstructionCorpusOutcomeDisposition,
    direct_construction_digest: String,
    branch_local_digest: String,
    replay_digest: String,
    birth_digest: Option<String>,
    realization_strategy: Option<PrimitiveRealizationStrategy>,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: Option<PrimitiveStabilityClass>,
    feature_conditioning_class: Option<PrimitiveFeatureConditioningClass>,
    support_normal_class: Option<PrimitiveSupportNormalClass>,
    normalization_disposition: Option<PrimitiveNormalizationDisposition>,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    rejection_class: Option<PrimitiveConstructionRejectionClass>,
    rejection_locality: Option<PrimitiveConstructionRejectionLocality>,
    blocking_boundary: Option<PrimitiveConstructionBlockingBoundary>,
    construction_breadth: usize,
    birth_attachment_breadth: usize,
    certification_breadth: usize,
    row_digest: String,
}

impl PrimitiveConstructionCorpusReplaySiegeRow {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        scenario_id: String,
        family: PrimitiveConstructionFamily,
        parameter_role: PrimitiveConstructionCorpusParameterRole,
        outcome_disposition: PrimitiveConstructionCorpusOutcomeDisposition,
        direct_construction_digest: String,
        branch_local_digest: String,
        replay_digest: String,
        birth_digest: Option<String>,
        realization_strategy: Option<PrimitiveRealizationStrategy>,
        attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
        stability_class: Option<PrimitiveStabilityClass>,
        feature_conditioning_class: Option<PrimitiveFeatureConditioningClass>,
        support_normal_class: Option<PrimitiveSupportNormalClass>,
        normalization_disposition: Option<PrimitiveNormalizationDisposition>,
        exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
        rejection_class: Option<PrimitiveConstructionRejectionClass>,
        rejection_locality: Option<PrimitiveConstructionRejectionLocality>,
        blocking_boundary: Option<PrimitiveConstructionBlockingBoundary>,
        construction_breadth: usize,
        birth_attachment_breadth: usize,
        certification_breadth: usize,
    ) -> Self {
        let row_digest = digest_owned_parts(&[
            scenario_id.clone(),
            family.as_str().to_string(),
            parameter_role.as_str().to_string(),
            outcome_disposition.as_str().to_string(),
            direct_construction_digest.clone(),
            branch_local_digest.clone(),
            replay_digest.clone(),
            birth_digest.clone().unwrap_or_default(),
            realization_strategy
                .map(PrimitiveRealizationStrategy::as_str)
                .unwrap_or("none")
                .to_string(),
            attempted_realization_strategies
                .iter()
                .map(|strategy| strategy.as_str())
                .collect::<Vec<_>>()
                .join("->"),
            stability_class
                .map(PrimitiveStabilityClass::as_str)
                .unwrap_or("none")
                .to_string(),
            feature_conditioning_class
                .map(PrimitiveFeatureConditioningClass::as_str)
                .unwrap_or("none")
                .to_string(),
            support_normal_class
                .map(PrimitiveSupportNormalClass::as_str)
                .unwrap_or("none")
                .to_string(),
            normalization_disposition
                .map(PrimitiveNormalizationDisposition::as_str)
                .unwrap_or("none")
                .to_string(),
            exhaustion_reason
                .map(PrimitiveRealizationExhaustionReason::as_str)
                .unwrap_or("none")
                .to_string(),
            rejection_class
                .map(PrimitiveConstructionRejectionClass::as_str)
                .unwrap_or("none")
                .to_string(),
            rejection_locality
                .map(PrimitiveConstructionRejectionLocality::as_str)
                .unwrap_or("none")
                .to_string(),
            blocking_boundary
                .map(PrimitiveConstructionBlockingBoundary::as_str)
                .unwrap_or("none")
                .to_string(),
            construction_breadth.to_string(),
            birth_attachment_breadth.to_string(),
            certification_breadth.to_string(),
        ]);
        Self {
            scenario_id,
            family,
            parameter_role,
            outcome_disposition,
            direct_construction_digest,
            branch_local_digest,
            replay_digest,
            birth_digest,
            realization_strategy,
            attempted_realization_strategies,
            stability_class,
            feature_conditioning_class,
            support_normal_class,
            normalization_disposition,
            exhaustion_reason,
            rejection_class,
            rejection_locality,
            blocking_boundary,
            construction_breadth,
            birth_attachment_breadth,
            certification_breadth,
            row_digest,
        }
    }

    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn parameter_role(&self) -> PrimitiveConstructionCorpusParameterRole {
        self.parameter_role
    }

    pub fn outcome_disposition(&self) -> PrimitiveConstructionCorpusOutcomeDisposition {
        self.outcome_disposition
    }

    pub fn direct_construction_digest(&self) -> &str {
        &self.direct_construction_digest
    }

    pub fn branch_local_digest(&self) -> &str {
        &self.branch_local_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn birth_digest(&self) -> Option<&str> {
        self.birth_digest.as_deref()
    }

    pub fn realization_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        self.realization_strategy
    }

    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_realization_strategies
    }

    pub fn attempted_realization_strategy_count(&self) -> usize {
        self.attempted_realization_strategies.len()
    }

    pub fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.stability_class
    }

    pub fn feature_conditioning_class(&self) -> Option<PrimitiveFeatureConditioningClass> {
        self.feature_conditioning_class
    }

    pub fn support_normal_class(&self) -> Option<PrimitiveSupportNormalClass> {
        self.support_normal_class
    }

    pub fn normalization_disposition(&self) -> Option<PrimitiveNormalizationDisposition> {
        self.normalization_disposition
    }

    pub fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.exhaustion_reason
    }

    pub fn rejection_class(&self) -> Option<PrimitiveConstructionRejectionClass> {
        self.rejection_class
    }

    pub fn rejection_locality(&self) -> Option<PrimitiveConstructionRejectionLocality> {
        self.rejection_locality
    }

    pub fn blocking_boundary(&self) -> Option<PrimitiveConstructionBlockingBoundary> {
        self.blocking_boundary
    }

    pub fn construction_breadth(&self) -> usize {
        self.construction_breadth
    }

    pub fn birth_attachment_breadth(&self) -> usize {
        self.birth_attachment_breadth
    }

    pub fn certification_breadth(&self) -> usize {
        self.certification_breadth
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCorpusReplaySiegeReport {
    rows: Vec<PrimitiveConstructionCorpusReplaySiegeRow>,
    accepted_count: usize,
    rejected_count: usize,
    authoring_order_rows: Vec<PrimitiveConstructionCorpusAuthoringOrderRow>,
    rejection_witness_rows: Vec<PrimitiveConstructionCorpusRejectionWitnessRow>,
    report_digest: String,
}

impl PrimitiveConstructionCorpusReplaySiegeReport {
    pub(super) fn new(
        rows: Vec<PrimitiveConstructionCorpusReplaySiegeRow>,
        accepted_count: usize,
        rejected_count: usize,
        authoring_order_rows: Vec<PrimitiveConstructionCorpusAuthoringOrderRow>,
        rejection_witness_rows: Vec<PrimitiveConstructionCorpusRejectionWitnessRow>,
    ) -> Self {
        let mut parts = rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>();
        parts.push(format!("accepted-count:{accepted_count}"));
        parts.push(format!("rejected-count:{rejected_count}"));
        parts.extend(
            authoring_order_rows
                .iter()
                .map(|row| row.row_digest().to_string()),
        );
        parts.extend(
            rejection_witness_rows
                .iter()
                .map(|row| row.row_digest().to_string()),
        );
        Self {
            rows,
            accepted_count,
            rejected_count,
            authoring_order_rows,
            rejection_witness_rows,
            report_digest: digest_owned_parts(&parts),
        }
    }

    pub fn rows(&self) -> &[PrimitiveConstructionCorpusReplaySiegeRow] {
        &self.rows
    }

    pub fn row_for(
        &self,
        family: PrimitiveConstructionFamily,
        parameter_role: PrimitiveConstructionCorpusParameterRole,
    ) -> Option<&PrimitiveConstructionCorpusReplaySiegeRow> {
        self.rows
            .iter()
            .find(|row| row.family() == family && row.parameter_role() == parameter_role)
    }

    pub fn accepted_count(&self) -> usize {
        self.accepted_count
    }

    pub fn rejected_count(&self) -> usize {
        self.rejected_count
    }

    pub fn authoring_order_rows(&self) -> &[PrimitiveConstructionCorpusAuthoringOrderRow] {
        &self.authoring_order_rows
    }

    pub fn authoring_order_parity_verified(&self) -> bool {
        self.authoring_order_rows
            .iter()
            .all(|row| row.parity_verified())
    }

    pub fn rejection_witness_rows(&self) -> &[PrimitiveConstructionCorpusRejectionWitnessRow] {
        &self.rejection_witness_rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
