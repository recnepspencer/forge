use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::projection::runtime_boundary::query_runtime::TopologyRuntimeSupport;
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
use forge_query::facade::ForgeQueryWorkspace;

pub(super) fn execute_current_head_topology_declaration<D>(
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

pub(super) fn current_operator_bindings(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> TopologyQueryBindingIndex {
    TopologyQueryBindingIndex::from_query_rows(
        &workspace.read(surfaces.entities()),
        &workspace.read(surfaces.relations()),
    )
    .expect("current-head operator bindings should decode")
}

pub(super) fn current_head_unsupported_declaration_families<D>(
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

pub(super) trait TopologyCurrentHeadRuntimeDeclaration:
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
