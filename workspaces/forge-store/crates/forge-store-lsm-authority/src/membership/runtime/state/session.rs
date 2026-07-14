use super::super::super::model::{
    LsmMembershipKey, LsmMembershipReadmissionAuthority, LsmMembershipRecord,
};
use crate::{AdmittedWalArtifactStore, PublishedLsmMembershipReplacement};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LsmMembershipDenial {
    CanonicalKeyRequired,
    DurableRecordBindingMismatch,
    StoreBindingMismatch,
    UnsupportedRecordKind,
    MembershipAmbiguous,
    MembershipIncomplete,
    ValueRecordRequired,
    GenerationRecordRequired,
    TombstoneRecordRequired,
    MembershipStale,
    ManifestMembershipMismatch,
    ReplacementOutputMismatch,
    PhysicalPublicationBindingMismatch,
    PersistedMembershipArtifactInvalid,
    Io,
}

impl LsmMembershipDenial {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalKeyRequired => "canonical_key_required",
            Self::DurableRecordBindingMismatch => "durable_record_binding_mismatch",
            Self::StoreBindingMismatch => "store_binding_mismatch",
            Self::UnsupportedRecordKind => "unsupported_record_kind",
            Self::MembershipAmbiguous => "membership_ambiguous",
            Self::MembershipIncomplete => "membership_incomplete",
            Self::ValueRecordRequired => "value_record_required",
            Self::GenerationRecordRequired => "generation_record_required",
            Self::TombstoneRecordRequired => "tombstone_record_required",
            Self::MembershipStale => "membership_stale",
            Self::ManifestMembershipMismatch => "manifest_membership_mismatch",
            Self::ReplacementOutputMismatch => "replacement_output_mismatch",
            Self::PhysicalPublicationBindingMismatch => "physical_publication_binding_mismatch",
            Self::PersistedMembershipArtifactInvalid => "persisted_membership_artifact_invalid",
            Self::Io => "io",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LsmMembershipReplayPosture {
    DurableArtifactsReadmitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LsmMembershipReopenCounters {
    pub(in crate::membership::runtime) artifacts_examined: u64,
    pub(in crate::membership::runtime) artifacts_readmitted: u64,
    pub(in crate::membership::runtime) bytes_examined: u64,
}

impl LsmMembershipReopenCounters {
    pub const fn artifacts_examined(self) -> u64 {
        self.artifacts_examined
    }

    pub const fn artifacts_readmitted(self) -> u64 {
        self.artifacts_readmitted
    }

    pub const fn bytes_examined(self) -> u64 {
        self.bytes_examined
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::membership::runtime) struct RecordState {
    pub(in crate::membership::runtime) record: LsmMembershipRecord,
    pub(in crate::membership::runtime) retired: bool,
}

#[derive(Debug, Default)]
pub(in crate::membership::runtime) struct KeyState {
    pub(in crate::membership::runtime) records: [Option<RecordState>; 3],
    pub(in crate::membership::runtime) version: u64,
    pub(in crate::membership::runtime) published_replacement:
        Option<PublishedLsmMembershipReplacement>,
}

#[derive(Debug)]
pub struct LsmMembershipSession {
    pub(in crate::membership::runtime) keys: HashMap<LsmMembershipKey, KeyState>,
    pub(in crate::membership::runtime) store: AdmittedWalArtifactStore,
    pub(in crate::membership::runtime) store_binding: String,
    pub(in crate::membership::runtime) readmission_authority: LsmMembershipReadmissionAuthority,
    pub(in crate::membership::runtime) segment_id: u64,
    pub(in crate::membership::runtime) generation: u64,
    pub(in crate::membership::runtime) replay_posture: LsmMembershipReplayPosture,
    pub(in crate::membership::runtime) reopen_counters: LsmMembershipReopenCounters,
}

impl LsmMembershipSession {
    pub const fn replay_posture(&self) -> LsmMembershipReplayPosture {
        self.replay_posture
    }

    pub const fn reopen_counters(&self) -> LsmMembershipReopenCounters {
        self.reopen_counters
    }
}
