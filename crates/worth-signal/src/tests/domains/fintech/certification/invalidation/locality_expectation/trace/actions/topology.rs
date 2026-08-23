use super::InterpreterState;
use crate::tests::domains::fintech::certification::invalidation::locality_expectation::ExpectedActionCheckpointKind;
use crate::tests::domains::fintech::world::{
    FinancialLocalityOutput, FinancialLocalityStagedWork, FinancialLocalitySubscription,
    FinancialLocalityTopologyChange, FinancialStructuralMutation, LocalityEconomicOwner,
    LocalitySemanticOutputId,
};

impl InterpreterState {
    pub(super) fn stage_pre_rewire(
        &mut self,
        action_ordinal: u32,
        round: u16,
        binding: FinancialLocalityStagedWork,
    ) {
        assert_eq!(self.revisions[&binding.target], binding.dependency_revision);
        assert_eq!(binding.readiness_epoch, self.readiness_epoch);
        assert!(self.staged.insert(round, binding).is_none());
        self.checkpoint(
            action_ordinal,
            ExpectedActionCheckpointKind::PreRewireStaged(binding),
        );
    }

    pub(super) fn accept_owner_move(
        &mut self,
        action_ordinal: u32,
        round: u16,
        change: FinancialLocalityTopologyChange,
    ) {
        assert!(self.staged.contains_key(&round));
        let prior_revision = self.revisions[&change.target];
        let output = self.output(change.target);
        assert_eq!(output.owner, change.before_owner);
        assert_eq!(output.subscriptions, [change.before_subscription]);
        assert_eq!(change.structural.target, change.target);
        self.accept_topology_ordinal(change.structural.topology_mutation_ordinal);
        assert_eq!(
            change.structural.resulting_dependency_revision,
            prior_revision + 1
        );
        let output = self.output_mut(change.target);
        output.owner = change.after_owner;
        output.subscriptions = vec![change.after_subscription];
        self.accept_structural(action_ordinal, change.structural);
    }

    pub(super) fn reject_stale_work(
        &mut self,
        action_ordinal: u32,
        round: u16,
        stale: FinancialLocalityStagedWork,
        current_dependency_revision: u64,
    ) {
        assert_eq!(self.staged.remove(&round), Some(stale));
        assert_eq!(self.revisions[&stale.target], current_dependency_revision);
        assert!(stale.dependency_revision < current_dependency_revision);
        self.stale_denials += 1;
        self.checkpoint(
            action_ordinal,
            ExpectedActionCheckpointKind::StaleWorkDenied {
                stale,
                current_dependency_revision,
            },
        );
    }

    pub(super) fn accept_dependency_removal(
        &mut self,
        action_ordinal: u32,
        round: u16,
        owner: LocalityEconomicOwner,
        removed_subscription: FinancialLocalitySubscription,
        structural: FinancialStructuralMutation,
    ) {
        assert!(!self.staged.contains_key(&round));
        let output = self.output(structural.target);
        assert_eq!(output.owner, owner);
        assert_eq!(output.subscriptions, [removed_subscription]);
        self.validate_next_structural(structural);
        self.output_mut(structural.target).subscriptions.clear();
        self.accept_structural(action_ordinal, structural);
    }

    pub(super) fn accept_dependency_recreation(
        &mut self,
        action_ordinal: u32,
        round: u16,
        owner: LocalityEconomicOwner,
        subscription: FinancialLocalitySubscription,
        structural: FinancialStructuralMutation,
    ) {
        assert!(!self.staged.contains_key(&round));
        let output = self.output(structural.target);
        assert_eq!(output.owner, owner);
        assert!(output.subscriptions.is_empty());
        self.validate_next_structural(structural);
        self.output_mut(structural.target).subscriptions = vec![subscription];
        self.accept_structural(action_ordinal, structural);
    }

    pub(super) fn reject_cycle(
        &mut self,
        action_ordinal: u32,
        target: LocalitySemanticOutputId,
        attempted_subscription: FinancialLocalitySubscription,
        attempted_topology_ordinal: u64,
        retained_dependency_revision: u64,
    ) {
        assert_eq!(self.revisions[&target], retained_dependency_revision);
        assert_eq!(attempted_topology_ordinal, self.last_topology_ordinal + 1);
        assert!(depends_on(
            &self.outputs,
            attempted_subscription.upstream,
            target
        ));
        self.rejected_topology_mutations += 1;
        self.checkpoint(
            action_ordinal,
            ExpectedActionCheckpointKind::CycleRejected {
                target,
                attempted_topology_ordinal,
                retained_dependency_revision,
            },
        );
    }

    fn validate_next_structural(&mut self, structural: FinancialStructuralMutation) {
        assert_eq!(
            structural.resulting_dependency_revision,
            self.revisions[&structural.target] + 1
        );
        self.accept_topology_ordinal(structural.topology_mutation_ordinal);
    }

    fn accept_structural(&mut self, action_ordinal: u32, structural: FinancialStructuralMutation) {
        self.revisions
            .insert(structural.target, structural.resulting_dependency_revision);
        self.structural.push((action_ordinal, structural));
        self.pending_structural = Some(structural);
        self.checkpoint(
            action_ordinal,
            ExpectedActionCheckpointKind::TopologyAccepted(structural),
        );
        self.topology_revalidations += 1;
    }

    fn accept_topology_ordinal(&mut self, ordinal: u64) {
        assert_eq!(ordinal, self.last_topology_ordinal + 1);
        self.last_topology_ordinal = ordinal;
    }
}

fn depends_on(
    outputs: &[FinancialLocalityOutput],
    start: LocalitySemanticOutputId,
    target: LocalitySemanticOutputId,
) -> bool {
    start == target
        || outputs[start.ordinal() as usize]
            .subscriptions
            .iter()
            .any(|subscription| depends_on(outputs, subscription.upstream, target))
}
