const LOCAL_GUARD_RESIDUE_OCCURRENCE_COUNTS: [usize; 4] = [1, 2, 2, 1];

pub(super) fn topology_operator_local_guard_residue_total() -> usize {
    LOCAL_GUARD_RESIDUE_OCCURRENCE_COUNTS.iter().sum()
}
