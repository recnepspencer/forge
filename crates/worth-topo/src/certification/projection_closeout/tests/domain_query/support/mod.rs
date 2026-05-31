use crate::facade::TopologyQueryMutationEvidence;
use crate::facade::{
    topology_current_head_authoritative_context, topology_query_domain_entry, topology_runtime,
    topology_snapshot_read_only_context, TopologyCurrentHeadConfiguredDomainHandle,
    TopologyRuntimeAdapters, TopologySnapshotReadOnlyConfiguredDomainHandle,
};
use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::projection::runtime_boundary::query_runtime::TopologyRuntimeSupport;
use crate::projection::runtime_boundary::read_stage::open_topology_read_view;
use crate::projection::TopologyQueryRowLookup;
use crate::topology_operators::application::{
    TopologyDeclarationContractPayload, TopologyOperatorRunner,
};
use crate::topology_operators::{
    TopologyCreateInnerLoopOnExistingFaceDeclaration, TopologyCreateTopologyEntityDeclaration,
    TopologyDetachBoundaryMembershipDeclaration, TopologyDetachRadialAdjacencyDeclaration,
    TopologyDetachShellOrWireMembershipDeclaration, TopologyEditFamily, TopologyOperatorExecution,
    TopologyOperatorExecutionError, TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration, TopologyRetireTopologyEntityDeclaration,
    TopologyRewireLoopEndpointDeclaration, TopologyRewireLoopSuccessorProgramDeclaration,
    TopologySpliceRadialAdjacencyDeclaration, TopologySpliceRadialAdjacencyProgramDeclaration,
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
};
use crate::validation::reference_integrity::milestone_one_runtime_builder;
use forge_query::facade::ForgeQueryApplicationFacade;
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

pub(in crate::certification::projection_closeout::tests) fn current_operator_bindings(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> TopologyQueryBindingIndex {
    TopologyQueryBindingIndex::from_query_rows(
        &workspace.read(surfaces.entities()),
        &workspace.read(surfaces.relations()),
    )
    .expect("current-head operator bindings should decode")
}

pub(in crate::certification::projection_closeout::tests) fn current_head_unsupported_declaration_families<
    D,
>(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
    declaration: &D,
) -> Vec<TopologyEditFamily>
where
    D: TopologyDeclarationContractPayload,
{
    let bindings = current_operator_bindings(workspace, surfaces);
    let support = TopologyRuntimeSupport::current_head_authoritative();
    crate::topology_operators::application::admission::unsupported_declaration_families(
        &support,
        &bindings,
        declaration,
    )
}

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

pub(in crate::certification::projection_closeout::tests) fn execute_current_head_topology_declaration<
    D,
>(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
    declaration: D,
) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError>
where
    D: TopologyCurrentHeadRuntimeDeclaration,
{
    let bindings = current_operator_bindings(workspace, surfaces);
    let mut runner = TopologyOperatorRunner::new(workspace, surfaces);
    declaration.execute_on_runner(&mut runner, &bindings)
}

pub(in crate::certification::projection_closeout::tests) trait TopologyCurrentHeadRuntimeDeclaration:
    Clone + TopologyDeclarationContractPayload
{
    fn execute_on_runner(
        self,
        runner: &mut TopologyOperatorRunner<'_, '_>,
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError>;
}

impl TopologyCurrentHeadRuntimeDeclaration for TopologyCreateTopologyEntityDeclaration {
    fn execute_on_runner(
        self,
        runner: &mut TopologyOperatorRunner<'_, '_>,
        _bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
        runner.apply_create_topology_entity_declaration(
            self,
            crate::facade::TopologyEditApplicationMode::Mainline,
        )
    }
}

impl TopologyCurrentHeadRuntimeDeclaration for TopologyCreateInnerLoopOnExistingFaceDeclaration {
    fn execute_on_runner(
        self,
        runner: &mut TopologyOperatorRunner<'_, '_>,
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
        runner.apply_create_inner_loop_on_existing_face_declaration(
            self,
            bindings,
            crate::facade::TopologyEditApplicationMode::Mainline,
        )
    }
}

macro_rules! impl_single_family_runtime_declaration {
    ($ty:ty, $method:ident) => {
        impl TopologyCurrentHeadRuntimeDeclaration for $ty {
            fn execute_on_runner(
                self,
                runner: &mut TopologyOperatorRunner<'_, '_>,
                bindings: &TopologyQueryBindingIndex,
            ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
                runner.$method(
                    self,
                    bindings,
                    crate::facade::TopologyEditApplicationMode::Mainline,
                )
            }
        }
    };
}

impl_single_family_runtime_declaration!(
    TopologyRetireTopologyEntityDeclaration,
    apply_retire_topology_entity_declaration
);
impl_single_family_runtime_declaration!(
    TopologyDetachBoundaryMembershipDeclaration,
    apply_detach_boundary_membership_declaration
);
impl_single_family_runtime_declaration!(
    TopologyRewireLoopEndpointDeclaration,
    apply_rewire_loop_endpoint_declaration
);
impl_single_family_runtime_declaration!(
    TopologyDetachShellOrWireMembershipDeclaration,
    apply_detach_shell_or_wire_membership_declaration
);
impl_single_family_runtime_declaration!(
    TopologySpliceRadialAdjacencyDeclaration,
    apply_splice_radial_adjacency_declaration
);
impl_single_family_runtime_declaration!(
    TopologyDetachRadialAdjacencyDeclaration,
    apply_detach_radial_adjacency_declaration
);

macro_rules! impl_grouped_runtime_declaration {
    ($ty:ty, $method:ident) => {
        impl TopologyCurrentHeadRuntimeDeclaration for $ty {
            fn execute_on_runner(
                self,
                runner: &mut TopologyOperatorRunner<'_, '_>,
                bindings: &TopologyQueryBindingIndex,
            ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
                runner.$method(
                    self,
                    bindings,
                    crate::facade::TopologyEditApplicationMode::Mainline,
                )
            }
        }
    };
}

impl_grouped_runtime_declaration!(
    TopologyRewireLoopSuccessorProgramDeclaration,
    apply_rewire_loop_successor_program_declaration
);
impl_grouped_runtime_declaration!(
    TopologySpliceRadialAdjacencyProgramDeclaration,
    apply_splice_radial_adjacency_program_declaration
);
impl_grouped_runtime_declaration!(
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    apply_rehome_all_owned_half_edges_to_new_wire_declaration
);
impl_grouped_runtime_declaration!(
    TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    apply_rehome_all_owned_faces_to_new_shell_declaration
);
impl_grouped_runtime_declaration!(
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    apply_split_connected_half_edge_set_to_new_wire_declaration
);
impl_grouped_runtime_declaration!(
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    apply_split_single_face_from_two_face_shell_to_new_shell_declaration
);
