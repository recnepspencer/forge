use crate::access::execution::{access_lowering, DegradedScanReadinessView};
use crate::planning::AccessPlanSelectionView;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_physical_format::PlatformPhysicalFacade;

use super::{denial::DegradedExactScanExecutionDenied, request::DegradedExactScanExecutionRequest};

pub(super) fn execute(
    request: DegradedExactScanExecutionRequest<'_>,
    physical: &mut PlatformPhysicalFacade,
) -> Result<crate::DegradedScanExecution, DegradedExactScanExecutionDenied> {
    let declarations = crate::layout_declarations();
    let declaration = declarations
        .declaration(DurableArtifactFamilyId::PhysicalPage)
        .map_err(DegradedExactScanExecutionDenied::ArtifactFamily)?;
    let family = declarations
        .admit_physical_artifact_family(declaration, request.security)
        .map_err(DegradedExactScanExecutionDenied::ArtifactFamily)?;
    let key_domain = declarations
        .admit_physical_key_domain(family, request.security)
        .map_err(DegradedExactScanExecutionDenied::KeyDomain)?;
    let concrete_key = declarations
        .admit_page_key(key_domain, request.segment, request.page)
        .map_err(DegradedExactScanExecutionDenied::ConcreteKey)?;
    let materialization = crate::access_planning()
        .admit_current_catalog_root_materialization(family, request.catalog)
        .map_err(DegradedExactScanExecutionDenied::Materialization)?;
    let frontier = crate::access_planning().current_materialization_frontier(request.catalog);
    let shape = crate::access_shapes()
        .explicit_degraded_exact_scan(
            crate::DegradedExactScanRequest::new().with_budget_rows(request.budget_rows),
        )
        .map_err(DegradedExactScanExecutionDenied::Shape)?;
    let admitted = crate::AccessPlanSelector
        .admit_read_request(family, concrete_key, materialization, shape)
        .map_err(DegradedExactScanExecutionDenied::RequestAdmission)?;
    let outcome = crate::AccessPlanSelector.select_admitted_with_budget(admitted, request.budget);
    let selected = match outcome.view() {
        AccessPlanSelectionView::Degraded(_) => outcome
            .into_degraded()
            .expect("view established degraded selection"),
        AccessPlanSelectionView::Denied(denial) => {
            return Err(DegradedExactScanExecutionDenied::Selection(denial.clone()))
        }
        _ => return Err(DegradedExactScanExecutionDenied::UnexpectedSelectedOperation),
    };
    let readiness = access_lowering()
        .admit_degraded_ready(access_lowering().lower_degraded(selected), frontier);
    let ready = match readiness.view() {
        DegradedScanReadinessView::Ready(_) => readiness
            .into_ready()
            .expect("view established degraded readiness"),
        DegradedScanReadinessView::Stale(stale) => {
            return Err(DegradedExactScanExecutionDenied::Stale(
                stale.stale_materialization().clone(),
            ))
        }
    };
    access_lowering()
        .execute_physical_degraded_exact_scan(ready, physical)
        .map_err(DegradedExactScanExecutionDenied::Physical)
}
