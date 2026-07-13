use super::{
    BTreeLookupBranch, BTreeNodeFormatLaw, BTreeRebuildMigrationLaw, BTreeRootPublicationLaw,
    BTreeSearchPathLaw, BTreeSeparatorLaw, BTreeSiblingLinkLaw, BTreeSplitMergeLaw,
    BTreeStableReadLaw, BTreeTombstoneLaw,
};
use crate::keyspace::{
    declare_comparator_law, require_canonical_key_encoding, require_prefix_law,
    require_range_bound_law,
};
use crate::strategy::{StrategyDeclaration, StrategyDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BTreeInvariantSuite {
    declaration: StrategyDeclaration,
    node_format: BTreeNodeFormatLaw,
    separator: BTreeSeparatorLaw,
    search_path: BTreeSearchPathLaw,
    split_merge: BTreeSplitMergeLaw,
    sibling_link: BTreeSiblingLinkLaw,
    tombstone: BTreeTombstoneLaw,
    stable_read: BTreeStableReadLaw,
    root_publication: BTreeRootPublicationLaw,
    rebuild_migration: BTreeRebuildMigrationLaw,
}

impl BTreeInvariantSuite {
    pub(crate) const fn new(
        declaration: StrategyDeclaration,
        node_format: BTreeNodeFormatLaw,
        separator: BTreeSeparatorLaw,
        search_path: BTreeSearchPathLaw,
        split_merge: BTreeSplitMergeLaw,
        sibling_link: BTreeSiblingLinkLaw,
        tombstone: BTreeTombstoneLaw,
        stable_read: BTreeStableReadLaw,
        root_publication: BTreeRootPublicationLaw,
        rebuild_migration: BTreeRebuildMigrationLaw,
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

    #[cfg(test)]
    pub(crate) const fn search_path_law(self) -> BTreeSearchPathLaw {
        self.search_path
    }

    pub fn verify_declared_baseline_lookup(self) -> super::BTreeSearchOutcome<BTreeLookupBranch> {
        let branch = BTreeLookupBranch::Left;
        let result = self
            .search_path
            .verify_search_and_insertion_path_from_observation(true, true, true, branch)
            .map(|()| branch);
        super::BTreeSearchOutcome::issue(result)
    }

    pub fn verify_declared_baseline_mutation_and_integrity(
        self,
    ) -> Result<super::BTreeCorruptionRegion, StrategyDenial> {
        self.node_format.verify_leaf_occupancy(2)?;
        self.split_merge.verify_split(2, 2, true)?;
        self.sibling_link.verify_sibling_link_posture(false)?;
        self.tombstone.verify_tombstone_posture(false)?;
        self.stable_read.verify_stable_read(1, 1, 1)?;
        self.node_format.verify_checksum_localization(false, true)
    }

    pub fn verify_declared_baseline_publication(self) -> Result<(), StrategyDenial> {
        self.root_publication
            .verify_root_publication_progress(true, true)
    }

    pub fn verify_declared_baseline_recovery(self) -> Result<(), StrategyDenial> {
        self.root_publication
            .verify_recovery_replay_progress(true, true)?;
        self.rebuild_migration
            .verify_rebuild_from_authority(4, 4, true)
    }
}

pub(crate) fn declare_btree_invariant_suite(
    declaration: StrategyDeclaration,
) -> Result<BTreeInvariantSuite, StrategyDenial> {
    let encoding = require_canonical_key_encoding(declaration.key_domain());
    let comparator = declare_comparator_law(encoding);
    let prefix =
        require_prefix_law(encoding).map_err(|_| StrategyDenial::RangeOrPrefixLawRequired)?;
    let range = require_range_bound_law(comparator)
        .map_err(|_| StrategyDenial::RangeOrPrefixLawRequired)?;
    let node_format = BTreeNodeFormatLaw::baseline();
    let separator_law = BTreeSeparatorLaw::new(comparator, prefix, range);

    Ok(BTreeInvariantSuite::new(
        declaration,
        node_format,
        separator_law,
        BTreeSearchPathLaw::new(separator_law),
        BTreeSplitMergeLaw::baseline(
            node_format.minimum_occupancy(),
            node_format.maximum_occupancy(),
        ),
        BTreeSiblingLinkLaw::baseline_absent(),
        BTreeTombstoneLaw::baseline_absent(),
        BTreeStableReadLaw::baseline(),
        BTreeRootPublicationLaw::baseline(),
        BTreeRebuildMigrationLaw::baseline(),
    ))
}
