use super::{
    S8LayoutMachineState, S8LayoutMachineTransition, S8LayoutProductionOperation,
    S8LayoutProductionTransition, S8LayoutStateMachine,
};

impl S8LayoutProductionTransition {
    pub(crate) fn from_compaction_owner(
        owner: forge_store_physical_isolation::CompactionCutoverTransition,
    ) -> Self {
        Self::new(
            S8LayoutStateMachine::CompactionCutover,
            owner_operation(owner.kind()),
            owner_case_name(owner.kind()),
            owner_state(owner.from()),
            owner_transition(owner.kind()),
            owner_state(owner.to()),
        )
    }
}

const fn owner_operation(
    transition: forge_store_physical_isolation::CompactionCutoverTransitionKind,
) -> S8LayoutProductionOperation {
    use forge_store_physical_isolation::CompactionCutoverTransitionKind as Owner;
    match transition {
        Owner::LowerRewrite => S8LayoutProductionOperation::LowerCompactionRewrite,
        Owner::AdmitLsmTombstoneRetention | Owner::DenyLsmPhysicalTarget => {
            S8LayoutProductionOperation::AdmitCompactionTombstoneRetention
        }
        Owner::PublishRewrite => S8LayoutProductionOperation::PublishCompactionRewrite,
        Owner::AdmitRecoveryVisibility => {
            S8LayoutProductionOperation::AdmitCompactionRecoveryVisibility
        }
        Owner::DeferReclaim => S8LayoutProductionOperation::DeferCompactionReclaim,
        Owner::DrainReclaimAfterReadRelease => S8LayoutProductionOperation::DrainCompactionReclaim,
        Owner::DenyInPlaceOverwrite
        | Owner::DenyEarlyReclaim
        | Owner::DenyStaleEpochReuse
        | Owner::DenyBackendResidue
        | Owner::DenyLatchHierarchyInversion
        | Owner::DenyMixedRootRead => S8LayoutProductionOperation::DenyCompactionMutation,
    }
}

const fn owner_case_name(
    transition: forge_store_physical_isolation::CompactionCutoverTransitionKind,
) -> &'static str {
    use forge_store_physical_isolation::CompactionCutoverTransitionKind as Owner;
    match transition {
        Owner::LowerRewrite => "LOWER_REWRITE",
        Owner::PublishRewrite => "PUBLISH_REWRITE",
        Owner::AdmitLsmTombstoneRetention => "ADMIT_LSM_TOMBSTONE_RETENTION",
        Owner::DenyLsmPhysicalTarget => "DENY_LSM_PHYSICAL_TARGET",
        Owner::AdmitRecoveryVisibility => "ADMIT_RECOVERY_VISIBILITY",
        Owner::DeferReclaim => "DEFER_RECLAIM",
        Owner::DrainReclaimAfterReadRelease => "DRAIN_RECLAIM_AFTER_READ_RELEASE",
        Owner::DenyInPlaceOverwrite => "DENY_IN_PLACE_OVERWRITE",
        Owner::DenyEarlyReclaim => "DENY_EARLY_RECLAIM",
        Owner::DenyStaleEpochReuse => "DENY_STALE_EPOCH_REUSE",
        Owner::DenyBackendResidue => "DENY_BACKEND_RESIDUE",
        Owner::DenyLatchHierarchyInversion => "DENY_LATCH_HIERARCHY_INVERSION",
        Owner::DenyMixedRootRead => "DENY_MIXED_ROOT_READ",
    }
}

const fn owner_state(
    state: forge_store_physical_isolation::CompactionCutoverState,
) -> S8LayoutMachineState {
    use forge_store_physical_isolation::CompactionCutoverState as Owner;
    match state {
        Owner::PlanAdmitted => S8LayoutMachineState::CompactionPlanAdmitted,
        Owner::RewriteLowered => S8LayoutMachineState::CompactionRewriteLowered,
        Owner::LsmTombstoneRetentionAdmitted => {
            S8LayoutMachineState::CompactionTombstoneRetentionAdmitted
        }
        Owner::PublicationCommitted => S8LayoutMachineState::CompactionPublicationCommitted,
        Owner::RecoveryVisibilityAdmitted => {
            S8LayoutMachineState::CompactionRecoveryVisibilityAdmitted
        }
        Owner::ReclaimDeferred => S8LayoutMachineState::CompactionReclaimDeferred,
        Owner::Reclaimed => S8LayoutMachineState::CompactionReclaimed,
        Owner::Denied => S8LayoutMachineState::Denied,
    }
}

const fn owner_transition(
    transition: forge_store_physical_isolation::CompactionCutoverTransitionKind,
) -> S8LayoutMachineTransition {
    use forge_store_physical_isolation::CompactionCutoverTransitionKind as Owner;
    match transition {
        Owner::LowerRewrite => S8LayoutMachineTransition::Lower,
        Owner::PublishRewrite => S8LayoutMachineTransition::Publish,
        Owner::AdmitLsmTombstoneRetention => S8LayoutMachineTransition::AdmitTombstoneRetention,
        Owner::AdmitRecoveryVisibility => S8LayoutMachineTransition::AdmitRecoveryVisibility,
        Owner::DeferReclaim => S8LayoutMachineTransition::DeferReclaim,
        Owner::DrainReclaimAfterReadRelease => S8LayoutMachineTransition::Reclaim,
        Owner::DenyLsmPhysicalTarget
        | Owner::DenyInPlaceOverwrite
        | Owner::DenyEarlyReclaim
        | Owner::DenyStaleEpochReuse
        | Owner::DenyBackendResidue
        | Owner::DenyLatchHierarchyInversion
        | Owner::DenyMixedRootRead => S8LayoutMachineTransition::Deny,
    }
}
