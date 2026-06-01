use crate::facade::TopologyQueryMutationEvidence;
use crate::facade::{topology_runtime, TopologyRuntimeAdapters};
use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::runtime_boundary::read_stage::open_topology_read_view;
use crate::projection::TopologyQueryRowLookup;
<<<<<<< HEAD:crates/worth-topo/src/certification/projection_closeout/tests/domain_query/support.rs
use crate::validation::reference_integrity::milestone_one_runtime_builder;
=======
use crate::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain_entry,
    topology_snapshot_read_only_context, TopologyCurrentHeadConfiguredDomainHandle,
    TopologySnapshotReadOnlyConfiguredDomainHandle,
};
use crate::validation::reference_integrity::milestone_one_runtime_builder;
use forge_query::facade::ForgeQueryApplicationFacade;
>>>>>>> origin/master:crates/worth-topo/src/certification/projection_closeout/tests/topology_reads/support/mod.rs
use forge_query::facade::ForgeQueryEntity;
use forge_query::facade::ForgeQueryWorkspace;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::authority::MutationOrigin;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use serde_json::Value;

pub(in crate::certification::projection_closeout::tests) fn seeded_sheet_disk_workspace(
    stem: &str,
) -> (
    ForgeQueryWorkspace,
    TopologyDeclaredQuerySurfaces,
    DerivedTopologyReadBasis,
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
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("query surfaces should declare");
    (workspace, surfaces, verified.read_basis().clone())
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
) -> (ForgeQueryWorkspace, TopologyDeclaredQuerySurfaces) {
    let read_view =
        open_topology_read_view(runtime, read_basis).expect("snapshot read view should open");
    let adapters =
        TopologyRuntimeAdapters::snapshot_read_only(read_view, read_basis.snapshot().clone());
    let mut workspace = topology_runtime(adapters, stem).expect("workspace should build");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("query surfaces should declare");
    (workspace, surfaces)
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
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> QueryLookupRows {
    QueryLookupRows {
        entity_rows: workspace.read::<Value>(surfaces.entities()),
        relation_rows: workspace.read::<Value>(surfaces.relations()),
    }
}
<<<<<<< HEAD:crates/worth-topo/src/certification/projection_closeout/tests/domain_query/support.rs
=======

pub(in crate::certification::projection_closeout::tests) fn current_head_query_handle(
) -> TopologyCurrentHeadConfiguredDomainHandle {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    topology_query_domain_entry(&facade)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .expect("current-head context should validate")
        .admit()
        .expect("current-head context should admit")
}

pub(in crate::certification::projection_closeout::tests) fn snapshot_query_handle(
) -> TopologySnapshotReadOnlyConfiguredDomainHandle {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    topology_query_domain_entry(&facade)
        .with_operating_context(topology_snapshot_read_only_context())
        .validate()
        .expect("snapshot context should validate")
        .admit()
        .expect("snapshot context should admit")
}
>>>>>>> origin/master:crates/worth-topo/src/certification/projection_closeout/tests/topology_reads/support/mod.rs
