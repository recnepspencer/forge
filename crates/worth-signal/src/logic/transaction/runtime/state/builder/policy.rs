use crate::data::resource::FrozenResourcePolicyRegistry;
use crate::data::tier::TierPolicy;
use crate::runtime_policy::SignalRuntimePolicy;
use crate::schema::data::SignalSchemaRegistry;

use super::super::merge::{
    AspectMergePolicyRegistration, ConflictIsolationPolicyRegistration, ConflictPolicyRegistration,
    DeletionPolicyRegistration, FrozenAspectMergePolicyRegistry, FrozenConflictIsolationRegistry,
    FrozenConflictPolicyRegistry, FrozenDeletionPolicyRegistry, FrozenIdentityMatcherRegistry,
    FrozenMergeBaseStrategyRegistry, FrozenMergeStrategyRegistry, FrozenSourceOnlyPolicyRegistry,
    IdentityMatcherRegistration, MergeBaseStrategyRegistration, MergeStrategyRegistration,
    SourceOnlyPolicyRegistration,
};
use super::SignalRuntimeBuilder;

impl<CheckpointState, ComparatorState, D, I, E, Ctx, T>
    SignalRuntimeBuilder<CheckpointState, ComparatorState, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn runtime_policy(mut self, runtime_policy: SignalRuntimePolicy) -> Self {
        self.runtime_policy = runtime_policy;
        self
    }

    /// Bound the total heavy branch snapshot states retained by this runtime.
    pub fn maximum_stored_branch_snapshots(mut self, maximum: usize) -> Self {
        self.maximum_stored_branch_snapshots = maximum;
        self
    }

    pub fn adjust_runtime_policy<F>(mut self, adjust: F) -> Self
    where
        F: FnOnce(SignalRuntimePolicy) -> SignalRuntimePolicy,
    {
        self.runtime_policy = adjust(self.runtime_policy);
        self
    }

    pub fn development_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::development();
        self
    }

    pub fn operational_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::operational();
        self
    }

    pub fn forensic_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::forensic();
        self
    }

    pub fn web_development_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::web_development();
        self
    }

    pub fn fintech_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::fintech();
        self
    }

    pub fn kernel_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::kernel();
        self
    }

    pub fn game_engine_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::game_engine();
        self
    }

    pub fn tier_policy(mut self, policy: TierPolicy<T>) -> Self {
        self.tier_policies.push(policy);
        self
    }

    pub fn schema_registry(mut self, schema_registry: SignalSchemaRegistry) -> Self {
        self.schema_registry = schema_registry;
        self
    }

    pub fn merge_strategy_registry(
        mut self,
        merge_strategy_registry: FrozenMergeStrategyRegistry,
    ) -> Self {
        self.merge_strategy_registry = merge_strategy_registry;
        self
    }

    pub fn merge_strategies(
        mut self,
        registrations: Vec<MergeStrategyRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateMergeStrategyRegistration> {
        self.merge_strategy_registry =
            FrozenMergeStrategyRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    pub fn merge_base_strategy_registry(
        mut self,
        merge_base_strategy_registry: FrozenMergeBaseStrategyRegistry,
    ) -> Self {
        self.merge_base_strategy_registry = merge_base_strategy_registry;
        self
    }

    pub fn merge_base_strategies(
        mut self,
        registrations: Vec<MergeBaseStrategyRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateMergeBaseStrategyRegistration>
    {
        self.merge_base_strategy_registry =
            FrozenMergeBaseStrategyRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    pub fn conflict_policy_registry(
        mut self,
        conflict_policy_registry: FrozenConflictPolicyRegistry,
    ) -> Self {
        self.conflict_policy_registry = conflict_policy_registry;
        self
    }

    pub fn conflict_policies(
        mut self,
        registrations: Vec<ConflictPolicyRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateConflictPolicyRegistration> {
        self.conflict_policy_registry =
            FrozenConflictPolicyRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    pub fn aspect_merge_policy_registry(
        mut self,
        aspect_merge_policy_registry: FrozenAspectMergePolicyRegistry,
    ) -> Self {
        self.aspect_merge_policy_registry = aspect_merge_policy_registry;
        self
    }

    pub fn conflict_isolation_registry(
        mut self,
        conflict_isolation_registry: FrozenConflictIsolationRegistry,
    ) -> Self {
        self.conflict_isolation_registry = conflict_isolation_registry;
        self
    }

    pub fn aspect_merge_policies(
        mut self,
        registrations: Vec<AspectMergePolicyRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateAspectMergePolicyRegistration>
    {
        self.aspect_merge_policy_registry =
            FrozenAspectMergePolicyRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    pub fn conflict_isolation_policies(
        mut self,
        registrations: Vec<ConflictIsolationPolicyRegistration>,
    ) -> Result<
        Self,
        crate::logic::transaction::runtime::DuplicateConflictIsolationPolicyRegistration,
    > {
        self.conflict_isolation_registry =
            FrozenConflictIsolationRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    pub fn identity_matcher_registry(
        mut self,
        identity_matcher_registry: FrozenIdentityMatcherRegistry,
    ) -> Self {
        self.identity_matcher_registry = identity_matcher_registry;
        self
    }

    pub fn identity_matchers(
        mut self,
        registrations: Vec<IdentityMatcherRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateIdentityMatcherRegistration>
    {
        self.identity_matcher_registry =
            FrozenIdentityMatcherRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    pub fn source_only_policy_registry(
        mut self,
        source_only_policy_registry: FrozenSourceOnlyPolicyRegistry,
    ) -> Self {
        self.source_only_policy_registry = source_only_policy_registry;
        self
    }

    pub fn source_only_policies(
        mut self,
        registrations: Vec<SourceOnlyPolicyRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateSourceOnlyPolicyRegistration>
    {
        self.source_only_policy_registry =
            FrozenSourceOnlyPolicyRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    pub fn deletion_policy_registry(
        mut self,
        deletion_policy_registry: FrozenDeletionPolicyRegistry,
    ) -> Self {
        self.deletion_policy_registry = deletion_policy_registry;
        self
    }

    pub fn deletion_policies(
        mut self,
        registrations: Vec<DeletionPolicyRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateDeletionPolicyRegistration> {
        self.deletion_policy_registry =
            FrozenDeletionPolicyRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    pub fn resource_policy_registry(
        mut self,
        resource_policy_registry: FrozenResourcePolicyRegistry,
    ) -> Self {
        self.resource_policy_registry = resource_policy_registry;
        self
    }
}
