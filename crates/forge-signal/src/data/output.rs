use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::data::aspect::AspectVersion;
use crate::data::core_profile::StableHashValue;

macro_rules! define_string_token {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(from = "String", into = "String")]
        pub struct $name {
            value: String,
            stable_hash: StableHashValue,
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new(String::new())
            }
        }

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                let value = value.into();
                Self {
                    stable_hash: stable_string_hash(&value),
                    value,
                }
            }

            pub fn as_str(&self) -> &str {
                &self.value
            }

            pub fn stable_hash(&self) -> StableHashValue {
                self.stable_hash
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.value
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.value.cmp(&other.value)
            }
        }

        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.value.hash(state);
            }
        }
    };
}

define_string_token!(
    /// Host-supplied stable identity token for one evaluated output artifact.
    OutputIdentity
);

define_string_token!(
    /// Host-supplied continuity token for lineage preservation when output
    /// identity is too coarse or intentionally absent.
    ///
    /// This is domain-agnostic. Host code can use it to express “this result
    /// should continue the same artifact lineage” without forcing that meaning
    /// onto `OutputIdentity`.
    ArtifactContinuityToken
);

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct ChangedRegion {
    pub partition: PartitionToken,
    #[serde(default)]
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CanonicalChangedRegions {
    regions: Vec<ChangedRegion>,
}

impl CanonicalChangedRegions {
    pub fn new(regions: impl IntoIterator<Item = ChangedRegion>) -> Self {
        Self::canonicalize_unordered(regions)
    }

    pub fn canonicalize_unordered(regions: impl IntoIterator<Item = ChangedRegion>) -> Self {
        let mut regions = regions.into_iter().collect::<Vec<_>>();
        if regions.len() > 1 {
            regions.sort_unstable();
            regions.dedup();
        }
        Self { regions }
    }

    pub fn from_ordered_unique(regions: impl IntoIterator<Item = ChangedRegion>) -> Self {
        let regions = regions.into_iter().collect::<Vec<_>>();
        debug_assert!(is_strict_region_order(regions.as_slice()));
        Self { regions }
    }

    pub fn from_slice(regions: &[ChangedRegion]) -> Self {
        Self::new(regions.iter().cloned())
    }

    pub fn as_slice(&self) -> &[ChangedRegion] {
        &self.regions
    }

    pub fn into_vec(self) -> Vec<ChangedRegion> {
        self.regions
    }

    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }
}

impl From<Vec<ChangedRegion>> for CanonicalChangedRegions {
    fn from(regions: Vec<ChangedRegion>) -> Self {
        Self::new(regions)
    }
}

impl From<&[ChangedRegion]> for CanonicalChangedRegions {
    fn from(regions: &[ChangedRegion]) -> Self {
        Self::from_slice(regions)
    }
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct PartitionTokenId(pub u32);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct DetailTokenId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct InternedPartitionSubscription {
    pub partition: PartitionTokenId,
    pub detail: Option<DetailTokenId>,
    pub match_mode: PartitionMatchMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PartitionTokenRef<'a> {
    Public(&'a PartitionToken),
    Interned(PartitionTokenId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DetailTokenRef<'a> {
    Public(&'a str),
    Interned(DetailTokenId),
}

fn is_strict_region_order(regions: &[ChangedRegion]) -> bool {
    regions.windows(2).all(|pair| pair[0] < pair[1])
}

pub(crate) trait PartitionScoped {
    fn partition_token_ref(&self) -> PartitionTokenRef<'_>;
    fn detail_token_ref(&self) -> Option<DetailTokenRef<'_>>;
    fn match_mode(&self) -> PartitionMatchMode;
}

impl PartitionScoped for PartitionSubscription {
    fn partition_token_ref(&self) -> PartitionTokenRef<'_> {
        PartitionTokenRef::Public(&self.partition)
    }

    fn detail_token_ref(&self) -> Option<DetailTokenRef<'_>> {
        self.detail.as_deref().map(DetailTokenRef::Public)
    }

    fn match_mode(&self) -> PartitionMatchMode {
        self.match_mode
    }
}

impl PartitionScoped for InternedPartitionSubscription {
    fn partition_token_ref(&self) -> PartitionTokenRef<'_> {
        PartitionTokenRef::Interned(self.partition)
    }

    fn detail_token_ref(&self) -> Option<DetailTokenRef<'_>> {
        self.detail.map(DetailTokenRef::Interned)
    }

    fn match_mode(&self) -> PartitionMatchMode {
        self.match_mode
    }
}

impl PartitionScoped for ChangedRegion {
    fn partition_token_ref(&self) -> PartitionTokenRef<'_> {
        PartitionTokenRef::Public(&self.partition)
    }

    fn detail_token_ref(&self) -> Option<DetailTokenRef<'_>> {
        self.detail.as_deref().map(DetailTokenRef::Public)
    }

    fn match_mode(&self) -> PartitionMatchMode {
        if self.detail.is_some() {
            PartitionMatchMode::PartitionAndDetail
        } else {
            PartitionMatchMode::WholePartition
        }
    }
}

pub(crate) fn scopes_overlap(left: &impl PartitionScoped, right: &impl PartitionScoped) -> bool {
    if left.partition_token_ref() != right.partition_token_ref() {
        return false;
    }
    match (left.match_mode(), right.match_mode()) {
        (PartitionMatchMode::WholePartition, _) | (_, PartitionMatchMode::WholePartition) => true,
        (PartitionMatchMode::PartitionAndDetail, PartitionMatchMode::PartitionAndDetail) => {
            left.detail_token_ref() == right.detail_token_ref()
        }
    }
}

pub(crate) fn scope_touched_by_artifact_state(
    artifact_state: Option<&crate::data::trace::RuntimeArtifactState>,
    scope: &PartitionSubscription,
) -> bool {
    let Some(artifact_state) = artifact_state else {
        return false;
    };
    if artifact_state.output_change == OutputChange::Unchanged {
        return false;
    }
    if artifact_state.changed_scopes.is_empty() {
        return true;
    }
    artifact_state
        .changed_scopes
        .as_slice()
        .iter()
        .any(|changed_scope| scopes_overlap(scope, changed_scope))
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
            detail: subscription
                .detail
                .as_deref()
                .map(|detail| self.intern_detail(detail)),
            match_mode: subscription.match_mode,
        }
    }

    pub fn intern_changed_region(
        &mut self,
        region: &ChangedRegion,
    ) -> InternedPartitionSubscription {
        InternedPartitionSubscription {
            partition: self.intern_partition(&region.partition.0),
            detail: region
                .detail
                .as_deref()
                .map(|detail| self.intern_detail(detail)),
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

    pub fn detail_count(&self) -> usize {
        self.details.len()
    }

    pub fn token_count(&self) -> usize {
        self.partition_count() + self.detail_count()
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

define_string_token!(
    /// Family namespace for keyed computations.
    ComputationFamily
);

define_string_token!(
    /// Stable key for one keyed computation inside a family.
    ComputationKey
);

define_string_token!(
    /// Stable host-provided structural memoization key.
    StructuralMemoKey
);

fn stable_string_hash(value: &str) -> StableHashValue {
    #[cfg(feature = "profile-compact")]
    let mut hash: StableHashValue = 0xcbf29ce484222325_u64;
    #[cfg(any(feature = "profile-standard", feature = "profile-extended"))]
    let mut hash: StableHashValue = 0x6c62272e07bb014262b821756295c58d_u128;
    for byte in value.as_bytes() {
        hash ^= *byte as StableHashValue;
        #[cfg(feature = "profile-compact")]
        {
            hash = hash.wrapping_mul(0x100000001b3_u64);
        }
        #[cfg(any(feature = "profile-standard", feature = "profile-extended"))]
        {
            hash = hash.wrapping_mul(0x0000000001000000000000000000013B_u128);
        }
    }
    hash
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
    pub fn new(family: impl Into<ComputationFamily>, key: impl Into<ComputationKey>) -> Self {
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
    pub continuity_token: Option<ArtifactContinuityToken>,
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
            continuity_token: None,
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

    pub fn with_continuity_token(
        mut self,
        continuity_token: impl Into<ArtifactContinuityToken>,
    ) -> Self {
        self.continuity_token = Some(continuity_token.into());
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
