use forge_query::facade::ForgeQueryWorkspace;
use worth_spatial::facade::{admit_spatial_placement, SpatialFrameRef};

use super::super::execution::prepare_corpus_execution_proof_ingredients;
use super::super::ordering::{lane_digest as ordering_lane_digest, normalized_matrix_digest};
use super::cases::{
    PrimitiveConstructionCompoundGrazingPlan, PrimitiveConstructionCompoundScenario,
};
use super::lane_report::PrimitiveConstructionCompoundAuthoringOrderRow;
use super::rows::PrimitiveConstructionCompoundRow;
use super::schema::{
    PrimitiveConstructionCompoundRowClass, PrimitiveConstructionCompoundTopologyClass,
    PrimitiveConstructionCompoundWorkloadFamily,
};
use crate::construction::diagnostics::prepare_primitive_construction_rejection_locality_report;
use crate::construction::digest::digest_owned_parts;
use crate::construction::outcome::PrimitiveConstructionPreparedOutcome;
use crate::construction::result::prepare_primitive_construction_result;
use crate::construction::{
    prepare_primitive_construction_query_inspection_parity_report,
    prepare_primitive_construction_query_projection_consumption_receipt_report,
    PrimitiveConstructionIntent, PrimitiveConstructionQueryInspectionParityError,
    PrimitiveConstructionQueryProjectionConsumptionReceiptError,
};

use super::super::row_support::{
    birth_attachment_breadth, certification_breadth, construction_breadth,
};
use super::builder::PrimitiveConstructionCompoundAdversarialSiegeError;

pub(super) fn build_rows_for_lane(
    workspace: &mut ForgeQueryWorkspace,
    scenarios: &[PrimitiveConstructionCompoundScenario],
) -> Result<Vec<PrimitiveConstructionCompoundRow>, PrimitiveConstructionCompoundAdversarialSiegeError>
{
    let mut rows = scenarios
        .iter()
        .map(|scenario| build_row(workspace, scenario))
        .collect::<Result<Vec<_>, _>>()?;
    rows.push(mixed_topology_batch_row(&rows));
    Ok(rows)
}

pub(super) fn authoring_order_row(
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

pub(super) fn compute_normalized_matrix_digest(
    rows: &[PrimitiveConstructionCompoundRow],
) -> String {
    normalized_matrix_digest(
        rows.iter()
            .map(|row| (row.scenario_id().to_string(), row.row_digest().to_string())),
    )
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
    let execution = prepare_corpus_execution_proof_ingredients(
        workspace,
        intent.clone(),
        || {
            PrimitiveConstructionCompoundAdversarialSiegeError::ReplayParityDrift(format!(
                "compound siege row '{}' lost replay parity",
                scenario.scenario_id
            ))
        },
        || {
            PrimitiveConstructionCompoundAdversarialSiegeError::BranchLocalParityDrift(format!(
                "compound siege row '{}' lost branch-local parity",
                scenario.scenario_id
            ))
        },
    )?;
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

    match execution.direct_outcome().clone() {
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
                execution.replay_digest().to_string(),
                execution.branch_digest().to_string(),
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
                execution.replay_digest().to_string(),
                execution.branch_digest().to_string(),
                None,
                None,
                None,
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

fn lane_digest(rows: &[PrimitiveConstructionCompoundRow]) -> String {
    ordering_lane_digest(rows.iter().map(|row| row.row_digest().to_string()))
}

fn mixed_topology_batch_row(
    rows: &[PrimitiveConstructionCompoundRow],
) -> PrimitiveConstructionCompoundRow {
    let mut constituent = rows
        .iter()
        .filter(|row| {
            row.topology_class() == PrimitiveConstructionCompoundTopologyClass::ClosedSolid
                || row.topology_class() == PrimitiveConstructionCompoundTopologyClass::OpenShell
                || row.topology_class() == PrimitiveConstructionCompoundTopologyClass::OpenWire
        })
        .map(|row| format!("{}:{}", row.scenario_id(), row.row_digest()))
        .collect::<Vec<_>>();
    constituent.sort();
    let aggregate_digest = digest_owned_parts(&constituent);
    PrimitiveConstructionCompoundRow::new(
        "mixed_topology_class_batch".to_string(),
        PrimitiveConstructionCompoundWorkloadFamily::MixedTopologyClassBatch,
        PrimitiveConstructionCompoundTopologyClass::MixedBatch,
        PrimitiveConstructionCompoundRowClass::MixedTopologyBatch,
        aggregate_digest.clone(),
        aggregate_digest.clone(),
        aggregate_digest,
        None,
        None,
        None,
        Vec::new(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        rows.iter().map(|row| row.construction_breadth()).sum(),
        rows.iter().map(|row| row.birth_attachment_breadth()).sum(),
        rows.iter().map(|row| row.certification_breadth()).sum(),
    )
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
