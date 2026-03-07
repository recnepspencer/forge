//! Public API Boundary for forge-signal.
//! External components must depend ONLY on this module.
//!
//! Contract:
//! - `forge-signal` owns evaluation DAG scheduling.
//! - Host crates own structural graphs (including cyclic topology).
//! - Compute closures may consume opaque host snapshots directly.

// Re-export Data constructs
pub use crate::data::aspect::{Aspect, AspectMask, AspectVersion};
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
pub use crate::data::graph::SignalGraph;
pub use crate::data::handle::NodeId;
pub use crate::data::node::{EvaluationCondition, NodeEntry, NodeEvaluationConfig, NodeState};
pub use crate::data::subscriber_context::{SubscriberContext, SubscriberContextError};
pub use crate::data::telemetry::RuntimeTelemetry;
#[allow(deprecated)]
pub use crate::data::tier::{
    DependencyMode, DirtyPropagation, EvaluationTier, EvaluationTrigger, TierPolicy,
};
pub use crate::data::trace::TraceSummary;

// Re-export Logic constructs
pub use crate::logic::checkpoint::CheckpointRuntime;
pub use crate::logic::context::EvaluationContext;
pub use crate::logic::evaluation::{evaluate, evaluate_with_policy_resolver, evaluate_with_resolver};
pub use crate::logic::events::{EventBus, EventFlushError, SubscriberRegistryError};
pub use crate::logic::invalidation::mark_dirty;
pub use crate::logic::transaction::{
    emit_event_in_txn, evaluate_in_txn, flush_checkpoint_in_txn, SignalTransaction,
    SignalTransactionRuntime, TransactionOutcome,
};
pub use crate::presentation::contracts::{
    DependencyGraphContract, RawPathComputeContract, StructuralStateBoundaryContract,
};
