use super::{
    S8BTreeLookupBranch, S8BTreeNodeFormatLaw, S8BTreeRebuildMigrationLaw,
    S8BTreeRootPublicationLaw, S8BTreeSearchPathLaw, S8BTreeSeparatorLaw, S8BTreeSiblingLinkLaw,
    S8BTreeSplitMergeLaw, S8BTreeStableReadLaw, S8BTreeTombstoneLaw,
};
use crate::keyspace::{
    declare_comparator_law, require_canonical_key_encoding, require_prefix_law,
    require_range_bound_law,
};
use crate::strategy::{S8StrategyDeclaration, S8StrategyDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8BTreeInvariantSuite {
    declaration: S8StrategyDeclaration,
    node_format: S8BTreeNodeFormatLaw,
    separator: S8BTreeSeparatorLaw,
    search_path: S8BTreeSearchPathLaw,
    split_merge: S8BTreeSplitMergeLaw,
    sibling_link: S8BTreeSiblingLinkLaw,
    tombstone: S8BTreeTombstoneLaw,
    stable_read: S8BTreeStableReadLaw,
    root_publication: S8BTreeRootPublicationLaw,
    rebuild_migration: S8BTreeRebuildMigrationLaw,
}

impl S8BTreeInvariantSuite {
    pub(crate) const fn new(
        declaration: S8StrategyDeclaration,
        node_format: S8BTreeNodeFormatLaw,
        separator: S8BTreeSeparatorLaw,
        search_path: S8BTreeSearchPathLaw,
        split_merge: S8BTreeSplitMergeLaw,
        sibling_link: S8BTreeSiblingLinkLaw,
        tombstone: S8BTreeTombstoneLaw,
        stable_read: S8BTreeStableReadLaw,
        root_publication: S8BTreeRootPublicationLaw,
        rebuild_migration: S8BTreeRebuildMigrationLaw,
    ) -> Self {
        Self {
            declaration,
            node_format,
            separator,
            search_path,
            split_merge,
            sibling_link,
            tombstone,
            stable_read,
            root_publication,
            rebuild_migration,
        }
    }

    pub(crate) const fn node_format_law(self) -> S8BTreeNodeFormatLaw {
        self.node_format
    }

    pub(crate) const fn separator_law(self) -> S8BTreeSeparatorLaw {
        self.separator
    }

    pub(crate) const fn search_path_law(self) -> S8BTreeSearchPathLaw {
        self.search_path
    }

    pub(crate) const fn split_merge_law(self) -> S8BTreeSplitMergeLaw {
        self.split_merge
    }

    pub(crate) const fn sibling_link_law(self) -> S8BTreeSiblingLinkLaw {
        self.sibling_link
    }

    pub(crate) const fn tombstone_law(self) -> S8BTreeTombstoneLaw {
        self.tombstone
    }

    pub(crate) const fn stable_read_law(self) -> S8BTreeStableReadLaw {
        self.stable_read
    }

    pub(crate) const fn root_publication_law(self) -> S8BTreeRootPublicationLaw {
        self.root_publication
    }

    pub(crate) const fn rebuild_migration_law(self) -> S8BTreeRebuildMigrationLaw {
        self.rebuild_migration
    }

    pub fn verify_declared_baseline_lookup(
        self,
    ) -> super::S8BTreeSearchOutcome<S8BTreeLookupBranch> {
        let branch = S8BTreeLookupBranch::Left;
        let result = self
            .search_path
            .verify_search_and_insertion_path_from_observation(true, true, true, branch)
            .map(|()| branch);
        super::S8BTreeSearchOutcome::issue(result)
    }

    pub fn verify_declared_baseline_mutation_and_integrity(
        self,
    ) -> Result<super::S8BTreeCorruptionRegion, S8StrategyDenial> {
        self.node_format.verify_leaf_occupancy(2)?;
        self.split_merge.verify_split(2, 2, true)?;
        self.sibling_link.verify_sibling_link_posture(false)?;
        self.tombstone.verify_tombstone_posture(false)?;
        self.stable_read.verify_stable_read(1, 1, 1)?;
        self.node_format.verify_checksum_localization(false, true)
    }

    pub fn verify_declared_baseline_publication(self) -> Result<(), S8StrategyDenial> {
        self.root_publication
            .verify_root_publication_progress(true, true)
    }

    pub fn verify_declared_baseline_recovery(self) -> Result<(), S8StrategyDenial> {
        self.root_publication
            .verify_recovery_replay_progress(true, true)?;
        self.rebuild_migration
            .verify_rebuild_from_authority(4, 4, true)
    }
}

pub(crate) fn declare_btree_invariant_suite(
    declaration: S8StrategyDeclaration,
) -> Result<S8BTreeInvariantSuite, S8StrategyDenial> {
    let encoding = require_canonical_key_encoding(declaration.key_domain());
    let comparator = declare_comparator_law(encoding);
    let prefix =
        require_prefix_law(encoding).map_err(|_| S8StrategyDenial::RangeOrPrefixLawRequired)?;
    let range = require_range_bound_law(comparator)
        .map_err(|_| S8StrategyDenial::RangeOrPrefixLawRequired)?;
    let node_format = S8BTreeNodeFormatLaw::baseline();
    let separator_law = S8BTreeSeparatorLaw::new(comparator, prefix, range);

    Ok(S8BTreeInvariantSuite::new(
        declaration,
        node_format,
        separator_law,
        S8BTreeSearchPathLaw::new(separator_law),
        S8BTreeSplitMergeLaw::baseline(
            node_format.minimum_occupancy(),
            node_format.maximum_occupancy(),
        ),
        S8BTreeSiblingLinkLaw::baseline_absent(),
        S8BTreeTombstoneLaw::baseline_absent(),
        S8BTreeStableReadLaw::baseline(),
        S8BTreeRootPublicationLaw::baseline(),
        S8BTreeRebuildMigrationLaw::baseline(),
    ))
}
