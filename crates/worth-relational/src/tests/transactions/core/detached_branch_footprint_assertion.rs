use crate::facade::mvcc::{RelationalTransactionReadLocus, RelationalTransactionWriteLocus};
use crate::facade::transactions::CreatedEntityRef;

pub(super) fn assert_detached_branch_footprint(
    basis: &crate::facade::branch::AdmittedRelationalBranchBasis,
    footprint: &crate::facade::mvcc::RelationalTransactionFootprint,
    created: &CreatedEntityRef,
    absent_created: &CreatedEntityRef,
) {
    assert_eq!(footprint.branch(), basis.identity().branch_id());
    assert_eq!(footprint.reference(), basis.reference());
    assert_eq!(
        footprint
            .reads()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>(),
        [created, absent_created]
            .into_iter()
            .cloned()
            .map(RelationalTransactionReadLocus::CreatedEntity)
            .collect()
    );
    assert_eq!(
        footprint.writes().cloned().collect::<Vec<_>>(),
        vec![RelationalTransactionWriteLocus::CreatedEntity(
            created.clone()
        )]
    );
    assert_eq!(
        footprint.write_partitions().copied().collect::<Vec<_>>(),
        vec![created.partition_id]
    );
}
