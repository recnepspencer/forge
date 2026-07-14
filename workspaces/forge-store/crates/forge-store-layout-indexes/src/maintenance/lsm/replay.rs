use crate::planning::AccessPlanSelectionView;

use super::{
    denial::{map_selection_denial, LsmMaintenanceAdmissionDenied},
    operation_context::admit_wal_operation_context,
    request::LsmReplayAdmissionRequest,
    LsmMaintenanceAdmissionDenialKind, LsmMaintenanceOperation, LsmMaintenanceOwnerCaseDeclaration,
    LsmMaintenanceOwnerCaseId, LsmMaintenanceOwnerCaseObservation,
};

#[derive(Debug)]
enum ReplayAdmissionCase {
    Admitted(crate::BaselineLsmReplayAdmission),
    Denied(LsmMaintenanceAdmissionDenied),
}

#[derive(Debug)]
pub struct LsmReplayMaintenanceAdmissionOutcome {
    case: ReplayAdmissionCase,
}

#[derive(Debug)]
pub enum LsmReplayMaintenanceAdmissionView<'a> {
    Admitted(&'a crate::BaselineLsmReplayAdmission),
    Denied(&'a LsmMaintenanceAdmissionDenied),
}

impl LsmReplayMaintenanceAdmissionOutcome {
    fn issue(
        result: Result<crate::BaselineLsmReplayAdmission, LsmMaintenanceAdmissionDenied>,
    ) -> Self {
        Self {
            case: match result {
                Ok(value) => ReplayAdmissionCase::Admitted(value),
                Err(denial) => ReplayAdmissionCase::Denied(denial),
            },
        }
    }

    pub const fn view(&self) -> LsmReplayMaintenanceAdmissionView<'_> {
        match &self.case {
            ReplayAdmissionCase::Admitted(value) => {
                LsmReplayMaintenanceAdmissionView::Admitted(value)
            }
            ReplayAdmissionCase::Denied(denial) => {
                LsmReplayMaintenanceAdmissionView::Denied(denial)
            }
        }
    }

    pub fn into_result(
        self,
    ) -> Result<crate::BaselineLsmReplayAdmission, LsmMaintenanceAdmissionDenied> {
        match self.case {
            ReplayAdmissionCase::Admitted(value) => Ok(value),
            ReplayAdmissionCase::Denied(denial) => Err(denial),
        }
    }

    pub const fn owner_case_observation(&self) -> LsmMaintenanceOwnerCaseObservation {
        LsmMaintenanceOwnerCaseObservation::new(match &self.case {
            ReplayAdmissionCase::Admitted(_) => {
                LsmMaintenanceOwnerCaseId::admitted(LsmMaintenanceOperation::AdmitReplay)
            }
            ReplayAdmissionCase::Denied(denial) => LsmMaintenanceOwnerCaseId::denied(
                LsmMaintenanceOperation::AdmitReplay,
                denial.kind(),
            ),
        })
    }
}

pub(super) fn admit(
    request: LsmReplayAdmissionRequest<'_>,
) -> LsmReplayMaintenanceAdmissionOutcome {
    LsmReplayMaintenanceAdmissionOutcome::issue(admit_inner(request))
}

fn admit_inner(
    request: LsmReplayAdmissionRequest<'_>,
) -> Result<crate::BaselineLsmReplayAdmission, LsmMaintenanceAdmissionDenied> {
    let (family, concrete_key) = admit_wal_operation_context(
        request.security,
        request.record_family,
        request.record_identity,
    )?;
    let materialization = crate::access_planning()
        .admit_lsm_replay_materialization(family, request.catalog, request.source)
        .into_result()
        .map_err(|_| LsmMaintenanceAdmissionDenied::Shape)?;
    let frontier = crate::access_planning()
        .current_lsm_replay_materialization_frontier(request.catalog, request.source)
        .map_err(|_| LsmMaintenanceAdmissionDenied::Shape)?;
    let current = materialization
        .clone()
        .require_current_at(frontier)
        .map_err(|_| LsmMaintenanceAdmissionDenied::Shape)?;
    let shape = crate::access_planning()
        .rebuild_access(crate::AccessLaneClassification::Maintenance)
        .map_err(|_| LsmMaintenanceAdmissionDenied::Shape)?;
    let admitted = crate::AccessPlanSelector
        .admit_recovery_request(family, concrete_key, materialization, shape)
        .map_err(LsmMaintenanceAdmissionDenied::RequestAdmission)?;
    let outcome = crate::AccessPlanSelector.select_admitted_with_budget(admitted, request.budget);
    match outcome.view() {
        AccessPlanSelectionView::LsmReplayRecovery(_) => crate::BaselineLsmReplayAdmission::admit(
            outcome
                .into_lsm_replay_recovery()
                .expect("view established LSM replay selection"),
            request.source.clone(),
            current,
        )
        .map_err(|_| LsmMaintenanceAdmissionDenied::UnexpectedSelectedOperation),
        AccessPlanSelectionView::Denied(denial) => Err(map_selection_denial(denial.clone())),
        _ => Err(LsmMaintenanceAdmissionDenied::UnexpectedSelectedOperation),
    }
}

pub(super) fn owner_cases() -> impl Iterator<Item = LsmMaintenanceOwnerCaseDeclaration> {
    use LsmMaintenanceAdmissionDenialKind as Denial;
    const DENIALS: [Denial; 3] = [Denial::SecurityScope, Denial::Shape, Denial::Budget];
    std::iter::once(LsmMaintenanceOwnerCaseDeclaration::new(
        LsmMaintenanceOwnerCaseId::admitted(LsmMaintenanceOperation::AdmitReplay),
    ))
    .chain(DENIALS.into_iter().map(|denial| {
        LsmMaintenanceOwnerCaseDeclaration::new(LsmMaintenanceOwnerCaseId::denied(
            LsmMaintenanceOperation::AdmitReplay,
            denial,
        ))
    }))
}
