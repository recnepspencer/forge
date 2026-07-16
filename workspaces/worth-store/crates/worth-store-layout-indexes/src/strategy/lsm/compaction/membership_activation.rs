use super::super::{
    BaselineLsmExecutionAdmissionDenial, InterlockedLsmCompaction, LsmExecutionOperation,
    LsmExecutionOwnerCaseDeclaration, LsmExecutionOwnerCaseId, LsmExecutionOwnerCaseObservation,
};

#[derive(Debug)]
enum MembershipActivationCase {
    Admitted(Box<worth_store_lsm_authority::LsmMembershipActivationDeclaration>),
    Denied(BaselineLsmExecutionAdmissionDenial),
}

#[derive(Debug)]
pub struct LsmMembershipActivationOutcome {
    case: MembershipActivationCase,
}

#[derive(Debug)]
pub enum LsmMembershipActivationView<'a> {
    Admitted(&'a worth_store_lsm_authority::LsmMembershipActivationDeclaration),
    Denied(&'a BaselineLsmExecutionAdmissionDenial),
}

impl LsmMembershipActivationOutcome {
    fn issue(
        result: Result<
            worth_store_lsm_authority::LsmMembershipActivationDeclaration,
            BaselineLsmExecutionAdmissionDenial,
        >,
    ) -> Self {
        Self {
            case: match result {
                Ok(value) => MembershipActivationCase::Admitted(Box::new(value)),
                Err(denial) => MembershipActivationCase::Denied(denial),
            },
        }
    }

    pub const fn view(&self) -> LsmMembershipActivationView<'_> {
        match &self.case {
            MembershipActivationCase::Admitted(value) => {
                LsmMembershipActivationView::Admitted(value)
            }
            MembershipActivationCase::Denied(denial) => LsmMembershipActivationView::Denied(denial),
        }
    }

    pub fn into_result(
        self,
    ) -> Result<
        worth_store_lsm_authority::LsmMembershipActivationDeclaration,
        BaselineLsmExecutionAdmissionDenial,
    > {
        match self.case {
            MembershipActivationCase::Admitted(value) => Ok(*value),
            MembershipActivationCase::Denied(denial) => Err(denial),
        }
    }

    pub const fn owner_case_observation(&self) -> LsmExecutionOwnerCaseObservation {
        LsmExecutionOwnerCaseObservation::new(match &self.case {
            MembershipActivationCase::Admitted(_) => LsmExecutionOwnerCaseId::admitted(
                LsmExecutionOperation::PrepareMembershipActivation,
            ),
            MembershipActivationCase::Denied(denial) => LsmExecutionOwnerCaseId::denied(
                LsmExecutionOperation::PrepareMembershipActivation,
                denial.kind(),
            ),
        })
    }
}

impl InterlockedLsmCompaction {
    pub fn prepare_membership_activation(&self) -> LsmMembershipActivationOutcome {
        LsmMembershipActivationOutcome::issue(
            worth_store_lsm_authority::prepare_lsm_membership_activation(
                &self.prepared.membership,
                self.prepared.output.clone(),
                &self.physical,
            )
            .map_err(super::super::execution::map_membership_denial),
        )
    }
}

pub(super) fn owner_cases() -> impl Iterator<Item = LsmExecutionOwnerCaseDeclaration> {
    std::iter::once(LsmExecutionOwnerCaseDeclaration::new(
        LsmExecutionOwnerCaseId::admitted(LsmExecutionOperation::PrepareMembershipActivation),
    ))
}
