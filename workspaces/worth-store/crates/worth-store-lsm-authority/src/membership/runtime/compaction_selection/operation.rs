use crate::membership::{
    LsmCompactionMembership, LsmCompactionRecordSet, LsmMembershipDenial, LsmMembershipKey,
    LsmMembershipOperation, LsmMembershipOwnerCaseDeclaration, LsmMembershipOwnerCaseId,
    LsmMembershipOwnerCaseObservation, LsmMembershipRecord, LsmMembershipSession,
};

#[derive(Debug)]
enum SelectionCase {
    Admitted(Box<LsmCompactionMembership>),
    Denied(LsmMembershipDenial),
}

#[derive(Debug)]
pub struct LsmMembershipSelectionOutcome {
    case: SelectionCase,
}

#[derive(Debug, Clone, Copy)]
pub enum LsmMembershipSelectionView<'a> {
    Admitted(&'a LsmCompactionMembership),
    Denied(LsmMembershipDenial),
}

impl LsmMembershipSelectionOutcome {
    fn issue(result: Result<LsmCompactionMembership, LsmMembershipDenial>) -> Self {
        Self {
            case: match result {
                Ok(selected) => SelectionCase::Admitted(Box::new(selected)),
                Err(denial) => SelectionCase::Denied(denial),
            },
        }
    }

    pub const fn view(&self) -> LsmMembershipSelectionView<'_> {
        match &self.case {
            SelectionCase::Admitted(selected) => LsmMembershipSelectionView::Admitted(selected),
            SelectionCase::Denied(denial) => LsmMembershipSelectionView::Denied(*denial),
        }
    }

    pub const fn owner_case_observation(&self) -> LsmMembershipOwnerCaseObservation {
        LsmMembershipOwnerCaseObservation::issued(match &self.case {
            SelectionCase::Admitted(_) => {
                LsmMembershipOwnerCaseId::admitted(LsmMembershipOperation::SelectCompaction)
            }
            SelectionCase::Denied(denial) => {
                LsmMembershipOwnerCaseId::denied(LsmMembershipOperation::SelectCompaction, *denial)
            }
        })
    }

    pub fn into_result(self) -> Result<LsmCompactionMembership, LsmMembershipDenial> {
        match self.case {
            SelectionCase::Admitted(selected) => Ok(*selected),
            SelectionCase::Denied(denial) => Err(denial),
        }
    }
}

pub fn select_lsm_compaction_membership(
    session: &LsmMembershipSession,
    key: LsmMembershipKey,
) -> LsmMembershipSelectionOutcome {
    LsmMembershipSelectionOutcome::issue(execute_selection(session, key))
}

fn execute_selection(
    session: &LsmMembershipSession,
    key: LsmMembershipKey,
) -> Result<LsmCompactionMembership, LsmMembershipDenial> {
    let state = session
        .keys
        .get(&key)
        .ok_or(LsmMembershipDenial::ValueRecordRequired)?;
    if state
        .published_replacement
        .as_ref()
        .is_some_and(|base| !base.artifact_is_current())
    {
        return Err(LsmMembershipDenial::ReplacementOutputMismatch);
    }
    let value = active_record(&state.records[0]).ok_or(LsmMembershipDenial::ValueRecordRequired)?;
    let generation =
        active_record(&state.records[1]).ok_or(LsmMembershipDenial::GenerationRecordRequired)?;
    let tombstone =
        active_record(&state.records[2]).ok_or(LsmMembershipDenial::TombstoneRecordRequired)?;
    let record_set = LsmCompactionRecordSet::issue(key, value, generation, tombstone)?;
    Ok(LsmCompactionMembership {
        key,
        record_set,
        base: state.published_replacement.clone(),
        version: state.version,
        store_binding: session.store_binding.clone(),
        partition_probes: 1,
        component_probes: 3,
    })
}

fn active_record(entry: &Option<super::super::state::RecordState>) -> Option<LsmMembershipRecord> {
    entry
        .as_ref()
        .filter(|entry| !entry.retired)
        .map(|entry| entry.record.clone())
}

pub(in crate::membership::runtime) fn owner_cases(
) -> impl Iterator<Item = LsmMembershipOwnerCaseDeclaration> {
    const DENIALS: [LsmMembershipDenial; 4] = [
        LsmMembershipDenial::ValueRecordRequired,
        LsmMembershipDenial::GenerationRecordRequired,
        LsmMembershipDenial::TombstoneRecordRequired,
        LsmMembershipDenial::ReplacementOutputMismatch,
    ];
    std::iter::once(LsmMembershipOwnerCaseDeclaration::owned(
        LsmMembershipOwnerCaseId::admitted(LsmMembershipOperation::SelectCompaction),
    ))
    .chain(DENIALS.into_iter().map(|denial| {
        LsmMembershipOwnerCaseDeclaration::owned(LsmMembershipOwnerCaseId::denied(
            LsmMembershipOperation::SelectCompaction,
            denial,
        ))
    }))
}
