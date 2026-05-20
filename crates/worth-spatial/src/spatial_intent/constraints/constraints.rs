use crate::spatial_intent::refs::{
    admit_spatial_frame, AdmittedSpatialFrameRef, EmptySpatialWitnessCatalog, SpatialAnchorRef,
    SpatialFrameError, SpatialFrameRef, SpatialPointWitnessRef, SpatialWitnessCatalog,
};
use crate::spatial_intent::resolution::{
    resolve_spatial_point_witness_with_catalog, ResolvedSpatialPointWitness,
    SpatialWitnessFailureClass,
};

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialLiesOnConstraintSpec {
    anchor: SpatialAnchorRef,
    frame: SpatialFrameRef,
}

impl SpatialLiesOnConstraintSpec {
    pub fn new(anchor: SpatialAnchorRef, frame: SpatialFrameRef) -> Self {
        Self { anchor, frame }
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn frame(&self) -> &SpatialFrameRef {
        &self.frame
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialPointsTowardConstraintSpec {
    anchor: SpatialAnchorRef,
    target_witness: SpatialPointWitnessRef,
}

impl SpatialPointsTowardConstraintSpec {
    pub fn new(anchor: SpatialAnchorRef, target_point: [f64; 3]) -> Self {
        Self::with_witness(anchor, SpatialPointWitnessRef::world_point(target_point))
    }

    pub fn with_witness(anchor: SpatialAnchorRef, target_witness: SpatialPointWitnessRef) -> Self {
        Self {
            anchor,
            target_witness,
        }
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn target_witness(&self) -> &SpatialPointWitnessRef {
        &self.target_witness
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialAnchorMatchConstraintSpec {
    anchor: SpatialAnchorRef,
    other_anchor: SpatialAnchorRef,
}

impl SpatialAnchorMatchConstraintSpec {
    pub fn new(anchor: SpatialAnchorRef, other_anchor: SpatialAnchorRef) -> Self {
        Self {
            anchor,
            other_anchor,
        }
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn other_anchor(&self) -> &SpatialAnchorRef {
        &self.other_anchor
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedSpatialLiesOnConstraint {
    spec: SpatialLiesOnConstraintSpec,
    frame: AdmittedSpatialFrameRef,
}

impl AdmittedSpatialLiesOnConstraint {
    pub fn spec(&self) -> &SpatialLiesOnConstraintSpec {
        &self.spec
    }

    pub fn frame(&self) -> &AdmittedSpatialFrameRef {
        &self.frame
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedSpatialPointsTowardConstraint {
    spec: SpatialPointsTowardConstraintSpec,
    resolved_target_witness: ResolvedSpatialPointWitness,
}

impl AdmittedSpatialPointsTowardConstraint {
    pub fn spec(&self) -> &SpatialPointsTowardConstraintSpec {
        &self.spec
    }

    pub fn resolved_target_witness(&self) -> &ResolvedSpatialPointWitness {
        &self.resolved_target_witness
    }

    pub fn target_point(&self) -> [f64; 3] {
        self.resolved_target_witness.resolved_world_point()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedSpatialAnchorMatchConstraint {
    spec: SpatialAnchorMatchConstraintSpec,
}

impl AdmittedSpatialAnchorMatchConstraint {
    pub fn spec(&self) -> &SpatialAnchorMatchConstraintSpec {
        &self.spec
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialConstraintError {
    TargetWitnessFailure(SpatialWitnessFailureClass),
    InvalidFrame(SpatialFrameError),
}

impl std::fmt::Display for SpatialConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TargetWitnessFailure(class) => {
                write!(f, "target witness failed with {class:?} semantics")
            }
            Self::InvalidFrame(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for SpatialConstraintError {}

pub fn admit_spatial_lies_on_constraint(
    spec: SpatialLiesOnConstraintSpec,
) -> Result<AdmittedSpatialLiesOnConstraint, SpatialConstraintError> {
    let frame =
        admit_spatial_frame(spec.frame.clone()).map_err(SpatialConstraintError::InvalidFrame)?;
    Ok(AdmittedSpatialLiesOnConstraint { spec, frame })
}

pub fn admit_spatial_points_toward_constraint(
    spec: SpatialPointsTowardConstraintSpec,
) -> Result<AdmittedSpatialPointsTowardConstraint, SpatialConstraintError> {
    admit_spatial_points_toward_constraint_with_catalog(spec, &EmptySpatialWitnessCatalog)
}

pub fn admit_spatial_points_toward_constraint_with_catalog(
    spec: SpatialPointsTowardConstraintSpec,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<AdmittedSpatialPointsTowardConstraint, SpatialConstraintError> {
    let resolved_target_witness =
        resolve_spatial_point_witness_with_catalog(spec.target_witness.clone(), catalog)
            .map_err(SpatialConstraintError::TargetWitnessFailure)?;
    Ok(AdmittedSpatialPointsTowardConstraint {
        spec,
        resolved_target_witness,
    })
}

pub fn admit_spatial_anchor_match_constraint(
    spec: SpatialAnchorMatchConstraintSpec,
) -> Result<AdmittedSpatialAnchorMatchConstraint, SpatialConstraintError> {
    Ok(AdmittedSpatialAnchorMatchConstraint { spec })
}

#[cfg(test)]
mod tests {
    use super::{
        admit_spatial_anchor_match_constraint, admit_spatial_lies_on_constraint,
        admit_spatial_points_toward_constraint, SpatialAnchorMatchConstraintSpec,
        SpatialConstraintError, SpatialLiesOnConstraintSpec, SpatialPointsTowardConstraintSpec,
    };
    use crate::facade::{
        SpatialAnchorRef, SpatialAxis, SpatialFrameRef, SpatialPointWitnessRef,
        SpatialWitnessFailureClass,
    };

    #[test]
    fn admitted_constraints_preserve_frame_anchor_and_target_truth() {
        let workplane = SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        let lies_on = admit_spatial_lies_on_constraint(SpatialLiesOnConstraintSpec::new(
            SpatialAnchorRef::shape_axis(SpatialAxis::W),
            workplane.clone(),
        ))
        .expect("lies on");
        let toward =
            admit_spatial_points_toward_constraint(SpatialPointsTowardConstraintSpec::new(
                SpatialAnchorRef::shape_origin(),
                [1.0, 0.0, 2.0],
            ))
            .expect("toward");
        let matched = admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            SpatialAnchorRef::frame_origin(workplane),
        ))
        .expect("match");

        assert_eq!(
            lies_on.spec().anchor(),
            &SpatialAnchorRef::shape_axis(SpatialAxis::W)
        );
        assert_eq!(lies_on.frame().basis().origin(), [0.0, 0.0, 5.0]);
        assert_eq!(toward.target_point(), [1.0, 0.0, 2.0]);
        assert_eq!(matched.spec().anchor(), &SpatialAnchorRef::shape_origin());
    }

    #[test]
    fn points_toward_constraint_preserves_target_witness_failure_semantics() {
        let err = admit_spatial_points_toward_constraint(
            SpatialPointsTowardConstraintSpec::with_witness(
                SpatialAnchorRef::shape_origin(),
                SpatialPointWitnessRef::ambiguous_surface_point("surface-1"),
            ),
        )
        .expect_err("ambiguous");

        assert_eq!(
            err,
            SpatialConstraintError::TargetWitnessFailure(SpatialWitnessFailureClass::Ambiguous)
        );
    }
}
