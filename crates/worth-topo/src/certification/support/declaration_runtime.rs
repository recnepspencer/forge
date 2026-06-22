use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::{
    TopologyDeclarationMutationPayload, TopologyDeclaredMutationArtifact,
    TopologyMutationApplicationError, TopologyMutationApplicationOutcome,
    TopologyMutationApplicationRunner,
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

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn execute_current_head_topology_declaration_outcome<D>(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
    declaration: D,
) -> TopologyMutationApplicationOutcome
where
    D: TopologyCurrentHeadRuntimeDeclaration,
{
    TopologyMutationApplicationOutcome::from_result(execute_current_head_topology_declaration(
        workspace,
        surfaces,
        declaration,
    ))
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

#[cfg(test)]
mod tests {
    use forge_query::facade::{ForgeQueryApplicationFacade, ForgeQueryDeclarationFamilyMarker};
    use schema::facade::platform::entities::TopologyEntityKind;

    use super::{
        execute_current_head_topology_declaration,
        execute_current_head_topology_declaration_outcome,
    };
    use crate::topology_operators::application::{
        TopologyDeclarationEntryStopClass, TopologyDeclarationMutationPayload,
        TopologyDeclaredMutationArtifact, TopologyMutationApplicationError,
        TopologyMutationApplicationOutcome, TopologyRetainedApplicationHandoff,
    };
    use crate::topology_operators::{
        TopologyCreateTopologyEntityDeclaration, TopologyCreateTopologyEntityFamily,
        TopologyOperatorWorkflowHandleExt,
    };

    #[test]
    fn current_head_execution_outcome_projects_success_without_losing_artifact() {
        let runtime =
            crate::validation::reference_integrity::build_milestone_one_runtime().expect("runtime");
        let adapters = crate::facade::TopologyRuntimeAdapters::current_head(runtime);
        let mut workspace =
            crate::facade::topology_runtime(adapters, "phase3.execution-outcome.success")
                .expect("workspace");
        let surfaces = crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");

        let outcome = execute_current_head_topology_declaration_outcome(
            &mut workspace,
            &surfaces,
            TopologyCreateTopologyEntityDeclaration::new(
                "phase3.execution-outcome.vertex",
                TopologyEntityKind::Vertex,
            ),
        );

        match outcome {
            TopologyMutationApplicationOutcome::Applied(artifact) => {
                let synopsis = artifact.accepted_mutation_projection();
                let semantic_projection = artifact.accepted_mutation_projection();
                assert_eq!(
                    synopsis.semantic_family_key(),
                    "topology.create_topology_entity"
                );
                assert_eq!(
                    synopsis.mutation_families(),
                    &[crate::topology_operators::TopologyMutationFamily::CreateTopologyEntity]
                );
                assert_eq!(synopsis.topology_mutation_digest().mutation_record_count, 1);
                assert_eq!(synopsis.topology_mutation_digest().family_count, 1);
                assert_eq!(
                    semantic_projection
                        .naming_mutation_continuity_matrix()
                        .preserved_count,
                    1
                );
                assert_eq!(
                    semantic_projection
                        .naming_mutation_continuity_matrix()
                        .rows
                        .len(),
                    1
                );
                assert_eq!(
                    semantic_projection.derived_fallback_policy(),
                    crate::topology_operators::TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback
                );
            }
            _ => panic!("expected applied execution outcome"),
        }
    }

    #[test]
    fn declaration_entry_errors_project_to_stopped_execution_outcomes() {
        let outcome = TopologyMutationApplicationOutcome::from_result(Err(
            TopologyMutationApplicationError::DeclarationEntry {
                family: crate::topology_operators::TopologyMutationFamily::CreateTopologyEntity,
                stop_class: TopologyDeclarationEntryStopClass::RebindRequired,
                stop_stage: None,
                refusal_class: None,
                recovery: None,
                graph_obligation_envelope_digest: None,
                reason: "snapshot contexts require explicit rebind".to_string(),
            },
        ));

        match outcome {
            TopologyMutationApplicationOutcome::Stopped(stop) => {
                assert_eq!(
                    stop.stop_class(),
                    Some(TopologyDeclarationEntryStopClass::RebindRequired)
                );
                assert!(stop.recovery().is_none());
                assert!(stop.graph_obligation_envelope_digest().is_none());
            }
            _ => panic!("expected declaration-entry stop outcome"),
        }
    }

    #[test]
    fn retained_query_semantic_aftermath_mismatch_fails_closed_before_projection() {
        let declaration = TopologyCreateTopologyEntityDeclaration::new(
            "semantic-aftermath.vertex",
            TopologyEntityKind::Vertex,
        );
        let second_declaration = TopologyCreateTopologyEntityDeclaration::new(
            "semantic-aftermath.vertex.successor",
            TopologyEntityKind::Vertex,
        );
        let facade = ForgeQueryApplicationFacade::runtime_backed_default();
        let handle = crate::query_domain::topology_query_domain_entry(&facade)
            .with_operating_context(
                crate::query_domain::topology_current_head_authoritative_context(),
            )
            .validate()
            .expect("current-head context should validate")
            .admit()
            .expect("current-head context should admit");
        let handoff = TopologyRetainedApplicationHandoff::new(
            handle
                .orchestrate_topology_operator_with_contributions(
                    crate::topology_operators::topology_operator_contribution_workflow(
                        declaration.clone(),
                    ),
                )
                .unwrap_or_else(|_| panic!("topology contribution-composed lane should admit")),
            crate::topology_operators::TopologyDeclaredTouchedGraphBasis::from_sequence(
                TopologyCreateTopologyEntityFamily::semantic_family_key(),
                declaration.clone(),
                &declaration.clone().into_mutation_sequence(),
                crate::topology_operators::TopologyTouchedOperatingWorld::mainline(),
            )
            .expect("declared touched graph basis"),
        );
        let runtime =
            crate::validation::reference_integrity::build_milestone_one_runtime().expect("runtime");
        let adapters = crate::facade::TopologyRuntimeAdapters::current_head(runtime);
        let mut workspace =
            crate::facade::topology_runtime(adapters, "semantic-aftermath.mismatch")
                .expect("workspace");
        let surfaces = crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
        let execution = execute_current_head_topology_declaration(
            &mut workspace,
            &surfaces,
            declaration.clone(),
        )
        .expect("declaration should execute through current-head runtime");
        assert!(
            !execution.declared_touched_basis().basis_digest().is_empty(),
            "executed topology declaration must retain declared touched-basis proof"
        );
        let mismatched_sequence =
            crate::topology_operators::TopologyDeclaredMutationSequence::concatenate([
                declaration.declared_mutation_sequence(),
                second_declaration.declared_mutation_sequence(),
            ]);

        assert!(matches!(
            TopologyDeclaredMutationArtifact::from_receipt(
                TopologyCreateTopologyEntityFamily::semantic_family_key(),
                &handoff,
                &mismatched_sequence,
                execution.post_write_query_artifact_for_test(),
            ),
            Err(
                TopologyMutationApplicationError::RetainedSemanticAftermathMismatch {
                    semantic_family_key: "topology.create_topology_entity",
                    ..
                }
            )
        ));
    }
}
