use forge_store_layout_indexes::{
    btree_lookup_readiness_cases, BTreeLookupReadinessCaseId, OwnerCaseObservation,
};

fn main() {
    let case_id = btree_lookup_readiness_cases().next().unwrap();
    let _forged: OwnerCaseObservation<BTreeLookupReadinessCaseId> =
        OwnerCaseObservation { case_id };
}
