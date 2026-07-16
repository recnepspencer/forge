use crate::membership::{
    LsmMembershipDenial, LsmMembershipKey, LsmMembershipOperation,
    LsmMembershipOwnerCaseDeclaration, LsmMembershipOwnerCaseId, LsmMembershipOwnerCaseObservation,
    LsmMembershipSession, PublishedLsmMembershipReplacement,
};

#[derive(Debug)]
enum LookupCase {
    Admitted(Box<PublishedLsmMembershipReplacement>),
    Denied(LsmMembershipDenial),
}

#[derive(Debug)]
pub struct LsmPublishedMembershipLookupOutcome {
    case: LookupCase,
}

#[derive(Debug, Clone, Copy)]
pub enum LsmPublishedMembershipLookupView<'a> {
    Admitted(&'a PublishedLsmMembershipReplacement),
    Denied(LsmMembershipDenial),
}

impl LsmPublishedMembershipLookupOutcome {
    fn issue(result: Result<PublishedLsmMembershipReplacement, LsmMembershipDenial>) -> Self {
        Self {
            case: match result {
                Ok(replacement) => LookupCase::Admitted(Box::new(replacement)),
                Err(denial) => LookupCase::Denied(denial),
            },
        }
    }

    pub const fn view(&self) -> LsmPublishedMembershipLookupView<'_> {
        match &self.case {
            LookupCase::Admitted(replacement) => {
                LsmPublishedMembershipLookupView::Admitted(replacement)
            }
            LookupCase::Denied(denial) => LsmPublishedMembershipLookupView::Denied(*denial),
        }
    }

    pub const fn owner_case_observation(&self) -> LsmMembershipOwnerCaseObservation {
        LsmMembershipOwnerCaseObservation::issued(match &self.case {
            LookupCase::Admitted(_) => LsmMembershipOwnerCaseId::admitted(
                LsmMembershipOperation::LookupPublishedReplacement,
            ),
            LookupCase::Denied(denial) => LsmMembershipOwnerCaseId::denied(
                LsmMembershipOperation::LookupPublishedReplacement,
                *denial,
            ),
        })
    }

    pub fn into_result(self) -> Result<PublishedLsmMembershipReplacement, LsmMembershipDenial> {
        match self.case {
            LookupCase::Admitted(replacement) => Ok(*replacement),
            LookupCase::Denied(denial) => Err(denial),
        }
    }
}

pub fn lookup_published_lsm_membership(
    session: &LsmMembershipSession,
    key: LsmMembershipKey,
) -> LsmPublishedMembershipLookupOutcome {
    let result = session
        .keys
        .get(&key)
        .and_then(|state| state.published_replacement.clone())
        .ok_or(LsmMembershipDenial::MembershipIncomplete);
    LsmPublishedMembershipLookupOutcome::issue(result)
}

pub(in crate::membership::runtime) fn owner_cases(
) -> impl Iterator<Item = LsmMembershipOwnerCaseDeclaration> {
    [
        LsmMembershipOwnerCaseDeclaration::owned(LsmMembershipOwnerCaseId::admitted(
            LsmMembershipOperation::LookupPublishedReplacement,
        )),
        LsmMembershipOwnerCaseDeclaration::owned(LsmMembershipOwnerCaseId::denied(
            LsmMembershipOperation::LookupPublishedReplacement,
            LsmMembershipDenial::MembershipIncomplete,
        )),
    ]
    .into_iter()
}
