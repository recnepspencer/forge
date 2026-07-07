use super::{
    WorthTouchedGraphConflictDeletionDisposition, WorthTouchedGraphConflictDeletionLedgerRow,
};
use crate::workload_composition::{
    compiled_product_consumer_cutover::vertical_slice::lookup_consumed::{
        current_lookup_consumed_vertical_slice_displaced_surfaces,
        LookupConsumedVerticalSliceDisplacedSurfaceDisposition,
    },
    ConflictBatchAdmissionOwner,
};

pub(super) fn current_phase_fifteen_deleted_surface_rows(
) -> Vec<WorthTouchedGraphConflictDeletionLedgerRow> {
    current_lookup_consumed_vertical_slice_displaced_surfaces()
        .iter()
        .copied()
        .map(|row| {
            WorthTouchedGraphConflictDeletionLedgerRow::explicit(
                row.current_path().to_string(),
                row.current_surface().to_string(),
                row.family_kind(),
                parse_owner(row.owner()),
                match row.disposition() {
                    LookupConsumedVerticalSliceDisplacedSurfaceDisposition::DeletedNow => {
                        WorthTouchedGraphConflictDeletionDisposition::DeletedAuthority
                    }
                },
                row.blocker().to_string(),
                row.removal_trigger().to_string(),
            )
        })
        .collect()
}

fn parse_owner(owner: &str) -> ConflictBatchAdmissionOwner {
    match owner {
        "worth-kernel" => ConflictBatchAdmissionOwner::WorthKernel,
        "worth-topo" => ConflictBatchAdmissionOwner::WorthTopo,
        "worth-spatial" => ConflictBatchAdmissionOwner::WorthSpatial,
        "forge-query" => ConflictBatchAdmissionOwner::ForgeQuery,
        _ => panic!("unknown phase 15 displaced-surface owner `{owner}`"),
    }
}
