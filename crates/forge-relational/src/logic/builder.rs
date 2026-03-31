use crate::commit_strategies::data::{
    CommitStrategyExecutionRegistration, CommitStrategyRegistration,
};
use crate::config::data::{
    AdjacencyPolicy, CascadeDeletePolicy, CompiledLanePolicy, CrossContextPolicy, DurabilityPolicy,
    DurableLogPolicy, MvccConfig, PublicationConfig, RelationIntegrityScopeBudget,
    RelationalConfigOverride, RelationalRuntimeProfile, StorageLayoutConfig, VisibilityCachePolicy,
};
use crate::diagnostics::data::RelationalDiagnosticsProfile;
use crate::durability::data::{DurabilityMode, DurableStoreLayout};
use crate::logic::runtime::{RelationalRuntime, RelationalRuntimeConfig};
use crate::payloads::data::PayloadPolicy;
use crate::schema::data::RelationalSchemaRegistry;
use crate::symbols::data::SymbolPolicy;
use crate::validation::data::{CustomInvariantRegistration, InvariantCatalog};

#[derive(Debug, Clone)]
pub struct RelationalRuntimeBuilder {
    profile: RelationalRuntimeProfile,
    overrides: RelationalConfigOverride,
    custom_invariants: Vec<CustomInvariantRegistration>,
    commit_strategy_executors: Vec<CommitStrategyExecutionRegistration>,
}

impl Default for RelationalRuntimeBuilder {
    fn default() -> Self {
        Self {
            profile: RelationalRuntimeProfile::CertificationCore,
            overrides: RelationalConfigOverride::default(),
            custom_invariants: Vec::new(),
            commit_strategy_executors: Vec::new(),
        }
    }
}

impl RelationalRuntimeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn profile(mut self, profile: RelationalRuntimeProfile) -> Self {
        self.profile = profile;
        self
    }

    pub fn runtime_name(mut self, runtime_name: impl Into<String>) -> Self {
        self.overrides.execution.runtime_name = Some(runtime_name.into());
        self
    }

    pub fn execution_model(
        mut self,
        execution_model: crate::logic::planning::RelationalExecutionModel,
    ) -> Self {
        self.overrides.execution.execution_model = Some(execution_model);
        self
    }

    pub fn planning(mut self, planning: crate::logic::planning::PlanningContract) -> Self {
        self.overrides.execution.planning = Some(planning);
        self
    }

    pub fn commit_authority(
        mut self,
        commit_authority: crate::logic::commit::CommitAuthorityContract,
    ) -> Self {
        self.overrides.execution.commit_authority = Some(commit_authority);
        self
    }

    pub fn durability_mode(mut self, durability_mode: DurabilityMode) -> Self {
        self.overrides.durability.mode = Some(durability_mode);
        self
    }

    pub fn diagnostics(mut self, diagnostics: RelationalDiagnosticsProfile) -> Self {
        self.overrides.diagnostics.profile = Some(diagnostics);
        self
    }

    pub fn schema_registry(mut self, schema_registry: RelationalSchemaRegistry) -> Self {
        self.overrides.schema.registry = Some(schema_registry);
        self
    }

    pub fn invariant_catalog(mut self, invariant_catalog: InvariantCatalog) -> Self {
        self.overrides.schema.invariant_catalog = Some(invariant_catalog);
        self
    }

    pub fn custom_invariant(mut self, custom_invariant: CustomInvariantRegistration) -> Self {
        self.custom_invariants.push(custom_invariant);
        self
    }

    pub fn commit_strategy(mut self, commit_strategy: CommitStrategyRegistration) -> Self {
        self.overrides
            .commit_strategies
            .registrations
            .get_or_insert_with(Vec::new)
            .push(commit_strategy);
        self
    }

    pub fn commit_strategy_executor(
        mut self,
        commit_strategy_executor: CommitStrategyExecutionRegistration,
    ) -> Self {
        self.commit_strategy_executors
            .push(commit_strategy_executor);
        self
    }

    pub fn entity_capacity(mut self, capacity: usize) -> Self {
        self.overrides.storage.initial_entity_capacity = Some(capacity);
        self
    }

    pub fn relation_capacity(mut self, capacity: usize) -> Self {
        self.overrides.storage.initial_relation_capacity = Some(capacity);
        self
    }

    pub fn mvcc(mut self, mvcc: MvccConfig) -> Self {
        self.overrides.storage.mvcc = Some(mvcc);
        self
    }

    pub fn storage_layout(mut self, storage_layout: StorageLayoutConfig) -> Self {
        self.overrides.storage.layout = Some(storage_layout);
        self
    }

    pub fn publication(mut self, publication: PublicationConfig) -> Self {
        self.overrides.publication.policy = Some(publication);
        self
    }

    pub fn payload_policy(mut self, payload_policy: PayloadPolicy) -> Self {
        self.overrides.storage.payload_policy = Some(payload_policy);
        self
    }

    pub fn symbol_policy(mut self, symbol_policy: SymbolPolicy) -> Self {
        self.overrides.identity.symbol_policy = Some(symbol_policy);
        self
    }

    pub fn visibility_cache_policy(
        mut self,
        visibility_cache_policy: VisibilityCachePolicy,
    ) -> Self {
        self.overrides.visibility.cache_policy = Some(visibility_cache_policy);
        self
    }

    pub fn durable_log_policy(mut self, durable_log_policy: DurableLogPolicy) -> Self {
        self.overrides.durability.log = Some(durable_log_policy);
        self
    }

    pub fn durability_policy(mut self, durability_policy: DurabilityPolicy) -> Self {
        self.overrides.durability.policy = Some(durability_policy);
        self
    }

    pub fn durable_store_layout(mut self, durable_store_layout: DurableStoreLayout) -> Self {
        self.overrides.durability.store_layout = Some(durable_store_layout);
        self
    }

    pub fn adjacency_policy(mut self, adjacency_policy: AdjacencyPolicy) -> Self {
        self.overrides.storage.adjacency_policy = Some(adjacency_policy);
        self
    }

    pub fn cross_context_policy(mut self, cross_context_policy: CrossContextPolicy) -> Self {
        self.overrides.storage.cross_context_policy = Some(cross_context_policy);
        self
    }

    pub fn cascade_delete_policy(mut self, cascade_delete_policy: CascadeDeletePolicy) -> Self {
        self.overrides.storage.cascade_delete_policy = Some(cascade_delete_policy);
        self
    }

    pub fn compiled_lane_policy(mut self, compiled_lane_policy: CompiledLanePolicy) -> Self {
        self.overrides.execution.compiled_lane_policy = Some(compiled_lane_policy);
        self
    }

    pub fn relation_integrity_scope_budget(
        mut self,
        relation_integrity_scope_budget: RelationIntegrityScopeBudget,
    ) -> Self {
        self.overrides.execution.relation_integrity_scope_budget =
            Some(relation_integrity_scope_budget);
        self
    }

    pub fn build(self) -> RelationalRuntime {
        RelationalRuntime::new_with_extensions(
            RelationalRuntimeConfig::resolved(self.profile, self.overrides),
            self.custom_invariants,
            self.commit_strategy_executors,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::RelationalRuntimeBuilder;
    use crate::commit_strategies::data::{
        CommitStrategyDescriptor, CommitStrategyFamilyName, CommitStrategyId,
        CommitStrategyRegistration, CommitStrategySemanticName, CommitStrategyVersion,
        PersistentArtifactName, StrategyInputSchemaName, StrategyInputSchemaVersion,
        StrategyIntentName, StrategyOutputSchemaName, StrategyPacketContract, StrategyReadContract,
        StrategyReadCostClass, StrategyReadLocalityClass, StrategyReadScopeClass,
        StrategyRequestCanonicalization, StrategyTraversalBasis,
    };
    use crate::validation::data::{
        CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
        CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
        CustomInvariantRegistration, CustomInvariantRule, CustomInvariantRuleId,
        CustomInvariantScopePlanner, CustomInvariantSemanticIdentity,
        CustomInvariantSemanticVersion, CustomInvariantVerdict, InvariantCostClass,
        InvariantExecutionPoint, InvariantFailureEffect, InvariantGroup, InvariantGroupSet,
    };

    struct BuilderTestRule;

    impl CustomInvariantRule for BuilderTestRule {
        type Scope = ();

        fn descriptor(&self) -> CustomInvariantDescriptor {
            CustomInvariantDescriptor {
                identity: CustomInvariantSemanticIdentity {
                    rule_id: CustomInvariantRuleId::new("builder.test.rule"),
                    semantic_version: CustomInvariantSemanticVersion::new(1, 0),
                },
                display_name: Arc::from("Builder Test Rule"),
                operational: CustomInvariantOperationalMetadata {
                    execution_point: InvariantExecutionPoint::CommitBoundary,
                    groups: InvariantGroupSet::of(InvariantGroup::SchemaCompliance),
                    cost_class: InvariantCostClass::Touched,
                    failure_effect: InvariantFailureEffect::BlockCommit,
                },
            }
        }

        fn prepare_scope(
            &self,
            _planner: &mut CustomInvariantScopePlanner<'_>,
        ) -> Result<Self::Scope, CustomInvariantPreparationError> {
            Ok(())
        }

        fn evaluate(
            &self,
            _context: &CustomInvariantExecutionContext<'_>,
            _scope: &Self::Scope,
        ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError> {
            Ok(CustomInvariantVerdict::Pass)
        }
    }

    #[test]
    fn builder_attaches_custom_invariants_without_polluting_config() {
        let registration = CustomInvariantRegistration::new(BuilderTestRule).unwrap();
        let runtime = RelationalRuntimeBuilder::new()
            .custom_invariant(registration)
            .build();

        assert_eq!(
            runtime.config.schema.invariant_catalog.registrations.len(),
            2
        );
        assert_eq!(
            runtime.aspect_semantics.custom_invariant_registries.len(),
            1
        );
        assert_eq!(
            runtime
                .aspect_semantics
                .custom_invariant_registries
                .iter()
                .next()
                .unwrap()
                .rule_id()
                .as_str(),
            "builder.test.rule"
        );
    }

    #[test]
    fn builder_attaches_frozen_commit_strategy_registry() {
        let registration = CommitStrategyRegistration::new(CommitStrategyDescriptor::new(
            CommitStrategyId(41),
            CommitStrategySemanticName::new("strategy.intent.reconcile"),
            CommitStrategyFamilyName::new("strategy.intent"),
            CommitStrategyVersion::new(1, 0),
            StrategyIntentName::new("reconcile.desired.state"),
            StrategyInputSchemaName::new("intent.reconcile.input.v1"),
            StrategyInputSchemaVersion(1),
            StrategyOutputSchemaName::new("intent.reconcile.output.v1"),
            StrategyRequestCanonicalization::JsonStableObjectOrderV1,
            StrategyReadContract {
                scope_class: StrategyReadScopeClass::ExplicitTargetsOnly,
                locality_class: StrategyReadLocalityClass::SinglePartition,
                traversal_basis: StrategyTraversalBasis::NoTraversal,
                packet_contract: StrategyPacketContract::ProjectionOnly,
                cost_class: StrategyReadCostClass::ORequestedSurface,
            },
            PersistentArtifactName::new("strategy.intent.reconcile"),
        ))
        .expect("valid strategy registration");
        let runtime = RelationalRuntimeBuilder::new()
            .commit_strategy(registration.clone())
            .build();

        assert_eq!(runtime.commit_strategy_registry().len(), 1);
        assert_eq!(
            runtime
                .commit_strategy_registry()
                .iter()
                .next()
                .expect("registered strategy")
                .descriptor()
                .semantic_name()
                .as_str(),
            "strategy.intent.reconcile"
        );
        assert!(!runtime
            .commit_strategy_registry()
            .registry_digest()
            .is_empty());
        assert_eq!(
            runtime
                .commit_strategy_registry()
                .iter()
                .next()
                .expect("registered strategy")
                .descriptor()
                .digest(),
            registration.descriptor().digest()
        );
        assert_eq!(
            runtime
                .config()
                .commit_strategies
                .registrations
                .first()
                .expect("config registration")
                .descriptor()
                .semantic_name()
                .as_str(),
            "strategy.intent.reconcile"
        );
        assert_eq!(
            runtime
                .config()
                .provenance
                .source_for("commit_strategies.registrations")
                .expect("strategy provenance")
                .source,
            crate::config::data::ConfigValueSource::BuilderOverride
        );
        assert!(runtime
            .config()
            .provenance
            .source_for("commit_strategies.registrations")
            .expect("strategy provenance")
            .detail
            .contains("descriptor_set_digest="));
    }

    #[test]
    fn runtime_commit_strategy_facade_canonicalizes_requests_against_frozen_registry() {
        let registration = CommitStrategyRegistration::new(CommitStrategyDescriptor::new(
            CommitStrategyId(41),
            CommitStrategySemanticName::new("strategy.intent.reconcile"),
            CommitStrategyFamilyName::new("strategy.intent"),
            CommitStrategyVersion::new(1, 0),
            StrategyIntentName::new("reconcile.desired.state"),
            StrategyInputSchemaName::new("intent.reconcile.input.v1"),
            StrategyInputSchemaVersion(1),
            StrategyOutputSchemaName::new("intent.reconcile.output.v1"),
            StrategyRequestCanonicalization::JsonStableObjectOrderV1,
            StrategyReadContract {
                scope_class: StrategyReadScopeClass::ExplicitTargetsOnly,
                locality_class: StrategyReadLocalityClass::SinglePartition,
                traversal_basis: StrategyTraversalBasis::NoTraversal,
                packet_contract: StrategyPacketContract::ProjectionOnly,
                cost_class: StrategyReadCostClass::ORequestedSurface,
            },
            PersistentArtifactName::new("strategy.intent.reconcile"),
        ))
        .expect("valid strategy registration");
        let runtime = RelationalRuntimeBuilder::new()
            .commit_strategy(registration)
            .build();

        let request = crate::facade::commit_strategies::RawStrategyCommitRequest::new(
            CommitStrategySemanticName::new("strategy.intent.reconcile"),
            br#"{"z":3,"a":1}"#.to_vec(),
            crate::facade::commit_strategies::StrategyCallerProvenance {
                request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Api,
                actor_identity: Some("user-1".to_string()),
                correlation_id: Some("corr-1".to_string()),
            },
        );

        let canonical = runtime
            .commit_strategies()
            .canonicalize_request(&request)
            .expect("canonical request");

        assert_eq!(canonical.strategy_id(), CommitStrategyId(41));
        assert_eq!(
            canonical.canonical_input().canonical_bytes(),
            br#"{"a":1,"z":3}"#
        );
        assert_eq!(
            canonical.canonical_input().canonicalization(),
            StrategyRequestCanonicalization::JsonStableObjectOrderV1
        );
    }
}
