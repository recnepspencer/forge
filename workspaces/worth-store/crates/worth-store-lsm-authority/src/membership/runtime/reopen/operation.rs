use crate::membership::model::LsmMembershipReadmissionAuthority;
use crate::membership::{
    LsmMembershipDenial, LsmMembershipOperation, LsmMembershipOwnerCaseDeclaration,
    LsmMembershipOwnerCaseId, LsmMembershipOwnerCaseObservation, LsmMembershipReopenCounters,
    LsmMembershipReplayPosture, LsmMembershipSession,
};
use crate::{WalArtifactInventory, WalFrameArtifactObservation};
use std::collections::HashMap;
use worth_store_wal::WalArtifactStoreDenial;

#[derive(Debug)]
enum OpenCase {
    Admitted(Box<LsmMembershipSession>),
    Denied(LsmMembershipDenial),
}

#[derive(Debug)]
pub struct LsmMembershipOpenOutcome {
    case: OpenCase,
}

#[derive(Debug)]
pub enum LsmMembershipOpenView<'a> {
    Admitted(&'a LsmMembershipSession),
    Denied(LsmMembershipDenial),
}

impl LsmMembershipOpenOutcome {
    fn issue(result: Result<LsmMembershipSession, LsmMembershipDenial>) -> Self {
        Self {
            case: match result {
                Ok(session) => OpenCase::Admitted(Box::new(session)),
                Err(denial) => OpenCase::Denied(denial),
            },
        }
    }

    pub const fn view(&self) -> LsmMembershipOpenView<'_> {
        match &self.case {
            OpenCase::Admitted(session) => LsmMembershipOpenView::Admitted(session),
            OpenCase::Denied(denial) => LsmMembershipOpenView::Denied(*denial),
        }
    }

    pub const fn owner_case_observation(&self) -> LsmMembershipOwnerCaseObservation {
        LsmMembershipOwnerCaseObservation::issued(match &self.case {
            OpenCase::Admitted(_) => {
                LsmMembershipOwnerCaseId::admitted(LsmMembershipOperation::Open)
            }
            OpenCase::Denied(denial) => {
                LsmMembershipOwnerCaseId::denied(LsmMembershipOperation::Open, *denial)
            }
        })
    }

    pub fn into_result(self) -> Result<LsmMembershipSession, LsmMembershipDenial> {
        match self.case {
            OpenCase::Admitted(session) => Ok(*session),
            OpenCase::Denied(denial) => Err(denial),
        }
    }
}

pub fn open_lsm_membership(
    anchor: &WalFrameArtifactObservation,
    current_scope: &worth_store_security::StoreCurrentSecurityScopeWitnessSet,
) -> LsmMembershipOpenOutcome {
    let Some(wal_directory) = anchor.path().parent() else {
        return LsmMembershipOpenOutcome::issue(Err(LsmMembershipDenial::StoreBindingMismatch));
    };
    let Some(root) = wal_directory.parent() else {
        return LsmMembershipOpenOutcome::issue(Err(LsmMembershipDenial::StoreBindingMismatch));
    };
    let store = match WalArtifactInventory::open(
        root,
        anchor.scope().segment_id(),
        anchor.scope().generation(),
    ) {
        Ok(store) => store,
        Err(denial) => {
            return LsmMembershipOpenOutcome::issue(Err(map_store_denial(denial)));
        }
    };
    reopen_lsm_membership_from_store(store, current_scope)
}

pub fn reopen_lsm_membership_from_store(
    store: WalArtifactInventory,
    current_scope: &worth_store_security::StoreCurrentSecurityScopeWitnessSet,
) -> LsmMembershipOpenOutcome {
    LsmMembershipOpenOutcome::issue(execute_reopen(store, current_scope))
}

fn execute_reopen(
    store: WalArtifactInventory,
    current_scope: &worth_store_security::StoreCurrentSecurityScopeWitnessSet,
) -> Result<LsmMembershipSession, LsmMembershipDenial> {
    let segment_id = store.identity().segment_id();
    let generation = store.identity().generation();
    let mut session = LsmMembershipSession {
        keys: HashMap::new(),
        store_binding: store.identity().stable_binding(),
        store,
        readmission_authority: LsmMembershipReadmissionAuthority::from_current_scope(current_scope),
        segment_id,
        generation,
        replay_posture: LsmMembershipReplayPosture::DurableArtifactsReadmitted,
        reopen_counters: LsmMembershipReopenCounters::default(),
    };
    super::replay::rebuild_from_store(&mut session)?;
    Ok(session)
}

pub(super) const fn map_store_denial(denial: WalArtifactStoreDenial) -> LsmMembershipDenial {
    match denial {
        WalArtifactStoreDenial::Io => LsmMembershipDenial::Io,
        WalArtifactStoreDenial::InvalidArtifactPath
        | WalArtifactStoreDenial::StoreBindingMismatch => LsmMembershipDenial::StoreBindingMismatch,
        WalArtifactStoreDenial::InvalidFrame
        | WalArtifactStoreDenial::DigestMismatch
        | WalArtifactStoreDenial::NonContiguousLsn
        | WalArtifactStoreDenial::ArtifactReadBudgetExceeded { .. } => {
            LsmMembershipDenial::PersistedMembershipArtifactInvalid
        }
    }
}

pub(in crate::membership::runtime) fn owner_cases(
) -> impl Iterator<Item = LsmMembershipOwnerCaseDeclaration> {
    const DENIALS: [LsmMembershipDenial; 9] = [
        LsmMembershipDenial::CanonicalKeyRequired,
        LsmMembershipDenial::DurableRecordBindingMismatch,
        LsmMembershipDenial::UnsupportedRecordKind,
        LsmMembershipDenial::MembershipAmbiguous,
        LsmMembershipDenial::MembershipStale,
        LsmMembershipDenial::ManifestMembershipMismatch,
        LsmMembershipDenial::ReplacementOutputMismatch,
        LsmMembershipDenial::PersistedMembershipArtifactInvalid,
        LsmMembershipDenial::Io,
    ];
    std::iter::once(LsmMembershipOwnerCaseDeclaration::owned(
        LsmMembershipOwnerCaseId::admitted(LsmMembershipOperation::Open),
    ))
    .chain(DENIALS.into_iter().map(|denial| {
        LsmMembershipOwnerCaseDeclaration::owned(LsmMembershipOwnerCaseId::denied(
            LsmMembershipOperation::Open,
            denial,
        ))
    }))
}
