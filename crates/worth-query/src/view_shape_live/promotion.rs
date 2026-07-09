use crate::basis::{preflight_execution_basis, ResolvedSnapshotBasis};
use crate::identity_evolution::InspectorIdentityArtifact;
use crate::view_shape::{ViewShapeComplexityStatus, ViewShapePlanArtifact};

use super::artifact::{LiveViewShapeArtifact, ViewShapeLiveLowering};
use super::counters::ViewShapeLiveCounters;
use super::error::{ViewShapeLiveError, ViewShapeLiveFailureClass};
use super::family::LiveViewShapeFamily;
use super::grouped_baseline::AuthoritativeGroupedBaselineArtifact;

pub fn lower_view_shape_plan_to_live(
    plan: &ViewShapePlanArtifact,
    basis: ResolvedSnapshotBasis,
    grouped_baseline: Option<AuthoritativeGroupedBaselineArtifact>,
    inspector_identity: Option<InspectorIdentityArtifact>,
) -> Result<LiveViewShapeArtifact, ViewShapeLiveError> {
    if basis.identity().schema_basis() != plan.validated().query().schema_basis()
        || basis.identity().schema_basis() != plan.validated().result_shape().schema_basis()
    {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::BasisInvariantRejected,
            format!(
                "live lowering basis schema '{}' does not match validated query/result-shape schema '{}'",
                basis.identity().schema_basis().as_str(),
                plan.validated().query().schema_basis().as_str()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }

    let family = LiveViewShapeFamily::from(plan.family());
    match plan.delivery_metadata().identity_consumption() {
        crate::view_shape::ViewShapeIdentityConsumption::None => {
            if inspector_identity.is_some() {
                return Err(ViewShapeLiveError::new(
                    ViewShapeLiveFailureClass::InspectorIdentityBindingRejected,
                    "ordinary view-shape live lowering may not accept inspector identity evidence",
                    ViewShapeLiveCounters::default(),
                ));
            }
        }
        crate::view_shape::ViewShapeIdentityConsumption::InspectorIdentitySummary => {
            if inspector_identity.is_none() {
                return Err(ViewShapeLiveError::new(
                    ViewShapeLiveFailureClass::InspectorIdentityBindingRejected,
                    "identity-aware inspector summary lowering requires bound identity evidence",
                    ViewShapeLiveCounters::default(),
                ));
            }
        }
        crate::view_shape::ViewShapeIdentityConsumption::FocusedInspectorIdentityClassification(
            expected_classification,
        ) => {
            let Some(bound_identity) = inspector_identity.as_ref() else {
                return Err(ViewShapeLiveError::new(
                    ViewShapeLiveFailureClass::InspectorIdentityBindingRejected,
                    "identity-aware focused inspector lowering requires bound identity evidence",
                    ViewShapeLiveCounters::default(),
                ));
            };
            if bound_identity.classification() != *expected_classification {
                return Err(ViewShapeLiveError::new(
                    ViewShapeLiveFailureClass::InspectorIdentityBindingRejected,
                    format!(
                        "identity-aware focused inspector expected identity classification '{}' but received '{}'",
                        expected_classification.as_str(),
                        bound_identity.classification().as_str()
                    ),
                    ViewShapeLiveCounters::default(),
                ));
            }
        }
    }
    let (grouped_state, grouped_policy) = if family == LiveViewShapeFamily::KanbanGrouped {
        let Some(grouped_baseline) = grouped_baseline else {
            return Err(ViewShapeLiveError::new(
                ViewShapeLiveFailureClass::GroupedBaselineRequired,
                "kanban grouped live lowering requires an authoritative grouped baseline artifact",
                ViewShapeLiveCounters::default(),
            ));
        };
        if grouped_baseline.plan_digest().as_str() != plan.view_plan_digest().as_str() {
            return Err(ViewShapeLiveError::new(
                ViewShapeLiveFailureClass::GroupedBaselineMismatch,
                format!(
                    "grouped baseline plan digest '{}' does not match live plan digest '{}'",
                    grouped_baseline.plan_digest().as_str(),
                    plan.view_plan_digest().as_str()
                ),
                ViewShapeLiveCounters::default(),
            ));
        }
        if grouped_baseline.basis_digest() != basis.proof().digest() {
            return Err(ViewShapeLiveError::new(
                ViewShapeLiveFailureClass::GroupedBaselineMismatch,
                format!(
                    "grouped baseline basis digest '{}' does not match live basis digest '{}'",
                    grouped_baseline.basis_digest().as_str(),
                    basis.proof().digest().as_str()
                ),
                ViewShapeLiveCounters::default(),
            ));
        }
        let Some(grouped_policy) = plan.grouped_delta_policy().cloned() else {
            return Err(ViewShapeLiveError::new(
                ViewShapeLiveFailureClass::GroupedBaselineMismatch,
                "kanban grouped live lowering requires a planner-issued grouped delta policy",
                ViewShapeLiveCounters::default(),
            ));
        };
        (
            Some(grouped_baseline.desired_state().clone()),
            Some(grouped_policy),
        )
    } else {
        (None, None)
    };

    let preflight = preflight_execution_basis(plan.execution_plan().clone(), basis.clone())?;
    let core_live_plan = crate::live::promote_preflight_bundle_to_live(&preflight)?;
    if core_live_plan.descriptor().family() != &family.underlying_live_family() {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::UnderlyingLiveFamilyMismatch,
            format!(
                "view family '{}' expected live family '{}', but core promotion yielded '{}'",
                family.as_str(),
                family.underlying_live_family().as_str(),
                core_live_plan.descriptor().family().as_str()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }

    let mut counters = ViewShapeLiveCounters::default();
    if plan.complexity().status() == ViewShapeComplexityStatus::Debt {
        counters.add_complexity_status_debt();
    }
    if family == LiveViewShapeFamily::Table {
        counters.set_table_ordering_key_count(plan.validated().query().ordering().entries().len());
    }

    Ok(LiveViewShapeArtifact::new(
        plan.clone(),
        basis,
        ViewShapeLiveLowering::new(family),
        core_live_plan,
        counters,
        grouped_state,
        grouped_policy,
        inspector_identity,
    ))
}
