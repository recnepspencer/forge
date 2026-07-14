use forge_store_layout_indexes::{
    btree_lookup_readiness_cases, BTreeLookupReadinessCaseId, OwnerCaseObservation,
};

fn record_execution(_observed: OwnerCaseObservation<BTreeLookupReadinessCaseId>) {}

fn main() {
    let declared = btree_lookup_readiness_cases().next().unwrap();
    record_execution(declared);
}
