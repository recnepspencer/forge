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
            false,
            false,
            false,
        ),
        invalidation_posture: ViewShapeInvalidationPosture::OrderedCollectionMembershipAndOrdering,
        patch_posture: ViewShapePatchPosture::TableRowPatch,
        cost_class: ViewShapeCostClass::OrderedCollectionTable,
        maintenance_contract: ViewShapeMaintenanceContract::Ungrouped,
    }
}
