use forge_query::facade::ForgeQueryWorkspace;

use super::cases::{
    canonical_order, compound_scenarios, reversed_order, topology_clustered_order,
    PrimitiveConstructionCompoundGrazingPlan, PrimitiveConstructionCompoundScenario,
};
use super::report::{
    PrimitiveConstructionCompoundAdversarialSiegeReport,
    PrimitiveConstructionCompoundGrazingBoundaryReport,
    PrimitiveConstructionCompoundMotionParityReport,
};
use super::schema::{
    PrimitiveConstructionCompoundAuthoringOrderRow,
    PrimitiveConstructionCompoundGrazingBoundaryRow, PrimitiveConstructionCompoundMotionParityRow,
    PrimitiveConstructionCompoundRow,
};
use crate::construction::diagnostics::prepare_primitive_construction_rejection_locality_report;
use crate::construction::digest::digest_owned_parts;
use crate::construction::outcome::PrimitiveConstructionPreparedOutcome;
use crate::construction::result::prepare_primitive_construction_result;
use crate::construction::{
    prepare_primitive_construction_branch_local_parity_report,
    prepare_primitive_construction_query_inspection_parity_report,
    prepare_primitive_construction_query_projection_consumption_receipt_report,
    prepare_primitive_construction_replay_parity_report, PrimitiveConstructionIntent,
    PrimitiveConstructionQueryInspectionParityError,
    PrimitiveConstructionQueryProjectionConsumptionReceiptError, PrimitiveConstructionResultError,
    PrimitiveConstructionRuntimeBasisError,
};
use crate::facade::PrimitiveConstructionSpatialIntentError;
use worth_spatial::facade::{admit_spatial_placement, SpatialFrameRef};

use super::super::row_support::{
    birth_attachment_breadth, certification_breadth, construction_breadth,
};

#[derive(Debug)]
pub enum PrimitiveConstructionCompoundAdversarialSiegeError {
    Motion(PrimitiveConstructionSpatialIntentError),
    Placement(String),
    RuntimeBasis(PrimitiveConstructionRuntimeBasisError),
    Result(PrimitiveConstructionResultError),
    InvalidRejectedLocality(String),
    Inspection(String),
    Projection(String),
}

impl std::fmt::Display for PrimitiveConstructionCompoundAdversarialSiegeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Motion(error) => write!(f, "{error}"),
            Self::Placement(error) => write!(f, "{error}"),
            Self::RuntimeBasis(error) => write!(f, "{error}"),
            Self::Result(error) => write!(f, "{error}"),
            Self::InvalidRejectedLocality(reason) => write!(f, "{reason}"),
            Self::Inspection(reason) => write!(f, "{reason}"),
            Self::Projection(reason) => write!(f, "{reason}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionCompoundAdversarialSiegeError {}

pub fn prepare_primitive_construction_compound_adversarial_siege_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionCompoundAdversarialSiegeReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let scenarios = compound_scenarios();
    let canonical = build_rows_for_lane(workspace, &canonical_order(&scenarios))?;
    let normalized = compute_normalized_matrix_digest(&canonical);
    let authoring_order_rows = vec![
        authoring_order_row("canonical", &canonical, &normalized),
        authoring_order_row(
            "reversed",
            &build_rows_for_lane(workspace, &reversed_order(&scenarios))?,
            &normalized,
        ),
        authoring_order_row(
            "topology_clustered",
            &build_rows_for_lane(workspace, &topology_clustered_order(&scenarios))?,
            &normalized,
        ),
    ];
    Ok(PrimitiveConstructionCompoundAdversarialSiegeReport::new(
        canonical,
        authoring_order_rows,
    ))
}

pub fn prepare_primitive_construction_compound_motion_parity_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionCompoundMotionParityReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let siege = prepare_primitive_construction_compound_adversarial_siege_report(workspace)?;
    let rows = siege
        .rows()
        .iter()
        .filter_map(|row| {
            Some(PrimitiveConstructionCompoundMotionParityRow::new(
                row.scenario_id().to_string(),
                row.motion_kind()?,
                row.motion_digest()?.to_string(),
            ))
        })
        .collect::<Vec<_>>();
    Ok(PrimitiveConstructionCompoundMotionParityReport::new(
        rows,
        siege.authoring_order_parity_verified(),
    ))
}

pub fn prepare_primitive_construction_compound_grazing_boundary_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionCompoundGrazingBoundaryReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let siege = prepare_primitive_construction_compound_adversarial_siege_report(workspace)?;
    let rows = siege
        .rows()
        .iter()
        .filter_map(|row| {
            Some(PrimitiveConstructionCompoundGrazingBoundaryRow::new(
                row.scenario_id().to_string(),
                row.grazing_kind()?,
                row.grazing_digest()?.to_string(),
            ))
        })
        .collect::<Vec<_>>();
    Ok(PrimitiveConstructionCompoundGrazingBoundaryReport::new(
        rows,
        siege.authoring_order_parity_verified(),
    ))
}

fn build_rows_for_lane(
    workspace: &mut ForgeQueryWorkspace,
    scenarios: &[PrimitiveConstructionCompoundScenario],
) -> Result<Vec<PrimitiveConstructionCompoundRow>, PrimitiveConstructionCompoundAdversarialSiegeError>
{
    scenarios
        .iter()
        .map(|scenario| build_row(workspace, scenario))
        .collect()
}

fn build_row(
    workspace: &mut ForgeQueryWorkspace,
    scenario: &PrimitiveConstructionCompoundScenario,
) -> Result<PrimitiveConstructionCompoundRow, PrimitiveConstructionCompoundAdversarialSiegeError> {
    let intent = scenario
        .resolved_intent()
        .map_err(PrimitiveConstructionCompoundAdversarialSiegeError::Motion)?;
    let admitted_placement = admit_spatial_placement(intent.placement_spec()).map_err(|error| {
        PrimitiveConstructionCompoundAdversarialSiegeError::Placement(error.to_string())
    })?;
    let replay = prepare_primitive_construction_replay_parity_report(intent.clone());
    let branch =
        prepare_primitive_construction_branch_local_parity_report(workspace, intent.clone())
            .map_err(PrimitiveConstructionCompoundAdversarialSiegeError::RuntimeBasis)?;
    let motion_digest = scenario.motion().map(|motion| {
        digest_owned_parts(&[
            motion.kind().as_str().to_string(),
            format!("{:?}", admitted_placement.origin().map(f64::to_bits)),
            format!("{:?}", admitted_placement.facing_vector().map(f64::to_bits)),
        ])
    });
    let grazing_digest = scenario.grazing().map(|grazing| {
        grazing_digest(
            grazing,
            admitted_placement.origin(),
            admitted_placement.facing_vector(),
        )
    });

    match replay.direct_outcome() {
        PrimitiveConstructionPreparedOutcome::Accepted(outcome) => {
            let inspection = prepare_primitive_construction_query_inspection_parity_report(
                workspace,
                intent.clone(),
            )
            .map_err(|error: PrimitiveConstructionQueryInspectionParityError| {
                PrimitiveConstructionCompoundAdversarialSiegeError::Inspection(error.to_string())
            })?;
            let projection =
                prepare_primitive_construction_query_projection_consumption_receipt_report(
                    workspace,
                    intent.clone(),
                )
                .map_err(
                    |error: PrimitiveConstructionQueryProjectionConsumptionReceiptError| {
                        PrimitiveConstructionCompoundAdversarialSiegeError::Projection(
                            error.to_string(),
                        )
                    },
                )?;
            let result = prepare_primitive_construction_result(intent.clone())
                .map_err(PrimitiveConstructionCompoundAdversarialSiegeError::Result)?;
            Ok(PrimitiveConstructionCompoundRow::new(
                scenario.scenario_id.to_string(),
                scenario.workload_family,
                scenario.topology_class,
                scenario.row_class,
                outcome.outcome_digest().to_string(),
                replay.replay_outcome().outcome_digest().to_string(),
                branch
                    .branch_preview_runtime_report()
                    .outcome()
                    .outcome_digest()
                    .to_string(),
                Some(inspection.report_digest().to_string()),
                Some(projection.report_digest().to_string()),
                Some(outcome.realization_strategy()),
                outcome.attempted_realization_strategies().to_vec(),
                Some(outcome.stability_class()),
                Some(outcome.feature_conditioning_class()),
                Some(outcome.support_normal_class()),
                Some(outcome.normalization_disposition()),
                None,
                None,
                None,
                None,
                scenario.motion().map(|motion| motion.kind()),
                motion_digest,
                scenario.grazing().map(|grazing| grazing.kind()),
                grazing_digest,
                construction_breadth(intent.request()).map_err(
                    PrimitiveConstructionCompoundAdversarialSiegeError::InvalidRejectedLocality,
                )?,
                birth_attachment_breadth(&result),
                certification_breadth(&result),
            ))
        }
        PrimitiveConstructionPreparedOutcome::Rejected(outcome) => {
            let rejection_locality = rejected_locality(intent.clone())?;
            Ok(PrimitiveConstructionCompoundRow::new(
                scenario.scenario_id.to_string(),
                scenario.workload_family,
                scenario.topology_class,
                scenario.row_class,
                outcome.failure_digest().to_string(),
                replay.replay_outcome().outcome_digest().to_string(),
                branch
                    .branch_preview_runtime_report()
                    .outcome()
                    .outcome_digest()
                    .to_string(),
                None,
                None,
                outcome.selected_realization_strategy(),
                outcome.attempted_realization_strategies().to_vec(),
                outcome.stability_class(),
                outcome.feature_conditioning_class(),
                outcome.support_normal_class(),
                outcome.normalization_disposition(),
                outcome.exhaustion_reason(),
                Some(outcome.rejection_class()),
                Some(rejection_locality.rejection_locality()),
                Some(rejection_locality.blocking_boundary()),
                scenario.motion().map(|motion| motion.kind()),
                motion_digest,
                scenario.grazing().map(|grazing| grazing.kind()),
                grazing_digest,
                0,
                0,
                0,
            ))
        }
    }
}

fn authoring_order_row(
    lane_name: &str,
    rows: &[PrimitiveConstructionCompoundRow],
    expected_normalized_matrix_digest: &str,
) -> PrimitiveConstructionCompoundAuthoringOrderRow {
    PrimitiveConstructionCompoundAuthoringOrderRow::new(
        lane_name.to_string(),
        lane_digest(rows),
        expected_normalized_matrix_digest.to_string(),
        expected_normalized_matrix_digest == compute_normalized_matrix_digest(rows),
    )
}

fn lane_digest(rows: &[PrimitiveConstructionCompoundRow]) -> String {
    digest_owned_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    )
}

fn compute_normalized_matrix_digest(rows: &[PrimitiveConstructionCompoundRow]) -> String {
    let mut parts = rows
        .iter()
        .map(|row| format!("{}:{}", row.scenario_id(), row.row_digest()))
        .collect::<Vec<_>>();
    parts.sort();
    digest_owned_parts(&parts)
}

fn grazing_digest(
    grazing: &PrimitiveConstructionCompoundGrazingPlan,
    origin: [f64; 3],
    facing: [f64; 3],
) -> String {
    match grazing {
        PrimitiveConstructionCompoundGrazingPlan::NearFrameNormal {
            frame,
            max_angle_radians,
        } => {
            let facing = normalize(facing);
            let normal = normalize(frame_normal(frame));
            let dot = (facing[0] * normal[0] + facing[1] * normal[1] + facing[2] * normal[2])
                .clamp(-1.0, 1.0);
            let angle = dot.acos();
            digest_owned_parts(&[
                "frame-normal".to_string(),
                angle.to_string(),
                max_angle_radians.to_string(),
                (angle <= *max_angle_radians).to_string(),
            ])
        }
        PrimitiveConstructionCompoundGrazingPlan::NearReferenceAnchor {
            reference_point,
            max_distance,
        } => {
            let dx = origin[0] - reference_point[0];
            let dy = origin[1] - reference_point[1];
            let dz = origin[2] - reference_point[2];
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();
            digest_owned_parts(&[
                "anchor-distance".to_string(),
                distance.to_string(),
                max_distance.to_string(),
                (distance <= *max_distance).to_string(),
            ])
        }
    }
}

fn rejected_locality(
    intent: PrimitiveConstructionIntent,
) -> Result<
    crate::construction::PrimitiveConstructionRejectionLocalityRow,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let report =
        prepare_primitive_construction_rejection_locality_report(vec![intent.into_request()]);
    match report.rows() {
        [row] => Ok(row.clone()),
        [] => Err(
            PrimitiveConstructionCompoundAdversarialSiegeError::InvalidRejectedLocality(
                "compound rejected row did not produce a locality row".to_string(),
            ),
        ),
        _ => Err(
            PrimitiveConstructionCompoundAdversarialSiegeError::InvalidRejectedLocality(
                "compound rejected row produced multiple locality rows".to_string(),
            ),
        ),
    }
}

fn frame_normal(frame: &SpatialFrameRef) -> [f64; 3] {
    match frame {
        SpatialFrameRef::World | SpatialFrameRef::ShapeLocal => [0.0, 0.0, 1.0],
        SpatialFrameRef::Workplane { normal, .. }
        | SpatialFrameRef::FeatureLocal { normal, .. } => *normal,
    }
}

fn normalize(vector: [f64; 3]) -> [f64; 3] {
    let magnitude = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
    [
        vector[0] / magnitude,
        vector[1] / magnitude,
        vector[2] / magnitude,
    ]
}
