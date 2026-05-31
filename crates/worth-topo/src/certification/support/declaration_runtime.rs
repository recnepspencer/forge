use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::{
    TopologyDeclarationMutationPayload, TopologyDeclaredMutationArtifact,
    TopologyMutationApplicationError, TopologyMutationApplicationRunner,
};
#[cfg(test)]
use crate::topology_operators::TopologyMutationFamily;
use crate::topology_operators::{
    TopologyCreateInnerLoopOnExistingFaceDeclaration, TopologyCreateTopologyEntityDeclaration,
    TopologyDetachBoundaryMembershipDeclaration, TopologyDetachRadialAdjacencyDeclaration,
    TopologyDetachShellOrWireMembershipDeclaration,
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
) -> Result<TopologyQueryBindingIndex, TopologyMutationApplicationError> {
    TopologyQueryBindingIndex::from_query_rows(
        &workspace.read(surfaces.entities()),
        &workspace.read(surfaces.relations()),
    )
}

pub(crate) fn execute_current_head_topology_declaration<D>(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
    declaration: D,
) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError>
where
    D: TopologyCurrentHeadRuntimeDeclaration,
{
    let bindings = current_operator_bindings(workspace, surfaces)?;
    let mut runner = TopologyMutationApplicationRunner::new(workspace, surfaces);
    declaration.execute_on_runner(&mut runner, &bindings)
}

#[cfg(test)]
pub(crate) fn current_head_unsupported_declaration_families<D>(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
    declaration: &D,
) -> Vec<TopologyMutationFamily>
where
    D: TopologyDeclarationMutationPayload,
{
    let bindings = current_operator_bindings(workspace, surfaces)
        .expect("current-head unsupported-family review should decode");
    let support = TopologyRuntimeSupport::current_head_authoritative();
    unsupported_declaration_families(&support, &bindings, declaration)
}

pub(crate) trait TopologyCurrentHeadRuntimeDeclaration:
    Clone + TopologyDeclarationMutationPayload
{
    fn execute_on_runner(
        self,
        runner: &mut TopologyMutationApplicationRunner<'_, '_>,
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError>;
}

impl TopologyCurrentHeadRuntimeDeclaration for TopologyCreateInnerLoopOnExistingFaceDeclaration {
    fn execute_on_runner(
        self,
        runner: &mut TopologyMutationApplicationRunner<'_, '_>,
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
        runner.apply_create_inner_loop_on_existing_face_declaration(
            self,
            bindings,
            crate::facade::TopologyMutationApplicationMode::Mainline,
        )
    }
}

macro_rules! impl_single_family_runtime_declaration {
    ($ty:ty, $method:ident) => {
        impl TopologyCurrentHeadRuntimeDeclaration for $ty {
            fn execute_on_runner(
                self,
                runner: &mut TopologyMutationApplicationRunner<'_, '_>,
                bindings: &TopologyQueryBindingIndex,
            ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
                runner.$method(
                    self,
                    bindings,
                    crate::facade::TopologyMutationApplicationMode::Mainline,
                )
            }
        }
    };
}

impl_single_family_runtime_declaration!(
    TopologyDetachBoundaryMembershipDeclaration,
    apply_detach_boundary_membership_declaration
);
impl_single_family_runtime_declaration!(
    TopologyDetachShellOrWireMembershipDeclaration,
    apply_detach_shell_or_wire_membership_declaration
);
impl_single_family_runtime_declaration!(
    TopologyDetachRadialAdjacencyDeclaration,
    apply_detach_radial_adjacency_declaration
);
impl_single_family_runtime_declaration!(
    TopologyRetireTopologyEntityDeclaration,
    apply_retire_topology_entity_declaration
);
impl_single_family_runtime_declaration!(
    TopologyRewireLoopEndpointDeclaration,
    apply_rewire_loop_endpoint_declaration
);
impl_single_family_runtime_declaration!(
    TopologySpliceRadialAdjacencyDeclaration,
    apply_splice_radial_adjacency_declaration
);

impl TopologyCurrentHeadRuntimeDeclaration for TopologyCreateTopologyEntityDeclaration {
    fn execute_on_runner(
        self,
        runner: &mut TopologyMutationApplicationRunner<'_, '_>,
        _bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
        runner.apply_create_topology_entity_declaration(
            self,
            crate::facade::TopologyMutationApplicationMode::Mainline,
        )
    }
}

macro_rules! impl_grouped_runtime_declaration {
    ($ty:ty, $method:ident) => {
        impl TopologyCurrentHeadRuntimeDeclaration for $ty {
            fn execute_on_runner(
                self,
                runner: &mut TopologyMutationApplicationRunner<'_, '_>,
                bindings: &TopologyQueryBindingIndex,
            ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
                runner.$method(
                    self,
                    bindings,
                    crate::facade::TopologyMutationApplicationMode::Mainline,
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
#[cfg(test)]
use crate::projection::runtime_boundary::query_runtime::TopologyRuntimeSupport;
#[cfg(test)]
use crate::topology_operators::application::admission::unsupported_declaration_families;
