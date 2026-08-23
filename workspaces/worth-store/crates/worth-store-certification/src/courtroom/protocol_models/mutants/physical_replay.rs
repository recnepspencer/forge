use worth_store_formal_models::protocol_bindings::OwnerOperationFamily;
use worth_store_formal_models::runner::{
    CanonicalProtocolAction, CanonicalProtocolTrace, ProtocolFrontierIdentity,
};
use worth_store_formal_models::{compose_compaction_action, compose_lease_action};
use worth_store_physical_backend::BackendDurabilityProfileId;
use worth_store_physical_certification::{
    SchedulePerturbationSeed, ScheduleReplayIdentity, ScheduleShrinkTrace,
};

use super::mapped_guard::require_mapped_guard;
use super::scheduled_shrink::shrink_mapped_counterexample_schedule;
use super::{ControlledMutantLocalization, ControlledProtocolMutant};
use crate::courtroom::protocol_models::{
    compaction_visibility::scenarios::replay_compaction_publication_guard,
    durability_recovery::scenario::replay_acknowledgment_ordering_guard,
    import_publication::scenario::replay_import_publication_guard,
    lease_reclaim::scenario::replay_live_lease_reclaim_guard,
    quarantine_readmission::scenario::replay_unverified_readmission_guard,
    replication_admission::scenarios::replay_replication_divergence_guard,
    source_precedence::scenario::replay_quarantined_source_guard,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterexampleOwnerIdentity {
    BoundOperation(OwnerOperationFamily),
    SharedReachabilityOwners,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcreteCounterexampleGuard {
    FailedFencePreventedAcknowledgment,
    QuarantinedSourceNotSelected,
    CompactionLoweredBeforePublication,
    LiveLeaseBlockedReclaim,
    UnverifiedQuarantineNotReadmitted,
    CrashedImportNotPublished,
    DivergentReplicationNotResumed,
    ReachableAuthorityNotReclaimed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CounterexamplePhysicalReplayEvidence {
    mutant: ControlledProtocolMutant,
    seed: SchedulePerturbationSeed,
    backend_profile: Option<BackendDurabilityProfileId>,
    illegal_edge: &'static str,
    owner: CounterexampleOwnerIdentity,
    concrete_guard: ConcreteCounterexampleGuard,
    mapped_transcript: CanonicalProtocolTrace,
    schedule_identity: Option<ScheduleReplayIdentity>,
    schedule_shrink: Option<ScheduleShrinkTrace>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CounterexampleReplayEvidenceIdentity {
    mutant: ControlledProtocolMutant,
    seed: SchedulePerturbationSeed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CounterexamplePhysicalReplayDenial {
    OwnerIdentityMismatch,
    InvalidMappedTranscript,
    MappedGuardMissing,
    ScheduleDefinitionFailed(worth_store_physical_certification::PhysicalScenarioDefinitionDenial),
    SchedulePlanFailed(worth_store_physical_certification::SimulationPlanDenial),
    ScheduleReplayFailed(worth_store_physical_certification::ScheduleReplayDenial),
}

pub(super) fn replay_controlled_counterexample(
    mutant: ControlledProtocolMutant,
    localization: &ControlledMutantLocalization,
) -> Result<CounterexamplePhysicalReplayEvidence, CounterexamplePhysicalReplayDenial> {
    let owner = localized_owner(localization);
    if owner != expected_owner(mutant) {
        return Err(CounterexamplePhysicalReplayDenial::OwnerIdentityMismatch);
    }
    let seed = replay_seed(mutant);
    let (frontier, concrete_guard, actions) = execute_guarded_owner_scenario(mutant, seed);
    let mapped_transcript = CanonicalProtocolTrace::admit(mutant.protocol(), frontier, actions)
        .map_err(|_| CounterexamplePhysicalReplayDenial::InvalidMappedTranscript)?;
    require_mapped_guard(mutant, &mapped_transcript)?;
    Ok(CounterexamplePhysicalReplayEvidence {
        mutant,
        seed,
        backend_profile: replay_backend_profile(mutant),
        illegal_edge: super::localization::expected_checker_edge(mutant),
        owner,
        concrete_guard,
        mapped_transcript,
        schedule_identity: None,
        schedule_shrink: None,
    })
}

const fn replay_backend_profile(
    mutant: ControlledProtocolMutant,
) -> Option<BackendDurabilityProfileId> {
    match mutant {
        ControlledProtocolMutant::DurabilityAcknowledgmentBeforeFence => {
            Some(
                crate::courtroom::protocol_models::durability_recovery::scenario::ordinary_durability_profile(),
            )
        }
        _ => None,
    }
}

fn localized_owner(localization: &ControlledMutantLocalization) -> CounterexampleOwnerIdentity {
    match localization {
        ControlledMutantLocalization::Owner(localization) => {
            CounterexampleOwnerIdentity::BoundOperation(localization.owner_binding().operation())
        }
        ControlledMutantLocalization::Shared(_) => {
            CounterexampleOwnerIdentity::SharedReachabilityOwners
        }
    }
}

fn expected_owner(mutant: ControlledProtocolMutant) -> CounterexampleOwnerIdentity {
    use OwnerOperationFamily as Owner;
    let owner = match mutant {
        ControlledProtocolMutant::DurabilityAcknowledgmentBeforeFence => {
            Owner::WalDurabilityObservation
        }
        ControlledProtocolMutant::RecoveryQuarantinedSourceSelected => {
            Owner::RecoverySourceSelection
        }
        ControlledProtocolMutant::CompactionPublicationBeforeCutover => {
            Owner::PhysicalCompactionCutover
        }
        ControlledProtocolMutant::LeaseIdentityReuseWithLiveLease => Owner::ReclaimReuseFence,
        ControlledProtocolMutant::QuarantineReleaseWithoutVerification => Owner::LayoutReadmission,
        ControlledProtocolMutant::ImportPublicationWithoutDurability => {
            Owner::ImportPublicationCompletion
        }
        ControlledProtocolMutant::ReplicationDivergenceAcceptedAsResume => {
            Owner::ReplicationProgressObservation
        }
        ControlledProtocolMutant::SharedReachableAuthorityReclaimed => {
            return CounterexampleOwnerIdentity::SharedReachabilityOwners;
        }
    };
    CounterexampleOwnerIdentity::BoundOperation(owner)
}

fn replay_seed(mutant: ControlledProtocolMutant) -> SchedulePerturbationSeed {
    let ordinal = match mutant {
        ControlledProtocolMutant::DurabilityAcknowledgmentBeforeFence => 1,
        ControlledProtocolMutant::RecoveryQuarantinedSourceSelected => 2,
        ControlledProtocolMutant::CompactionPublicationBeforeCutover => 3,
        ControlledProtocolMutant::LeaseIdentityReuseWithLiveLease => 4,
        ControlledProtocolMutant::QuarantineReleaseWithoutVerification => 5,
        ControlledProtocolMutant::ImportPublicationWithoutDurability => 6,
        ControlledProtocolMutant::ReplicationDivergenceAcceptedAsResume => 7,
        ControlledProtocolMutant::SharedReachableAuthorityReclaimed => 8,
    };
    SchedulePerturbationSeed::from_u64(0x53_39_00_00 + ordinal)
}

fn execute_guarded_owner_scenario(
    mutant: ControlledProtocolMutant,
    seed: SchedulePerturbationSeed,
) -> (
    ProtocolFrontierIdentity,
    ConcreteCounterexampleGuard,
    Vec<CanonicalProtocolAction>,
) {
    let raw_seed = seed.value();
    match mutant {
        ControlledProtocolMutant::DurabilityAcknowledgmentBeforeFence => (
            ProtocolFrontierIdentity::Durability,
            ConcreteCounterexampleGuard::FailedFencePreventedAcknowledgment,
            replay_acknowledgment_ordering_guard(raw_seed)
                .into_iter()
                .map(CanonicalProtocolAction::DurabilityRecovery)
                .collect(),
        ),
        ControlledProtocolMutant::RecoveryQuarantinedSourceSelected => (
            ProtocolFrontierIdentity::RecoveryPrecedence,
            ConcreteCounterexampleGuard::QuarantinedSourceNotSelected,
            replay_quarantined_source_guard(raw_seed)
                .into_iter()
                .map(CanonicalProtocolAction::RecoverySourcePrecedence)
                .collect(),
        ),
        ControlledProtocolMutant::CompactionPublicationBeforeCutover => (
            ProtocolFrontierIdentity::Visibility,
            ConcreteCounterexampleGuard::CompactionLoweredBeforePublication,
            replay_compaction_publication_guard(raw_seed)
                .into_iter()
                .map(CanonicalProtocolAction::CompactionVisibility)
                .collect(),
        ),
        ControlledProtocolMutant::LeaseIdentityReuseWithLiveLease => (
            ProtocolFrontierIdentity::Reachability,
            ConcreteCounterexampleGuard::LiveLeaseBlockedReclaim,
            replay_live_lease_reclaim_guard(raw_seed)
                .into_iter()
                .map(CanonicalProtocolAction::LeaseReclaim)
                .collect(),
        ),
        ControlledProtocolMutant::QuarantineReleaseWithoutVerification => (
            ProtocolFrontierIdentity::Quarantine,
            ConcreteCounterexampleGuard::UnverifiedQuarantineNotReadmitted,
            replay_unverified_readmission_guard(raw_seed)
                .into_iter()
                .map(CanonicalProtocolAction::QuarantineReadmission)
                .collect(),
        ),
        ControlledProtocolMutant::ImportPublicationWithoutDurability => (
            ProtocolFrontierIdentity::Admission,
            ConcreteCounterexampleGuard::CrashedImportNotPublished,
            replay_import_publication_guard(raw_seed)
                .into_iter()
                .map(CanonicalProtocolAction::ImportPublication)
                .collect(),
        ),
        ControlledProtocolMutant::ReplicationDivergenceAcceptedAsResume => (
            ProtocolFrontierIdentity::Admission,
            ConcreteCounterexampleGuard::DivergentReplicationNotResumed,
            replay_replication_divergence_guard(raw_seed)
                .into_iter()
                .map(CanonicalProtocolAction::ReplicationAdmission)
                .collect(),
        ),
        ControlledProtocolMutant::SharedReachableAuthorityReclaimed => {
            shared_reclaim_guard(raw_seed)
        }
    }
}

fn shared_reclaim_guard(
    seed: u64,
) -> (
    ProtocolFrontierIdentity,
    ConcreteCounterexampleGuard,
    Vec<CanonicalProtocolAction>,
) {
    let compaction = replay_compaction_publication_guard(seed)
        .into_iter()
        .filter_map(compose_compaction_action);
    let lease = replay_live_lease_reclaim_guard(seed)
        .into_iter()
        .filter_map(compose_lease_action);
    (
        ProtocolFrontierIdentity::Reachability,
        ConcreteCounterexampleGuard::ReachableAuthorityNotReclaimed,
        compaction
            .chain(lease)
            .map(CanonicalProtocolAction::SharedFrontier)
            .collect(),
    )
}

impl CounterexamplePhysicalReplayEvidence {
    pub const fn identity(&self) -> CounterexampleReplayEvidenceIdentity {
        CounterexampleReplayEvidenceIdentity {
            mutant: self.mutant,
            seed: self.seed,
        }
    }

    pub const fn seed(&self) -> SchedulePerturbationSeed {
        self.seed
    }

    pub const fn backend_profile(&self) -> Option<BackendDurabilityProfileId> {
        self.backend_profile
    }

    pub const fn illegal_edge(&self) -> &'static str {
        self.illegal_edge
    }

    pub const fn owner(&self) -> CounterexampleOwnerIdentity {
        self.owner
    }

    pub const fn concrete_guard(&self) -> ConcreteCounterexampleGuard {
        self.concrete_guard
    }

    pub const fn mapped_transcript(&self) -> &CanonicalProtocolTrace {
        &self.mapped_transcript
    }

    pub const fn schedule_shrink(&self) -> Option<&ScheduleShrinkTrace> {
        self.schedule_shrink.as_ref()
    }

    pub const fn schedule_identity(&self) -> Option<&ScheduleReplayIdentity> {
        self.schedule_identity.as_ref()
    }

    pub fn shrink_preserving_guard(&self) -> Result<Self, CounterexamplePhysicalReplayDenial> {
        let shrunk = shrink_mapped_counterexample_schedule(
            self.mutant,
            self.seed,
            self.concrete_guard,
            &self.mapped_transcript,
            || execute_guarded_owner_scenario(self.mutant, self.seed).1,
        )?;
        Ok(Self {
            mutant: self.mutant,
            seed: self.seed,
            backend_profile: self.backend_profile,
            illegal_edge: self.illegal_edge,
            owner: self.owner,
            concrete_guard: self.concrete_guard,
            mapped_transcript: shrunk.mapped_transcript,
            schedule_identity: Some(shrunk.schedule_identity),
            schedule_shrink: Some(shrunk.schedule_shrink),
        })
    }
}

impl CounterexampleReplayEvidenceIdentity {
    pub const fn mutant(self) -> ControlledProtocolMutant {
        self.mutant
    }

    pub const fn seed(self) -> SchedulePerturbationSeed {
        self.seed
    }
}

#[cfg(test)]
mod tests {
    use worth_store_formal_models::ProtocolFamily;

    use super::*;

    #[test]
    fn concrete_oracle_survives_a_corrupted_mapping_transcript() {
        let mutant = ControlledProtocolMutant::DurabilityAcknowledgmentBeforeFence;
        let (_, concrete_guard, _) = execute_guarded_owner_scenario(mutant, replay_seed(mutant));
        assert_eq!(
            concrete_guard,
            ConcreteCounterexampleGuard::FailedFencePreventedAcknowledgment
        );
        let corrupted = CanonicalProtocolTrace::admit(
            ProtocolFamily::DurabilityRecovery,
            ProtocolFrontierIdentity::Durability,
            [CanonicalProtocolAction::DurabilityRecovery(
                worth_store_formal_models::DurabilityRecoveryAction::PhysicalMutationAcknowledged,
            )],
        )
        .unwrap();
        assert_eq!(
            require_mapped_guard(mutant, &corrupted),
            Err(CounterexamplePhysicalReplayDenial::MappedGuardMissing)
        );
    }
}
