use crate::data::aspect::{Aspect, AspectVersion};
use crate::data::dependency::DependencySnapshotId;
use crate::data::graph::{DependencySetId, SubscriberSetId};
use crate::data::node::NodeEvaluationConfig;
use crate::data::output::{ChangedRegion, PartitionSubscription};

use super::NodeEntry;

#[cfg_attr(not(test), allow(dead_code))]
impl NodeEntry {
    /// The current aspect versions.
    pub fn get_aspect_version(&self) -> AspectVersion {
        self.hot.aspect_version_header.global()
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn get_partitioned_aspect_version(&self, scope: &PartitionSubscription) -> AspectVersion {
        self.warm
            .aspect_version_overrides
            .scoped_or_global(scope, self.hot.aspect_version_header.global())
    }

    pub fn version_for_scope(&self, aspect: Aspect, scope: Option<&PartitionSubscription>) -> u64 {
        self.warm.aspect_version_overrides.version_for_scope(
            aspect,
            scope,
            self.hot.aspect_version_header.global(),
        )
    }

    /// Set the aspect version after evaluation.
    pub fn set_aspect_version(&mut self, version: AspectVersion) {
        self.hot.aspect_version_header.set_global(version);
        self.warm.aspect_version_overrides.set_global(version);
        self.hot
            .aspect_version_header
            .set_has_partition_overrides(self.warm.aspect_version_overrides.has_overrides());
    }

    pub fn apply_aspect_version(
        &mut self,
        version: AspectVersion,
        changed_regions: &[ChangedRegion],
    ) {
        self.hot.aspect_version_header.set_global(version);
        self.warm
            .aspect_version_overrides
            .apply_evaluation(version, changed_regions);
        self.hot
            .aspect_version_header
            .set_has_partition_overrides(self.warm.aspect_version_overrides.has_overrides());
    }

    /// Graph-owned dependency set handle.
    pub fn get_dependencies_id(&self) -> DependencySetId {
        self.hot.dependencies_id
    }

    /// Replace the dependency set handle.
    pub fn set_dependencies_id(&mut self, dependencies_id: DependencySetId) {
        self.hot.dependencies_id = dependencies_id;
    }

    /// Graph-owned subscriber set handle.
    pub fn get_subscribers_id(&self) -> SubscriberSetId {
        self.hot.subscribers_id
    }

    /// Replace the subscriber set handle.
    pub fn set_subscribers_id(&mut self, subscribers_id: SubscriberSetId) {
        self.hot.subscribers_id = subscribers_id;
    }

    /// The graph-owned dependency snapshot handle from the last clean evaluation.
    pub fn get_dep_snapshot_id(&self) -> DependencySnapshotId {
        self.hot.dep_snapshot_id
    }

    /// Replace the dependency snapshot handle.
    pub fn set_dep_snapshot_id(&mut self, snapshot_id: DependencySnapshotId) {
        self.hot.dep_snapshot_id = snapshot_id;
    }

    /// Whether this node is tombstoned.
    pub fn is_tombstoned(&self) -> bool {
        self.warm.tombstoned
    }

    /// Mark this node as tombstoned.
    #[cfg(test)]
    pub fn set_tombstoned(&mut self, tombstoned: bool) {
        self.warm.tombstoned = tombstoned;
    }

    /// Per-node evaluation policy descriptor.
    pub fn get_eval_config(&self) -> &NodeEvaluationConfig {
        &self.warm.eval_config
    }

    /// Replace per-node evaluation policy descriptor.
    pub fn set_eval_config(&mut self, config: NodeEvaluationConfig) {
        self.warm.eval_config = config;
    }
}
