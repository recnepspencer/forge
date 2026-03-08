use crate::data::aspect::AspectMask;
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::{EvaluationCondition, NodeEvaluationConfig};

/// Fluent node builder for accessible public graph configuration.
pub struct NodeBuilder<'a> {
    graph: &'a mut SignalGraph,
    config: NodeEvaluationConfig,
}

impl<'a> NodeBuilder<'a> {
    pub(crate) fn new(graph: &'a mut SignalGraph) -> Self {
        Self {
            graph,
            config: NodeEvaluationConfig::default(),
        }
    }

    /// Declarative aspect intent for this node.
    pub fn depends_on_aspects(mut self, aspects: impl Into<AspectMask>) -> Self {
        self.config.depends_on_aspects = Some(aspects.into());
        self
    }

    /// Set the node evaluation condition directly.
    pub fn condition(mut self, condition: EvaluationCondition) -> Self {
        self.config.condition = condition;
        self
    }

    /// Always evaluate the node when dirty.
    pub fn always(self) -> Self {
        self.condition(EvaluationCondition::Always)
    }

    /// Evaluate the node only on explicit request.
    pub fn on_demand(self) -> Self {
        self.condition(EvaluationCondition::OnDemand)
    }

    /// Evaluate the node only after the quiet period has elapsed.
    pub fn debounce(self, milliseconds: u64) -> Self {
        self.condition(EvaluationCondition::Debounce(milliseconds))
    }

    /// Evaluate the node only when the matching aspects are touched.
    pub fn aspect_filter(self, mask: impl Into<AspectMask>) -> Self {
        self.condition(EvaluationCondition::AspectFilter(mask.into()))
    }

    /// Evaluate the node only when the upstream delta crosses the threshold.
    pub fn delta_threshold(self, threshold: f64) -> Self {
        self.condition(EvaluationCondition::DeltaThreshold(threshold))
    }

    /// Defer the condition decision to a host-provided resolver.
    pub fn custom_condition(self, key: impl Into<String>) -> Self {
        self.condition(EvaluationCondition::Custom(key.into()))
    }

    /// Override the comparator policy for this node.
    pub fn comparator(mut self, comparator: VersionComparatorPolicy) -> Self {
        self.config.comparator = Some(comparator);
        self
    }

    /// Convenience override for tolerance-based comparison.
    pub fn tolerance(self, epsilon: u64) -> Self {
        self.comparator(VersionComparatorPolicy::Tolerance { epsilon })
    }

    /// Build the node into the graph.
    pub fn build(self) -> NodeId {
        self.graph.create_node_with_config(self.config)
    }
}
