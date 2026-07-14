use crate::membership::{
    LsmCompactionMembership, LsmMembershipDenial, LsmMembershipOperation,
    LsmMembershipOwnerCaseDeclaration, LsmMembershipOwnerCaseId, LsmMembershipOwnerCaseObservation,
    LsmMembershipSession, PublishedLsmMembershipReplacement,
};
use crate::AdmittedLsmMembershipReplacement;

#[derive(Debug)]
enum ReplacementCase {
    Admitted(PublishedLsmMembershipReplacement),
    Denied(LsmMembershipDenial),
}

#[derive(Debug)]
pub struct LsmMembershipReplacementOutcome {
    case: ReplacementCase,
}

#[derive(Debug, Clone, Copy)]
pub enum LsmMembershipReplacementView<'a> {
    Admitted(&'a PublishedLsmMembershipReplacement),
    Denied(LsmMembershipDenial),
}

impl LsmMembershipReplacementOutcome {
    fn issue(result: Result<PublishedLsmMembershipReplacement, LsmMembershipDenial>) -> Self {
        Self {
            case: match result {
                Ok(replacement) => ReplacementCase::Admitted(replacement),
                Err(denial) => ReplacementCase::Denied(denial),
            },
        }
    }

    pub const fn view(&self) -> LsmMembershipReplacementView<'_> {
        match &self.case {
            ReplacementCase::Admitted(replacement) => {
                LsmMembershipReplacementView::Admitted(replacement)
            }
            ReplacementCase::Denied(denial) => LsmMembershipReplacementView::Denied(*denial),
        }
    }

    pub const fn owner_case_observation(&self) -> LsmMembershipOwnerCaseObservation {
        LsmMembershipOwnerCaseObservation::issued(match &self.case {
            ReplacementCase::Admitted(_) => {
                LsmMembershipOwnerCaseId::admitted(LsmMembershipOperation::ReplaceMembership)
            }
            ReplacementCase::Denied(denial) => {
                LsmMembershipOwnerCaseId::denied(LsmMembershipOperation::ReplaceMembership, *denial)
            }
        })
    }

    pub fn into_result(self) -> Result<PublishedLsmMembershipReplacement, LsmMembershipDenial> {
        match self.case {
            ReplacementCase::Admitted(replacement) => Ok(replacement),
            ReplacementCase::Denied(denial) => Err(denial),
        }
    }
}

pub fn replace_lsm_membership(
    session: &mut LsmMembershipSession,
    selected: &LsmCompactionMembership,
    replacement: &AdmittedLsmMembershipReplacement,
) -> LsmMembershipReplacementOutcome {
    LsmMembershipReplacementOutcome::issue(execute_replacement(session, selected, replacement))
}

fn execute_replacement(
    session: &mut LsmMembershipSession,
    selected: &LsmCompactionMembership,
    replacement: &AdmittedLsmMembershipReplacement,
) -> Result<PublishedLsmMembershipReplacement, LsmMembershipDenial> {
    let expected = selected.identities();
    let state = session
        .keys
        .get_mut(&selected.key())
        .ok_or(LsmMembershipDenial::MembershipStale)?;
    if !replacement.binds(selected)
        || selected.store_binding() != session.store_binding
        || !super::binding::selected_state_matches(
            state,
            expected,
            selected.base().map(|base| base.output()),
            selected.version(),
        )
    {
        return Err(LsmMembershipDenial::MembershipStale);
    }
    if !super::binding::manifest_matches_membership(
        selected,
        replacement.output().identity(),
        replacement.output().scope(),
        replacement.scope(),
        replacement.persisted_path(),
        replacement.persisted_bytes(),
    ) {
        return Err(LsmMembershipDenial::ManifestMembershipMismatch);
    }
    if !super::binding::replacement_output_matches(
        selected,
        replacement.output().identity(),
        replacement.output().scope(),
        replacement.output().persisted_path(),
        replacement.output().persisted_bytes(),
    ) {
        return Err(LsmMembershipDenial::ReplacementOutputMismatch);
    }
    for entry in &mut state.records {
        entry.as_mut().expect("complete membership checked").retired = true;
    }
    let published = PublishedLsmMembershipReplacement::issued(
        replacement.identity(),
        selected.key(),
        selected.identity_set(),
        replacement.output().identity(),
        replacement.output().scope().clone(),
        replacement.scope().clone(),
        replacement.output().persisted_path().to_path_buf(),
        replacement.output().persisted_bytes(),
    );
    state.published_replacement = Some(published.clone());
    state.version = state.version.saturating_add(1);
    Ok(published)
}

pub(in crate::membership::runtime) fn owner_cases(
) -> impl Iterator<Item = LsmMembershipOwnerCaseDeclaration> {
    const DENIALS: [LsmMembershipDenial; 3] = [
        LsmMembershipDenial::MembershipStale,
        LsmMembershipDenial::ManifestMembershipMismatch,
        LsmMembershipDenial::ReplacementOutputMismatch,
    ];
    std::iter::once(LsmMembershipOwnerCaseDeclaration::owned(
        LsmMembershipOwnerCaseId::admitted(LsmMembershipOperation::ReplaceMembership),
    ))
    .chain(DENIALS.into_iter().map(|denial| {
        LsmMembershipOwnerCaseDeclaration::owned(LsmMembershipOwnerCaseId::denied(
            LsmMembershipOperation::ReplaceMembership,
            denial,
        ))
    }))
}
