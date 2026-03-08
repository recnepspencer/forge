//! Kernel-side signal policy contract for feature graph nodes.
//!
//! DOMAIN: Declares how a feature node participates in `forge-signal`
//! scheduling without leaking runtime wiring details across the engine.

use serde::{Deserialize, Serialize};

use forge_signal::facade::{EvaluationCondition, NodeEvaluationConfig, TierPolicy, VersionComparatorPolicy};

/// Kernel-owned signal tiers for feature graph nodes.
///
/// Core feature execution remains in the `Core` tier today. Additional tiers
/// can be introduced later for analysis/query-style derived nodes without
/// changing the `FeatureTree` runtime shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FeatureSignalTier {
    Core,
}

/// Static signal policy for one feature node.
///
/// Defaults intentionally match the current mission-critical kernel contract:
/// static dependencies, `Always` evaluation, and exact comparator inheritance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureSignalPolicy {
    node_config: NodeEvaluationConfig,
    tier: Option<FeatureSignalTier>,
}

impl FeatureSignalPolicy {
    /// Explicit policy constructor for advanced feature nodes.
    pub const fn new(node_config: NodeEvaluationConfig, tier: Option<FeatureSignalTier>) -> Self {
        Self { node_config, tier }
    }

    /// Default policy for core feature execution nodes.
    pub fn core() -> Self {
        Self {
            node_config: NodeEvaluationConfig::default(),
            tier: Some(FeatureSignalTier::Core),
        }
    }

    /// Use a custom evaluation condition while keeping the same tier.
    pub fn with_condition(mut self, condition: EvaluationCondition) -> Self {
        self.node_config.condition = condition;
        self
    }

    /// Use an explicit comparator policy for this node.
    pub fn with_comparator(mut self, comparator: VersionComparatorPolicy) -> Self {
        self.node_config.comparator = Some(comparator);
        self
    }

    /// Assign a specific signal tier.
    pub fn with_tier(mut self, tier: FeatureSignalTier) -> Self {
        self.tier = Some(tier);
        self
    }

    /// Remove explicit tier assignment so runtime fallback behavior applies.
    pub fn without_tier(mut self) -> Self {
        self.tier = None;
        self
    }

    /// Per-node signal evaluation configuration.
    pub fn node_config(&self) -> &NodeEvaluationConfig {
        &self.node_config
    }

    /// Optional tier assignment for comparator inheritance and future planning.
    pub const fn tier(&self) -> Option<FeatureSignalTier> {
        self.tier
    }

    /// Tier policy table entry for the default core tier.
    pub fn core_tier_policy() -> TierPolicy<FeatureSignalTier> {
        TierPolicy::new(
            FeatureSignalTier::Core,
            forge_signal::facade::DependencyMode::Static,
            forge_signal::facade::DirtyPropagation::Immediate,
            forge_signal::facade::EvaluationTrigger::LazyPull,
        )
    }
}

impl Default for FeatureSignalPolicy {
    fn default() -> Self {
        Self::core()
    }
}
