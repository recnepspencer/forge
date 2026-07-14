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
) -> Result<crate::BTreeLookupExecutionOutcome, LayoutReadAdmissionDenied> {
    let admitted = admit_page_lookup(request)?;
    let selected = select_page_lookup(admitted)?;
    execute_selected_page_lookup(selected)
}

pub(super) fn prepare(
    request: PageLookupRequest<'_>,
) -> Result<crate::BTreeLookupReadinessOutcome, LayoutReadAdmissionDenied> {
    let admitted = admit_page_lookup(request)?;
    let selected = select_page_lookup(admitted)?;
    Ok(crate::access::execution::prepare_btree_lookup(
        selected.selected,
        selected.frontier,
    ))
}

struct AdmittedPageLookup {
    family: crate::AdmittedPhysicalArtifactFamily,
    concrete_key: crate::AdmittedConcretePhysicalKey,
    materialization: crate::AdmittedLayoutMaterialization,
    shape: crate::access::shape::AccessShapeContract,
    frontier: crate::CurrentMaterializationFrontier,
    probe_slot: forge_store_physical_format::PhysicalRecordSlot,
    budget: forge_store_budgets::PreExecutionBudgetEnvelope,
    source: crate::BaselineBTreeReadSource,
}

struct SelectedPageLookup {
    selected: crate::SelectedBTreeLookup,
    frontier: crate::CurrentMaterializationFrontier,
    probe_slot: forge_store_physical_format::PhysicalRecordSlot,
    source: crate::BaselineBTreeReadSource,
}

fn admit_page_lookup(
    request: PageLookupRequest<'_>,
) -> Result<AdmittedPageLookup, LayoutReadAdmissionDenied> {
    let declarations = crate::layout_declarations();
    let declaration = declarations
        .declaration(DurableArtifactFamilyId::PhysicalPage)
        .map_err(map_artifact_denial)?;
    let family = declarations
        .admit_physical_artifact_family(declaration, request.security)
        .into_result()
        .map_err(map_artifact_denial)?;
    let key_domain = declarations
        .admit_physical_key_domain(family, request.security)
        .into_result()
        .map_err(map_key_domain_denial)?;
    let concrete_key = declarations
        .admit_page_key(key_domain, request.segment, request.page)
        .map_err(|_| LayoutReadAdmissionDenied::ConcreteKey)?;
    let materialization = crate::access_planning()
        .admit_btree_lookup_materialization(family, request.catalog, &request.source)
        .into_result()
        .map_err(LayoutReadAdmissionDenied::ExactCoverage)?;
    let frontier = crate::access_planning().current_btree_materialization_frontier(
        request.current_catalog,
        request.current_source.as_ref().unwrap_or(&request.source),
    );
    let shape = match request.kind {
        PageLookupKind::Point => crate::access_planning().point_access(),
        PageLookupKind::Range => crate::access_planning().range_access(),
        PageLookupKind::Prefix => crate::access_planning().prefix_access(),
    };
    Ok(AdmittedPageLookup {
        family,
        concrete_key,
        materialization,
        shape,
        frontier,
        probe_slot: request.probe_slot,
        budget: request.budget,
        source: request.source,
    })
}

fn select_page_lookup(
    stage: AdmittedPageLookup,
) -> Result<SelectedPageLookup, LayoutReadAdmissionDenied> {
    let admitted_request = crate::AccessPlanSelector
        .admit_read_request(
            stage.family,
            stage.concrete_key,
            stage.materialization,
            stage.shape,
        )
        .map_err(LayoutReadAdmissionDenied::RequestAdmission)?;
    let outcome =
        crate::AccessPlanSelector.select_admitted_with_budget(admitted_request, stage.budget);
    let selected = match outcome.view() {
        AccessPlanSelectionView::BTreeLookup(_) => outcome
            .into_btree_lookup()
            .expect("view established B-tree lookup selection"),
        AccessPlanSelectionView::Denied(denial) => {
            return Err(map_selection_denial(denial.clone()))
        }
        _ => return Err(LayoutReadAdmissionDenied::UnexpectedSelectedOperation),
    };
    Ok(SelectedPageLookup {
        selected,
        frontier: stage.frontier,
        probe_slot: stage.probe_slot,
        source: stage.source,
    })
}

fn execute_selected_page_lookup(
    selected: SelectedPageLookup,
) -> Result<crate::BTreeLookupExecutionOutcome, LayoutReadAdmissionDenied> {
    crate::access::execution::execute_btree_lookup(
        selected.selected,
        selected.frontier,
        selected.source,
        selected.probe_slot,
    )
    .map_err(|denial| match denial {
        crate::access::execution::BTreeLookupOperationDenied::Stale(stale) => {
            LayoutReadAdmissionDenied::StaleMaterialization(stale)
        }
    })
}
