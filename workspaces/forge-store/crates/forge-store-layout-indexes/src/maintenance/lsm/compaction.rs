use crate::planning::AccessPlanSelectionView;

use super::{
    denial::{map_selection_denial, LsmMaintenanceAdmissionDenied},
    operation_context::admit_wal_operation_context,
    request::LsmCompactionAdmissionRequest,
    LsmMaintenanceAdmissionDenialKind, LsmMaintenanceOperation, LsmMaintenanceOwnerCaseDeclaration,
    LsmMaintenanceOwnerCaseId, LsmMaintenanceOwnerCaseObservation,
};

#[derive(Debug)]
enum CompactionAdmissionCase {
    Admitted(crate::BaselineLsmCompactionAdmission),
    Denied(LsmMaintenanceAdmissionDenied),
}

#[derive(Debug)]
pub struct LsmCompactionMaintenanceAdmissionOutcome {
    case: CompactionAdmissionCase,
}

#[derive(Debug)]
pub enum LsmCompactionMaintenanceAdmissionView<'a> {
    Admitted(&'a crate::BaselineLsmCompactionAdmission),
    Denied(&'a LsmMaintenanceAdmissionDenied),
}

impl LsmCompactionMaintenanceAdmissionOutcome {
    fn issue(
        result: Result<crate::BaselineLsmCompactionAdmission, LsmMaintenanceAdmissionDenied>,
    ) -> Self {
        Self {
            case: match result {
                Ok(value) => CompactionAdmissionCase::Admitted(value),
                Err(denial) => CompactionAdmissionCase::Denied(denial),
            },
        }
    }

    pub const fn view(&self) -> LsmCompactionMaintenanceAdmissionView<'_> {
        match &self.case {
            CompactionAdmissionCase::Admitted(value) => {
                LsmCompactionMaintenanceAdmissionView::Admitted(value)
            }
            CompactionAdmissionCase::Denied(denial) => {
                LsmCompactionMaintenanceAdmissionView::Denied(denial)
            }
        }
    }

    pub fn into_result(
        self,
    ) -> Result<crate::BaselineLsmCompactionAdmission, LsmMaintenanceAdmissionDenied> {
        match self.case {
            CompactionAdmissionCase::Admitted(value) => Ok(value),
            CompactionAdmissionCase::Denied(denial) => Err(denial),
        }
    }

    pub const fn owner_case_observation(&self) -> LsmMaintenanceOwnerCaseObservation {
        LsmMaintenanceOwnerCaseObservation::new(match &self.case {
            CompactionAdmissionCase::Admitted(_) => {
                LsmMaintenanceOwnerCaseId::admitted(LsmMaintenanceOperation::AdmitCompaction)
            }
            CompactionAdmissionCase::Denied(denial) => LsmMaintenanceOwnerCaseId::denied(
                LsmMaintenanceOperation::AdmitCompaction,
                denial.kind(),
            ),
        })
    }
}

pub(super) fn admit(
    request: LsmCompactionAdmissionRequest<'_>,
) -> LsmCompactionMaintenanceAdmissionOutcome {
    LsmCompactionMaintenanceAdmissionOutcome::issue(admit_inner(request))
}

fn admit_inner(
    request: LsmCompactionAdmissionRequest<'_>,
) -> Result<crate::BaselineLsmCompactionAdmission, LsmMaintenanceAdmissionDenied> {
    let (family, concrete_key) = admit_wal_operation_context(
        request.security,
        request.record_family,
        request.record_identity,
    )?;
    let shape = crate::access_shapes()
        .compaction_read(crate::PhysicalMutationShape::CompactionRewrite)
        .map_err(|_| LsmMaintenanceAdmissionDenied::Shape)?;
    let admitted = crate::AccessPlanSelector
        .admit_mutation_request(family, concrete_key, shape)
        .map_err(LsmMaintenanceAdmissionDenied::RequestAdmission)?;
    let outcome = crate::AccessPlanSelector.select_admitted_with_budget(admitted, request.budget);
    match outcome.view() {
        AccessPlanSelectionView::LsmCompaction(_) => {
            Ok(crate::BaselineLsmCompactionAdmission::admit(
                outcome
                    .into_lsm_compaction()
                    .expect("view established LSM compaction selection"),
            ))
        }
        AccessPlanSelectionView::Denied(denial) => Err(map_selection_denial(denial.clone())),
        _ => Err(LsmMaintenanceAdmissionDenied::UnexpectedSelectedOperation),
    }
}

pub(super) fn owner_cases() -> impl Iterator<Item = LsmMaintenanceOwnerCaseDeclaration> {
    use LsmMaintenanceAdmissionDenialKind as Denial;
    const DENIALS: [Denial; 2] = [Denial::SecurityScope, Denial::Budget];
    std::iter::once(LsmMaintenanceOwnerCaseDeclaration::new(
        LsmMaintenanceOwnerCaseId::admitted(LsmMaintenanceOperation::AdmitCompaction),
    ))
    .chain(DENIALS.into_iter().map(|denial| {
        LsmMaintenanceOwnerCaseDeclaration::new(LsmMaintenanceOwnerCaseId::denied(
            LsmMaintenanceOperation::AdmitCompaction,
            denial,
        ))
    }))
}
