use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::{
    TopologyDeclarationContractPayload, TopologyDeclaredMutationArtifact, TopologyOperatorRunner,
};
use crate::topology_operators::{
    TopologyCreateInnerLoopOnExistingFaceDeclaration, TopologyCreateTopologyEntityDeclaration,
    TopologyDetachBoundaryMembershipDeclaration, TopologyDetachRadialAdjacencyDeclaration,
    TopologyDetachShellOrWireMembershipDeclaration, TopologyOperatorExecutionError,
    TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration, TopologyRetireTopologyEntityDeclaration,
    TopologyRewireLoopEndpointDeclaration, TopologyRewireLoopSuccessorProgramDeclaration,
    TopologySpliceRadialAdjacencyDeclaration, TopologySpliceRadialAdjacencyProgramDeclaration,
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
};
use forge_query::facade::ForgeQueryWorkspace;

pub(crate) fn current_operator_bindings(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> Result<TopologyQueryBindingIndex, TopologyOperatorExecutionError> {
    TopologyQueryBindingIndex::from_query_rows(
        &workspace.read(surfaces.entities()),
        &workspace.read(surfaces.relations()),
    )
}

pub(crate) fn execute_current_head_topology_declaration<D>(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
    declaration: D,
) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError>
where
    D: TopologyCurrentHeadRuntimeDeclaration,
{
    let bindings = current_operator_bindings(workspace, surfaces)?;
    let mut runner = TopologyOperatorRunner::new(workspace, surfaces);
    declaration.execute_on_runner(&mut runner, &bindings)
}

pub(crate) trait TopologyCurrentHeadRuntimeDeclaration:
    Clone + TopologyDeclarationContractPayload
{
    fn execute_on_runner(
        self,
        runner: &mut TopologyOperatorRunner<'_, '_>,
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError>;
}

impl TopologyCurrentHeadRuntimeDeclaration for TopologyCreateInnerLoopOnExistingFaceDeclaration {
    fn execute_on_runner(
        self,
        runner: &mut TopologyOperatorRunner<'_, '_>,
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError> {
        runner.apply_create_inner_loop_on_existing_face_declaration(
            self,
            bindings,
            crate::facade::TopologyEditApplicationMode::Mainline,
        )
    }
}

macro_rules! impl_single_family_runtime_declaration {
    ($ty:ty, $family:path, $method:ident) => {
        impl TopologyCurrentHeadRuntimeDeclaration for $ty {
            fn execute_on_runner(
                self,
                runner: &mut TopologyOperatorRunner<'_, '_>,
                bindings: &TopologyQueryBindingIndex,
            ) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError> {
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
    TopologyDetachBoundaryMembershipDeclaration,
    TopologyEditFamily::DetachBoundaryMembership,
    apply_detach_boundary_membership_declaration
);
impl_single_family_runtime_declaration!(
    TopologyDetachShellOrWireMembershipDeclaration,
    TopologyEditFamily::DetachShellOrWireMembership,
    apply_detach_shell_or_wire_membership_declaration
);
impl_single_family_runtime_declaration!(
    TopologyDetachRadialAdjacencyDeclaration,
    TopologyEditFamily::DetachRadialAdjacency,
    apply_detach_radial_adjacency_declaration
);
impl_single_family_runtime_declaration!(
    TopologyRetireTopologyEntityDeclaration,
    TopologyEditFamily::RetireTopologyEntity,
    apply_retire_topology_entity_declaration
);
impl_single_family_runtime_declaration!(
    TopologyRewireLoopEndpointDeclaration,
    TopologyEditFamily::RewireLoopEndpoint,
    apply_rewire_loop_endpoint_declaration
);
impl_single_family_runtime_declaration!(
    TopologySpliceRadialAdjacencyDeclaration,
    TopologyEditFamily::SpliceRadialAdjacency,
    apply_splice_radial_adjacency_declaration
);

impl TopologyCurrentHeadRuntimeDeclaration for TopologyCreateTopologyEntityDeclaration {
    fn execute_on_runner(
        self,
        runner: &mut TopologyOperatorRunner<'_, '_>,
        _bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError> {
        runner.apply_create_topology_entity_declaration(
            self,
            crate::facade::TopologyEditApplicationMode::Mainline,
        )
    }
}

macro_rules! impl_grouped_runtime_declaration {
    ($ty:ty, $method:ident) => {
        impl TopologyCurrentHeadRuntimeDeclaration for $ty {
            fn execute_on_runner(
                self,
                runner: &mut TopologyOperatorRunner<'_, '_>,
                bindings: &TopologyQueryBindingIndex,
            ) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError> {
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
    TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    apply_rehome_all_owned_faces_to_new_shell_declaration
);
impl_grouped_runtime_declaration!(
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    apply_rehome_all_owned_half_edges_to_new_wire_declaration
);
impl_grouped_runtime_declaration!(
    TopologyRewireLoopSuccessorProgramDeclaration,
    apply_rewire_loop_successor_program_declaration
);
impl_grouped_runtime_declaration!(
    TopologySpliceRadialAdjacencyProgramDeclaration,
    apply_splice_radial_adjacency_program_declaration
);
impl_grouped_runtime_declaration!(
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    apply_split_connected_half_edge_set_to_new_wire_declaration
);
impl_grouped_runtime_declaration!(
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    apply_split_single_face_from_two_face_shell_to_new_shell_declaration
);
