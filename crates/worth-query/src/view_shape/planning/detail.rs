use super::ViewShapePlanningPosture;
use crate::view_shape::admission::AdmittedViewShape;
use crate::view_shape::delivery::{
    ViewShapeDeliveryMetadata, ViewShapeInvalidationPosture, ViewShapePatchPosture,
};
use crate::view_shape::grouped_maintenance::ViewShapeMaintenanceContract;
use crate::view_shape::performance::ViewShapeCostClass;

pub(super) fn planning_posture(admitted: &AdmittedViewShape) -> ViewShapePlanningPosture {
    ViewShapePlanningPosture {
        delivery_metadata: ViewShapeDeliveryMetadata::new(
            None,
            None,
            admitted.identity_binding().identity_consumption().clone(),
            true,
            false,
            false,
        ),
        invalidation_posture: ViewShapeInvalidationPosture::DetailProjectionFields,
        patch_posture: ViewShapePatchPosture::DetailFieldPatch,
        cost_class: ViewShapeCostClass::DetailProjection,
        maintenance_contract: ViewShapeMaintenanceContract::Ungrouped,
    }
}
