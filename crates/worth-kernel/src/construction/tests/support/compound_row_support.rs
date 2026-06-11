use worth_math::error::MathError;
use worth_math::numeric::metrics::{
    angle_between_unit_vectors, distance_between_points, FiniteNonNegativeF64, UnitVector3,
};
use worth_spatial::facade::refs::SpatialFrameRef;

use crate::construction::digest::digest_owned_parts;
use crate::construction::outcome::{
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
use crate::construction::tests::support::blocking_boundary::{
    blocking_boundary_for, PrimitiveConstructionBlockingBoundary,
};
use crate::construction::tests::support::branch_basis_digest::prepare_branch_basis_digest;
use crate::construction::tests::support::compound_corpus::compound_workspace;
use crate::construction::tests::support::compound_runtime::{
    compound_scenarios, PrimitiveConstructionCompoundAdversarialSiegeError,
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundGrazingPlan,
    PrimitiveConstructionCompoundMotionKind, PrimitiveConstructionCompoundMotionPlan,
    PrimitiveConstructionCompoundRow, PrimitiveConstructionCompoundScenario,
};
use crate::construction::tests::support::projection_consumption::prepare_primitive_construction_query_projection_consumption_surface_digest;
use crate::construction::tests::support::runtime_truth::{
    PrimitiveConstructionAdmittedRuntimeTruth, PrimitiveConstructionCertificationRuntimeTruth,
    PrimitiveConstructionRejectedRuntimeTruth,
};
use worth_geom::facade::{
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

pub(crate) fn realization_strategy(
    row: &PrimitiveConstructionCompoundRow,
) -> Option<PrimitiveRealizationStrategy> {
    admitted_runtime_truth(row).map(PrimitiveConstructionAdmittedRuntimeTruth::realization_strategy)
}

pub(crate) fn attempted_realization_strategies(
    row: &PrimitiveConstructionCompoundRow,
) -> &[PrimitiveRealizationStrategy] {
    runtime_truth_attempted_realization_strategies(row.runtime_truth())
}

pub(crate) fn stability_class(
    row: &PrimitiveConstructionCompoundRow,
) -> Option<PrimitiveStabilityClass> {
    runtime_truth_stability_class(row.runtime_truth())
}

pub(crate) fn outcome_digest(row: &PrimitiveConstructionCompoundRow) -> &str {
    row.runtime_truth().outcome_digest()
}

pub(crate) fn row_digest(row: &PrimitiveConstructionCompoundRow) -> String {
    let mut parts = vec![
        row.scenario_id().to_string(),
        row.workload_family().as_str().to_string(),
        row.topology_class().as_str().to_string(),
        row.row_class().as_str().to_string(),
        outcome_digest(row).to_string(),
        branch_basis_digest(row),
        query_surface_digest(row).unwrap_or_else(|| "none".to_string()),
        construction_breadth(row).to_string(),
    ];
    parts.push(
        motion_kind(row)
            .map(|kind| kind.as_str().to_string())
            .unwrap_or_else(|| "none".to_string()),
    );
    parts.push(motion_digest(row).unwrap_or_else(|| "none".to_string()));
    parts.push(
        grazing_kind(row)
            .map(|kind| kind.as_str().to_string())
            .unwrap_or_else(|| "none".to_string()),
    );
    parts.push(grazing_digest(row).unwrap_or_else(|| "none".to_string()));
    digest_owned_parts(&parts)
}

pub(crate) fn branch_basis_digest(row: &PrimitiveConstructionCompoundRow) -> String {
    let scenario = scenario_for(row).expect("compound row must map back to scenario");
    let intent = scenario
        .resolved_intent()
        .expect("compound row scenario must resolve intent");
    let mut workspace = compound_workspace(&format!(
        "worth-kernel.compound-branch-basis.{}",
        row.scenario_id()
    ));
    prepare_branch_basis_digest(&mut workspace, &intent)
        .expect("compound branch basis digest should rederive in test support")
}

pub(crate) fn query_surface_digest(row: &PrimitiveConstructionCompoundRow) -> Option<String> {
    admitted_runtime_truth(row)?;
    let scenario = scenario_for(row)?;
    let intent = scenario.resolved_intent().ok()?;
    let mut workspace = compound_workspace(&format!(
        "worth-kernel.compound-query-surface.{}",
        row.scenario_id()
    ));
    prepare_primitive_construction_query_projection_consumption_surface_digest(
        &mut workspace,
        intent,
    )
    .ok()
}

pub(crate) fn exhaustion_reason(
    row: &PrimitiveConstructionCompoundRow,
) -> Option<PrimitiveRealizationExhaustionReason> {
    rejected_runtime_truth(row)
        .and_then(PrimitiveConstructionRejectedRuntimeTruth::exhaustion_reason)
}

pub(crate) fn rejection_class(
    row: &PrimitiveConstructionCompoundRow,
) -> Option<PrimitiveConstructionRejectionClass> {
    rejected_runtime_truth(row).map(PrimitiveConstructionRejectedRuntimeTruth::rejection_class)
}

pub(crate) fn rejection_locality(
    row: &PrimitiveConstructionCompoundRow,
) -> Option<PrimitiveConstructionRejectionLocality> {
    rejected_runtime_truth(row).map(PrimitiveConstructionRejectedRuntimeTruth::rejection_locality)
}

pub(crate) fn blocking_boundary(
    row: &PrimitiveConstructionCompoundRow,
) -> Option<PrimitiveConstructionBlockingBoundary> {
    rejection_locality(row).map(blocking_boundary_for)
}

pub(crate) fn motion_kind(
    row: &PrimitiveConstructionCompoundRow,
) -> Option<PrimitiveConstructionCompoundMotionKind> {
    scenario_for(row)
        .and_then(|scenario: PrimitiveConstructionCompoundScenario| scenario.motion_kind())
}

pub(crate) fn grazing_kind(
    row: &PrimitiveConstructionCompoundRow,
) -> Option<PrimitiveConstructionCompoundGrazingKind> {
    scenario_for(row)
        .and_then(|scenario: PrimitiveConstructionCompoundScenario| scenario.grazing_kind())
}

pub(crate) fn motion_digest(row: &PrimitiveConstructionCompoundRow) -> Option<String> {
    let scenario = scenario_for(row)?;
    let motion = scenario.motion()?;
    match admitted_runtime_truth(row) {
        Some(outcome) => Some(digest_admitted_motion(motion, outcome)),
        None => scenario.resolved_intent().ok().map(
            |intent: crate::construction::intent::PrimitiveConstructionIntent| {
                requested_motion_digest(motion, intent.request())
            },
        ),
    }
}

pub(crate) fn grazing_digest(row: &PrimitiveConstructionCompoundRow) -> Option<String> {
    let scenario = scenario_for(row)?;
    let grazing = scenario.grazing()?;
    let outcome = admitted_runtime_truth(row)?;
    let placement_facts = outcome.placement_facts();
    grazing_digest_for_plan(
        grazing,
        placement_facts.origin(),
        placement_facts.facing_vector(),
    )
    .ok()
}

fn admitted_runtime_truth(
    row: &PrimitiveConstructionCompoundRow,
) -> Option<&PrimitiveConstructionAdmittedRuntimeTruth> {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => Some(&outcome),
        _ => None,
    }
}

fn rejected_runtime_truth(
    row: &PrimitiveConstructionCompoundRow,
) -> Option<&PrimitiveConstructionRejectedRuntimeTruth> {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => Some(&rejected),
        _ => None,
    }
}

fn runtime_truth_attempted_realization_strategies(
    runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth,
) -> &[PrimitiveRealizationStrategy] {
    match runtime_truth {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            outcome.attempted_realization_strategies()
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            rejected.attempted_realization_strategies()
        }
    }
}

fn runtime_truth_stability_class(
    runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth,
) -> Option<PrimitiveStabilityClass> {
    match runtime_truth {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            Some(outcome.stability_class())
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            rejected.stability_class()
        }
    }
}

fn construction_breadth(row: &PrimitiveConstructionCompoundRow) -> usize {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            outcome.topology_fact_breadth()
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(_) => 0,
    }
}

fn scenario_for(
    row: &PrimitiveConstructionCompoundRow,
) -> Option<PrimitiveConstructionCompoundScenario> {
    compound_scenarios()
        .into_iter()
        .find(|scenario| scenario.scenario_id() == row.scenario_id())
}

fn digest_admitted_motion(
    motion: &PrimitiveConstructionCompoundMotionPlan,
    outcome: &PrimitiveConstructionAdmittedRuntimeTruth,
) -> String {
    let placement_facts = outcome.placement_facts();
    crate::construction::digest::digest_owned_parts(&[
        motion.kind().as_str().to_string(),
        format!("{:?}", placement_facts.origin().map(f64::to_bits)),
        format!("{:?}", placement_facts.facing_vector().map(f64::to_bits)),
    ])
}

fn requested_motion_digest(
    motion: &PrimitiveConstructionCompoundMotionPlan,
    request: &crate::construction::request::PrimitiveConstructionRequest,
) -> String {
    let placement = request.placement_spec();
    let motion_projection = match motion {
        PrimitiveConstructionCompoundMotionPlan::Move { destination } => {
            format!("move:{:?}", destination.map(f64::to_bits))
        }
        PrimitiveConstructionCompoundMotionPlan::Offset { offset } => {
            format!("offset:{:?}", offset.map(f64::to_bits))
        }
        PrimitiveConstructionCompoundMotionPlan::Reorient { facing } => {
            format!("reorient:{:?}", facing.map(f64::to_bits))
        }
    };
    digest_owned_parts(&[
        motion.kind().as_str().to_string(),
        format!("{:?}", placement.origin().map(f64::to_bits)),
        format!("{:?}", placement.direction_witness()),
        format!("{:?}", placement.reference_frame()),
        motion_projection,
    ])
}

fn grazing_digest_for_plan(
    grazing: &PrimitiveConstructionCompoundGrazingPlan,
    origin: [f64; 3],
    facing: [f64; 3],
) -> Result<String, PrimitiveConstructionCompoundAdversarialSiegeError> {
    match grazing {
        PrimitiveConstructionCompoundGrazingPlan::NearFrameNormal {
            frame,
            max_angle_radians,
        } => {
            let angle =
                admitted_angle_between(facing, frame_normal(frame)).map_err(numeric_error)?;
            let max_angle = FiniteNonNegativeF64::try_new(*max_angle_radians, "max grazing angle")
                .map_err(numeric_error)?;
            Ok(digest_owned_parts(&[
                "frame-normal".to_string(),
                angle.get().to_string(),
                max_angle.get().to_string(),
                (angle.get() <= max_angle.get()).to_string(),
            ]))
        }
        PrimitiveConstructionCompoundGrazingPlan::NearReferenceAnchor {
            reference_point,
            max_distance,
        } => {
            let distance = admitted_distance(origin, *reference_point).map_err(numeric_error)?;
            let max_distance = FiniteNonNegativeF64::try_new(*max_distance, "max grazing distance")
                .map_err(numeric_error)?;
            Ok(digest_owned_parts(&[
                "anchor-distance".to_string(),
                distance.get().to_string(),
                max_distance.get().to_string(),
                (distance.get() <= max_distance.get()).to_string(),
            ]))
        }
    }
}

fn frame_normal(frame: &SpatialFrameRef) -> [f64; 3] {
    match frame {
        SpatialFrameRef::World | SpatialFrameRef::ShapeLocal => [0.0, 0.0, 1.0],
        SpatialFrameRef::Workplane { normal, .. }
        | SpatialFrameRef::FeatureLocal { normal, .. } => *normal,
    }
}

fn admitted_angle_between(
    source: [f64; 3],
    target: [f64; 3],
) -> Result<FiniteNonNegativeF64, MathError> {
    angle_between_unit_vectors(UnitVector3::try_new(source)?, UnitVector3::try_new(target)?)
}

fn admitted_distance(
    source: [f64; 3],
    target: [f64; 3],
) -> Result<FiniteNonNegativeF64, MathError> {
    distance_between_points(source, target)
}

fn numeric_error(error: MathError) -> PrimitiveConstructionCompoundAdversarialSiegeError {
    PrimitiveConstructionCompoundAdversarialSiegeError::NumericWitness(error.to_string())
}
