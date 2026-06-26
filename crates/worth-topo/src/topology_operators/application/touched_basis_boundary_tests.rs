#[test]
fn declaration_entry_lowers_touched_basis_before_query_contribution_orchestration() {
    let orchestration_boundary =
        include_str!("declaration_entry/orchestration_boundary.rs").replace("\r\n", "\n");
    let retained_handoff =
        include_str!("declaration_entry/retained_application_handoff.rs").replace("\r\n", "\n");
    let basis_index = orchestration_boundary
        .find("TopologyDeclaredTouchedGraphBasis::from_sequence(")
        .expect("declaration entry must lower admitted topology intent to touched basis");
    let query_index = orchestration_boundary
        .find(".orchestrate_declaration_with_contributions(")
        .expect("declaration entry should still orchestrate through Query after basis proof");

    assert!(
        basis_index < query_index,
        "topology touched graph basis must be produced before Query contribution orchestration"
    );
    assert!(
        orchestration_boundary.contains("TopologyRetainedApplicationHandoff::new(\n        artifact,\n        declared_touched_basis")
            && retained_handoff.contains(
                "declared_touched_basis: TopologyDeclaredTouchedGraphBasis<I>"
            )
            && retained_handoff.contains("declared_touched_basis_proof(&self)"),
        "retained application handoff must carry the declared touched-basis proof rather than a Query-only contribution artifact"
    );
}

#[test]
fn query_workflow_fronts_are_public_dx_but_declaration_entry_stays_basis_first() {
    let query_workflow_mod = include_str!("../query_workflow/mod.rs").replace("\r\n", "\n");
    let topology_operators_mod = include_str!("../mod.rs").replace("\r\n", "\n");
    let public_facade = include_str!("../../facade.rs").replace("\r\n", "\n");
    let orchestration_boundary =
        include_str!("declaration_entry/orchestration_boundary.rs").replace("\r\n", "\n");

    assert!(
        query_workflow_mod.contains(
            "pub use grouped_and_contribution_builders::topology_operator_contribution_workflow;"
        ) && query_workflow_mod
            .contains("pub use workflow_handle_ext::TopologyOperatorWorkflowHandleExt;")
            && topology_operators_mod.contains("pub use query_workflow::{")
            && public_facade.contains("pub use crate::topology_operators::{"),
        "topology operator workflow fronts are public DX and must not remain cfg(test) residue",
    );

    let basis_index = orchestration_boundary
        .find("TopologyDeclaredTouchedGraphBasis::from_sequence(")
        .expect("declaration entry must produce touched-basis proof");
    let contribution_input_index = orchestration_boundary
        .find("ForgeQueryContributionComposedOrchestrationInput::new(")
        .expect("declaration entry must build Query contribution input locally");
    let query_index = orchestration_boundary
        .find(".orchestrate_declaration_with_contributions(")
        .expect("declaration entry must lower to Query after basis proof");

    assert!(
        basis_index < contribution_input_index && contribution_input_index < query_index,
        "Query contribution orchestration must stay inside the basis-first declaration-entry boundary",
    );
}

#[test]
fn graph_composition_closeout_uses_declared_touched_basis_guard_before_artifact() {
    let application_mod = include_str!("mod.rs").replace("\r\n", "\n");
    let membership_program =
        include_str!("../local_rewrites/sheet_wire_laminar/membership_programs/mod.rs")
            .replace("\r\n", "\n");
    let successor_program =
        include_str!("../local_rewrites/boundary_wiring/composed_successor_program.rs")
            .replace("\r\n", "\n");
    let grouped_membership_entries = [
        (
            "face inner loop",
            include_str!("declaration_entry/grouped/create_inner_loop_on_existing_face.rs")
                .replace("\r\n", "\n"),
            ".compose_face_inner_loop_program(",
        ),
        (
            "shell rehome",
            include_str!("declaration_entry/grouped/rehome_all_owned_faces_to_new_shell.rs")
                .replace("\r\n", "\n"),
            ".compose_shell_rehome_program(",
        ),
        (
            "wire rehome",
            include_str!("declaration_entry/grouped/rehome_all_owned_half_edges_to_new_wire.rs")
                .replace("\r\n", "\n"),
            ".compose_wire_rehome_program(",
        ),
        (
            "wire split",
            include_str!("declaration_entry/grouped/split_connected_half_edge_set_to_new_wire.rs")
                .replace("\r\n", "\n"),
            ".compose_wire_split_program(",
        ),
        (
            "shell split",
            include_str!(
                "declaration_entry/grouped/split_single_face_from_two_face_shell_to_new_shell.rs"
            )
            .replace("\r\n", "\n"),
            ".compose_shell_split_program(",
        ),
    ];
    let sheet_wire_membership_programs = [
        (
            "face inner loop compose",
            include_str!("../local_rewrites/sheet_wire_laminar/membership_programs/face_inner_loop_program.rs")
                .replace("\r\n", "\n"),
        ),
        (
            "wire membership compose",
            include_str!("../local_rewrites/sheet_wire_laminar/membership_programs/wire_membership_program.rs")
                .replace("\r\n", "\n"),
        ),
        (
            "shell split compose",
            include_str!("../local_rewrites/sheet_wire_laminar/membership_programs/shell_split_program.rs")
                .replace("\r\n", "\n"),
        ),
        (
            "shell membership compose",
            include_str!("../local_rewrites/sheet_wire_laminar/membership_programs/shell_membership_program.rs")
                .replace("\r\n", "\n"),
        ),
    ];

    fn assert_prewrite_guard_inside_graph_composition_method(label: &str, source: &str) {
        let mut search_from = 0;
        let mut compose_count = 0;
        while let Some(relative_compose_index) = source[search_from..].find(".compose_graph(") {
            compose_count += 1;
            let compose_index = search_from + relative_compose_index;
            let before_compose = &source[..compose_index];
            let method_index = before_compose
                .rfind("pub(crate) fn compose_")
                .unwrap_or_else(|| panic!("{label} compose_graph must live in a compose method"));
            let guard_index = before_compose
                .rfind("ensure_declared_touched_basis_covers_sequence_before_write(")
                .unwrap_or_else(|| {
                    panic!("{label} compose method must preflight declared touched basis")
                });
            assert!(
                method_index < guard_index,
                "{label} compose method must check declared touched basis inside the graph-writing API",
            );
            search_from = compose_index + ".compose_graph(".len();
        }
        assert!(
            compose_count > 0,
            "{label} must exercise a graph-composition write path",
        );
    }

    let closeout_index = application_mod
        .find("pub(crate) fn finalize_graph_or_batch_receipt_closeout")
        .expect("shared graph/batch receipt closeout must exist");
    let closeout_body = &application_mod[closeout_index..];
    let guard_index = closeout_body
        .find("ensure_declared_touched_basis_covers_sequence(&retained_handoff, sequence, mode)?;")
        .expect("shared graph/batch closeout must check declared touched basis coverage");
    let artifact_index = closeout_body
        .find("TopologyDeclaredMutationArtifact::from_receipt(")
        .expect("shared graph/batch closeout must build declared mutation artifact");

    assert!(
        guard_index < artifact_index,
        "declared touched-basis coverage must be checked before committed artifact construction",
    );

    let batch_closeout_index = application_mod
        .find("pub(crate) fn finalize_batch_write_closeout")
        .expect("scalar batch closeout must exist");
    let batch_closeout_body = &application_mod[batch_closeout_index..closeout_index];
    let prewrite_guard_index = batch_closeout_body
        .find("ensure_declared_touched_basis_covers_sequence_before_write(")
        .expect("scalar closeout must preflight declared touched-basis coverage before write");
    let submit_index = batch_closeout_body
        .find(".submit_batch_builder(")
        .expect("scalar closeout must submit through Query batch builder");
    assert!(
        prewrite_guard_index < submit_index,
        "scalar mutation path must deny touched-basis omissions before Query write submission",
    );

    let successor_guard_index = successor_program
        .find("ensure_declared_touched_basis_covers_sequence_before_write(")
        .expect("boundary successor graph program must preflight declared touched basis");
    let successor_compose_index = successor_program
        .find(".compose_graph(")
        .expect("boundary successor graph program must compose through Query graph lane");
    assert!(
        successor_guard_index < successor_compose_index,
        "boundary successor graph program must deny touched-basis omissions before compose_graph",
    );

    for (label, source, compose_needle) in grouped_membership_entries {
        let retained_handoff_index = source
            .find("let retained_handoff = orchestrate_topology_declaration_entry(")
            .unwrap_or_else(|| panic!("{label} grouped entry must admit topology intent first"));
        let sequence_index = source
            .find("let sequence = declaration.clone().into_mutation_sequence();")
            .unwrap_or_else(|| panic!("{label} grouped entry must lower declaration to sequence"));
        let compose_index = source
            .find(compose_needle)
            .unwrap_or_else(|| panic!("{label} grouped entry must dispatch graph composition"));
        assert!(
            retained_handoff_index < sequence_index && sequence_index < compose_index,
            "{label} grouped entry must admit intent and sequence before proof-bearing graph composition dispatch",
        );
        assert!(
            source.contains("retained_handoff,\n            mode,"),
            "{label} grouped entry must pass retained handoff and mode into graph composition",
        );
        assert!(
            !source.contains("finish_composed_membership_execution("),
            "{label} grouped entry must not close out a raw graph receipt outside the proof-bearing compose method",
        );
    }

    for (label, source) in &sheet_wire_membership_programs {
        assert_prewrite_guard_inside_graph_composition_method(label, source);
        assert!(
            source.contains("retained_handoff: TopologyRetainedApplicationHandoff<I>")
                && source.contains("mode: TopologyMutationApplicationMode")
                && source.contains("semantic_family_key: &'static str")
                && source.contains("TopologyDeclaredMutationArtifact"),
            "{label} graph-writing method must require proof-bearing handoff/mode inputs and return the committed artifact, not a raw receipt",
        );
    }

    for (label, source) in [
        ("sheet-wire membership", membership_program.as_str()),
        ("boundary successor", successor_program.as_str()),
    ] {
        assert!(
            !source.contains("_mode: TopologyMutationApplicationMode"),
            "{label} must not discard application mode before graph-composition closeout",
        );
        assert!(
            source.contains("mode: TopologyMutationApplicationMode")
                && source.contains("semantic_family_key,\n            mode,\n"),
            "{label} must pass operating-world mode into shared graph-composition closeout",
        );
    }
    assert!(
        !membership_program.contains("pub(crate) fn finish_composed_membership_execution"),
        "sheet-wire membership receipt closeout must not remain a crate-wide raw receipt helper",
    );
}

#[test]
fn declared_touched_basis_omission_denies_mismatched_mutation_before_commit() {
    use forge_query::facade::{ForgeQueryApplicationFacade, ForgeQueryDeclarationFamilyMarker};
    use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};

    use crate::topology_operators::application::{
        TopologyDeclarationEntryStopClass, TopologyDeclarationMutationPayload,
        TopologyMutationApplicationError, TopologyRetainedApplicationHandoff,
    };
    use crate::topology_operators::{
        LoopEndpointKind, TopologyDeclaredTouchedGraphBasis, TopologyOperatorWorkflowHandleExt,
        TopologyRewireLoopEndpointDeclaration, TopologyRewireLoopEndpointFamily,
        TopologySpliceRadialAdjacencyDeclaration, TopologyTouchedOperatingWorld,
    };

    fn entity_id(slot: u64) -> EntityId {
        EntityId::new(PartitionId::main(), slot, 1)
    }

    fn relation_id(slot: u64) -> RelationId {
        RelationId::new(PartitionId::main(), slot, 1)
    }

    let declared = TopologyRewireLoopEndpointDeclaration::new(
        relation_id(31),
        LoopEndpointKind::Start,
        entity_id(32),
        entity_id(33),
    );
    let omitted = TopologySpliceRadialAdjacencyDeclaration::new(
        relation_id(41),
        entity_id(42),
        entity_id(43),
    );
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = crate::query_domain::topology_query_domain_entry(&facade)
        .with_operating_context(crate::query_domain::topology_current_head_authoritative_context())
        .validate()
        .expect("current-head context should validate")
        .admit()
        .expect("current-head context should admit");
    let declared_touched_basis = TopologyDeclaredTouchedGraphBasis::from_sequence(
        TopologyRewireLoopEndpointFamily::semantic_family_key(),
        declared.clone(),
        &declared.clone().into_mutation_sequence(),
        TopologyTouchedOperatingWorld::mainline(),
    )
    .expect("declared touched graph basis");
    let handoff = TopologyRetainedApplicationHandoff::new(
        handle
            .orchestrate_topology_operator_with_contributions(
                crate::topology_operators::topology_operator_contribution_workflow(declared),
            )
            .unwrap_or_else(|_| panic!("topology contribution-composed lane should admit")),
        declared_touched_basis,
    );

    let error = super::declaration_entry::execution_finalize::ensure_declared_touched_basis_covers_sequence(
        &handoff,
        &omitted.into_mutation_sequence(),
        crate::topology_operators::TopologyMutationApplicationMode::Mainline,
    )
    .expect_err("mismatched omitted mutation must deny before commit");

    assert!(matches!(
        error,
        TopologyMutationApplicationError::DeclarationEntry {
            stop_class: TopologyDeclarationEntryStopClass::BasisMismatch,
            ..
        }
    ));
}
