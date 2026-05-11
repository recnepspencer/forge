use crate::facade::milestone_one_runtime_builder;
use crate::facade::TopologyQueryMutationEvidence;
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::runtime_boundary::read_stage::open_topology_read_view;
use crate::projection::TopologyQueryRowLookup;
use forge_query::facade::ForgeQueryEntity;
use forge_query::facade::ForgeQueryWorkspace;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use schema::facade::{DerivedTopologyReadBasis, MutationOrigin};
use serde_json::Value;

pub(in crate::certification::projection_closeout::tests) fn seeded_sheet_disk_workspace(
    stem: &str,
) -> (
    ForgeQueryWorkspace,
    TopologyQueryAssembly,
    schema::facade::DerivedTopologyReadBasis,
) {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        stem,
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, stem).expect("workspace should build");
    let assembly =
        TopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    (workspace, assembly, verified.read_basis)
}

pub(in crate::certification::projection_closeout::tests) fn default_query_mutation_evidence(
    touched_aspect_paths: impl IntoIterator<Item = String>,
) -> TopologyQueryMutationEvidence {
    TopologyQueryMutationEvidence {
        authority_snapshot_id: 7,
        authority_branch_id: ".query.main".to_string(),
        authoritative_mutation_origin: MutationOrigin::LocalEdit,
        derivation_origin: MutationOrigin::LocalEdit,
        truth_basis_digest_hex: "query-topology-test-basis".to_string(),
        touched_aspect_paths: touched_aspect_paths.into_iter().collect(),
        precision_fallback_count: 0,
        precision_budget_fallback_count: 0,
    }
}

pub(in crate::certification::projection_closeout::tests) fn snapshot_basis_workspace(
    runtime: &RelationalRuntime,
    stem: &str,
    read_basis: &DerivedTopologyReadBasis,
) -> (ForgeQueryWorkspace, TopologyQueryAssembly) {
    let read_view =
        open_topology_read_view(runtime, read_basis).expect("snapshot read view should open");
    let adapters =
        TopologyRuntimeAdapters::snapshot_read_only(read_view, read_basis.snapshot().clone());
    let mut workspace = topology_runtime(adapters, stem).expect("workspace should build");
    let assembly =
        TopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    (workspace, assembly)
}

pub(in crate::certification::projection_closeout::tests) struct QueryLookupRows {
    entity_rows: Vec<ForgeQueryEntity>,
    relation_rows: Vec<ForgeQueryEntity>,
}

impl QueryLookupRows {
    pub(in crate::certification::projection_closeout::tests) fn lookup(
        &self,
    ) -> TopologyQueryRowLookup<'_> {
        TopologyQueryRowLookup::new(&self.entity_rows, &self.relation_rows)
    }
}

pub(in crate::certification::projection_closeout::tests) fn current_lookup_rows(
    workspace: &ForgeQueryWorkspace,
    assembly: &TopologyQueryAssembly,
) -> QueryLookupRows {
    QueryLookupRows {
        entity_rows: workspace.read::<Value>(assembly.entities()),
        relation_rows: workspace.read::<Value>(assembly.relations()),
    }
}
