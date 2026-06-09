use super::ViewShapePlanningPosture;
use crate::planning::ExecutionPlanBundle;
use crate::validation::ValidatedQueryBundle;
use crate::view_shape::admission::AdmittedViewShape;
use crate::view_shape::delivery::{
    ViewShapeDeliveryMetadata, ViewShapeInvalidationPosture, ViewShapePatchPosture,
};
use crate::view_shape::error::{ViewShapeError, ViewShapeFailureClass};
use crate::view_shape::family::ViewShapeFamily;
use crate::view_shape::grouped_maintenance::ViewShapeMaintenanceContract;
use crate::view_shape::grouped_planning::GroupedViewPlanningArtifact;
use crate::view_shape::performance::ViewShapeCostClass;

pub(super) fn planning_posture(
    admitted: &AdmittedViewShape,
    validated_view: &ValidatedQueryBundle,
    execution_plan: &ExecutionPlanBundle,
) -> Result<ViewShapePlanningPosture, ViewShapeError> {
    match admitted.family() {
        ViewShapeFamily::KanbanGrouped => {
            let grouping_aspect = admitted
                .descriptor()
                .native_grouping_aspect_key()
                .expect("grouped admission guarantees a native grouping aspect key");
            let grouped_planning = GroupedViewPlanningArtifact::derive(
                validated_view,
                execution_plan,
                grouping_aspect,
            )
            .expect("grouped admission guarantees identity and grouping bindings");

            Ok(ViewShapePlanningPosture {
                delivery_metadata: ViewShapeDeliveryMetadata::new(
                    None,
                    admitted.descriptor().native_grouping_aspect_key().cloned(),
                    admitted.identity_binding().identity_consumption().clone(),
                    false,
                    false,
                    true,
                ),
                invalidation_posture:
                    ViewShapeInvalidationPosture::KanbanGroupedMembershipAndAspect,
                patch_posture: ViewShapePatchPosture::KanbanGroupMembershipPatch,
                cost_class: ViewShapeCostClass::KanbanGroupedDeltaBound,
                maintenance_contract: ViewShapeMaintenanceContract::KanbanGrouped {
                    grouped_planning,
                },
            })
        }
        ViewShapeFamily::Table
        | ViewShapeFamily::Detail
        | ViewShapeFamily::InspectorDetailObserved
        | ViewShapeFamily::InspectorDetailFocused => Err(ViewShapeError::new(
            ViewShapeFailureClass::PlanningInvariantRejected,
            "grouped planning posture requested for a non-grouped view family",
        )),
    }
}
