use crate::basis::ExecutionBasisIntent;
use crate::canonicalization::CanonicalQueryBundle;
use crate::collection::CollectionResultFamily;
use crate::planning::{self};
use crate::schema_view::QuerySchemaView;
use crate::validation::validate_canonical_bundle;

use super::admission::AdmittedViewShape;
use super::delivery::{
    ViewShapeDeliveryMetadata, ViewShapeInvalidationPosture, ViewShapePatchPosture,
};
use super::error::{ViewShapeError, ViewShapeFailureClass};
use super::family::ViewShapeFamily;
use super::grouped_maintenance::ViewShapeMaintenanceContract;
use super::grouped_planning::GroupedViewPlanningArtifact;
use super::performance::{
    ViewShapeComplexityReport, ViewShapeComplexityStatus, ViewShapeCostClass,
};
use super::plan_artifact::{ViewShapePlanArtifact, ViewShapeValidatedBundle};

pub fn validate_canonical_bundle_for_admitted_view_shape(
    canonical: &CanonicalQueryBundle,
    schema_view: QuerySchemaView,
    admitted: AdmittedViewShape,
) -> Result<ViewShapeValidatedBundle, ViewShapeError> {
    if admitted.digest().as_str()
        != super::admission::admit_view_shape(canonical, admitted.descriptor().clone())
            .map_err(|error| {
                ViewShapeError::new(
                    ViewShapeFailureClass::AdmissionInvariantRejected,
                    format!("re-admitting view shape failed: {:?}", error),
                )
            })?
            .digest()
            .as_str()
    {
        return Err(ViewShapeError::new(
            ViewShapeFailureClass::AdmissionInvariantRejected,
            "admitted view shape does not match the canonical bundle it was minted from",
        ));
    }

    let validated = validate_canonical_bundle(canonical.clone(), schema_view).map_err(|error| {
        ViewShapeError::new(
            ViewShapeFailureClass::ValidationRejected,
            format!("view-shape validation failed: {:?}", error),
        )
    })?;
    Ok(ViewShapeValidatedBundle::new(
        canonical.clone(),
        admitted,
        validated,
    ))
}

pub fn plan_admitted_view_shape(
    validated_view: ViewShapeValidatedBundle,
    basis_intent: ExecutionBasisIntent,
) -> Result<ViewShapePlanArtifact, ViewShapeError> {
    let request_context =
        planning::planning_request_context_for_direct(validated_view.validated(), basis_intent)
            .map_err(|error| {
                ViewShapeError::new(
                    ViewShapeFailureClass::PlanningRejected,
                    format!("view-shape planning request context rejected: {:?}", error),
                )
            })?;
    let execution_plan = match validated_view.admitted().family() {
        ViewShapeFamily::Table => planning::plan_validated_bundle_for_collection_family(
            validated_view.validated(),
            request_context,
            CollectionResultFamily::OrdinaryCollection,
        ),
        ViewShapeFamily::Detail
        | ViewShapeFamily::InspectorDetailObserved
        | ViewShapeFamily::InspectorDetailFocused => {
            planning::plan_validated_bundle(validated_view.validated(), request_context)
        }
        ViewShapeFamily::KanbanGrouped => planning::plan_validated_bundle_for_collection_family(
            validated_view.validated(),
            request_context,
            CollectionResultFamily::OrdinaryCollection,
        ),
    }
    .map_err(|error| {
        ViewShapeError::new(
            ViewShapeFailureClass::PlanningRejected,
            format!("view-shape planning rejected: {:?}", error),
        )
    })?;

    let (delivery_metadata, invalidation_posture, patch_posture, cost_class) =
        metadata_for_admitted_view(validated_view.admitted());
    let maintenance_contract = maintenance_contract_for_admitted_view(
        validated_view.admitted(),
        validated_view.validated(),
        &execution_plan,
    );
    let complexity = ViewShapeComplexityReport::new(
        ViewShapeComplexityStatus::Debt,
        cost_class,
        execution_plan.query().fallback().clone(),
    );

    ViewShapePlanArtifact::new(
        validated_view,
        execution_plan,
        delivery_metadata,
        invalidation_posture,
        patch_posture,
        complexity,
        maintenance_contract,
    )
}

fn metadata_for_admitted_view(
    admitted: &AdmittedViewShape,
) -> (
    ViewShapeDeliveryMetadata,
    ViewShapeInvalidationPosture,
    ViewShapePatchPosture,
    ViewShapeCostClass,
) {
    match admitted.family() {
        ViewShapeFamily::Table => (
            ViewShapeDeliveryMetadata::new(
                None,
                None,
                admitted.identity_binding().identity_consumption().clone(),
                false,
                false,
                false,
            ),
            ViewShapeInvalidationPosture::OrderedCollectionMembershipAndOrdering,
            ViewShapePatchPosture::TableRowPatch,
            ViewShapeCostClass::OrderedCollectionTable,
        ),
        ViewShapeFamily::Detail => (
            ViewShapeDeliveryMetadata::new(
                None,
                None,
                admitted.identity_binding().identity_consumption().clone(),
                true,
                false,
                false,
            ),
            ViewShapeInvalidationPosture::DetailProjectionFields,
            ViewShapePatchPosture::DetailFieldPatch,
            ViewShapeCostClass::DetailProjection,
        ),
        ViewShapeFamily::InspectorDetailObserved => (
            ViewShapeDeliveryMetadata::new(
                None,
                None,
                admitted.identity_binding().identity_consumption().clone(),
                true,
                false,
                false,
            ),
            ViewShapeInvalidationPosture::InspectorObservedNarrowDetail,
            ViewShapePatchPosture::ObservedInspectorPatch,
            ViewShapeCostClass::InspectorObservedNarrow,
        ),
        ViewShapeFamily::InspectorDetailFocused => (
            ViewShapeDeliveryMetadata::new(
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
            ViewShapeInvalidationPosture::InspectorFocusedAspect,
            ViewShapePatchPosture::FocusedInspectorAspectPatch,
            ViewShapeCostClass::InspectorFocusedNarrow,
        ),
        ViewShapeFamily::KanbanGrouped => (
            ViewShapeDeliveryMetadata::new(
                None,
                admitted
                    .descriptor()
                    .grouping_aspect()
                    .map(ToString::to_string),
                admitted.identity_binding().identity_consumption().clone(),
                false,
                false,
                true,
            ),
            ViewShapeInvalidationPosture::KanbanGroupedMembershipAndAspect,
            ViewShapePatchPosture::KanbanGroupMembershipPatch,
            ViewShapeCostClass::KanbanGroupedDeltaBound,
        ),
    }
}

fn maintenance_contract_for_admitted_view(
    admitted: &AdmittedViewShape,
    validated_view: &crate::validation::ValidatedQueryBundle,
    execution_plan: &crate::planning::ExecutionPlanBundle,
) -> ViewShapeMaintenanceContract {
    match admitted.family() {
        ViewShapeFamily::KanbanGrouped => {
            let grouped_planning = GroupedViewPlanningArtifact::derive(
                validated_view,
                execution_plan,
                admitted.descriptor().grouping_aspect().unwrap_or("none"),
            )
            .expect("grouped admission guarantees identity and grouping bindings");
            ViewShapeMaintenanceContract::KanbanGrouped { grouped_planning }
        }
        _ => ViewShapeMaintenanceContract::Ungrouped,
    }
}
