use super::*;
use crate::facade::worth_milestone_one_runtime_builder;
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use worth_schema::facade::{seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase};

pub(super) fn seeded_sheet_disk_workspace(
    stem: &str,
) -> (
    ForgeQueryWorkspace,
    WorthTopologyQueryAssembly,
    worth_schema::facade::DerivedTopologyReadBasis,
) {
    let mut runtime = worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        stem,
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, stem).expect("workspace should build");
    let assembly =
        WorthTopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    (workspace, assembly, verified.read_basis)
}

pub(super) fn default_query_mutation_evidence(
    touched_aspect_paths: impl IntoIterator<Item = String>,
) -> WorthTopologyQueryMutationEvidence {
    WorthTopologyQueryMutationEvidence {
        authority_snapshot_id: 7,
        authority_branch_id: "worth.query.main".to_string(),
        authoritative_mutation_origin: WorthMutationOrigin::LocalEdit,
        derivation_origin: WorthMutationOrigin::LocalEdit,
        truth_basis_digest_hex: "query-topology-test-basis".to_string(),
        touched_aspect_paths: touched_aspect_paths.into_iter().collect(),
        precision_fallback_count: 0,
        precision_budget_fallback_count: 0,
    }
}
