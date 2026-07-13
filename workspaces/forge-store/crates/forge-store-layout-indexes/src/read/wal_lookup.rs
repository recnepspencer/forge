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
    let frontier = crate::access_planning()
        .current_lsm_materialization_frontier(request.catalog, &request.source);
    let declarations = crate::layout_declarations();
    let declaration = declarations
        .declaration(DurableArtifactFamilyId::PublicationWalIntent)
        .map_err(map_artifact_denial)?;
    let family = declarations
        .admit_physical_artifact_family(declaration, request.security)
        .map_err(map_artifact_denial)?;
    let key_domain = declarations
        .admit_physical_key_domain(family, request.security)
        .map_err(map_key_domain_denial)?;
    let concrete_key = declarations
        .admit_wal_key(key_domain, request.record_family, request.record_identity)
        .map_err(|_| LayoutReadAdmissionDenied::ConcreteKey)?;
    let materialization = crate::access_planning()
        .admit_lsm_lookup_materialization(family, request.catalog, &request.source)
        .map_err(LayoutReadAdmissionDenied::ExactCoverage)?;
    let admitted = crate::AccessPlanSelector
        .admit_read_request(
            family,
            concrete_key,
            materialization,
            crate::access_planning().point_access(),
        )
        .map_err(LayoutReadAdmissionDenied::RequestAdmission)?;
    let outcome = crate::AccessPlanSelector.select_admitted_with_budget(admitted, request.budget);
    let selected = match outcome.view() {
        AccessPlanSelectionView::LsmLookup(_) => outcome
            .into_lsm_lookup()
            .expect("view established LSM lookup selection"),
        AccessPlanSelectionView::Denied(denial) => {
            return Err(map_selection_denial(denial.clone()))
        }
        _ => return Err(LayoutReadAdmissionDenied::UnexpectedSelectedOperation),
    };
    crate::lsm_lookup_runtime()
        .execute(selected, request.source, request.probe_sequence, frontier)
        .map_err(|denial| match denial {
            crate::strategy::LsmLookupAdmissionDenied::Stale(stale) => {
                LayoutReadAdmissionDenied::StaleMaterialization(stale)
            }
            crate::strategy::LsmLookupAdmissionDenied::InvariantViolation(denial) => {
                LayoutReadAdmissionDenied::StrategyInvariant(denial)
            }
        })
}
