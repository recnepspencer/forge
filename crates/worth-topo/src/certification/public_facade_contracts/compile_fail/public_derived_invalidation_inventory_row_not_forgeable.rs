use topology::derived_invalidation_authority_inventory::{
    DerivedInvalidationAuthorityDisposition, DerivedInvalidationAuthorityInventoryRow,
    DerivedInvalidationAuthorityOwner, DerivedInvalidationOldAuthorityKind,
    DerivedInvalidationProductCategory, DerivedInvalidationReplacementPhase,
};

fn main() {
    let _ = DerivedInvalidationAuthorityInventoryRow {
        source_path: "crates/worth-topo/src/derived_topology/materialized_graph/mod.rs",
        surface: "TopologyMaterializer::materialize_from_truth",
        product_category: DerivedInvalidationProductCategory::MaterializedGraph,
        authority_kind: DerivedInvalidationOldAuthorityKind::WholeViewMaterialization,
        disposition: DerivedInvalidationAuthorityDisposition::Migrate,
        owner: DerivedInvalidationAuthorityOwner::WorthTopoDerivedTopology,
        blocker: "blocked",
        removal_trigger: "trigger",
        replacement_phase: DerivedInvalidationReplacementPhase::PhaseSixProductMigrationSweep,
        ordinary_path: true,
        certification_or_bootstrap_only: false,
        cap: None,
        row_digest: String::new(),
    };
}
