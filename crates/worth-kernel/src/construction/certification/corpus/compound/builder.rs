use forge_query::facade::ForgeQueryWorkspace;

use super::super::ordering::{
    apply_compound_authoring_order_lane, PrimitiveConstructionAdversarialAuthoringOrderLane,
};
use super::super::parity::{derive_specialized_rows, require_specialized_row_field};
use super::cases::compound_scenarios;
use super::lane_report::PrimitiveConstructionCompoundOrderLaneReport;
use super::ordering_report::PrimitiveConstructionCompoundOrderingParityReport;
use super::parity::exhaustion_witness_kind_for;
use super::parity::{
    verify_bundle, PrimitiveConstructionCompoundParityReport,
    PrimitiveConstructionCompoundParityReportBundle,
    PrimitiveConstructionCompoundParityVerificationFailure,
};
use super::report::{
    PrimitiveConstructionCompoundAdversarialSiegeReport,
    PrimitiveConstructionCompoundExhaustionWitnessParityReport,
    PrimitiveConstructionCompoundGrazingBoundaryReport,
    PrimitiveConstructionCompoundMotionParityReport,
};
use super::row_builder::{
    authoring_order_row, build_rows_for_lane, compute_normalized_matrix_digest,
};
use super::rows::{
    PrimitiveConstructionCompoundExhaustionWitnessParityRow,
    PrimitiveConstructionCompoundGrazingBoundaryRow, PrimitiveConstructionCompoundMotionParityRow,
};
use crate::construction::prepare_primitive_construction_realization_exhaustion_witness_report;
use crate::construction::{
    PrimitiveConstructionResultError, PrimitiveConstructionRuntimeBasisError,
};
use crate::facade::PrimitiveConstructionSpatialIntentError;

#[derive(Debug)]
pub enum PrimitiveConstructionCompoundAdversarialSiegeError {
    Motion(PrimitiveConstructionSpatialIntentError),
    Placement(String),
    RuntimeBasis(PrimitiveConstructionRuntimeBasisError),
    Result(PrimitiveConstructionResultError),
    ReplayParityDrift(String),
    BranchLocalParityDrift(String),
    InvalidRejectedLocality(String),
    InvalidSpecializedRow(String),
    Inspection(String),
    Projection(String),
    NumericWitness(String),
    Verification(PrimitiveConstructionCompoundParityVerificationFailure),
}

impl std::fmt::Display for PrimitiveConstructionCompoundAdversarialSiegeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Motion(error) => write!(f, "{error}"),
            Self::Placement(error) => write!(f, "{error}"),
            Self::RuntimeBasis(error) => write!(f, "{error}"),
            Self::Result(error) => write!(f, "{error}"),
            Self::ReplayParityDrift(reason) => write!(f, "{reason}"),
            Self::BranchLocalParityDrift(reason) => write!(f, "{reason}"),
            Self::InvalidRejectedLocality(reason) => write!(f, "{reason}"),
            Self::InvalidSpecializedRow(reason) => write!(f, "{reason}"),
            Self::Inspection(reason) => write!(f, "{reason}"),
            Self::Projection(reason) => write!(f, "{reason}"),
            Self::NumericWitness(reason) => write!(f, "{reason}"),
            Self::Verification(error) => write!(f, "{error:?}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionCompoundAdversarialSiegeError {}

impl From<PrimitiveConstructionRuntimeBasisError>
    for PrimitiveConstructionCompoundAdversarialSiegeError
{
    fn from(error: PrimitiveConstructionRuntimeBasisError) -> Self {
        Self::RuntimeBasis(error)
    }
}

pub fn prepare_primitive_construction_compound_adversarial_siege_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionCompoundAdversarialSiegeReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let scenarios = compound_scenarios();
    let canonical = build_rows_for_lane(
        workspace,
        &apply_compound_authoring_order_lane(
            PrimitiveConstructionAdversarialAuthoringOrderLane::Canonical,
            &scenarios,
        ),
    )?;
    let normalized = compute_normalized_matrix_digest(&canonical);
    let mut lane_reports = Vec::with_capacity(
        PrimitiveConstructionAdversarialAuthoringOrderLane::all_compound().len(),
    );
    lane_reports.push(build_lane_report(
        PrimitiveConstructionAdversarialAuthoringOrderLane::Canonical,
        canonical,
        &normalized,
    ));
    for lane in PrimitiveConstructionAdversarialAuthoringOrderLane::all_compound()
        .into_iter()
        .filter(|lane| *lane != PrimitiveConstructionAdversarialAuthoringOrderLane::Canonical)
    {
        lane_reports.push(build_lane_report(
            lane,
            build_rows_for_lane(
                workspace,
                &apply_compound_authoring_order_lane(lane, &scenarios),
            )?,
            &normalized,
        ));
    }
    Ok(PrimitiveConstructionCompoundAdversarialSiegeReport::new(
        lane_reports,
    ))
}

pub fn prepare_primitive_construction_compound_ordering_parity_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionCompoundOrderingParityReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let siege = prepare_primitive_construction_compound_adversarial_siege_report(workspace)?;
    Ok(PrimitiveConstructionCompoundOrderingParityReport::new(
        siege.lane_reports().to_vec(),
    ))
}

pub fn prepare_primitive_construction_compound_motion_parity_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionCompoundMotionParityReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let siege = prepare_primitive_construction_compound_adversarial_siege_report(workspace)?;
    build_motion_parity_report_from_siege(&siege)
}

pub fn prepare_primitive_construction_compound_grazing_boundary_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionCompoundGrazingBoundaryReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let siege = prepare_primitive_construction_compound_adversarial_siege_report(workspace)?;
    build_grazing_boundary_report_from_siege(&siege)
}

pub(super) fn build_motion_parity_report_from_siege(
    siege: &PrimitiveConstructionCompoundAdversarialSiegeReport,
) -> Result<
    PrimitiveConstructionCompoundMotionParityReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let ordering =
        PrimitiveConstructionCompoundOrderingParityReport::new(siege.lane_reports().to_vec());
    let rows = derive_specialized_rows(
        siege.rows().iter(),
        |row: &super::rows::PrimitiveConstructionCompoundRow| {
            row.motion_kind().is_some() || row.motion_digest().is_some()
        },
        |row: &super::rows::PrimitiveConstructionCompoundRow| {
            let motion_kind = require_specialized_row_field(
                row.scenario_id(),
                "motion kind",
                row.motion_kind(),
                PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow,
            )?;
            let motion_digest = require_specialized_row_field(
                row.scenario_id(),
                "motion digest",
                row.motion_digest(),
                PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow,
            )?;
            Ok::<
                PrimitiveConstructionCompoundMotionParityRow,
                PrimitiveConstructionCompoundAdversarialSiegeError,
            >(PrimitiveConstructionCompoundMotionParityRow::new(
                row.scenario_id().to_string(),
                motion_kind,
                motion_digest.to_string(),
            ))
        },
    )?;
    Ok(PrimitiveConstructionCompoundMotionParityReport::new(
        rows, &ordering,
    ))
}

pub(super) fn build_grazing_boundary_report_from_siege(
    siege: &PrimitiveConstructionCompoundAdversarialSiegeReport,
) -> Result<
    PrimitiveConstructionCompoundGrazingBoundaryReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let ordering =
        PrimitiveConstructionCompoundOrderingParityReport::new(siege.lane_reports().to_vec());
    let rows = derive_specialized_rows(
        siege.rows().iter(),
        |row: &super::rows::PrimitiveConstructionCompoundRow| {
            row.grazing_kind().is_some() || row.grazing_digest().is_some()
        },
        |row: &super::rows::PrimitiveConstructionCompoundRow| {
            let grazing_kind = require_specialized_row_field(
                row.scenario_id(),
                "grazing kind",
                row.grazing_kind(),
                PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow,
            )?;
            let grazing_digest = require_specialized_row_field(
                row.scenario_id(),
                "grazing digest",
                row.grazing_digest(),
                PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow,
            )?;
            Ok::<
                PrimitiveConstructionCompoundGrazingBoundaryRow,
                PrimitiveConstructionCompoundAdversarialSiegeError,
            >(PrimitiveConstructionCompoundGrazingBoundaryRow::new(
                row.scenario_id().to_string(),
                grazing_kind,
                grazing_digest.to_string(),
            ))
        },
    )?;
    Ok(PrimitiveConstructionCompoundGrazingBoundaryReport::new(
        rows, &ordering,
    ))
}

pub fn prepare_primitive_construction_compound_exhaustion_witness_parity_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionCompoundExhaustionWitnessParityReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let siege = prepare_primitive_construction_compound_adversarial_siege_report(workspace)?;
    build_exhaustion_witness_parity_report_from_siege(&siege)
}

pub fn prepare_primitive_construction_compound_parity_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionCompoundParityReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let siege = prepare_primitive_construction_compound_adversarial_siege_report(workspace)?;
    build_compound_parity_report_from_siege(&siege)
}

pub(super) fn build_compound_parity_report_from_siege(
    siege: &PrimitiveConstructionCompoundAdversarialSiegeReport,
) -> Result<
    PrimitiveConstructionCompoundParityReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let ordering =
        PrimitiveConstructionCompoundOrderingParityReport::new(siege.lane_reports().to_vec());
    let motion = build_motion_parity_report_from_siege(siege)?;
    let grazing = build_grazing_boundary_report_from_siege(siege)?;
    let exhaustion = build_exhaustion_witness_parity_report_from_siege(siege)?;
    verify_bundle(PrimitiveConstructionCompoundParityReportBundle::new(
        siege.clone(),
        ordering,
        motion,
        grazing,
        exhaustion,
    ))
}

pub(super) fn build_exhaustion_witness_parity_report_from_siege(
    siege: &PrimitiveConstructionCompoundAdversarialSiegeReport,
) -> Result<
    PrimitiveConstructionCompoundExhaustionWitnessParityReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let witness_report = prepare_primitive_construction_realization_exhaustion_witness_report();
    let rows = siege
        .rows()
        .iter()
        .filter_map(|row| exhaustion_witness_kind_for(row.scenario_id()).map(|kind| (row, kind)))
        .map(|(row, witness_kind)| {
            let witness_row = witness_report.row_for(witness_kind).ok_or_else(|| {
                PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow(format!(
                    "compound exhaustion row '{}' is missing lower-layer witness row",
                    row.scenario_id()
                ))
            })?;
            if witness_row.exhaustion_reason()
                != row.exhaustion_reason().ok_or_else(|| {
                    PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow(
                        format!(
                            "compound exhaustion row '{}' is missing exhaustion reason",
                            row.scenario_id()
                        ),
                    )
                })?
            {
                return Err(
                    PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow(
                        format!(
                            "compound exhaustion row '{}' drifted from lower-layer witness truth",
                            row.scenario_id()
                        ),
                    ),
                );
            }
            Ok(
                PrimitiveConstructionCompoundExhaustionWitnessParityRow::new(
                    row.scenario_id().to_string(),
                    witness_kind,
                    row.row_digest().to_string(),
                    witness_row.row_digest().to_string(),
                ),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let ordering =
        PrimitiveConstructionCompoundOrderingParityReport::new(siege.lane_reports().to_vec());
    Ok(PrimitiveConstructionCompoundExhaustionWitnessParityReport::new(rows, &ordering))
}

fn build_lane_report(
    lane: PrimitiveConstructionAdversarialAuthoringOrderLane,
    rows: Vec<super::rows::PrimitiveConstructionCompoundRow>,
    expected_normalized_matrix_digest: &str,
) -> PrimitiveConstructionCompoundOrderLaneReport {
    let summary = authoring_order_row(lane.as_str(), &rows, expected_normalized_matrix_digest);
    PrimitiveConstructionCompoundOrderLaneReport::new(
        lane.as_str().to_string(),
        rows,
        summary.lane_digest().to_string(),
        summary.normalized_matrix_digest().to_string(),
        summary.parity_verified(),
    )
}
