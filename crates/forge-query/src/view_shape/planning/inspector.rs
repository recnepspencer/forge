use super::ViewShapePlanningPosture;
use crate::view_shape::admission::AdmittedViewShape;
use crate::view_shape::delivery::{
    ViewShapeDeliveryMetadata, ViewShapeInvalidationPosture, ViewShapePatchPosture,
};
use crate::view_shape::family::ViewShapeFamily;
use crate::view_shape::grouped_maintenance::ViewShapeMaintenanceContract;
use crate::view_shape::performance::ViewShapeCostClass;

pub(super) fn planning_posture(admitted: &AdmittedViewShape) -> ViewShapePlanningPosture {
    match admitted.family() {
        ViewShapeFamily::InspectorDetailObserved => ViewShapePlanningPosture {
            delivery_metadata: ViewShapeDeliveryMetadata::new(
                None,
                None,
                admitted.identity_binding().identity_consumption().clone(),
                true,
                false,
                false,
            ),
            invalidation_posture: ViewShapeInvalidationPosture::InspectorObservedNarrowDetail,
            patch_posture: ViewShapePatchPosture::ObservedInspectorPatch,
            cost_class: ViewShapeCostClass::InspectorObservedNarrow,
            maintenance_contract: ViewShapeMaintenanceContract::Ungrouped,
        },
        ViewShapeFamily::InspectorDetailFocused => ViewShapePlanningPosture {
            delivery_metadata: ViewShapeDeliveryMetadata::new(
                admitted
                    .descriptor()
                    .focused_aspect()
                    .map(ToString::to_string),
                None,
                admitted.identity_binding().identity_consumption().clone(),
                false,
                true,
                false,
            ),
            invalidation_posture: ViewShapeInvalidationPosture::InspectorFocusedAspect,
            patch_posture: ViewShapePatchPosture::FocusedInspectorAspectPatch,
            cost_class: ViewShapeCostClass::InspectorFocusedNarrow,
            maintenance_contract: ViewShapeMaintenanceContract::Ungrouped,
        },
        ViewShapeFamily::Table | ViewShapeFamily::Detail | ViewShapeFamily::KanbanGrouped => {
            unreachable!("inspector planning posture requested for a non-inspector view family")
        }
    }
}
