use forge_store_physical_isolation::{
    CompactionCutoverState, CompactionOwnerCase, CompactionOwnerCaseId,
};

pub const fn observe_physical_cutover(
    owner_case: &CompactionOwnerCase,
) -> (
    CompactionCutoverState,
    CompactionOwnerCaseId,
    CompactionCutoverState,
) {
    (owner_case.from(), owner_case.id(), owner_case.to())
}

#[cfg(test)]
mod tests {
    use super::observe_physical_cutover;
    use forge_store_physical_isolation::compaction_owner_case_inventory;
    use std::collections::BTreeSet;

    #[test]
    fn projection_preserves_every_physical_owner_case_bijectively() {
        let physical = compaction_owner_case_inventory().collect::<Vec<_>>();
        let projected = physical
            .iter()
            .map(observe_physical_cutover)
            .collect::<Vec<_>>();
        let unique_ids = projected
            .iter()
            .map(|(_, id, _)| id.name())
            .collect::<BTreeSet<_>>();

        assert_eq!(projected.len(), physical.len());
        assert_eq!(unique_ids.len(), physical.len());
        for (source, (from, id, to)) in physical.iter().zip(projected) {
            assert_eq!((from, id, to), (source.from(), source.id(), source.to()));
        }
    }
}
