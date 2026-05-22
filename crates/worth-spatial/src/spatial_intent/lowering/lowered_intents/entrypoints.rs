use forge_proof::TransitionOutcome;

use crate::spatial_intent::constraints::{
    AdmittedSpatialAnchorMatchConstraint, AdmittedSpatialLiesOnConstraint,
    AdmittedSpatialPointsTowardConstraint,
};
use crate::spatial_intent::lowering::{
    AdmittedSpatialMove, AdmittedSpatialOffset, AdmittedSpatialReorient, AdmittedSpatialRotate,
    SpatialPlacementSpec,
};
use crate::spatial_intent::refs::{EmptySpatialWitnessCatalog, SpatialWitnessCatalog};

use super::progression::{
    admit_requested_intent, lower_admitted_intent, request_intent, RequestedLoweringIntent,
};
use super::{LoweredSpatialIntentArtifact, SpatialLoweringDenial};

macro_rules! lower_entry {
    ($name:ident, $with_catalog:ident, $variant:ident, $arg:ty) => {
        pub fn $name(
            placement: SpatialPlacementSpec,
            arg: &$arg,
        ) -> Result<LoweredSpatialIntentArtifact, SpatialLoweringDenial> {
            $with_catalog(placement, arg, &EmptySpatialWitnessCatalog)
        }

        pub fn $with_catalog(
            placement: SpatialPlacementSpec,
            arg: &$arg,
            catalog: &impl SpatialWitnessCatalog,
        ) -> Result<LoweredSpatialIntentArtifact, SpatialLoweringDenial> {
            let requested =
                request_intent(RequestedLoweringIntent::$variant(placement, arg.clone()));
            let admitted = match admit_requested_intent(requested) {
                TransitionOutcome::Success(value) => value,
                TransitionOutcome::Denied(denial) => return Err(denial),
                _ => unreachable!(),
            };
            match lower_admitted_intent(admitted, catalog) {
                TransitionOutcome::Success(value) => Ok(value),
                TransitionOutcome::Denied(denial) => Err(denial),
                _ => unreachable!(),
            }
        }
    };
}

lower_entry!(
    lower_admitted_move_intent,
    lower_admitted_move_intent_with_catalog,
    Move,
    AdmittedSpatialMove
);
lower_entry!(
    lower_admitted_offset_intent,
    lower_admitted_offset_intent_with_catalog,
    Offset,
    AdmittedSpatialOffset
);
lower_entry!(
    lower_admitted_rotate_intent,
    lower_admitted_rotate_intent_with_catalog,
    Rotate,
    AdmittedSpatialRotate
);
lower_entry!(
    lower_admitted_reorient_intent,
    lower_admitted_reorient_intent_with_catalog,
    Reorient,
    AdmittedSpatialReorient
);
lower_entry!(
    lower_admitted_lies_on_constraint_intent,
    lower_admitted_lies_on_constraint_intent_with_catalog,
    LiesOn,
    AdmittedSpatialLiesOnConstraint
);
lower_entry!(
    lower_admitted_points_toward_constraint_intent,
    lower_admitted_points_toward_constraint_intent_with_catalog,
    PointsToward,
    AdmittedSpatialPointsTowardConstraint
);
lower_entry!(
    lower_admitted_anchor_match_constraint_intent,
    lower_admitted_anchor_match_constraint_intent_with_catalog,
    AnchorMatch,
    AdmittedSpatialAnchorMatchConstraint
);
