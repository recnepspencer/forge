use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::{PrimitiveConstructionFamily, PrimitiveConstructionIntent};
use crate::spatial_intent::{
    MoveSpatialIntent, PointsTowardSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent,
};
use worth_spatial::facade::{
    admit_spatial_move_with_catalog, admit_spatial_points_toward_constraint_with_catalog,
    admit_spatial_reorient_with_catalog, admit_spatial_rotate_with_catalog, SpatialAnchorRef,
    SpatialConstraintError, SpatialDirectionWitnessRef, SpatialMotionError, SpatialPointWitnessRef,
    SpatialWitnessCatalog, SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionMotionWitnessResolutionKind {
    Move,
    Rotate,
    Reorient,
    PointsToward,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionMotionWitnessResolutionStatus {
    Admitted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionMotionWitnessResolutionFailureKind {
    Witness(SpatialWitnessFailureClass),
    NonFiniteRotationAngle,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveConstructionRequestedMotionWitness {
    Point(SpatialPointWitnessRef),
    Direction(SpatialDirectionWitnessRef),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrimitiveConstructionResolvedMotionWitness {
    Point([f64; 3]),
    Direction([f64; 3]),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMotionWitnessResolutionReport {
    kind: PrimitiveConstructionMotionWitnessResolutionKind,
    subject_family: PrimitiveConstructionFamily,
    anchor: SpatialAnchorRef,
    requested_witness: PrimitiveConstructionRequestedMotionWitness,
    status: PrimitiveConstructionMotionWitnessResolutionStatus,
    resolved_witness: Option<PrimitiveConstructionResolvedMotionWitness>,
    resolution_class: Option<SpatialWitnessResolutionClass>,
    failure_kind: Option<PrimitiveConstructionMotionWitnessResolutionFailureKind>,
    report_digest: String,
}

impl PrimitiveConstructionMotionWitnessResolutionReport {
    pub fn kind(&self) -> PrimitiveConstructionMotionWitnessResolutionKind {
        self.kind
    }

    pub fn subject_family(&self) -> PrimitiveConstructionFamily {
        self.subject_family
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn requested_witness(&self) -> &PrimitiveConstructionRequestedMotionWitness {
        &self.requested_witness
    }

    pub fn requested_point_witness(&self) -> Option<&SpatialPointWitnessRef> {
        match &self.requested_witness {
            PrimitiveConstructionRequestedMotionWitness::Point(witness) => Some(witness),
            PrimitiveConstructionRequestedMotionWitness::Direction(_) => None,
        }
    }

    pub fn requested_direction_witness(&self) -> Option<&SpatialDirectionWitnessRef> {
        match &self.requested_witness {
            PrimitiveConstructionRequestedMotionWitness::Direction(witness) => Some(witness),
            PrimitiveConstructionRequestedMotionWitness::Point(_) => None,
        }
    }

    pub fn status(&self) -> PrimitiveConstructionMotionWitnessResolutionStatus {
        self.status
    }

    pub fn resolved_witness(&self) -> Option<PrimitiveConstructionResolvedMotionWitness> {
        self.resolved_witness
    }

    pub fn resolved_target_point(&self) -> Option<[f64; 3]> {
        match self.resolved_witness {
            Some(PrimitiveConstructionResolvedMotionWitness::Point(point)) => Some(point),
            Some(PrimitiveConstructionResolvedMotionWitness::Direction(_)) | None => None,
        }
    }

    pub fn resolved_world_direction(&self) -> Option<[f64; 3]> {
        match self.resolved_witness {
            Some(PrimitiveConstructionResolvedMotionWitness::Direction(direction)) => {
                Some(direction)
            }
            Some(PrimitiveConstructionResolvedMotionWitness::Point(_)) | None => None,
        }
    }

    pub fn resolution_class(&self) -> Option<SpatialWitnessResolutionClass> {
        self.resolution_class
    }

    pub fn failure_kind(&self) -> Option<PrimitiveConstructionMotionWitnessResolutionFailureKind> {
        self.failure_kind
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_move_witness_resolution_report(
    intent: MoveSpatialIntent<PrimitiveConstructionIntent>,
) -> PrimitiveConstructionMotionWitnessResolutionReport {
    prepare_primitive_construction_move_witness_resolution_report_with_catalog(
        intent,
        &worth_spatial::facade::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_move_witness_resolution_report_with_catalog(
    intent: MoveSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> PrimitiveConstructionMotionWitnessResolutionReport {
    let subject_family = intent.subject().family();
    let spec = intent.motion_spec();
    match admit_spatial_move_with_catalog(spec.clone(), catalog) {
        Ok(admitted) => build_report(
            PrimitiveConstructionMotionWitnessResolutionKind::Move,
            subject_family,
            spec.anchor().clone(),
            PrimitiveConstructionRequestedMotionWitness::Point(spec.destination_witness().clone()),
            PrimitiveConstructionMotionWitnessResolutionStatus::Admitted,
            Some(PrimitiveConstructionResolvedMotionWitness::Point(
                admitted.destination_point(),
            )),
            Some(admitted.resolved_destination_witness().resolution_class()),
            None,
        ),
        Err(SpatialMotionError::DestinationWitnessFailure(class)) => build_report(
            PrimitiveConstructionMotionWitnessResolutionKind::Move,
            subject_family,
            spec.anchor().clone(),
            PrimitiveConstructionRequestedMotionWitness::Point(spec.destination_witness().clone()),
            PrimitiveConstructionMotionWitnessResolutionStatus::Rejected,
            None,
            None,
            Some(PrimitiveConstructionMotionWitnessResolutionFailureKind::Witness(class)),
        ),
        Err(other) => unreachable!("unexpected move admission error for witness report: {other}"),
    }
}

pub fn prepare_primitive_construction_rotate_witness_resolution_report(
    intent: RotateSpatialIntent<PrimitiveConstructionIntent>,
) -> PrimitiveConstructionMotionWitnessResolutionReport {
    prepare_primitive_construction_rotate_witness_resolution_report_with_catalog(
        intent,
        &worth_spatial::facade::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_rotate_witness_resolution_report_with_catalog(
    intent: RotateSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> PrimitiveConstructionMotionWitnessResolutionReport {
    let subject_family = intent.subject().family();
    let spec = intent.motion_spec();
    match admit_spatial_rotate_with_catalog(spec.clone(), catalog) {
        Ok(admitted) => build_report(
            PrimitiveConstructionMotionWitnessResolutionKind::Rotate,
            subject_family,
            spec.anchor().clone(),
            PrimitiveConstructionRequestedMotionWitness::Direction(spec.axis_witness().clone()),
            PrimitiveConstructionMotionWitnessResolutionStatus::Admitted,
            Some(PrimitiveConstructionResolvedMotionWitness::Direction(
                admitted.normalized_axis(),
            )),
            Some(admitted.resolved_axis_witness().resolution_class()),
            None,
        ),
        Err(SpatialMotionError::RotationWitnessFailure(class)) => build_report(
            PrimitiveConstructionMotionWitnessResolutionKind::Rotate,
            subject_family,
            spec.anchor().clone(),
            PrimitiveConstructionRequestedMotionWitness::Direction(spec.axis_witness().clone()),
            PrimitiveConstructionMotionWitnessResolutionStatus::Rejected,
            None,
            None,
            Some(PrimitiveConstructionMotionWitnessResolutionFailureKind::Witness(class)),
        ),
        Err(SpatialMotionError::NonFiniteRotationAngle) => build_report(
            PrimitiveConstructionMotionWitnessResolutionKind::Rotate,
            subject_family,
            spec.anchor().clone(),
            PrimitiveConstructionRequestedMotionWitness::Direction(spec.axis_witness().clone()),
            PrimitiveConstructionMotionWitnessResolutionStatus::Rejected,
            None,
            None,
            Some(PrimitiveConstructionMotionWitnessResolutionFailureKind::NonFiniteRotationAngle),
        ),
        Err(other) => {
            unreachable!("unexpected rotate admission error for witness report: {other}")
        }
    }
}

pub fn prepare_primitive_construction_reorient_witness_resolution_report(
    intent: ReorientSpatialIntent<PrimitiveConstructionIntent>,
) -> PrimitiveConstructionMotionWitnessResolutionReport {
    prepare_primitive_construction_reorient_witness_resolution_report_with_catalog(
        intent,
        &worth_spatial::facade::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_reorient_witness_resolution_report_with_catalog(
    intent: ReorientSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> PrimitiveConstructionMotionWitnessResolutionReport {
    let subject_family = intent.subject().family();
    let spec = intent.motion_spec();
    match admit_spatial_reorient_with_catalog(spec.clone(), catalog) {
        Ok(admitted) => build_report(
            PrimitiveConstructionMotionWitnessResolutionKind::Reorient,
            subject_family,
            spec.anchor().clone(),
            PrimitiveConstructionRequestedMotionWitness::Direction(
                spec.direction_witness().clone(),
            ),
            PrimitiveConstructionMotionWitnessResolutionStatus::Admitted,
            Some(PrimitiveConstructionResolvedMotionWitness::Direction(
                admitted.normalized_facing(),
            )),
            Some(admitted.resolved_direction_witness().resolution_class()),
            None,
        ),
        Err(SpatialMotionError::DirectionWitnessFailure(class)) => build_report(
            PrimitiveConstructionMotionWitnessResolutionKind::Reorient,
            subject_family,
            spec.anchor().clone(),
            PrimitiveConstructionRequestedMotionWitness::Direction(
                spec.direction_witness().clone(),
            ),
            PrimitiveConstructionMotionWitnessResolutionStatus::Rejected,
            None,
            None,
            Some(PrimitiveConstructionMotionWitnessResolutionFailureKind::Witness(class)),
        ),
        Err(other) => {
            unreachable!("unexpected reorient admission error for witness report: {other}")
        }
    }
}

pub fn prepare_primitive_construction_points_toward_witness_resolution_report(
    intent: PointsTowardSpatialIntent<PrimitiveConstructionIntent>,
) -> PrimitiveConstructionMotionWitnessResolutionReport {
    prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog(
        intent,
        &worth_spatial::facade::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog(
    intent: PointsTowardSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> PrimitiveConstructionMotionWitnessResolutionReport {
    let subject_family = intent.subject().family();
    let spec = intent.constraint_spec().clone();
    match admit_spatial_points_toward_constraint_with_catalog(spec.clone(), catalog) {
        Ok(admitted) => build_report(
            PrimitiveConstructionMotionWitnessResolutionKind::PointsToward,
            subject_family,
            spec.anchor().clone(),
            PrimitiveConstructionRequestedMotionWitness::Point(spec.target_witness().clone()),
            PrimitiveConstructionMotionWitnessResolutionStatus::Admitted,
            Some(PrimitiveConstructionResolvedMotionWitness::Point(
                admitted.target_point(),
            )),
            Some(admitted.resolved_target_witness().resolution_class()),
            None,
        ),
        Err(SpatialConstraintError::TargetWitnessFailure(class)) => build_report(
            PrimitiveConstructionMotionWitnessResolutionKind::PointsToward,
            subject_family,
            spec.anchor().clone(),
            PrimitiveConstructionRequestedMotionWitness::Point(spec.target_witness().clone()),
            PrimitiveConstructionMotionWitnessResolutionStatus::Rejected,
            None,
            None,
            Some(PrimitiveConstructionMotionWitnessResolutionFailureKind::Witness(class)),
        ),
        Err(other) => {
            unreachable!("unexpected points-toward admission error for witness report: {other}")
        }
    }
}

fn build_report(
    kind: PrimitiveConstructionMotionWitnessResolutionKind,
    subject_family: PrimitiveConstructionFamily,
    anchor: SpatialAnchorRef,
    requested_witness: PrimitiveConstructionRequestedMotionWitness,
    status: PrimitiveConstructionMotionWitnessResolutionStatus,
    resolved_witness: Option<PrimitiveConstructionResolvedMotionWitness>,
    resolution_class: Option<SpatialWitnessResolutionClass>,
    failure_kind: Option<PrimitiveConstructionMotionWitnessResolutionFailureKind>,
) -> PrimitiveConstructionMotionWitnessResolutionReport {
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &[
            format!("{kind:?}"),
            subject_family.as_str().to_string(),
            format!("{anchor:?}"),
            format!("{requested_witness:?}"),
            format!("{status:?}"),
            format!("{resolved_witness:?}"),
            format!("{resolution_class:?}"),
            format!("{failure_kind:?}"),
        ],
    );
    PrimitiveConstructionMotionWitnessResolutionReport {
        kind,
        subject_family,
        anchor,
        requested_witness,
        status,
        resolved_witness,
        resolution_class,
        failure_kind,
        report_digest,
    }
}
