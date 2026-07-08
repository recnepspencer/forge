use super::baseline_btree_counter_observation::BaselineBTreeLookupBranch;
use super::baseline_btree_invariant_witness::collect_baseline_btree_invariant_witness;
pub use super::baseline_btree_invariant_witness::BaselineBTreeCorruptionObservation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreeLookupInvariantProof {
    probe_precedes_separator: bool,
    left_max_precedes_separator: bool,
    separator_precedes_right_min: bool,
    branch: BaselineBTreeLookupBranch,
}

impl BaselineBTreeLookupInvariantProof {
    pub const fn probe_precedes_separator(self) -> bool {
        self.probe_precedes_separator
    }

    pub const fn left_max_precedes_separator(self) -> bool {
        self.left_max_precedes_separator
    }

    pub const fn separator_precedes_right_min(self) -> bool {
        self.separator_precedes_right_min
    }

    pub const fn branch(self) -> BaselineBTreeLookupBranch {
        self.branch
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreeMutationInvariantProof {
    leaf_occupancy: u16,
    split_left_occupancy: u16,
    split_right_occupancy: u16,
    promoted_separator_between_halves: bool,
    sibling_links_present: bool,
    tombstones_present: bool,
    stable_generation: u64,
    corruption: BaselineBTreeCorruptionObservation,
}

impl BaselineBTreeMutationInvariantProof {
    pub const fn leaf_occupancy(self) -> u16 {
        self.leaf_occupancy
    }

    pub const fn split_left_occupancy(self) -> u16 {
        self.split_left_occupancy
    }

    pub const fn split_right_occupancy(self) -> u16 {
        self.split_right_occupancy
    }

    pub const fn promoted_separator_between_halves(self) -> bool {
        self.promoted_separator_between_halves
    }

    pub const fn sibling_links_present(self) -> bool {
        self.sibling_links_present
    }

    pub const fn tombstones_present(self) -> bool {
        self.tombstones_present
    }

    pub const fn stable_generation(self) -> u64 {
        self.stable_generation
    }

    pub const fn corruption(self) -> BaselineBTreeCorruptionObservation {
        self.corruption
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreePublicationInvariantProof {
    root_generation_advanced: bool,
    checksum_scope_matches: bool,
    root_manifest_candidates: u16,
}

impl BaselineBTreePublicationInvariantProof {
    pub const fn root_generation_advanced(self) -> bool {
        self.root_generation_advanced
    }

    pub const fn checksum_scope_matches(self) -> bool {
        self.checksum_scope_matches
    }

    pub const fn root_manifest_candidates(self) -> u16 {
        self.root_manifest_candidates
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreeRecoveryInvariantProof {
    replay_generation_monotonic: bool,
    manifest_advanced: bool,
    rebuild_authority_records: u16,
    rebuild_output_records: u16,
    rebuild_source_authoritative: bool,
}

impl BaselineBTreeRecoveryInvariantProof {
    pub const fn replay_generation_monotonic(self) -> bool {
        self.replay_generation_monotonic
    }

    pub const fn manifest_advanced(self) -> bool {
        self.manifest_advanced
    }

    pub const fn rebuild_authority_records(self) -> u16 {
        self.rebuild_authority_records
    }

    pub const fn rebuild_output_records(self) -> u16 {
        self.rebuild_output_records
    }

    pub const fn rebuild_source_authoritative(self) -> bool {
        self.rebuild_source_authoritative
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineBTreeInvariantProof {
    lookup: BaselineBTreeLookupInvariantProof,
    mutation: BaselineBTreeMutationInvariantProof,
    publication: BaselineBTreePublicationInvariantProof,
    recovery: BaselineBTreeRecoveryInvariantProof,
}

impl BaselineBTreeInvariantProof {
    pub const fn lookup(self) -> BaselineBTreeLookupInvariantProof {
        self.lookup
    }

    pub const fn mutation(self) -> BaselineBTreeMutationInvariantProof {
        self.mutation
    }

    pub const fn publication(self) -> BaselineBTreePublicationInvariantProof {
        self.publication
    }

    pub const fn recovery(self) -> BaselineBTreeRecoveryInvariantProof {
        self.recovery
    }
}

pub fn prove_baseline_btree_invariants() -> BaselineBTreeInvariantProof {
    let witness = collect_baseline_btree_invariant_witness();

    BaselineBTreeInvariantProof {
        lookup: BaselineBTreeLookupInvariantProof {
            probe_precedes_separator: witness.lookup().probe_precedes_separator(),
            left_max_precedes_separator: witness.lookup().left_max_precedes_separator(),
            separator_precedes_right_min: witness.lookup().separator_precedes_right_min(),
            branch: witness.lookup().branch(),
        },
        mutation: BaselineBTreeMutationInvariantProof {
            leaf_occupancy: witness.mutation().leaf_occupancy(),
            split_left_occupancy: witness.mutation().split_left_occupancy(),
            split_right_occupancy: witness.mutation().split_right_occupancy(),
            promoted_separator_between_halves: witness
                .mutation()
                .promoted_separator_between_halves(),
            sibling_links_present: witness.mutation().sibling_links_present(),
            tombstones_present: witness.mutation().tombstones_present(),
            stable_generation: witness.mutation().stable_generation(),
            corruption: witness.mutation().corruption(),
        },
        publication: BaselineBTreePublicationInvariantProof {
            root_generation_advanced: witness.publication().root_generation_advanced(),
            checksum_scope_matches: witness.publication().checksum_scope_matches(),
            root_manifest_candidates: witness.publication().root_manifest_candidates(),
        },
        recovery: BaselineBTreeRecoveryInvariantProof {
            replay_generation_monotonic: witness.recovery().replay_generation_monotonic(),
            manifest_advanced: witness.recovery().manifest_advanced(),
            rebuild_authority_records: witness.recovery().rebuild_authority_records(),
            rebuild_output_records: witness.recovery().rebuild_output_records(),
            rebuild_source_authoritative: witness.recovery().rebuild_source_authoritative(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        prove_baseline_btree_invariants, BaselineBTreeCorruptionObservation,
        BaselineBTreeLookupBranch,
    };
    use crate::layout_access::baseline_btree_invariant_witness::collect_baseline_btree_invariant_witness;

    #[test]
    fn baseline_btree_invariant_proof_is_lower_family_owned() {
        let proof = prove_baseline_btree_invariants();
        let witness = collect_baseline_btree_invariant_witness();

        assert_eq!(proof.lookup().branch(), BaselineBTreeLookupBranch::Left);
        assert!(proof.lookup().probe_precedes_separator());
        assert!(proof.lookup().left_max_precedes_separator());
        assert!(proof.lookup().separator_precedes_right_min());
        assert_eq!(
            proof.mutation().leaf_occupancy(),
            witness.mutation().leaf_occupancy()
        );
        assert_eq!(
            proof.mutation().corruption(),
            BaselineBTreeCorruptionObservation::Header
        );
        assert_eq!(proof.publication().root_manifest_candidates(), 1);
        assert_eq!(
            proof.recovery().rebuild_authority_records(),
            witness.recovery().rebuild_authority_records()
        );
        assert!(proof.recovery().rebuild_source_authoritative());
    }
}
