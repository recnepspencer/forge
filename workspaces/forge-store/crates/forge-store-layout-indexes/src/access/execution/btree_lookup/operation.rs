use forge_store_physical_format::PhysicalRecordSlot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BTreeLookupOperationDenied {
    Stale(crate::StaleLayoutMaterialization),
}

pub(crate) fn prepare(
    selected: crate::SelectedBTreeLookup,
    frontier: crate::CurrentMaterializationFrontier,
) -> super::BTreeLookupReadinessOutcome {
    super::admit_ready(super::lower(selected), frontier)
}

pub(crate) fn execute(
    selected: crate::planning::SelectedBTreeLookup,
    frontier: crate::CurrentMaterializationFrontier,
    source: crate::BaselineBTreeReadSource,
    probe_slot: PhysicalRecordSlot,
) -> Result<crate::BTreeLookupExecutionOutcome, BTreeLookupOperationDenied> {
    let readiness = prepare(selected, frontier);
    let ready = match readiness.view() {
        super::BTreeLookupReadinessView::Ready(_) => readiness
            .into_ready()
            .expect("owner view established B-tree lookup readiness"),
        super::BTreeLookupReadinessView::Stale(_) => {
            return Err(BTreeLookupOperationDenied::Stale(
                readiness
                    .into_stale()
                    .expect("owner view established stale B-tree lookup"),
            ));
        }
    };
    Ok(crate::btree_lookup_runtime().execute(ready, source, probe_slot))
}
