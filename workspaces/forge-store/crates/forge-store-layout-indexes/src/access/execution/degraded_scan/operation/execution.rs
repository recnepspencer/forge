use crate::access::execution::DegradedScanReadinessView;
use crate::planning::AccessPlanSelectionView;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_physical_format::PhysicalStoreRuntime;

use super::{denial::DegradedExactScanExecutionDenied, request::DegradedExactScanExecutionRequest};

pub(super) fn execute(
    request: DegradedExactScanExecutionRequest<'_>,
    physical: &mut PhysicalStoreRuntime,
) -> Result<crate::DegradedScanExecution, DegradedExactScanExecutionDenied> {
    let readiness = prepare(request)?;
    let ready = match readiness.view() {
        DegradedScanReadinessView::Ready(_) => readiness
            .into_ready()
            .expect("owner view established degraded readiness"),
        DegradedScanReadinessView::Stale(stale) => {
            return Err(DegradedExactScanExecutionDenied::Stale(
                stale.stale_materialization().clone(),
            ))
        }
    };
    execute_ready(ready, physical)
}

pub(super) fn prepare(
    request: DegradedExactScanExecutionRequest<'_>,
) -> Result<crate::DegradedScanReadinessOutcome, DegradedExactScanExecutionDenied> {
    let (selected, frontier) = select(request)?;
    Ok(super::super::classify_readiness(
        super::super::lower(selected),
        frontier,
    ))
}

pub(super) fn rebind(
    stale: crate::StaleDegradedExactScan,
    replacement_request: DegradedExactScanExecutionRequest<'_>,
) -> Result<crate::DegradedScanReady, DegradedExactScanExecutionDenied> {
    let (replacement, _) = select(replacement_request)?;
    let admission = super::super::admit_rebind(&stale, &replacement)
        .map_err(|denial| DegradedExactScanExecutionDenied::Rebind(Box::new(denial)))?;
    super::super::rebind(stale, replacement, admission)
        .map_err(|denial| DegradedExactScanExecutionDenied::Rebind(Box::new(denial)))
}

pub(super) fn execute_ready(
    ready: crate::DegradedScanReady,
    physical: &mut PhysicalStoreRuntime,
) -> Result<crate::DegradedScanExecution, DegradedExactScanExecutionDenied> {
    super::super::execute_ready(ready, physical).map_err(DegradedExactScanExecutionDenied::Physical)
}

fn select(
    request: DegradedExactScanExecutionRequest<'_>,
) -> Result<
    (
        crate::planning::SelectedDegradedExactScan,
        crate::CurrentMaterializationFrontier,
    ),
    DegradedExactScanExecutionDenied,
> {
    let declarations = crate::layout_declarations();
    let declaration = declarations
        .declaration(DurableArtifactFamilyId::PhysicalPage)
        .map_err(DegradedExactScanExecutionDenied::ArtifactFamily)?;
    let family = declarations
        .admit_physical_artifact_family(declaration, request.security)
        .into_result()
        .map_err(DegradedExactScanExecutionDenied::ArtifactFamily)?;
    let key_domain = declarations
        .admit_physical_key_domain(family, request.security)
        .into_result()
        .map_err(DegradedExactScanExecutionDenied::KeyDomain)?;
    let concrete_key = declarations
        .admit_page_key(key_domain, request.segment, request.page)
        .map_err(DegradedExactScanExecutionDenied::ConcreteKey)?;
    let materialization = crate::access_planning()
        .admit_current_catalog_root_materialization(family, request.catalog)
        .into_result()
        .map_err(DegradedExactScanExecutionDenied::Materialization)?;
    let frontier =
        crate::access_planning().current_materialization_frontier(request.current_catalog);
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
    Ok((selected, frontier))
}
