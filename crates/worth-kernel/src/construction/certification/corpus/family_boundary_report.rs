use forge_query::facade::ForgeQueryWorkspace;
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationExhaustionWitnessKind,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass, PrimitiveSupportNormalClass,
};

use super::{
    prepare_primitive_construction_corpus_replay_siege, PrimitiveConstructionCorpusParameterRole,
    PrimitiveConstructionCorpusReplaySiegeError, PrimitiveConstructionCorpusReplaySiegeReport,
};
use crate::construction::certification::prepare_primitive_construction_realization_exhaustion_witness_report;
use crate::construction::digest::digest_owned_parts;
use crate::construction::{
    PrimitiveConstructionBlockingBoundary, PrimitiveConstructionFamily,
    PrimitiveConstructionRealizationExhaustionWitnessReport, PrimitiveConstructionRejectionClass,
    PrimitiveConstructionRejectionLocality,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionFamilyBoundaryTransitionClass {
    DirectStableToTypedRejection,
    EscalatedStableToTypedRejection,
}

impl PrimitiveConstructionFamilyBoundaryTransitionClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DirectStableToTypedRejection => "direct_stable_to_typed_rejection",
            Self::EscalatedStableToTypedRejection => "escalated_stable_to_typed_rejection",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionFamilyBoundaryLowerLayerWitnessSummary {
    witness_kind: PrimitiveRealizationExhaustionWitnessKind,
    exhaustion_reason: PrimitiveRealizationExhaustionReason,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    row_digest: String,
}

impl PrimitiveConstructionFamilyBoundaryLowerLayerWitnessSummary {
    pub fn witness_kind(&self) -> PrimitiveRealizationExhaustionWitnessKind {
        self.witness_kind
    }

    pub fn exhaustion_reason(&self) -> PrimitiveRealizationExhaustionReason {
        self.exhaustion_reason
    }

    pub fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionFamilyBoundaryRow {
    family: PrimitiveConstructionFamily,
    transition_class: PrimitiveConstructionFamilyBoundaryTransitionClass,
    admitted_strategy: PrimitiveRealizationStrategy,
    admitted_attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    admitted_stability_class: PrimitiveStabilityClass,
    admitted_feature_conditioning_class: PrimitiveFeatureConditioningClass,
    admitted_support_normal_class: PrimitiveSupportNormalClass,
    admitted_normalization_disposition: PrimitiveNormalizationDisposition,
    rejected_rejection_class: PrimitiveConstructionRejectionClass,
    rejected_rejection_locality: PrimitiveConstructionRejectionLocality,
    rejected_blocking_boundary: PrimitiveConstructionBlockingBoundary,
    lower_layer_exhaustion_witnesses:
        Vec<PrimitiveConstructionFamilyBoundaryLowerLayerWitnessSummary>,
    row_digest: String,
}

impl PrimitiveConstructionFamilyBoundaryRow {
    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn transition_class(&self) -> PrimitiveConstructionFamilyBoundaryTransitionClass {
        self.transition_class
    }

    pub fn admitted_strategy(&self) -> PrimitiveRealizationStrategy {
        self.admitted_strategy
    }

    pub fn admitted_attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.admitted_attempted_strategies
    }

    pub fn admitted_stability_class(&self) -> PrimitiveStabilityClass {
        self.admitted_stability_class
    }

    pub fn admitted_feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.admitted_feature_conditioning_class
    }

    pub fn admitted_support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.admitted_support_normal_class
    }

    pub fn admitted_normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.admitted_normalization_disposition
    }

    pub fn rejected_rejection_class(&self) -> PrimitiveConstructionRejectionClass {
        self.rejected_rejection_class
    }

    pub fn rejected_rejection_locality(&self) -> PrimitiveConstructionRejectionLocality {
        self.rejected_rejection_locality
    }

    pub fn rejected_blocking_boundary(&self) -> PrimitiveConstructionBlockingBoundary {
        self.rejected_blocking_boundary
    }

    pub fn lower_layer_exhaustion_witness_kind(
        &self,
    ) -> Option<PrimitiveRealizationExhaustionWitnessKind> {
        match self.lower_layer_exhaustion_witnesses.as_slice() {
            [witness] => Some(witness.witness_kind()),
            _ => None,
        }
    }

    pub fn lower_layer_exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        match self.lower_layer_exhaustion_witnesses.as_slice() {
            [witness] => Some(witness.exhaustion_reason()),
            _ => None,
        }
    }

    pub fn lower_layer_exhaustion_attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        match self.lower_layer_exhaustion_witnesses.as_slice() {
            [witness] => witness.attempted_strategies(),
            _ => &[],
        }
    }

    pub fn lower_layer_exhaustion_witnesses(
        &self,
    ) -> &[PrimitiveConstructionFamilyBoundaryLowerLayerWitnessSummary] {
        &self.lower_layer_exhaustion_witnesses
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionFamilyBoundaryReport {
    rows: Vec<PrimitiveConstructionFamilyBoundaryRow>,
    report_digest: String,
}

impl PrimitiveConstructionFamilyBoundaryReport {
    pub fn rows(&self) -> &[PrimitiveConstructionFamilyBoundaryRow] {
        &self.rows
    }

    pub fn row_for(
        &self,
        family: PrimitiveConstructionFamily,
    ) -> Option<&PrimitiveConstructionFamilyBoundaryRow> {
        self.rows.iter().find(|row| row.family() == family)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionFamilyBoundaryReportError {
    Siege(PrimitiveConstructionCorpusReplaySiegeError),
    MissingThresholdAdmitted(PrimitiveConstructionFamily),
    MissingThresholdRejected(PrimitiveConstructionFamily),
    MissingAdmittedRealizationTruth(PrimitiveConstructionFamily),
    MissingRejectedBoundaryTruth(PrimitiveConstructionFamily),
}

impl std::fmt::Display for PrimitiveConstructionFamilyBoundaryReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Siege(error) => write!(f, "{error}"),
            Self::MissingThresholdAdmitted(family) => {
                write!(
                    f,
                    "missing threshold admitted corpus row for {}",
                    family.as_str()
                )
            }
            Self::MissingThresholdRejected(family) => {
                write!(
                    f,
                    "missing threshold rejected corpus row for {}",
                    family.as_str()
                )
            }
            Self::MissingAdmittedRealizationTruth(family) => write!(
                f,
                "threshold admitted corpus row for {} is missing realization truth",
                family.as_str()
            ),
            Self::MissingRejectedBoundaryTruth(family) => write!(
                f,
                "threshold rejected corpus row for {} is missing typed rejection boundary truth",
                family.as_str()
            ),
        }
    }
}

impl std::error::Error for PrimitiveConstructionFamilyBoundaryReportError {}

pub fn prepare_primitive_construction_family_boundary_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<PrimitiveConstructionFamilyBoundaryReport, PrimitiveConstructionFamilyBoundaryReportError>
{
    let siege = prepare_primitive_construction_corpus_replay_siege(workspace)
        .map_err(PrimitiveConstructionFamilyBoundaryReportError::Siege)?;
    let exhaustion = prepare_primitive_construction_realization_exhaustion_witness_report();
    let mut rows = Vec::new();
    for family in PrimitiveConstructionFamily::ALL {
        rows.push(boundary_row_for(&siege, &exhaustion, family)?);
    }
    let report_digest = digest_owned_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    Ok(PrimitiveConstructionFamilyBoundaryReport {
        rows,
        report_digest,
    })
}

fn boundary_row_for(
    siege: &PrimitiveConstructionCorpusReplaySiegeReport,
    exhaustion: &PrimitiveConstructionRealizationExhaustionWitnessReport,
    family: PrimitiveConstructionFamily,
) -> Result<PrimitiveConstructionFamilyBoundaryRow, PrimitiveConstructionFamilyBoundaryReportError>
{
    let admitted = siege
        .row_for(
            family,
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
        )
        .ok_or(PrimitiveConstructionFamilyBoundaryReportError::MissingThresholdAdmitted(family))?;
    let rejected = siege
        .row_for(
            family,
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
        )
        .ok_or(PrimitiveConstructionFamilyBoundaryReportError::MissingThresholdRejected(family))?;
    let admitted_strategy = admitted.realization_strategy().ok_or(
        PrimitiveConstructionFamilyBoundaryReportError::MissingAdmittedRealizationTruth(family),
    )?;
    let admitted_stability_class = admitted.stability_class().ok_or(
        PrimitiveConstructionFamilyBoundaryReportError::MissingAdmittedRealizationTruth(family),
    )?;
    let admitted_feature_conditioning_class = admitted.feature_conditioning_class().ok_or(
        PrimitiveConstructionFamilyBoundaryReportError::MissingAdmittedRealizationTruth(family),
    )?;
    let admitted_support_normal_class = admitted.support_normal_class().ok_or(
        PrimitiveConstructionFamilyBoundaryReportError::MissingAdmittedRealizationTruth(family),
    )?;
    let admitted_normalization_disposition = admitted.normalization_disposition().ok_or(
        PrimitiveConstructionFamilyBoundaryReportError::MissingAdmittedRealizationTruth(family),
    )?;
    let rejected_rejection_class = rejected.rejection_class().ok_or(
        PrimitiveConstructionFamilyBoundaryReportError::MissingRejectedBoundaryTruth(family),
    )?;
    let rejected_rejection_locality = rejected.rejection_locality().ok_or(
        PrimitiveConstructionFamilyBoundaryReportError::MissingRejectedBoundaryTruth(family),
    )?;
    let rejected_blocking_boundary = rejected.blocking_boundary().ok_or(
        PrimitiveConstructionFamilyBoundaryReportError::MissingRejectedBoundaryTruth(family),
    )?;
    let transition_class = match admitted_stability_class {
        PrimitiveStabilityClass::StableDirect => {
            PrimitiveConstructionFamilyBoundaryTransitionClass::DirectStableToTypedRejection
        }
        PrimitiveStabilityClass::StableAfterEscalation => {
            PrimitiveConstructionFamilyBoundaryTransitionClass::EscalatedStableToTypedRejection
        }
        PrimitiveStabilityClass::RejectedBelowConditioningFloor => {
            PrimitiveConstructionFamilyBoundaryTransitionClass::EscalatedStableToTypedRejection
        }
    };
    let lower_layer_witnesses = exhaustion
        .rows()
        .iter()
        .filter(|row| row.family() == family)
        .map(
            |row| PrimitiveConstructionFamilyBoundaryLowerLayerWitnessSummary {
                witness_kind: row.witness_kind(),
                exhaustion_reason: row.exhaustion_reason(),
                attempted_strategies: row.attempted_strategies().to_vec(),
                row_digest: row.row_digest().to_string(),
            },
        )
        .collect::<Vec<_>>();
    let row_digest = digest_owned_parts(&[
        family.as_str().to_string(),
        transition_class.as_str().to_string(),
        admitted_strategy.as_str().to_string(),
        admitted
            .attempted_realization_strategies()
            .iter()
            .map(|strategy| strategy.as_str())
            .collect::<Vec<_>>()
            .join("->"),
        admitted_stability_class.as_str().to_string(),
        admitted_feature_conditioning_class.as_str().to_string(),
        admitted_support_normal_class.as_str().to_string(),
        admitted_normalization_disposition.as_str().to_string(),
        rejected_rejection_class.as_str().to_string(),
        rejected_rejection_locality.as_str().to_string(),
        rejected_blocking_boundary.as_str().to_string(),
        lower_layer_witnesses
            .iter()
            .map(|witness| {
                format!(
                    "{}:{}:{}:{}",
                    witness.witness_kind().as_str(),
                    witness.exhaustion_reason().as_str(),
                    witness
                        .attempted_strategies()
                        .iter()
                        .map(|strategy| strategy.as_str())
                        .collect::<Vec<_>>()
                        .join("->"),
                    witness.row_digest()
                )
            })
            .collect::<Vec<_>>()
            .join("|"),
        admitted.row_digest().to_string(),
        rejected.row_digest().to_string(),
    ]);

    Ok(PrimitiveConstructionFamilyBoundaryRow {
        family,
        transition_class,
        admitted_strategy,
        admitted_attempted_strategies: admitted.attempted_realization_strategies().to_vec(),
        admitted_stability_class,
        admitted_feature_conditioning_class,
        admitted_support_normal_class,
        admitted_normalization_disposition,
        rejected_rejection_class,
        rejected_rejection_locality,
        rejected_blocking_boundary,
        lower_layer_exhaustion_witnesses: lower_layer_witnesses,
        row_digest,
    })
}
