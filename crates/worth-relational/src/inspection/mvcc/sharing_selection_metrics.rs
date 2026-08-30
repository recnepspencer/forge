use super::observation::RelationalBranchSharingObservation;

/// Selection-lane metrics: the shape of the caller's own selection and the
/// distinct owner objects it resolved to.
///
/// These metrics answer "what was asked for", not "how much storage exists".
/// They are never deduplicated against a different selection and never carry a
/// recorded cost.
impl RelationalBranchSharingObservation {
    /// Number of branch identities in the caller's selection.
    ///
    /// Truth source: the length of the slice passed to the observing call.
    /// Duplicate identities are refused rather than collapsed, so this is
    /// always the caller's own selection size.
    pub const fn branch_count(&self) -> u64 {
        self.branch_count
    }

    /// Number of distinct roots the selected branches resolve to.
    ///
    /// Truth source: the live roots of the selected branches, deduplicated by
    /// owner-issued root id. A value below [`Self::branch_count`] is exactly
    /// the observable signature of structural sharing between the selected
    /// branches.
    pub const fn unique_root_count(&self) -> u64 {
        self.unique_root_count
    }

    /// Owner-issued ids of those distinct roots, in ascending order.
    ///
    /// Truth source: the same live roots. Ids name roots for comparison only;
    /// they cannot be exchanged for a root or a branch basis.
    pub fn root_ids(&self) -> &[u64] {
        &self.root_ids
    }

    /// Number of distinct canonical commit artifacts the selected roots link
    /// to.
    ///
    /// Truth source: the commit ids carried by the selected roots and resolved
    /// through the commit catalog, deduplicated. Branches sharing one root
    /// necessarily share one artifact.
    pub const fn unique_canonical_commit_artifacts(&self) -> u64 {
        self.unique_canonical_commit_artifacts
    }

    /// Shallow inline bytes occupied by the live branch reference-state values
    /// selected for this observation.
    ///
    /// Truth source: the selection size and the compile-time layout of
    /// `RelationalBranchReferenceState`. It is exactly
    /// `branch_count * size_of::<RelationalBranchReferenceState>()`, and it is
    /// read neither from the owner allocation walk nor from any recorded
    /// counter.
    ///
    /// Byte scope: shallow inline reference-state storage only. It excludes
    /// branch-map and allocator capacity, synchronization objects,
    /// heap-reachable storage behind the reference state, selected or retained
    /// roots, and diagnostics. It is therefore not a total resident-memory
    /// measurement for the branches, and it is not governed by
    /// [`Self::byte_metric_scope`], which governs the live authoritative
    /// totals only.
    pub const fn branch_metadata_bytes(&self) -> u64 {
        self.branch_metadata_bytes
    }
}
