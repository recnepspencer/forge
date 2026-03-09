use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data::aspect::AspectVersion;

/// Host-supplied stable identity token for one evaluated output artifact.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct OutputIdentity(pub String);

impl OutputIdentity {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<String> for OutputIdentity {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for OutputIdentity {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

/// Generic opaque partition token for partitioned outputs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct PartitionToken(pub String);

impl PartitionToken {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<String> for PartitionToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for PartitionToken {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

/// Generic changed-region descriptor for partition-aware outputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChangedRegion {
    pub partition: PartitionToken,
    #[serde(default)]
    pub detail: Option<String>,
}

/// How one partition subscription should match changed-region data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PartitionMatchMode {
    WholePartition,
    PartitionAndDetail,
}

/// Public partition subscription descriptor for dependency edges.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PartitionSubscription {
    pub partition: PartitionToken,
    #[serde(default)]
    pub detail: Option<String>,
    pub match_mode: PartitionMatchMode,
}

impl PartitionSubscription {
    pub fn whole_partition(partition: impl Into<PartitionToken>) -> Self {
        Self {
            partition: partition.into(),
            detail: None,
            match_mode: PartitionMatchMode::WholePartition,
        }
    }

    pub fn partition_and_detail(
        partition: impl Into<PartitionToken>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            partition: partition.into(),
            detail: Some(detail.into()),
            match_mode: PartitionMatchMode::PartitionAndDetail,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct PartitionTokenId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct DetailTokenId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct InternedPartitionSubscription {
    pub partition: PartitionTokenId,
    pub detail: Option<DetailTokenId>,
    pub match_mode: PartitionMatchMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartitionInterner {
    partitions: Vec<String>,
    details: Vec<String>,
    #[serde(default)]
    partition_lookup: BTreeMap<String, PartitionTokenId>,
    #[serde(default)]
    detail_lookup: BTreeMap<String, DetailTokenId>,
}

impl PartitionInterner {
    pub fn intern_subscription(
        &mut self,
        subscription: &PartitionSubscription,
    ) -> InternedPartitionSubscription {
        InternedPartitionSubscription {
            partition: self.intern_partition(&subscription.partition.0),
            detail: subscription.detail.as_deref().map(|detail| self.intern_detail(detail)),
            match_mode: subscription.match_mode,
        }
    }

    pub fn intern_changed_region(&mut self, region: &ChangedRegion) -> InternedPartitionSubscription {
        InternedPartitionSubscription {
            partition: self.intern_partition(&region.partition.0),
            detail: region.detail.as_deref().map(|detail| self.intern_detail(detail)),
            match_mode: if region.detail.is_some() {
                PartitionMatchMode::PartitionAndDetail
            } else {
                PartitionMatchMode::WholePartition
            },
        }
    }

    pub fn partition_count(&self) -> usize {
        self.partitions.len()
    }

    fn intern_partition(&mut self, partition: &str) -> PartitionTokenId {
        if let Some(id) = self.partition_lookup.get(partition).copied() {
            return id;
        }
        let id = PartitionTokenId(self.partitions.len() as u32);
        self.partitions.push(partition.to_owned());
        self.partition_lookup.insert(partition.to_owned(), id);
        id
    }

    fn intern_detail(&mut self, detail: &str) -> DetailTokenId {
        if let Some(id) = self.detail_lookup.get(detail).copied() {
            return id;
        }
        let id = DetailTokenId(self.details.len() as u32);
        self.details.push(detail.to_owned());
        self.detail_lookup.insert(detail.to_owned(), id);
        id
    }
}

impl ChangedRegion {
    pub fn new(partition: impl Into<PartitionToken>) -> Self {
        Self {
            partition: partition.into(),
            detail: None,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

/// Host-declared output continuity after evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OutputChange {
    /// Default assumption when no richer semantics are supplied.
    #[default]
    Replaced,
    /// Same artifact identity refreshed with meaningful internal change.
    Refreshed,
    /// Inputs changed but the output artifact identity is unchanged.
    Unchanged,
}

/// Family namespace for keyed computations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ComputationFamily(pub String);

impl ComputationFamily {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<String> for ComputationFamily {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ComputationFamily {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

/// Stable key for one keyed computation inside a family.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ComputationKey(pub String);

impl ComputationKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<String> for ComputationKey {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ComputationKey {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

/// Stable host-provided structural memoization key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct StructuralMemoKey(pub String);

impl StructuralMemoKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<String> for StructuralMemoKey {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for StructuralMemoKey {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

/// How one evaluation result was produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MemoizedResultOrigin {
    #[default]
    DirectCompute,
    MemoizedFromCache,
}

/// Keyed execution metadata used by advanced runtime APIs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KeyedComputation {
    pub family: ComputationFamily,
    pub key: ComputationKey,
    #[serde(default)]
    pub memo_key: Option<StructuralMemoKey>,
}

impl KeyedComputation {
    pub fn new(
        family: impl Into<ComputationFamily>,
        key: impl Into<ComputationKey>,
    ) -> Self {
        Self {
            family: family.into(),
            key: key.into(),
            memo_key: None,
        }
    }

    pub fn with_memo_key(mut self, memo_key: impl Into<StructuralMemoKey>) -> Self {
        self.memo_key = Some(memo_key.into());
        self
    }
}

/// Rich evaluation report for diff-aware execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeEvaluationResult {
    pub aspect_version: AspectVersion,
    #[serde(default)]
    pub output_identity: Option<OutputIdentity>,
    #[serde(default)]
    pub output_change: OutputChange,
    #[serde(default)]
    pub changed_regions: Vec<ChangedRegion>,
    #[serde(default)]
    pub labels: Vec<String>,
}

impl NodeEvaluationResult {
    pub fn from_version(aspect_version: AspectVersion) -> Self {
        Self {
            aspect_version,
            output_identity: None,
            output_change: OutputChange::Replaced,
            changed_regions: Vec::new(),
            labels: Vec::new(),
        }
    }

    pub fn with_output_identity(mut self, output_identity: impl Into<OutputIdentity>) -> Self {
        self.output_identity = Some(output_identity.into());
        self
    }

    pub fn with_output_change(mut self, output_change: OutputChange) -> Self {
        self.output_change = output_change;
        self
    }

    pub fn with_changed_region(mut self, region: ChangedRegion) -> Self {
        self.changed_regions.push(region);
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.labels.push(label.into());
        self
    }
}

pub trait IntoNodeEvaluationResult {
    fn into_evaluation_result(self) -> NodeEvaluationResult;
}

impl IntoNodeEvaluationResult for AspectVersion {
    fn into_evaluation_result(self) -> NodeEvaluationResult {
        NodeEvaluationResult::from_version(self)
    }
}

impl IntoNodeEvaluationResult for NodeEvaluationResult {
    fn into_evaluation_result(self) -> NodeEvaluationResult {
        self
    }
}
