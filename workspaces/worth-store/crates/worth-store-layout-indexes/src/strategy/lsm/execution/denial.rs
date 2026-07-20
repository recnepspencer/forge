#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaselineLsmExecutionAdmissionDenial {
    StrategyInvariant(crate::strategy::StrategyDenial),
    CanonicalKeyRequired,
    MemtableDoesNotFollowSortedRuns,
    SortedRunsNotCanonical,
    ReplayTailNotCanonical,
    ReplayBindingMismatch,
    TombstoneRecordRequired,
    ValueRecordRequired,
    GenerationRecordRequired,
    OutputGenerationOverflow,
    OutputPublicationMismatch,
    ManifestPublicationRequired,
    ManifestDoesNotCoverCompaction,
    ManifestMembershipMismatch,
    PersistedMembershipAmbiguous,
    PersistedMembershipIncomplete,
    PersistedMembershipStale,
    PersistedArtifactInvalid,
    PersistedIndexIo,
    PhysicalTargetEpochRequired,
    DurableRecordBindingMismatch,
    RecordKeyScopeMismatch,
    PhysicalPublicationBindingMismatch,
    SelectedOperationKeyMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BaselineLsmExecutionAdmissionDenialKind {
    StrategyInvariant,
    CanonicalKeyRequired,
    MemtableDoesNotFollowSortedRuns,
    SortedRunsNotCanonical,
    ReplayTailNotCanonical,
    ReplayBindingMismatch,
    TombstoneRecordRequired,
    ValueRecordRequired,
    GenerationRecordRequired,
    OutputGenerationOverflow,
    OutputPublicationMismatch,
    ManifestPublicationRequired,
    ManifestDoesNotCoverCompaction,
    ManifestMembershipMismatch,
    PersistedMembershipAmbiguous,
    PersistedMembershipIncomplete,
    PersistedMembershipStale,
    PersistedArtifactInvalid,
    PersistedIndexIo,
    PhysicalTargetEpochRequired,
    DurableRecordBindingMismatch,
    RecordKeyScopeMismatch,
    PhysicalPublicationBindingMismatch,
    SelectedOperationKeyMismatch,
}

impl BaselineLsmExecutionAdmissionDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StrategyInvariant => "strategy_invariant",
            Self::CanonicalKeyRequired => "canonical_key_required",
            Self::MemtableDoesNotFollowSortedRuns => "memtable_does_not_follow_sorted_runs",
            Self::SortedRunsNotCanonical => "sorted_runs_not_canonical",
            Self::ReplayTailNotCanonical => "replay_tail_not_canonical",
            Self::ReplayBindingMismatch => "replay_binding_mismatch",
            Self::TombstoneRecordRequired => "tombstone_record_required",
            Self::ValueRecordRequired => "value_record_required",
            Self::GenerationRecordRequired => "generation_record_required",
            Self::OutputGenerationOverflow => "output_generation_overflow",
            Self::OutputPublicationMismatch => "output_publication_mismatch",
            Self::ManifestPublicationRequired => "manifest_publication_required",
            Self::ManifestDoesNotCoverCompaction => "manifest_does_not_cover_compaction",
            Self::ManifestMembershipMismatch => "manifest_membership_mismatch",
            Self::PersistedMembershipAmbiguous => "persisted_membership_ambiguous",
            Self::PersistedMembershipIncomplete => "persisted_membership_incomplete",
            Self::PersistedMembershipStale => "persisted_membership_stale",
            Self::PersistedArtifactInvalid => "persisted_artifact_invalid",
            Self::PersistedIndexIo => "persisted_index_io",
            Self::PhysicalTargetEpochRequired => "physical_target_epoch_required",
            Self::DurableRecordBindingMismatch => "durable_record_binding_mismatch",
            Self::RecordKeyScopeMismatch => "record_key_scope_mismatch",
            Self::PhysicalPublicationBindingMismatch => "physical_publication_binding_mismatch",
            Self::SelectedOperationKeyMismatch => "selected_operation_key_mismatch",
        }
    }
}

impl BaselineLsmExecutionAdmissionDenial {
    pub const fn kind(&self) -> BaselineLsmExecutionAdmissionDenialKind {
        match self {
            Self::StrategyInvariant(_) => {
                BaselineLsmExecutionAdmissionDenialKind::StrategyInvariant
            }
            Self::CanonicalKeyRequired => {
                BaselineLsmExecutionAdmissionDenialKind::CanonicalKeyRequired
            }
            Self::MemtableDoesNotFollowSortedRuns => {
                BaselineLsmExecutionAdmissionDenialKind::MemtableDoesNotFollowSortedRuns
            }
            Self::SortedRunsNotCanonical => {
                BaselineLsmExecutionAdmissionDenialKind::SortedRunsNotCanonical
            }
            Self::ReplayTailNotCanonical => {
                BaselineLsmExecutionAdmissionDenialKind::ReplayTailNotCanonical
            }
            Self::ReplayBindingMismatch => {
                BaselineLsmExecutionAdmissionDenialKind::ReplayBindingMismatch
            }
            Self::TombstoneRecordRequired => {
                BaselineLsmExecutionAdmissionDenialKind::TombstoneRecordRequired
            }
            Self::ValueRecordRequired => {
                BaselineLsmExecutionAdmissionDenialKind::ValueRecordRequired
            }
            Self::GenerationRecordRequired => {
                BaselineLsmExecutionAdmissionDenialKind::GenerationRecordRequired
            }
            Self::OutputGenerationOverflow => {
                BaselineLsmExecutionAdmissionDenialKind::OutputGenerationOverflow
            }
            Self::OutputPublicationMismatch => {
                BaselineLsmExecutionAdmissionDenialKind::OutputPublicationMismatch
            }
            Self::ManifestPublicationRequired => {
                BaselineLsmExecutionAdmissionDenialKind::ManifestPublicationRequired
            }
            Self::ManifestDoesNotCoverCompaction => {
                BaselineLsmExecutionAdmissionDenialKind::ManifestDoesNotCoverCompaction
            }
            Self::ManifestMembershipMismatch => {
                BaselineLsmExecutionAdmissionDenialKind::ManifestMembershipMismatch
            }
            Self::PersistedMembershipAmbiguous => {
                BaselineLsmExecutionAdmissionDenialKind::PersistedMembershipAmbiguous
            }
            Self::PersistedMembershipIncomplete => {
                BaselineLsmExecutionAdmissionDenialKind::PersistedMembershipIncomplete
            }
            Self::PersistedMembershipStale => {
                BaselineLsmExecutionAdmissionDenialKind::PersistedMembershipStale
            }
            Self::PersistedArtifactInvalid => {
                BaselineLsmExecutionAdmissionDenialKind::PersistedArtifactInvalid
            }
            Self::PersistedIndexIo => BaselineLsmExecutionAdmissionDenialKind::PersistedIndexIo,
            Self::PhysicalTargetEpochRequired => {
                BaselineLsmExecutionAdmissionDenialKind::PhysicalTargetEpochRequired
            }
            Self::DurableRecordBindingMismatch => {
                BaselineLsmExecutionAdmissionDenialKind::DurableRecordBindingMismatch
            }
            Self::RecordKeyScopeMismatch => {
                BaselineLsmExecutionAdmissionDenialKind::RecordKeyScopeMismatch
            }
            Self::PhysicalPublicationBindingMismatch => {
                BaselineLsmExecutionAdmissionDenialKind::PhysicalPublicationBindingMismatch
            }
            Self::SelectedOperationKeyMismatch => {
                BaselineLsmExecutionAdmissionDenialKind::SelectedOperationKeyMismatch
            }
        }
    }
}
