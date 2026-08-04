use crate::membership::{
    LsmMembershipDenial, LsmMembershipOperation, LsmMembershipOwnerCaseDeclaration,
    LsmMembershipOwnerCaseId, LsmMembershipOwnerCaseObservation, LsmMembershipRecord,
    LsmMembershipSession,
};

#[derive(Debug)]
enum PersistCase {
    Admitted,
    Denied(LsmMembershipDenial),
}

#[derive(Debug)]
pub struct LsmMembershipPersistOutcome {
    case: PersistCase,
}

#[derive(Debug, Clone, Copy)]
pub enum LsmMembershipPersistView {
    Admitted,
    Denied(LsmMembershipDenial),
}

impl LsmMembershipPersistOutcome {
    fn issue(result: Result<(), LsmMembershipDenial>) -> Self {
        Self {
            case: match result {
                Ok(()) => PersistCase::Admitted,
                Err(denial) => PersistCase::Denied(denial),
            },
        }
    }

    pub const fn view(&self) -> LsmMembershipPersistView {
        match &self.case {
            PersistCase::Admitted => LsmMembershipPersistView::Admitted,
            PersistCase::Denied(denial) => LsmMembershipPersistView::Denied(*denial),
        }
    }

    pub const fn owner_case_observation(&self) -> LsmMembershipOwnerCaseObservation {
        LsmMembershipOwnerCaseObservation::issued(match &self.case {
            PersistCase::Admitted => {
                LsmMembershipOwnerCaseId::admitted(LsmMembershipOperation::PersistRecord)
            }
            PersistCase::Denied(denial) => {
                LsmMembershipOwnerCaseId::denied(LsmMembershipOperation::PersistRecord, *denial)
            }
        })
    }

    pub fn into_result(self) -> Result<(), LsmMembershipDenial> {
        match self.case {
            PersistCase::Admitted => Ok(()),
            PersistCase::Denied(denial) => Err(denial),
        }
    }
}

pub fn persist_lsm_membership_record(
    session: &mut LsmMembershipSession,
    record: LsmMembershipRecord,
) -> LsmMembershipPersistOutcome {
    LsmMembershipPersistOutcome::issue(execute_persistence(session, record))
}

fn execute_persistence(
    session: &mut LsmMembershipSession,
    record: LsmMembershipRecord,
) -> Result<(), LsmMembershipDenial> {
    let slot = component_slot(record.kind()).ok_or(LsmMembershipDenial::UnsupportedRecordKind)?;
    if record.durable_scope().segment_id() != session.segment_id
        || record.durable_scope().generation() != session.generation
        || !session.store.admits_path(&record.persisted_path)
    {
        return Err(LsmMembershipDenial::StoreBindingMismatch);
    }
    if !crate::membership::durable_artifact::persisted_artifact_range_matches(
        &record.persisted_path,
        record.persisted_offset,
        record.persisted_bytes,
        &crate::membership::durable_artifact::lsm_membership_record_bytes(
            &record.envelope,
            record.key,
        ),
    ) {
        return Err(LsmMembershipDenial::DurableRecordBindingMismatch);
    }
    let state = session.keys.entry(record.key()).or_default();
    if let Some(existing) = state.records[slot].as_ref().filter(|entry| !entry.retired) {
        return same_persisted_record(&existing.record, &record)
            .then_some(())
            .ok_or(LsmMembershipDenial::MembershipAmbiguous);
    }
    state.records[slot] = Some(super::super::state::RecordState {
        record,
        retired: false,
    });
    state.version = state.version.saturating_add(1);
    Ok(())
}

fn same_persisted_record(left: &LsmMembershipRecord, right: &LsmMembershipRecord) -> bool {
    left.envelope == right.envelope
        && left.durable_scope == right.durable_scope
        && left.key == right.key
        && left.persisted_offset == right.persisted_offset
        && left.persisted_bytes == right.persisted_bytes
        && std::fs::canonicalize(&left.persisted_path).ok()
            == std::fs::canonicalize(&right.persisted_path).ok()
}

pub(in crate::membership::runtime) fn component_slot(
    kind: crate::BlobWalRecordKind,
) -> Option<usize> {
    match kind {
        crate::BlobWalRecordKind::LsmValue => Some(0),
        crate::BlobWalRecordKind::GenerationPublication => Some(1),
        crate::BlobWalRecordKind::LsmTombstone => Some(2),
        _ => None,
    }
}

pub(in crate::membership::runtime) fn owner_cases(
) -> impl Iterator<Item = LsmMembershipOwnerCaseDeclaration> {
    const DENIALS: [LsmMembershipDenial; 4] = [
        LsmMembershipDenial::DurableRecordBindingMismatch,
        LsmMembershipDenial::StoreBindingMismatch,
        LsmMembershipDenial::UnsupportedRecordKind,
        LsmMembershipDenial::MembershipAmbiguous,
    ];
    std::iter::once(LsmMembershipOwnerCaseDeclaration::owned(
        LsmMembershipOwnerCaseId::admitted(LsmMembershipOperation::PersistRecord),
    ))
    .chain(DENIALS.into_iter().map(|denial| {
        LsmMembershipOwnerCaseDeclaration::owned(LsmMembershipOwnerCaseId::denied(
            LsmMembershipOperation::PersistRecord,
            denial,
        ))
    }))
}
