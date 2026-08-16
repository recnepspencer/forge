use super::{
    FinancialAspect, FinancialLocalityMutation, FinancialLocalitySubscription,
    FinancialStructuralMutation, LocalityEconomicOwner, LocalityScope, LocalitySemanticOutputId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityTopologyChange {
    pub(in crate::tests::domains::fintech) target: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) before_owner: LocalityEconomicOwner,
    pub(in crate::tests::domains::fintech) after_owner: LocalityEconomicOwner,
    pub(in crate::tests::domains::fintech) before_subscription: FinancialLocalitySubscription,
    pub(in crate::tests::domains::fintech) after_subscription: FinancialLocalitySubscription,
    pub(in crate::tests::domains::fintech) structural: FinancialStructuralMutation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityStagedWork {
    pub(in crate::tests::domains::fintech) target: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) dependency_revision: u64,
    pub(in crate::tests::domains::fintech) readiness_epoch: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) struct FinancialLocalitySourceObligation {
    pub(in crate::tests::domains::fintech) source: LocalitySemanticOutputId,
    pub(in crate::tests::domains::fintech) aspect: FinancialAspect,
    pub(in crate::tests::domains::fintech) scope: Option<LocalityScope>,
    pub(in crate::tests::domains::fintech) admission_generation: u64,
    pub(in crate::tests::domains::fintech) dependency_revision: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum FinancialLocalityTraceIdentity {
    PrimaryMutation,
    PartitionWholeRegion,
    PartitionCorrelatedScopes,
    ProducerPermutation(u8),
    PortfolioChurn,
    BranchRestoreReplay,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) enum FinancialLocalityAction {
    CommitFactor(FinancialLocalityMutation),
    RetryAdmission {
        target: LocalitySemanticOutputId,
        retry_ordinal: u8,
    },
    StagePreRewireWork {
        round: u16,
        binding: FinancialLocalityStagedWork,
    },
    StageSourceRecompute {
        obligation: FinancialLocalitySourceObligation,
    },
    AcceptedOwnerMove {
        round: u16,
        change: FinancialLocalityTopologyChange,
    },
    RejectStaleWork {
        round: u16,
        stale: FinancialLocalityStagedWork,
        current_dependency_revision: u64,
    },
    AcceptedDependencyRemoval {
        round: u16,
        owner: LocalityEconomicOwner,
        removed_subscription: FinancialLocalitySubscription,
        structural: FinancialStructuralMutation,
    },
    AcceptedDependencyRecreation {
        round: u16,
        owner: LocalityEconomicOwner,
        subscription: FinancialLocalitySubscription,
        structural: FinancialStructuralMutation,
    },
    RejectedCycle {
        round: u16,
        target: LocalitySemanticOutputId,
        attempted_subscription: FinancialLocalitySubscription,
        attempted_topology_ordinal: u64,
        retained_dependency_revision: u64,
    },
    CaptureBranch {
        branch_ordinal: u64,
    },
    CaptureCheckpoint {
        checkpoint_ordinal: u64,
    },
    DestroyDerivedState {
        destruction_ordinal: u64,
    },
    ReadmitFreshRuntime {
        runtime_epoch: u64,
    },
    ReplayCanonicalTrace {
        replay_ordinal: u64,
    },
    DeterministicRerun {
        rerun_ordinal: u64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityActionTrace {
    identity: FinancialLocalityTraceIdentity,
    actions: Vec<FinancialLocalityAction>,
}

impl FinancialLocalityActionTrace {
    pub(super) fn new(
        identity: FinancialLocalityTraceIdentity,
        actions: Vec<FinancialLocalityAction>,
    ) -> Self {
        assert!(
            !actions.is_empty(),
            "a locality action trace must not be empty"
        );
        Self { identity, actions }
    }

    pub(in crate::tests::domains::fintech) const fn identity(
        &self,
    ) -> FinancialLocalityTraceIdentity {
        self.identity
    }

    pub(in crate::tests::domains::fintech) fn actions(&self) -> &[FinancialLocalityAction] {
        &self.actions
    }

    pub(in crate::tests::domains::fintech) fn committed_mutations(
        &self,
    ) -> Vec<FinancialLocalityMutation> {
        self.actions
            .iter()
            .filter_map(|action| match action {
                FinancialLocalityAction::CommitFactor(mutation) => Some(*mutation),
                _ => None,
            })
            .collect()
    }

    pub(in crate::tests::domains::fintech) fn structural_mutations(
        &self,
    ) -> Vec<FinancialStructuralMutation> {
        self.actions
            .iter()
            .filter_map(|action| match action {
                FinancialLocalityAction::AcceptedOwnerMove { change, .. } => {
                    Some(change.structural)
                }
                FinancialLocalityAction::AcceptedDependencyRemoval { structural, .. }
                | FinancialLocalityAction::AcceptedDependencyRecreation { structural, .. } => {
                    Some(*structural)
                }
                _ => None,
            })
            .collect()
    }

    pub(in crate::tests::domains::fintech) fn retry_count(&self) -> usize {
        self.actions
            .iter()
            .filter(|action| matches!(action, FinancialLocalityAction::RetryAdmission { .. }))
            .count()
    }

    pub(in crate::tests::domains::fintech) fn readiness_epoch(&self) -> u64 {
        1 + self
            .actions
            .iter()
            .filter(|action| matches!(action, FinancialLocalityAction::ReadmitFreshRuntime { .. }))
            .count() as u64
    }
}
