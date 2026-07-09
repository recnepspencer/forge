mod detail;
mod grouped;
mod inspector;
mod table;

use crate::basis::ExecutionBasisIntent;
use crate::canonicalization::CanonicalQueryBundle;
use crate::collection::CollectionResultFamily;
use crate::planning::{self, ExecutionPlanBundle};
use crate::schema_view::QuerySchemaView;
use crate::validation::validate_canonical_bundle;

use super::admission::AdmittedViewShape;
use super::error::{ViewShapeError, ViewShapeFailureClass};
use super::family::ViewShapeFamily;
use super::grouped_maintenance::ViewShapeMaintenanceContract;
use super::performance::{ViewShapeComplexityReport, ViewShapeCostClass};
use super::plan_artifact::{ViewShapePlanArtifact, ViewShapeValidatedBundle};
use super::support::runtime_backed_view_shape_complexity_status;

struct ViewShapePlanningPosture {
    delivery_metadata: super::delivery::ViewShapeDeliveryMetadata,
    invalidation_posture: super::delivery::ViewShapeInvalidationPosture,
    patch_posture: super::delivery::ViewShapePatchPosture,
    cost_class: ViewShapeCostClass,
    maintenance_contract: ViewShapeMaintenanceContract,
}

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

    let posture = planning_posture_for_validated_view(&validated_view, &execution_plan)?;
    let complexity = ViewShapeComplexityReport::new(
        runtime_backed_view_shape_complexity_status(validated_view.admitted().family()),
        posture.cost_class,
        execution_plan.query().fallback().clone(),
    );

    ViewShapePlanArtifact::new(
        validated_view,
        execution_plan,
        posture.delivery_metadata,
        posture.invalidation_posture,
        posture.patch_posture,
        complexity,
        posture.maintenance_contract,
    )
}

fn planning_posture_for_validated_view(
    validated_view: &ViewShapeValidatedBundle,
    execution_plan: &ExecutionPlanBundle,
) -> Result<ViewShapePlanningPosture, ViewShapeError> {
    match validated_view.admitted().family() {
        ViewShapeFamily::Table => Ok(table::planning_posture(validated_view.admitted())),
        ViewShapeFamily::Detail => Ok(detail::planning_posture(validated_view.admitted())),
        ViewShapeFamily::InspectorDetailObserved | ViewShapeFamily::InspectorDetailFocused => {
            Ok(inspector::planning_posture(validated_view.admitted()))
        }
        ViewShapeFamily::KanbanGrouped => grouped::planning_posture(
            validated_view.admitted(),
            validated_view.validated(),
            execution_plan,
        ),
    }
}
