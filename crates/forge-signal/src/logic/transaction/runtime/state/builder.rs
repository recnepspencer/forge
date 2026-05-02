use std::marker::PhantomData;

use crate::data::checkpoint::CheckpointBarrier;
use crate::data::checkpoint_policy::CheckpointPolicy;
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::graph::SignalGraph;
use crate::data::resource::FrozenResourcePolicyRegistry;
use crate::data::tier::TierPolicy;
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::events::EventBus;
use crate::schema::data::SignalSchemaRegistry;

use super::merge::{
    AspectMergePolicyRegistration, ConflictIsolationPolicyRegistration, ConflictPolicyRegistration,
    DeletionPolicyRegistration, FrozenAspectMergePolicyRegistry, FrozenConflictIsolationRegistry,
    FrozenConflictPolicyRegistry, FrozenDeletionPolicyRegistry, FrozenIdentityMatcherRegistry,
    FrozenMergeBaseStrategyRegistry, FrozenMergeStrategyRegistry, FrozenSourceOnlyPolicyRegistry,
    IdentityMatcherRegistration, MergeBaseStrategyRegistration, MergeStrategyRegistration,
    SourceOnlyPolicyRegistration,
};
use super::runtime_state::SignalRuntime;

pub struct Missing;
pub struct Present;

/// Builder for `SignalRuntime`.
///
/// Start here if you want the full runtime surface with transactions,
/// checkpoint control, runtime policy, keyed nodes, and diagnostics.
pub struct SignalRuntimeBuilder<
    CheckpointState = Missing,
    ComparatorState = Missing,
    D = (),
    I = (),
    E = (),
    Ctx = (),
    T = (),
> where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    graph: SignalGraph,
    schema_registry: SignalSchemaRegistry,
    merge_strategy_registry: FrozenMergeStrategyRegistry,
    merge_base_strategy_registry: FrozenMergeBaseStrategyRegistry,
    aspect_merge_policy_registry: FrozenAspectMergePolicyRegistry,
    conflict_isolation_registry: FrozenConflictIsolationRegistry,
    conflict_policy_registry: FrozenConflictPolicyRegistry,
    identity_matcher_registry: FrozenIdentityMatcherRegistry,
    source_only_policy_registry: FrozenSourceOnlyPolicyRegistry,
    deletion_policy_registry: FrozenDeletionPolicyRegistry,
    resource_policy_registry: FrozenResourcePolicyRegistry,
    checkpoint_policy: CheckpointPolicy<D>,
    fallback_comparator: VersionComparatorPolicy,
    runtime_policy: SignalRuntimePolicy,
    tier_policies: Vec<TierPolicy<T>>,
    _marker: PhantomData<fn(CheckpointState, ComparatorState, I, E, Ctx, T)>,
}

impl<CheckpointState, ComparatorState, D, I, E, Ctx, T>
    SignalRuntimeBuilder<CheckpointState, ComparatorState, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn new(graph: SignalGraph) -> Self {
        Self {
            graph,
            schema_registry: SignalSchemaRegistry::default(),
            merge_strategy_registry: FrozenMergeStrategyRegistry::built_in(),
            merge_base_strategy_registry: FrozenMergeBaseStrategyRegistry::built_in(),
            aspect_merge_policy_registry: FrozenAspectMergePolicyRegistry::built_in(),
            conflict_isolation_registry: FrozenConflictIsolationRegistry::built_in(),
            conflict_policy_registry: FrozenConflictPolicyRegistry::built_in(),
            identity_matcher_registry: FrozenIdentityMatcherRegistry::built_in(),
            source_only_policy_registry: FrozenSourceOnlyPolicyRegistry::built_in(),
            deletion_policy_registry: FrozenDeletionPolicyRegistry::built_in(),
            resource_policy_registry: FrozenResourcePolicyRegistry::built_in(),
            checkpoint_policy: CheckpointPolicy::new(CheckpointBarrier::PerOperation),
            fallback_comparator: VersionComparatorPolicy::Exact,
            runtime_policy: SignalRuntimePolicy::default(),
            tier_policies: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Set a simple checkpoint barrier policy.
    ///
    /// This is the shortest path when you want standard checkpoint behavior
    /// without constructing a full `CheckpointPolicy`.
    pub fn checkpoint_barrier(
        self,
        barrier: CheckpointBarrier,
    ) -> SignalRuntimeBuilder<Present, ComparatorState, D, I, E, Ctx, T> {
        SignalRuntimeBuilder {
            graph: self.graph,
            schema_registry: self.schema_registry,
            merge_strategy_registry: self.merge_strategy_registry,
            merge_base_strategy_registry: self.merge_base_strategy_registry,
            aspect_merge_policy_registry: self.aspect_merge_policy_registry,
            conflict_isolation_registry: self.conflict_isolation_registry,
            conflict_policy_registry: self.conflict_policy_registry,
            identity_matcher_registry: self.identity_matcher_registry,
            source_only_policy_registry: self.source_only_policy_registry,
            deletion_policy_registry: self.deletion_policy_registry,
            resource_policy_registry: self.resource_policy_registry,
            checkpoint_policy: CheckpointPolicy::new(barrier),
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: self.tier_policies,
            _marker: PhantomData,
        }
    }

    /// Set the full checkpoint policy.
    pub fn checkpoint_policy(
        self,
        policy: CheckpointPolicy<D>,
    ) -> SignalRuntimeBuilder<Present, ComparatorState, D, I, E, Ctx, T> {
        SignalRuntimeBuilder {
            graph: self.graph,
            schema_registry: self.schema_registry,
            merge_strategy_registry: self.merge_strategy_registry,
            merge_base_strategy_registry: self.merge_base_strategy_registry,
            aspect_merge_policy_registry: self.aspect_merge_policy_registry,
            conflict_isolation_registry: self.conflict_isolation_registry,
            conflict_policy_registry: self.conflict_policy_registry,
            identity_matcher_registry: self.identity_matcher_registry,
            source_only_policy_registry: self.source_only_policy_registry,
            deletion_policy_registry: self.deletion_policy_registry,
            resource_policy_registry: self.resource_policy_registry,
            checkpoint_policy: policy,
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: self.tier_policies,
            _marker: PhantomData,
        }
    }

    /// Adjust the current checkpoint setup in one place.
    pub fn adjust_checkpoints<F>(
        mut self,
        adjust: F,
    ) -> SignalRuntimeBuilder<Present, ComparatorState, D, I, E, Ctx, T>
    where
        F: FnOnce(&mut CheckpointPolicy<D>),
    {
        adjust(&mut self.checkpoint_policy);
        SignalRuntimeBuilder {
            graph: self.graph,
            schema_registry: self.schema_registry,
            merge_strategy_registry: self.merge_strategy_registry,
            merge_base_strategy_registry: self.merge_base_strategy_registry,
            aspect_merge_policy_registry: self.aspect_merge_policy_registry,
            conflict_isolation_registry: self.conflict_isolation_registry,
            conflict_policy_registry: self.conflict_policy_registry,
            identity_matcher_registry: self.identity_matcher_registry,
            source_only_policy_registry: self.source_only_policy_registry,
            deletion_policy_registry: self.deletion_policy_registry,
            resource_policy_registry: self.resource_policy_registry,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: self.tier_policies,
            _marker: PhantomData,
        }
    }

    /// Set the fallback comparator used when a node or tier does not provide one.
    pub fn fallback_comparator(
        self,
        comparator: VersionComparatorPolicy,
    ) -> SignalRuntimeBuilder<CheckpointState, Present, D, I, E, Ctx, T> {
        SignalRuntimeBuilder {
            graph: self.graph,
            schema_registry: self.schema_registry,
            merge_strategy_registry: self.merge_strategy_registry,
            merge_base_strategy_registry: self.merge_base_strategy_registry,
            aspect_merge_policy_registry: self.aspect_merge_policy_registry,
            conflict_isolation_registry: self.conflict_isolation_registry,
            conflict_policy_registry: self.conflict_policy_registry,
            identity_matcher_registry: self.identity_matcher_registry,
            source_only_policy_registry: self.source_only_policy_registry,
            deletion_policy_registry: self.deletion_policy_registry,
            resource_policy_registry: self.resource_policy_registry,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: self.tier_policies,
            _marker: PhantomData,
        }
    }

    /// Adjust the fallback comparator without restating the whole value.
    pub fn adjust_fallback_comparator<F>(
        mut self,
        adjust: F,
    ) -> SignalRuntimeBuilder<CheckpointState, Present, D, I, E, Ctx, T>
    where
        F: FnOnce(VersionComparatorPolicy) -> VersionComparatorPolicy,
    {
        self.fallback_comparator = adjust(self.fallback_comparator);
        SignalRuntimeBuilder {
            graph: self.graph,
            schema_registry: self.schema_registry,
            merge_strategy_registry: self.merge_strategy_registry,
            merge_base_strategy_registry: self.merge_base_strategy_registry,
            aspect_merge_policy_registry: self.aspect_merge_policy_registry,
            conflict_isolation_registry: self.conflict_isolation_registry,
            conflict_policy_registry: self.conflict_policy_registry,
            identity_matcher_registry: self.identity_matcher_registry,
            source_only_policy_registry: self.source_only_policy_registry,
            deletion_policy_registry: self.deletion_policy_registry,
            resource_policy_registry: self.resource_policy_registry,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: self.tier_policies,
            _marker: PhantomData,
        }
    }

    /// Set runtime observability and semantic retention policy.
    ///
    /// Use one of the named presets like `SignalRuntimePolicy::operational()`
    /// or `SignalRuntimePolicy::fintech()` unless you need a custom mix.
    pub fn runtime_policy(mut self, runtime_policy: SignalRuntimePolicy) -> Self {
        self.runtime_policy = runtime_policy;
        self
    }

    /// Adjust the current runtime policy without rebuilding it from scratch.
    pub fn adjust_runtime_policy<F>(mut self, adjust: F) -> Self
    where
        F: FnOnce(SignalRuntimePolicy) -> SignalRuntimePolicy,
    {
        self.runtime_policy = adjust(self.runtime_policy);
        self
    }

    /// Use the normal development posture for this builder.
    pub fn development_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::development();
        self
    }

    /// Use the lean operational posture for this builder.
    pub fn operational_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::operational();
        self
    }

    /// Use the heavier forensic posture for this builder.
    pub fn forensic_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::forensic();
        self
    }

    /// Use the web-development preset for this builder.
    pub fn web_development_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::web_development();
        self
    }

    /// Use the fintech preset for this builder.
    pub fn fintech_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::fintech();
        self
    }

    /// Use the kernel-oriented forensic preset for this builder.
    pub fn kernel_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::kernel();
        self
    }

    /// Use the game-engine preset for this builder.
    pub fn game_engine_policy(mut self) -> Self {
        self.runtime_policy = SignalRuntimePolicy::game_engine();
        self
    }

    /// Seed an initial tier policy into runtime config before build completes.
    pub fn tier_policy(mut self, policy: TierPolicy<T>) -> Self {
        self.tier_policies.push(policy);
        self
    }

    /// Bind a first-class schema registry to the runtime being built.
    pub fn schema_registry(mut self, schema_registry: SignalSchemaRegistry) -> Self {
        self.schema_registry = schema_registry;
        self
    }

    /// Replace the frozen merge strategy registry used to resolve merge plans.
    pub fn merge_strategy_registry(
        mut self,
        merge_strategy_registry: FrozenMergeStrategyRegistry,
    ) -> Self {
        self.merge_strategy_registry = merge_strategy_registry;
        self
    }

    /// Register merge strategies by freezing a new registry from explicit registrations.
    pub fn merge_strategies(
        mut self,
        registrations: Vec<MergeStrategyRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateMergeStrategyRegistration> {
        self.merge_strategy_registry =
            FrozenMergeStrategyRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    /// Replace the frozen merge-base strategy registry used during merge planning.
    pub fn merge_base_strategy_registry(
        mut self,
        merge_base_strategy_registry: FrozenMergeBaseStrategyRegistry,
    ) -> Self {
        self.merge_base_strategy_registry = merge_base_strategy_registry;
        self
    }

    /// Register merge-base strategies by freezing a new registry from explicit registrations.
    pub fn merge_base_strategies(
        mut self,
        registrations: Vec<MergeBaseStrategyRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateMergeBaseStrategyRegistration>
    {
        self.merge_base_strategy_registry =
            FrozenMergeBaseStrategyRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    /// Replace the frozen conflict policy registry used during merge planning.
    pub fn conflict_policy_registry(
        mut self,
        conflict_policy_registry: FrozenConflictPolicyRegistry,
    ) -> Self {
        self.conflict_policy_registry = conflict_policy_registry;
        self
    }

    /// Register conflict policies by freezing a new registry from explicit registrations.
    pub fn conflict_policies(
        mut self,
        registrations: Vec<ConflictPolicyRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateConflictPolicyRegistration> {
        self.conflict_policy_registry =
            FrozenConflictPolicyRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    /// Replace the frozen per-aspect merge policy registry used during merge planning.
    pub fn aspect_merge_policy_registry(
        mut self,
        aspect_merge_policy_registry: FrozenAspectMergePolicyRegistry,
    ) -> Self {
        self.aspect_merge_policy_registry = aspect_merge_policy_registry;
        self
    }

    /// Replace the frozen conflict isolation registry used during merge planning.
    pub fn conflict_isolation_registry(
        mut self,
        conflict_isolation_registry: FrozenConflictIsolationRegistry,
    ) -> Self {
        self.conflict_isolation_registry = conflict_isolation_registry;
        self
    }

    /// Register per-aspect merge policies by freezing a new registry from explicit registrations.
    pub fn aspect_merge_policies(
        mut self,
        registrations: Vec<AspectMergePolicyRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateAspectMergePolicyRegistration>
    {
        self.aspect_merge_policy_registry =
            FrozenAspectMergePolicyRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    /// Register conflict isolation policies by freezing a new registry from explicit registrations.
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

    /// Replace the frozen identity matcher registry used to resolve merge plans.
    pub fn identity_matcher_registry(
        mut self,
        identity_matcher_registry: FrozenIdentityMatcherRegistry,
    ) -> Self {
        self.identity_matcher_registry = identity_matcher_registry;
        self
    }

    /// Register identity matchers by freezing a new registry from explicit registrations.
    pub fn identity_matchers(
        mut self,
        registrations: Vec<IdentityMatcherRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateIdentityMatcherRegistration>
    {
        self.identity_matcher_registry =
            FrozenIdentityMatcherRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    /// Replace the frozen source-only policy registry used during merge planning.
    pub fn source_only_policy_registry(
        mut self,
        source_only_policy_registry: FrozenSourceOnlyPolicyRegistry,
    ) -> Self {
        self.source_only_policy_registry = source_only_policy_registry;
        self
    }

    /// Register source-only policies by freezing a new registry from explicit registrations.
    pub fn source_only_policies(
        mut self,
        registrations: Vec<SourceOnlyPolicyRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateSourceOnlyPolicyRegistration>
    {
        self.source_only_policy_registry =
            FrozenSourceOnlyPolicyRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    /// Replace the frozen deletion policy registry used during merge planning.
    pub fn deletion_policy_registry(
        mut self,
        deletion_policy_registry: FrozenDeletionPolicyRegistry,
    ) -> Self {
        self.deletion_policy_registry = deletion_policy_registry;
        self
    }

    /// Register deletion policies by freezing a new registry from explicit registrations.
    pub fn deletion_policies(
        mut self,
        registrations: Vec<DeletionPolicyRegistration>,
    ) -> Result<Self, crate::logic::transaction::runtime::DuplicateDeletionPolicyRegistration> {
        self.deletion_policy_registry =
            FrozenDeletionPolicyRegistry::from_registrations(registrations)?;
        Ok(self)
    }

    /// Replace the frozen resource policy registry used for resource descriptor lowering.
    pub fn resource_policy_registry(
        mut self,
        resource_policy_registry: FrozenResourcePolicyRegistry,
    ) -> Self {
        self.resource_policy_registry = resource_policy_registry;
        self
    }

    pub fn with_kernel_defaults(self) -> SignalRuntimeBuilder<Present, Present, D, I, E, Ctx, T> {
        self.checkpoint_barrier(CheckpointBarrier::PerOperation)
            .fallback_comparator(VersionComparatorPolicy::Exact)
    }

    /// Switch the runtime to a typed event payload.
    ///
    /// This is usually only needed once you start integrating an event bus.
    pub fn with_events<E2>(
        self,
    ) -> SignalRuntimeBuilder<CheckpointState, ComparatorState, D, I, E2, Ctx, T> {
        SignalRuntimeBuilder {
            graph: self.graph,
            schema_registry: self.schema_registry,
            merge_strategy_registry: self.merge_strategy_registry,
            merge_base_strategy_registry: self.merge_base_strategy_registry,
            aspect_merge_policy_registry: self.aspect_merge_policy_registry,
            conflict_isolation_registry: self.conflict_isolation_registry,
            conflict_policy_registry: self.conflict_policy_registry,
            identity_matcher_registry: self.identity_matcher_registry,
            source_only_policy_registry: self.source_only_policy_registry,
            deletion_policy_registry: self.deletion_policy_registry,
            resource_policy_registry: self.resource_policy_registry,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: self.tier_policies,
            _marker: PhantomData,
        }
    }

    /// Switch the runtime to a typed checkpoint domain key.
    pub fn with_domains<D2>(
        self,
    ) -> SignalRuntimeBuilder<CheckpointState, ComparatorState, D2, I, E, Ctx, T>
    where
        D2: Copy + Ord + std::fmt::Debug + 'static,
    {
        SignalRuntimeBuilder {
            graph: self.graph,
            schema_registry: self.schema_registry,
            merge_strategy_registry: self.merge_strategy_registry,
            merge_base_strategy_registry: self.merge_base_strategy_registry,
            aspect_merge_policy_registry: self.aspect_merge_policy_registry,
            conflict_isolation_registry: self.conflict_isolation_registry,
            conflict_policy_registry: self.conflict_policy_registry,
            identity_matcher_registry: self.identity_matcher_registry,
            source_only_policy_registry: self.source_only_policy_registry,
            deletion_policy_registry: self.deletion_policy_registry,
            resource_policy_registry: self.resource_policy_registry,
            checkpoint_policy: CheckpointPolicy::new(self.checkpoint_policy.barrier_for_default()),
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Switch the runtime to a typed checkpoint impact key.
    pub fn with_impacts<I2>(
        self,
    ) -> SignalRuntimeBuilder<CheckpointState, ComparatorState, D, I2, E, Ctx, T>
    where
        I2: Copy + Ord,
    {
        SignalRuntimeBuilder {
            graph: self.graph,
            schema_registry: self.schema_registry,
            merge_strategy_registry: self.merge_strategy_registry,
            merge_base_strategy_registry: self.merge_base_strategy_registry,
            aspect_merge_policy_registry: self.aspect_merge_policy_registry,
            conflict_isolation_registry: self.conflict_isolation_registry,
            conflict_policy_registry: self.conflict_policy_registry,
            identity_matcher_registry: self.identity_matcher_registry,
            source_only_policy_registry: self.source_only_policy_registry,
            deletion_policy_registry: self.deletion_policy_registry,
            resource_policy_registry: self.resource_policy_registry,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: self.tier_policies,
            _marker: PhantomData,
        }
    }

    /// Enable typed node tiers for tier policy configuration.
    pub fn with_tiers<T2>(
        self,
    ) -> SignalRuntimeBuilder<CheckpointState, ComparatorState, D, I, E, Ctx, T2>
    where
        T2: Copy + Ord,
    {
        SignalRuntimeBuilder {
            graph: self.graph,
            schema_registry: self.schema_registry,
            merge_strategy_registry: self.merge_strategy_registry,
            merge_base_strategy_registry: self.merge_base_strategy_registry,
            aspect_merge_policy_registry: self.aspect_merge_policy_registry,
            conflict_isolation_registry: self.conflict_isolation_registry,
            conflict_policy_registry: self.conflict_policy_registry,
            identity_matcher_registry: self.identity_matcher_registry,
            source_only_policy_registry: self.source_only_policy_registry,
            deletion_policy_registry: self.deletion_policy_registry,
            resource_policy_registry: self.resource_policy_registry,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Switch the runtime to a typed external transaction/event context.
    pub fn with_context<Ctx2>(
        self,
    ) -> SignalRuntimeBuilder<CheckpointState, ComparatorState, D, I, E, Ctx2, T> {
        SignalRuntimeBuilder {
            graph: self.graph,
            schema_registry: self.schema_registry,
            merge_strategy_registry: self.merge_strategy_registry,
            merge_base_strategy_registry: self.merge_base_strategy_registry,
            aspect_merge_policy_registry: self.aspect_merge_policy_registry,
            conflict_isolation_registry: self.conflict_isolation_registry,
            conflict_policy_registry: self.conflict_policy_registry,
            identity_matcher_registry: self.identity_matcher_registry,
            source_only_policy_registry: self.source_only_policy_registry,
            deletion_policy_registry: self.deletion_policy_registry,
            resource_policy_registry: self.resource_policy_registry,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: self.tier_policies,
            _marker: PhantomData,
        }
    }
}

impl<D, I, E, Ctx, T> SignalRuntimeBuilder<Present, Present, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    /// Build the runtime and validate that any graph-bound schema identities
    /// still match the effective frozen registry.
    pub fn build_validated(
        self,
    ) -> Result<SignalRuntime<D, I, E, Ctx, T>, crate::data::error::SignalError> {
        let checkpoint = CheckpointRuntime::new(self.checkpoint_policy);
        let event_bus = EventBus::new();
        let mut runtime =
            SignalRuntime::new(self.graph, self.schema_registry, checkpoint, event_bus);
        runtime.merge_strategy_registry = self.merge_strategy_registry;
        runtime.merge_base_strategy_registry = self.merge_base_strategy_registry;
        runtime.aspect_merge_policy_registry = self.aspect_merge_policy_registry;
        runtime.conflict_isolation_registry = self.conflict_isolation_registry;
        runtime.conflict_policy_registry = self.conflict_policy_registry;
        runtime.identity_matcher_registry = self.identity_matcher_registry;
        runtime.source_only_policy_registry = self.source_only_policy_registry;
        runtime.deletion_policy_registry = self.deletion_policy_registry;
        runtime
            .resource
            .set_policy_registry(self.resource_policy_registry);
        runtime.set_fallback_comparator(self.fallback_comparator);
        runtime.set_runtime_policy(self.runtime_policy);
        for policy in self.tier_policies {
            runtime.set_tier_policy(policy);
        }
        runtime.validate_schema_bindings()?;
        runtime.validate_merge_semantics()?;
        Ok(runtime)
    }

    /// Build the runtime.
    pub fn build(self) -> SignalRuntime<D, I, E, Ctx, T> {
        self.build_validated()
            .expect(
                "signal runtime schema bindings and merge semantics must match the effective frozen registries",
            )
    }
}
