//! Public API boundary for `forge-signal`.
//! External components should import through this module.
//!
//! Contract:
//! - `forge-signal` owns evaluation DAG scheduling.
//! - Host crates own external structural or state graphs, including cyclic ones.
//! - Compute closures may consume opaque host snapshots directly.

// Re-export Data constructs
pub use crate::data::aspect::{Aspect, AspectMask, AspectVersion, MAX_ASPECTS};
pub use crate::data::checkpoint::CheckpointBarrier;
pub use crate::data::checkpoint_policy::CheckpointPolicy;
pub use crate::data::bitset::{BitsetFrontier, DenseBitset};
pub use crate::data::comparator::{
    ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
    TierPolicyResolver, VersionComparatorPolicy, VersionComparatorResolver,
};
pub use crate::data::dependency::DependencyEdge;
pub use crate::data::dirty_set::{BatchedDirtySet, DomainImpact};
pub use crate::data::error::SignalError;
pub use crate::data::effect_mapping::EffectMapping;
pub use crate::data::evaluator::CheckpointEvaluator;
pub use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
pub use crate::data::graph::{NodeBuilder, SignalGraph};
pub use crate::data::handle::NodeId;
pub use crate::data::node_meta::NodeMetaStore;
pub use crate::data::node::{EvaluationCondition, NodeEntry, NodeEvaluationConfig, NodeState};
pub use crate::data::output::{
    ChangedRegion, ComputationFamily, ComputationKey, KeyedComputation, MemoizedResultOrigin,
    NodeEvaluationResult, OutputChange, OutputIdentity, PartitionMatchMode,
    PartitionSubscription, PartitionToken, StructuralMemoKey,
};
pub use crate::data::subscriber_context::{SubscriberContext, SubscriberContextError};
pub use crate::data::telemetry::RuntimeTelemetry;
pub use crate::data::tier::{DependencyMode, DirtyPropagation, EvaluationTrigger, TierPolicy};
pub use crate::data::tier_policy_table::TierPolicyTable;
pub use crate::data::trace::{CausalityMetadata, TraceSummary};

// Re-export Logic constructs
pub use crate::logic::checkpoint::CheckpointRuntime;
pub use crate::logic::context::EvaluationContext;
pub use crate::logic::evaluation::{
    apply_evaluation_result_with_policy_and_condition, evaluate, evaluate_on_demand,
    evaluate_with_policy_and_condition_resolvers,
    evaluate_with_policy_and_condition_resolvers_and_metadata, evaluate_with_policy_resolver,
    evaluate_with_resolver, evaluate_with_resolvers, ConditionEvaluationContext,
    ConditionResolver, DefaultConditionResolver, EvaluationExecutionMetadata,
    EvaluationRequestMode,
};
pub use crate::logic::events::{EventBus, EventFlushError, SubscriberRegistryError};
pub use crate::logic::explain::{
    ConditionDecision, MeaningfulChangeReason, NodeExplanation, UpstreamCause,
};
pub use crate::logic::invalidation::{mark_dirty, mark_dirty_with_regions};
pub use crate::logic::transaction::{
    emit_event_in_txn, evaluate_in_txn, evaluate_in_txn_with_mode, flush_checkpoint_in_txn,
    SignalRuntime, SignalRuntimeBuilder, SignalRuntimeConfig, SignalTransaction,
    TransactionOutcome,
};
pub use crate::presentation::metrics::{GraphMetrics, RuntimeMetrics};
pub use crate::presentation::contracts::{
    DependencyGraphContract, RawPathComputeContract, StructuralStateBoundaryContract,
};
pub use crate::presentation::transaction_contract::TransactionRuntimeContract;
