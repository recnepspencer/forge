use crate::access::execution::{access_lowering, BTreeLookupReadinessView};
use crate::planning::AccessPlanSelectionView;
use forge_store_contracts::DurableArtifactFamilyId;

use super::{
    denial::{
        map_artifact_denial, map_key_domain_denial, map_selection_denial, LayoutReadAdmissionDenied,
    },
    request::{PageLookupKind, PageLookupRequest},
};

pub(super) fn execute(
    request: PageLookupRequest<'_>,
) -> Result<crate::StableBTreeLookupExecution, LayoutReadAdmissionDenied> {
    let declarations = crate::layout_declarations();
    let declaration = declarations
        .declaration(DurableArtifactFamilyId::PhysicalPage)
        .map_err(map_artifact_denial)?;
    let family = declarations
        .admit_physical_artifact_family(declaration, request.security)
        .map_err(map_artifact_denial)?;
    let key_domain = declarations
        .admit_physical_key_domain(family, request.security)
        .map_err(map_key_domain_denial)?;
    let concrete_key = declarations
        .admit_page_key(key_domain, request.segment, request.page)
        .map_err(|_| LayoutReadAdmissionDenied::ConcreteKey)?;
    let materialization = crate::access_planning()
        .admit_btree_lookup_materialization(family, request.catalog, &request.source)
        .map_err(LayoutReadAdmissionDenied::ExactCoverage)?;
    let frontier = crate::access_planning()
        .current_btree_materialization_frontier(request.catalog, &request.source);
    let shape = match request.kind {
        PageLookupKind::Point => crate::access_planning().point_access(),
        PageLookupKind::Range => crate::access_planning().range_access(),
        PageLookupKind::Prefix => crate::access_planning().prefix_access(),
    };
    let admitted = crate::AccessPlanSelector
        .admit_read_request(family, concrete_key, materialization, shape)
        .map_err(LayoutReadAdmissionDenied::RequestAdmission)?;
    let outcome = crate::AccessPlanSelector.select_admitted_with_budget(admitted, request.budget);
    let selected = match outcome.view() {
        AccessPlanSelectionView::BTreeLookup(_) => outcome
            .into_btree_lookup()
            .expect("view established B-tree lookup selection"),
        AccessPlanSelectionView::Denied(denial) => {
            return Err(map_selection_denial(denial.clone()))
        }
        _ => return Err(LayoutReadAdmissionDenied::UnexpectedSelectedOperation),
    };
    let readiness = access_lowering()
        .admit_btree_lookup_ready(access_lowering().lower_btree_lookup(selected), frontier);
    let ready = match readiness.view() {
        BTreeLookupReadinessView::Ready(_) => readiness
            .into_ready()
            .expect("readiness view established owner-issued ready capability"),
        BTreeLookupReadinessView::Stale(stale) => {
            return Err(LayoutReadAdmissionDenied::StaleMaterialization(
                stale.materialization().clone(),
            ))
        }
    };
    crate::btree_lookup_runtime()
        .execute(ready, request.source, request.probe_slot)
        .map_err(LayoutReadAdmissionDenied::BTreeExecution)
}
