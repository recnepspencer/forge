use topology::derived_invalidation_family_catalog::{
    DerivedTopologyProductFamilyIdentity, DerivedTopologyProductFamilyRecord,
};

fn main() {
    consume_family_record(DerivedTopologyProductFamilyIdentity::LoopCycles);
}

fn consume_family_record(_record: DerivedTopologyProductFamilyRecord) {}
