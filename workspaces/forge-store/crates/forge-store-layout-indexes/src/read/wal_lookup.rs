use crate::planning::AccessPlanSelectionView;
use forge_store_contracts::DurableArtifactFamilyId;

use super::{
    denial::{
        map_artifact_denial, map_key_domain_denial, map_selection_denial, LayoutReadAdmissionDenied,
    },
    request::WalLookupRequest,
};

pub(super) fn execute(
    request: WalLookupRequest<'_>,
) -> Result<crate::BaselineLsmLookupExecution, LayoutReadAdmissionDenied> {
    let admitted = admit_wal_lookup(request)?;
    let selected = select_wal_lookup(admitted)?;
    execute_selected_wal_lookup(selected)
}

pub(super) fn prepare(
    request: WalLookupRequest<'_>,
) -> Result<crate::BaselineLsmLookupAdmissionOutcome, LayoutReadAdmissionDenied> {
    let admitted = admit_wal_lookup(request)?;
    let selected = select_wal_lookup(admitted)?;
    Ok(crate::BaselineLsmLookupAdmission::admit(
        selected.selected,
        selected.frontier,
    ))
}

struct AdmittedWalLookup {
    family: crate::AdmittedPhysicalArtifactFamily,
    concrete_key: crate::AdmittedConcretePhysicalKey,
    materialization: crate::AdmittedLayoutMaterialization,
    frontier: crate::CurrentMaterializationFrontier,
    probe_sequence: u64,
    budget: forge_store_budgets::PreExecutionBudgetEnvelope,
    source: crate::BaselineLsmLookupSource,
}

struct SelectedWalLookup {
    selected: crate::SelectedLsmLookup,
    frontier: crate::CurrentMaterializationFrontier,
    probe_sequence: u64,
    source: crate::BaselineLsmLookupSource,
}

fn admit_wal_lookup(
    request: WalLookupRequest<'_>,
) -> Result<AdmittedWalLookup, LayoutReadAdmissionDenied> {
    let frontier = crate::access_planning()
        .current_lsm_materialization_frontier(request.current_catalog, &request.source);
    let declarations = crate::layout_declarations();
    let declaration = declarations
        .declaration(DurableArtifactFamilyId::PublicationWalIntent)
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
        .admit_wal_key(key_domain, request.record_family, request.record_identity)
        .map_err(|_| LayoutReadAdmissionDenied::ConcreteKey)?;
    let materialization = crate::access_planning()
        .admit_lsm_lookup_materialization(family, request.catalog, &request.source)
        .into_result()
        .map_err(LayoutReadAdmissionDenied::ExactCoverage)?;
    Ok(AdmittedWalLookup {
        family,
        concrete_key,
        materialization,
        frontier,
        probe_sequence: request.probe_sequence,
        budget: request.budget,
        source: request.source,
    })
}

fn select_wal_lookup(
    stage: AdmittedWalLookup,
) -> Result<SelectedWalLookup, LayoutReadAdmissionDenied> {
    let admitted_request = crate::AccessPlanSelector
        .admit_read_request(
            stage.family,
            stage.concrete_key,
            stage.materialization,
            crate::access_planning().point_access(),
        )
        .map_err(LayoutReadAdmissionDenied::RequestAdmission)?;
    let outcome =
        crate::AccessPlanSelector.select_admitted_with_budget(admitted_request, stage.budget);
    let selected = match outcome.view() {
        AccessPlanSelectionView::LsmLookup(_) => outcome
            .into_lsm_lookup()
            .expect("view established LSM lookup selection"),
        AccessPlanSelectionView::Denied(denial) => {
            return Err(map_selection_denial(denial.clone()))
        }
        _ => return Err(LayoutReadAdmissionDenied::UnexpectedSelectedOperation),
    };
    Ok(SelectedWalLookup {
        selected,
        frontier: stage.frontier,
        probe_sequence: stage.probe_sequence,
        source: stage.source,
    })
}

fn execute_selected_wal_lookup(
    selected: SelectedWalLookup,
) -> Result<crate::BaselineLsmLookupExecution, LayoutReadAdmissionDenied> {
    crate::lsm_lookup_runtime()
        .execute(
            selected.selected,
            selected.source,
            selected.probe_sequence,
            selected.frontier,
        )
        .map_err(|denial| match denial {
            crate::strategy::LsmLookupAdmissionDenied::Stale(stale) => {
                LayoutReadAdmissionDenied::StaleMaterialization(stale)
            }
            crate::strategy::LsmLookupAdmissionDenied::InvariantViolation(denial) => {
                LayoutReadAdmissionDenied::StrategyInvariant(denial)
            }
            crate::strategy::LsmLookupAdmissionDenied::CounterEnvelope(violation) => {
                LayoutReadAdmissionDenied::CounterEnvelope(violation)
            }
        })
}
