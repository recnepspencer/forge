use super::assert_sources_exclude;

const PHYSICAL_RUNTIME: &str = "src/physical_runtime";

#[test]
fn store_cannot_own_a_branch_writer_registry() {
    assert_sources_exclude(
        PHYSICAL_RUNTIME,
        "branch-writer-registry",
        &["BranchWriterRegistry"],
    );
}

#[test]
fn branch_labels_cannot_define_physical_disjointness() {
    assert_sources_exclude(
        PHYSICAL_RUNTIME,
        "branch-label-disjointness",
        &["BranchLabelPhysicalDisjointness"],
    );
}
