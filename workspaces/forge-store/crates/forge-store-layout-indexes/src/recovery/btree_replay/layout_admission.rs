use crate::planning::AccessPlanSelectionView;
use forge_store_contracts::DurableArtifactFamilyId;

use super::{
    denial::{map_artifact_denial, map_key_domain_denial, map_selection_denial, BTreeReplayDenied},
    request::BTreeReplayRequest,
};

pub(super) fn admit(
    request: &BTreeReplayRequest<'_>,
    physical_source: &forge_store_recovery_physics::AdmittedBTreeReplayPhysicalSource,
) -> Result<crate::BaselineBTreeReplayAdmission, BTreeReplayDenied> {
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
        .map_err(|_| BTreeReplayDenied::ConcreteKey)?;
    let materialization = crate::access_planning()
        .admit_btree_replay_materialization(family, request.catalog, physical_source)
        .map_err(|_| BTreeReplayDenied::ExactCoverage)?;
    let shape = crate::access_planning()
        .rebuild_access(crate::AccessLaneClassification::Maintenance)
        .map_err(|_| BTreeReplayDenied::Shape)?;
    let admitted = crate::AccessPlanSelector
        .admit_recovery_request(family, concrete_key, materialization, shape)
        .map_err(BTreeReplayDenied::RequestAdmission)?;
    let outcome = crate::AccessPlanSelector.select_admitted_with_budget(admitted, request.budget);
    match outcome.view() {
        AccessPlanSelectionView::BTreeReplayRecovery(_) => {
            crate::BaselineBTreeReplayAdmission::admit(
                outcome
                    .into_btree_replay_recovery()
                    .expect("view established B-tree replay selection"),
            )
            .map_err(BTreeReplayDenied::Execution)
        }
        AccessPlanSelectionView::Denied(denial) => Err(map_selection_denial(denial.clone())),
        _ => Err(BTreeReplayDenied::UnexpectedSelectedOperation),
    }
}
