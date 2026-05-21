use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::certification::corpus::{
    prepare_primitive_construction_compound_milestone_closeout_report,
    prepare_primitive_construction_simplex_realization_exhaustion_witness_report,
    prepare_primitive_construction_simplex_realization_strategy_ladder_report,
    PrimitiveConstructionCompoundAdversarialSiegeError,
    PrimitiveConstructionCompoundMilestoneCloseoutReport,
    PrimitiveConstructionSimplexRealizationExhaustionWitnessReport,
    PrimitiveConstructionSimplexRealizationLadderReportError,
    PrimitiveConstructionSimplexRealizationStrategyLadderReport,
};
use crate::construction::certification::profile::{
    prepare_primitive_construction_policy_pressure_report_bundle,
    PrimitiveConstructionPolicyPressureReportBundle,
    PrimitiveConstructionPolicyPressureReportBundleError,
};
use crate::construction::digest::digest_owned_parts;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPhaseFiveSixCloseoutReport {
    compound_closeout: PrimitiveConstructionCompoundMilestoneCloseoutReport,
    simplex_ladder: PrimitiveConstructionSimplexRealizationStrategyLadderReport,
    simplex_exhaustion: PrimitiveConstructionSimplexRealizationExhaustionWitnessReport,
    policy_pressure: PrimitiveConstructionPolicyPressureReportBundle,
    required_simplex_rows_present: bool,
    closeout_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPhaseFiveSixCloseoutReport {
    fn new(
        compound_closeout: PrimitiveConstructionCompoundMilestoneCloseoutReport,
        simplex_ladder: PrimitiveConstructionSimplexRealizationStrategyLadderReport,
        simplex_exhaustion: PrimitiveConstructionSimplexRealizationExhaustionWitnessReport,
        policy_pressure: PrimitiveConstructionPolicyPressureReportBundle,
    ) -> Self {
        let required_simplex_rows_present = required_simplex_scenarios()
            .iter()
            .all(|scenario_id| simplex_ladder.row_for(scenario_id).is_some());
        let closeout_verified = compound_closeout.closeout_gate_verified()
            && policy_pressure.parity_verified()
            && required_simplex_rows_present
            && simplex_exhaustion.rows().len() == 2;
        let report_digest = digest_owned_parts(&[
            compound_closeout.report_digest().to_string(),
            simplex_ladder.report_digest().to_string(),
            simplex_exhaustion.report_digest().to_string(),
            policy_pressure.report_digest().to_string(),
            required_simplex_rows_present.to_string(),
            closeout_verified.to_string(),
        ]);
        Self {
            compound_closeout,
            simplex_ladder,
            simplex_exhaustion,
            policy_pressure,
            required_simplex_rows_present,
            closeout_verified,
            report_digest,
        }
    }

    pub fn compound_closeout(&self) -> &PrimitiveConstructionCompoundMilestoneCloseoutReport {
        &self.compound_closeout
    }

    pub fn simplex_ladder(&self) -> &PrimitiveConstructionSimplexRealizationStrategyLadderReport {
        &self.simplex_ladder
    }

    pub fn simplex_exhaustion(
        &self,
    ) -> &PrimitiveConstructionSimplexRealizationExhaustionWitnessReport {
        &self.simplex_exhaustion
    }

    pub fn policy_pressure(&self) -> &PrimitiveConstructionPolicyPressureReportBundle {
        &self.policy_pressure
    }

    pub fn required_simplex_scenarios(&self) -> &[&'static str] {
        required_simplex_scenarios()
    }

    pub fn required_simplex_rows_present(&self) -> bool {
        self.required_simplex_rows_present
    }

    pub fn closeout_verified(&self) -> bool {
        self.closeout_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionPhaseFiveSixCloseoutReportError {
    Compound(PrimitiveConstructionCompoundAdversarialSiegeError),
    SimplexLadder(PrimitiveConstructionSimplexRealizationLadderReportError),
    PolicyPressure(PrimitiveConstructionPolicyPressureReportBundleError),
}

impl std::fmt::Display for PrimitiveConstructionPhaseFiveSixCloseoutReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compound(error) => write!(f, "{error}"),
            Self::SimplexLadder(error) => write!(f, "{error}"),
            Self::PolicyPressure(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPhaseFiveSixCloseoutReportError {}

pub fn prepare_primitive_construction_phase_five_six_closeout_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionPhaseFiveSixCloseoutReport,
    PrimitiveConstructionPhaseFiveSixCloseoutReportError,
> {
    let compound_closeout =
        prepare_primitive_construction_compound_milestone_closeout_report(workspace)
            .map_err(PrimitiveConstructionPhaseFiveSixCloseoutReportError::Compound)?;
    let simplex_ladder =
        prepare_primitive_construction_simplex_realization_strategy_ladder_report(workspace)
            .map_err(PrimitiveConstructionPhaseFiveSixCloseoutReportError::SimplexLadder)?;
    let simplex_exhaustion =
        prepare_primitive_construction_simplex_realization_exhaustion_witness_report();
    let policy_pressure = prepare_primitive_construction_policy_pressure_report_bundle()
        .map_err(PrimitiveConstructionPhaseFiveSixCloseoutReportError::PolicyPressure)?;
    Ok(PrimitiveConstructionPhaseFiveSixCloseoutReport::new(
        compound_closeout,
        simplex_ladder,
        simplex_exhaustion,
        policy_pressure,
    ))
}

fn required_simplex_scenarios() -> &'static [&'static str] {
    &[
        "simplex_world_collapsed_admitted_local_or_exact",
        "simplex_world_collapsed_threshold_rejected",
        "simplex_world_collapsed_explicit_exhaustion",
    ]
}
