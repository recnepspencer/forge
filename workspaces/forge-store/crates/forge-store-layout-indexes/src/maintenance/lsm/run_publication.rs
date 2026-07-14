use crate::planning::AccessPlanSelectionView;

use super::{
    denial::{map_selection_denial, LsmMaintenanceAdmissionDenied},
    operation_context::admit_wal_operation_context,
    request::LsmRunPublicationAdmissionRequest,
    LsmMaintenanceAdmissionDenialKind, LsmMaintenanceOperation, LsmMaintenanceOwnerCaseDeclaration,
    LsmMaintenanceOwnerCaseId, LsmMaintenanceOwnerCaseObservation,
};

#[derive(Debug)]
enum RunPublicationAdmissionCase {
    Admitted(crate::BaselineLsmRunPublicationAdmission),
    Denied(LsmMaintenanceAdmissionDenied),
}

#[derive(Debug)]
pub struct LsmRunPublicationAdmissionOutcome {
    case: RunPublicationAdmissionCase,
}

#[derive(Debug)]
pub enum LsmRunPublicationAdmissionView<'a> {
    Admitted(&'a crate::BaselineLsmRunPublicationAdmission),
    Denied(&'a LsmMaintenanceAdmissionDenied),
}

impl LsmRunPublicationAdmissionOutcome {
    fn issue(
        result: Result<crate::BaselineLsmRunPublicationAdmission, LsmMaintenanceAdmissionDenied>,
    ) -> Self {
        Self {
            case: match result {
                Ok(value) => RunPublicationAdmissionCase::Admitted(value),
                Err(denial) => RunPublicationAdmissionCase::Denied(denial),
            },
        }
    }

    pub const fn view(&self) -> LsmRunPublicationAdmissionView<'_> {
        match &self.case {
            RunPublicationAdmissionCase::Admitted(value) => {
                LsmRunPublicationAdmissionView::Admitted(value)
            }
            RunPublicationAdmissionCase::Denied(denial) => {
                LsmRunPublicationAdmissionView::Denied(denial)
            }
        }
    }

    pub fn into_result(
        self,
    ) -> Result<crate::BaselineLsmRunPublicationAdmission, LsmMaintenanceAdmissionDenied> {
        match self.case {
            RunPublicationAdmissionCase::Admitted(value) => Ok(value),
            RunPublicationAdmissionCase::Denied(denial) => Err(denial),
        }
    }

    pub const fn owner_case_observation(&self) -> LsmMaintenanceOwnerCaseObservation {
        LsmMaintenanceOwnerCaseObservation::new(match &self.case {
            RunPublicationAdmissionCase::Admitted(_) => {
                LsmMaintenanceOwnerCaseId::admitted(LsmMaintenanceOperation::AdmitRunPublication)
            }
            RunPublicationAdmissionCase::Denied(denial) => LsmMaintenanceOwnerCaseId::denied(
                LsmMaintenanceOperation::AdmitRunPublication,
                denial.kind(),
            ),
        })
    }
}

pub(super) fn admit(
    request: LsmRunPublicationAdmissionRequest<'_>,
) -> LsmRunPublicationAdmissionOutcome {
    LsmRunPublicationAdmissionOutcome::issue(admit_inner(request))
}

fn admit_inner(
    request: LsmRunPublicationAdmissionRequest<'_>,
) -> Result<crate::BaselineLsmRunPublicationAdmission, LsmMaintenanceAdmissionDenied> {
    let (family, concrete_key) = admit_wal_operation_context(
        request.security,
        request.record_family,
        request.record_identity,
    )?;
    let shape = crate::access_shapes()
        .append(crate::PhysicalMutationShape::LogStructuredAppend)
        .map_err(|_| LsmMaintenanceAdmissionDenied::Shape)?;
    let admitted = crate::AccessPlanSelector
        .admit_mutation_request(family, concrete_key, shape)
        .map_err(LsmMaintenanceAdmissionDenied::RequestAdmission)?;
    let outcome = crate::AccessPlanSelector.select_admitted_with_budget(admitted, request.budget);
    match outcome.view() {
        AccessPlanSelectionView::LsmRunPublication(_) => {
            Ok(crate::BaselineLsmRunPublicationAdmission::admit(
                outcome
                    .into_lsm_run_publication()
                    .expect("view established LSM publication selection"),
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
        LsmMaintenanceOwnerCaseId::admitted(LsmMaintenanceOperation::AdmitRunPublication),
    ))
    .chain(DENIALS.into_iter().map(|denial| {
        LsmMaintenanceOwnerCaseDeclaration::new(LsmMaintenanceOwnerCaseId::denied(
            LsmMaintenanceOperation::AdmitRunPublication,
            denial,
        ))
    }))
}
