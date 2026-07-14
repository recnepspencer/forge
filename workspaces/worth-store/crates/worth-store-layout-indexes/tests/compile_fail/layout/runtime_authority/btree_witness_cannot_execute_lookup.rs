use worth_store_layout_indexes::{BaselineBTreeExecutionWitness, BaselineBTreeLookupAdmission};
use worth_store_physical_format::PhysicalRecordSlot;

fn bypass(
    witness: &BaselineBTreeExecutionWitness,
    admission: &BaselineBTreeLookupAdmission,
    slot: PhysicalRecordSlot,
) {
    let _ = witness.execute_separator_directed_lookup(admission, slot);
}

fn main() {}
