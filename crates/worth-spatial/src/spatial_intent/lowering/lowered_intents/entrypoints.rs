use crate::spatial_intent::constraints::{
    AdmittedSpatialAnchorMatchConstraint, AdmittedSpatialLiesOnConstraint,
    AdmittedSpatialPointsTowardConstraint,
};
use crate::spatial_intent::lowering::{
    AdmittedSpatialMove, AdmittedSpatialOffset, AdmittedSpatialReorient, AdmittedSpatialRotate,
    SpatialPlacementSpec,
};
use crate::spatial_intent::refs::{EmptySpatialWitnessCatalog, SpatialWitnessCatalog};

use super::progression::{admit_requested_intent, lower_admitted_intent, RequestedLoweringIntent};
use super::{LoweredSpatialIntent, SpatialLoweringDenial};

macro_rules! lower_entry {
    ($name:ident, $with_catalog:ident, $semantic_name:ident, $semantic_with_catalog:ident, $variant:ident, $arg:ty) => {
        pub(crate) fn $semantic_name(
            placement: SpatialPlacementSpec,
            arg: &$arg,
        ) -> Result<LoweredSpatialIntent, SpatialLoweringDenial> {
            $semantic_with_catalog(placement, arg, &EmptySpatialWitnessCatalog)
        }

        pub(crate) fn $semantic_with_catalog(
            placement: SpatialPlacementSpec,
            arg: &$arg,
            catalog: &impl SpatialWitnessCatalog,
        ) -> Result<LoweredSpatialIntent, SpatialLoweringDenial> {
            let requested = RequestedLoweringIntent::$variant(placement, arg.clone());
            let admitted = admit_requested_intent(requested)?;
            lower_admitted_intent(admitted, catalog)
        }

        pub fn $name(
            placement: SpatialPlacementSpec,
            arg: &$arg,
        ) -> Result<forge_query::facade::ForgeQueryIntentDeclaration, SpatialLoweringDenial> {
            $semantic_name(placement, arg).map(|intent| intent.to_query_intent_declaration())
        }

        pub fn $with_catalog(
            placement: SpatialPlacementSpec,
            arg: &$arg,
            catalog: &impl SpatialWitnessCatalog,
        ) -> Result<forge_query::facade::ForgeQueryIntentDeclaration, SpatialLoweringDenial> {
            $semantic_with_catalog(placement, arg, catalog)
                .map(|intent| intent.to_query_intent_declaration())
        }
    };
}

lower_entry!(
    lower_admitted_move_intent,
    lower_admitted_move_intent_with_catalog,
    lower_admitted_move_semantic_intent,
    lower_admitted_move_semantic_intent_with_catalog,
    Move,
    AdmittedSpatialMove
);
lower_entry!(
    lower_admitted_offset_intent,
    lower_admitted_offset_intent_with_catalog,
    lower_admitted_offset_semantic_intent,
    lower_admitted_offset_semantic_intent_with_catalog,
    Offset,
    AdmittedSpatialOffset
);
lower_entry!(
    lower_admitted_rotate_intent,
    lower_admitted_rotate_intent_with_catalog,
    lower_admitted_rotate_semantic_intent,
    lower_admitted_rotate_semantic_intent_with_catalog,
    Rotate,
    AdmittedSpatialRotate
);
lower_entry!(
    lower_admitted_reorient_intent,
    lower_admitted_reorient_intent_with_catalog,
    lower_admitted_reorient_semantic_intent,
    lower_admitted_reorient_semantic_intent_with_catalog,
    Reorient,
    AdmittedSpatialReorient
);
lower_entry!(
    lower_admitted_lies_on_constraint_intent,
    lower_admitted_lies_on_constraint_intent_with_catalog,
    lower_admitted_lies_on_constraint_semantic_intent,
    lower_admitted_lies_on_constraint_semantic_intent_with_catalog,
    LiesOn,
    AdmittedSpatialLiesOnConstraint
);
lower_entry!(
    lower_admitted_points_toward_constraint_intent,
    lower_admitted_points_toward_constraint_intent_with_catalog,
    lower_admitted_points_toward_constraint_semantic_intent,
    lower_admitted_points_toward_constraint_semantic_intent_with_catalog,
    PointsToward,
    AdmittedSpatialPointsTowardConstraint
);
lower_entry!(
    lower_admitted_anchor_match_constraint_intent,
    lower_admitted_anchor_match_constraint_intent_with_catalog,
    lower_admitted_anchor_match_constraint_semantic_intent,
    lower_admitted_anchor_match_constraint_semantic_intent_with_catalog,
    AnchorMatch,
    AdmittedSpatialAnchorMatchConstraint
);
