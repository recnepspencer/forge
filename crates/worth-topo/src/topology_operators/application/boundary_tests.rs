#[test]
fn application_operator_files_do_not_decode_raw_query_payloads() {
    let application_mod = include_str!("mod.rs");
    let admission = include_str!("admission.rs");
    let bindings = include_str!("bindings.rs");
    let existing_truth = include_str!("existing_truth.rs");

    for source in [application_mod, admission, bindings, existing_truth] {
        assert!(
            !source.contains(".payload"),
            "operator application surface should not decode raw query payloads",
        );
    }
}

#[test]
fn application_operator_files_do_not_own_post_write_materialized_decode() {
    let application_mod = include_str!("mod.rs");
    let bindings = include_str!("bindings.rs");
    let execution_finalize = include_str!("declaration_entry/execution_finalize.rs");
    let operator_post_write =
        include_str!("../../projection/runtime_boundary/query_runtime/operator_post_write.rs");
    let composed_successor_program =
        include_str!("../local_rewrites/boundary_wiring/composed_successor_program.rs");
    let composed_membership =
        include_str!("../local_rewrites/sheet_wire_laminar/membership_programs/mod.rs");

    for source in [
        application_mod,
        bindings,
        execution_finalize,
        composed_successor_program,
        composed_membership,
    ] {
        assert!(
            !source.contains("workspace.materialize("),
            "operator application surface should not own post-write materialized reads",
        );
        assert!(
            !source.contains("serde_json::from_value"),
            "operator application surface should not deserialize materialized rows directly",
        );
        assert!(
            !source.contains("workspace.inspect("),
            "operator application surface should not inspect post-write receipts directly",
        );
        assert!(
            !source.contains("load_post_write_materialized_topology("),
            "operator application surface should delegate post-write materialized loading to query runtime support",
        );
    }

    for source in [
        execution_finalize,
        composed_successor_program,
        composed_membership,
    ] {
        assert!(
            !source.contains("TopologyDeclaredMutationArtifact::from_receipt("),
            "local execution files should delegate declared-mutation artifact closeout to the shared post-write helper",
        );
        assert!(
            !source.contains("TopologyPostWriteQueryArtifact::build("),
            "local execution files should delegate post-write query artifact assembly to the shared post-write helper",
        );
    }

    assert!(
        application_mod.contains("TopologyDeclaredMutationArtifact::from_receipt("),
        "shared post-write closeout helper should own declared-mutation artifact assembly",
    );
    assert!(
        application_mod.contains("TopologyPostWriteQueryArtifact::build("),
        "shared post-write closeout helper should own post-write query artifact assembly",
    );
    assert!(
        operator_post_write.contains("materialize_batch_write_artifact_binding("),
        "shared post-write helper should cross the Query-owned post-write retained artifact boundary instead of rebuilding inspection and retained materialization locally",
    );
    assert!(
        !operator_post_write.contains("workspace.materialize(")
            && !operator_post_write.contains("workspace.inspect("),
        "shared post-write helper should not keep direct workspace inspection/materialization archaeology once Query owns the post-write retained artifact seam",
    );
}

#[test]
fn query_anchor_family_mismatch_fails_before_local_artifact_projection() {
    use forge_query::facade::{ForgeQueryApplicationFacade, ForgeQueryDeclarationFamilyMarker};
    use schema::facade::platform::entities::TopologyEntityKind;

    use crate::topology_operators::application::TopologyDeclarationMutationPayload;
    use crate::topology_operators::application::{
        TopologyMutationApplicationError, TopologyOperatorApplicationQueryAnchor,
        TopologyRetainedApplicationHandoff,
    };
    use crate::topology_operators::{
        TopologyCreateTopologyEntityDeclaration, TopologyCreateTopologyEntityFamily,
        TopologyDeclaredTouchedGraphBasis, TopologyOperatorWorkflowHandleExt,
        TopologyTouchedOperatingWorld,
    };

    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "anchor-mismatch.vertex",
        TopologyEntityKind::Vertex,
    );
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = crate::query_domain::topology_query_domain_entry(&facade)
        .with_operating_context(crate::query_domain::topology_current_head_authoritative_context())
        .validate()
        .expect("current-head context should validate")
        .admit()
        .expect("current-head context should admit");
    let declared_touched_basis = TopologyDeclaredTouchedGraphBasis::from_sequence(
        TopologyCreateTopologyEntityFamily::semantic_family_key(),
        declaration.clone(),
        &declaration.clone().into_mutation_sequence(),
        TopologyTouchedOperatingWorld::mainline(),
    )
    .expect("declared touched graph basis");
    let handoff = TopologyRetainedApplicationHandoff::new(
        handle
            .orchestrate_topology_operator_with_contributions(
                crate::topology_operators::topology_operator_contribution_workflow(declaration),
            )
            .unwrap_or_else(|_| panic!("topology contribution-composed lane should admit")),
        declared_touched_basis,
    );
    let anchor = TopologyOperatorApplicationQueryAnchor::from_retained_handoff(&handoff);
    let wrong_anchor =
        TopologyOperatorApplicationQueryAnchor::with_family_for_test("topology.other", &anchor);

    assert!(matches!(
        wrong_anchor
            .ensure_semantic_family(TopologyCreateTopologyEntityFamily::semantic_family_key()),
        Err(
            TopologyMutationApplicationError::QueryAnchorFamilyMismatch {
                semantic_family_key: "topology.create_topology_entity",
                query_declaration_family_key: "topology.other",
            }
        )
    ));
}

#[test]
fn retained_application_handoff_does_not_cache_a_parallel_semantic_aftermath_field() {
    let application_mod = include_str!("mod.rs").replace("\r\n", "\n");
    let retained_handoff =
        include_str!("declaration_entry/retained_application_handoff.rs").replace("\r\n", "\n");
    let declared_mutation_artifact =
        include_str!("declared_mutation_artifact.rs").replace("\r\n", "\n");
    let accepted_mutation_projection =
        include_str!("declared_mutation_artifact/accepted_mutation_projection.rs")
            .replace("\r\n", "\n");
    let workflow_artifacts =
        include_str!("../query_workflow/workflow_artifacts.rs").replace("\r\n", "\n");

    assert!(
        !retained_handoff.contains("semantic_aftermath: TopologyQuerySemanticAftermathEvidence"),
        "retained application handoff should derive semantic aftermath from the retained Query contribution artifact instead of caching a parallel topo-owned sidecar",
    );
    assert!(
        retained_handoff.contains("fn retain_accepted_query_contribution_semantic_projection("),
        "retained application handoff should expose a named validated semantic projection seam over the retained Query contribution artifact",
    );
    assert!(
        retained_handoff.contains("fn contribution_digest(&self)"),
        "retained application handoff should expose contribution digest directly from the retained Query contribution artifact rather than smuggling it through semantic aftermath evidence",
    );
    assert!(
        !application_mod.contains("TopologyOperatorRetainedSemanticEvidence"),
        "application facade should not re-export a topo-owned retained semantic wrapper once the retained handoff keeps the validated topology semantic projection seam directly",
    );
    assert!(
        declared_mutation_artifact.contains("accepted_mutation_projection: TopologyAcceptedMutationProjection"),
        "declared mutation artifact should keep one explicit accepted mutation projection seam instead of splitting live committed-artifact truth across separate synopsis and semantic projection payloads",
    );
    assert!(
        !declared_mutation_artifact.contains(
            "pub(crate) struct TopologyDeclaredMutationArtifact {\n    semantic_family_key:"
        ),
        "declared mutation artifact should not keep semantic family identity as a free-floating live field once that identity belongs to the declaration synopsis boundary",
    );
    assert!(
        declared_mutation_artifact.contains(
            "pub(crate) fn accepted_mutation_projection(&self) -> &TopologyAcceptedMutationProjection"
        ),
        "declared mutation artifact should expose one explicit accepted mutation projection seam for certification and closeout consumers in the normal library build",
    );
    assert!(
        !declared_mutation_artifact.contains("fn accepted_query_contribution_semantic_projection(")
            && !declared_mutation_artifact.contains("fn declared_mutation_synopsis("),
        "declared mutation artifact should not keep separate live synopsis and semantic-projection accessors once one accepted mutation projection seam exists",
    );
    assert!(
        declared_mutation_artifact.contains("#[cfg(test)]\n    pub(crate) fn receipt(")
            && declared_mutation_artifact.contains("#[cfg(test)]\n    pub(crate) fn inspection("),
        "declared mutation artifact should keep generic receipt and inspection behind test-only proof seams instead of exporting them as the live operator product surface",
    );
    assert!(
        declared_mutation_artifact.contains(
            "declared_touched_basis: TopologyDeclaredTouchedGraphBasisProof"
        )
            && declared_mutation_artifact.contains(
                "pub(crate) fn declared_touched_basis(&self) -> &TopologyDeclaredTouchedGraphBasisProof"
            ),
        "declared mutation artifact should retain the declared touched-basis proof as the live topology authority product",
    );
    assert!(
        declared_mutation_artifact.contains("#[cfg(test)]\n    query_anchor: TopologyOperatorApplicationQueryAnchor")
            && declared_mutation_artifact.contains("#[cfg(test)]\n    pub(crate) fn query_anchor(")
            && declared_mutation_artifact.contains("#[cfg(test)]\n    pub(crate) fn execution_shape("),
        "declared mutation artifact should keep Query lineage and execution-shape proof payload behind test-only seams instead of retaining them as live committed-artifact contract",
    );
    assert!(
        !declared_mutation_artifact.contains("TopologyOperatorRetainedContributionComposition"),
        "declared mutation artifact should not retain raw Query contribution composition once the live committed-artifact contract has collapsed onto the validated topology semantic projection",
    );
    assert!(
        accepted_mutation_projection.contains("semantic_family_key: &'static str")
            && accepted_mutation_projection.contains(
                "naming_mutation_continuity_matrix: NamingMutationContinuityMatrix"
            ),
        "accepted mutation projection should own the remaining declaration and retained semantic summary in one explicit seam",
    );
    assert!(
        !declared_mutation_artifact.contains("TopologyOperatorRetainedSemanticEvidence"),
        "application artifact boundary should not retain a topo-owned retained semantic wrapper once Query contribution composition owns that seam",
    );
    assert!(
        workflow_artifacts.contains("type TopologyOperatorRetainedContributionComposition"),
        "query workflow should expose the retained Query contribution-composition seam once application stops minting a local retained semantic wrapper",
    );
    assert!(
        !workflow_artifacts.contains("fallback_explanation_detail:"),
        "retained Query semantic evidence should not store deterministic fallback explanation text once that detail can be re-derived from retained fallback policy",
    );
}

#[test]
fn retained_query_contribution_interpretation_flows_through_one_projection_seam() {
    let query_workflow_mod = include_str!("../query_workflow/mod.rs");
    let retained_contribution_semantics =
        include_str!("../query_workflow/retained_contribution_semantics.rs");
    let replay_step_rows =
        include_str!("../../certification/topology_operator_closeout/replay_step_rows.rs");
    let declaration_runtime = include_str!("../../certification/support/declaration_runtime.rs");
    let mutation_application_core = include_str!(
        "../../projection/runtime_boundary/query_runtime/tests/mutation_application/core.rs"
    );
    let scalar_runtime = include_str!(
        "../../certification/projection_closeout/tests/topology_reads/declaration_entry/scalar/runtime.rs"
    );
    let successor_runtime = include_str!(
        "../../certification/projection_closeout/tests/topology_reads/declaration_entry/successor_runtime.rs"
    );

    assert!(
        query_workflow_mod.contains("topology_retained_contribution_semantic_projection"),
        "query workflow boundary should expose one retained contribution semantic projection seam",
    );
    assert!(
        retained_contribution_semantics.contains(
            "struct TopologyRetainedContributionSemanticProjection"
        ),
        "retained contribution interpretation should live behind one topology-named projection product",
    );

    for source in [
        replay_step_rows,
        declaration_runtime,
        mutation_application_core,
        scalar_runtime,
        successor_runtime,
    ] {
        assert!(
            source.contains("accepted_mutation_projection()"),
            "proof-bearing consumers should read retained Query semantics through the artifact's single accepted mutation projection seam",
        );
    }
}

#[test]
fn phase_four_contribution_and_recovery_boundary_is_closeout_ready() {
    let contribution_builders =
        include_str!("../query_workflow/grouped_and_contribution_builders.rs");
    let orchestration_boundary = include_str!("declaration_entry/orchestration_boundary.rs");
    let retained_handoff = include_str!("declaration_entry/retained_application_handoff.rs");
    let declared_mutation_artifact = include_str!("declared_mutation_artifact.rs");
    let topology_operators_mod = include_str!("../mod.rs");
    let facade = include_str!("../../facade.rs");
    let compile_fail_contracts =
        include_str!("../../certification/public_facade_contracts/compile_fail_contracts.rs");

    assert!(
        contribution_builders.contains(".with_contributions(declaration.topology_semantic_contributions())"),
        "phase 4 remains incomplete if topology declaration entry can bypass Query contribution composition instead of seeding topology continuity and fallback on the contribution lane",
    );
    assert!(
        orchestration_boundary.contains(
            "ForgeQueryContributionComposedOrchestrationInput::new(declaration.clone())"
        ) && orchestration_boundary.contains(".orchestrate_declaration_with_contributions("),
        "phase 4 remains incomplete if the declaration-entry runtime seam can enter through bare declaration orchestration instead of the topology contribution-composed workflow lane",
    );
    assert!(
        retained_handoff.contains("contribution_artifact: TopologyOperatorContributionArtifact"),
        "phase 4 remains incomplete if the retained application handoff no longer keeps the admitted Query contribution artifact as the continuity and fallback carrier",
    );
    for forbidden in [
        "TopologyAcceptedMutationSemanticSummary",
        "TopologyOperatorRetainedSemanticEvidence",
        "TopologyQuerySemanticAftermathEvidence",
        "semantic_aftermath",
    ] {
        assert!(
            !declared_mutation_artifact.contains(forbidden),
            "phase 4 remains incomplete if declared-mutation artifacts retain local continuity/fallback/explanation sidecars such as `{forbidden}`",
        );
    }
    assert!(
        !topology_operators_mod.contains("TopologyDeclarationEntryStopClass")
            && !topology_operators_mod.contains("TopologyDeclarationEntryRefusalClass")
            && !facade.contains("TopologyDeclarationEntryStopClass")
            && !facade.contains("TopologyDeclarationEntryRefusalClass"),
        "phase 4 remains incomplete if topo-local stop taxonomy is re-exported through the operator surface or root facade instead of staying behind Query ordinary outcomes and recovery",
    );
    for required in [
        "public_topology_declaration_entry_stop_class_not_exported",
        "public_topology_declaration_entry_refusal_class_not_exported",
    ] {
        assert!(
            compile_fail_contracts.contains(required),
            "phase 4 closeout proof must keep `{required}` in the public compile-fail contract set",
        );
    }
}
