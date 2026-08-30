use worth_ui::facade::app::{
    WorthUi, WorthUiApplicationPreparationDenial, WorthUiApplicationPreparationPhase,
};
use worth_ui::facade::declaration::{
    UiDeclarationFamilyKind, UiDeclarationGraphHandoffDenial, UiDeclarationStructuralRole,
    UiDeclaredPostureAdmissionDenial, UiDeclaredPostureLaneKind,
};
use worth_ui::facade::graph::{
    UiGraphContainmentClaim, UiGraphInstantiationPlan, UiGraphParticipationAxis,
    UiGraphParticipationEvidenceHandle, UiGraphParticipationReasonCode,
    UiGraphParticipationReasonSource, UiGraphParticipationStatus, UiGraphWorldProfile,
    UiRepeatedInstanceBasisDenial, UiRepeatedInstanceBasisKind,
};
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};

#[path = "graph_instantiation_runtime/support.rs"]
mod support;

use support::{
    artifact_from_file_provenance, assert_participation_seed_axis, control_graph_input_spec,
    graph_input_with_non_graph_obligations, graph_input_without_non_graph_obligations,
    invalid_graph_input_spec, root_page_artifact,
};

#[test]
fn only_sealed_graph_handoffs_instantiate_graph_truth_through_public_plan() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.graph-instantiation",
            )
            .with_semantic_artifact_spec(control_graph_input_spec()),
        )
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed");
    let handoff = artifact_from_file_provenance(&app, "app/graph_instantiation.wui", 0)
        .graph_handoff()
        .expect("control declaration should lower to graph handoff");
    let root_page_handoff = root_page_artifact(&app)
        .graph_handoff()
        .expect("bootstrap root page should lower to graph handoff");
    let plan = UiGraphInstantiationPlan::admit_handoffs(&[root_page_handoff, handoff.clone()], &[])
        .expect("sealed graph handoff should admit graph instantiation");
    let entry = plan
        .node_entries()
        .iter()
        .find(|entry| entry.declaration_identity() == handoff.identity())
        .expect("control handoff should admit one graph instantiation entry");
    let commit = plan
        .commit_initial_generation(UiGraphWorldProfile::authoritative())
        .expect("admitted plan should commit one coherent graph generation");
    let graph = commit.graph();
    let committed_node_identity = graph
        .lookup()
        .declaration_instances(handoff.identity())
        .value()
        .first()
        .copied()
        .expect("sealed handoff should commit one matching graph node");
    let committed_node = graph
        .lookup()
        .graph_node(committed_node_identity)
        .expect("committed graph node should exist")
        .value();

    assert!(plan.local_denials().is_empty());
    assert_eq!(
        entry.declaration_identity().digest().raw(),
        handoff.identity().digest().raw()
    );
    assert_eq!(
        entry.repeated_instance_basis().kind(),
        UiRepeatedInstanceBasisKind::DeclarationKeyed
    );
    assert_eq!(
        entry.topology_seed().role(),
        UiDeclarationStructuralRole::Control
    );
    assert_eq!(
        entry.topology_seed().containment_claim(),
        &UiGraphContainmentClaim::Control {
            control_name: "save".into(),
        }
    );
    assert!(entry.attachment_posture().query_binding_attached());
    assert!(entry.attachment_posture().service_usage_attached());
    assert_participation_seed_axis(
        entry,
        UiGraphParticipationAxis::Exists,
        UiGraphParticipationStatus::Admitted,
        UiGraphParticipationReasonSource::GraphInstantiation,
        UiGraphParticipationReasonCode::InstantiatedNodeExists,
        UiGraphParticipationEvidenceHandle::InstantiationPlan,
    );
    assert_participation_seed_axis(
        entry,
        UiGraphParticipationAxis::Mounted,
        UiGraphParticipationStatus::Deferred,
        UiGraphParticipationReasonSource::MountEligibility,
        UiGraphParticipationReasonCode::MountedAxisAwaitsRuntimeMutation,
        UiGraphParticipationEvidenceHandle::MountEligibilitySeed,
    );
    assert_participation_seed_axis(
        entry,
        UiGraphParticipationAxis::Visible,
        UiGraphParticipationStatus::Deferred,
        UiGraphParticipationReasonSource::ReservedRuntimeMutation,
        UiGraphParticipationReasonCode::VisibleAxisAwaitsRuntimeMutation,
        UiGraphParticipationEvidenceHandle::ReservedRuntimeMutationLane,
    );
    assert_participation_seed_axis(
        entry,
        UiGraphParticipationAxis::QueryBound,
        UiGraphParticipationStatus::Admitted,
        UiGraphParticipationReasonSource::AttachmentPosture,
        UiGraphParticipationReasonCode::QueryBindingAttached,
        UiGraphParticipationEvidenceHandle::QueryBindingAttachment,
    );
    assert_participation_seed_axis(
        entry,
        UiGraphParticipationAxis::ServiceBound,
        UiGraphParticipationStatus::Admitted,
        UiGraphParticipationReasonSource::AttachmentPosture,
        UiGraphParticipationReasonCode::ServiceUsageAttached,
        UiGraphParticipationEvidenceHandle::ServiceUsageAttachment,
    );
    assert_participation_seed_axis(
        entry,
        UiGraphParticipationAxis::Diagnostic,
        UiGraphParticipationStatus::Withheld,
        UiGraphParticipationReasonSource::ContainmentClaim,
        UiGraphParticipationReasonCode::DiagnosticSurfaceAbsent,
        UiGraphParticipationEvidenceHandle::DiagnosticContainmentClaim,
    );
    assert!(entry.mount_eligibility_seed().graph_eligibility_reserved());
    assert!(entry
        .core_index_contribution_seed()
        .declaration_correspondence());
    assert!(entry.core_index_contribution_seed().node_identity_lookup());
    assert_eq!(graph.node_count(), 2);
    assert!(
        graph
            .lookup()
            .declaration_instances(handoff.identity())
            .value()
            .len()
            == 1
    );
    assert_eq!(
        entry.participation_seed().posture(),
        committed_node.participation_posture()
    );
}

#[test]
fn basis_free_duplicate_handoffs_deny_before_snapshot_mutation() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.graph-instantiation.duplicate",
            )
            .with_semantic_artifact_spec(control_graph_input_spec()),
        )
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed");
    let handoff = artifact_from_file_provenance(&app, "app/graph_instantiation.wui", 0)
        .graph_handoff()
        .expect("declaration artifact should lower to graph handoff");
    let plan = UiGraphInstantiationPlan::admit_handoffs(&[handoff.clone(), handoff], &[])
        .expect("basis-free duplicate handoffs should deny locally instead of mutating graph");

    assert!(plan.node_entries().is_empty());
    assert_eq!(plan.local_denials().len(), 2);
    assert!(plan.local_denials().iter().all(|denial| {
        denial.repeated_instance_basis_denial()
            == Some(&UiRepeatedInstanceBasisDenial::BasisFreeRuntimeIdentityDenied)
    }));
    let denial = plan
        .commit_initial_generation(UiGraphWorldProfile::authoritative())
        .expect_err("denied graph mutation must not publish a replacement snapshot");

    assert_eq!(denial.local_denials(), plan.local_denials());
}

#[test]
fn freeze_path_returns_the_exact_graph_handoff_denial() {
    let denial = match WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.graph-instantiation.freeze-denial",
            )
            .with_semantic_artifact_spec(control_graph_input_spec())
            .with_semantic_artifact_spec(invalid_graph_input_spec()),
        )
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
    {
        Ok(_) => panic!("denied graph handoff must deny application preparation"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.phase(),
        WorthUiApplicationPreparationPhase::GraphHandoff
    );
    assert_eq!(
        denial,
        WorthUiApplicationPreparationDenial::GraphHandoff(Box::new(
            UiDeclarationGraphHandoffDenial::DeclaredPostureNotAdmitted {
                denial: UiDeclaredPostureAdmissionDenial::InvalidLaneClaim {
                    family: UiDeclarationFamilyKind::Control,
                    lane: UiDeclaredPostureLaneKind::ServiceUsage,
                    observed: vec!["service:unknown".to_owned()],
                },
            }
        ),)
    );
}

#[test]
fn touch_and_measurement_posture_do_not_change_graph_instantiation_truth() {
    let baseline = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.graph-instantiation.invariance",
            )
            .with_semantic_artifact_spec(graph_input_without_non_graph_obligations()),
        )
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed");
    let enriched = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.graph-instantiation.invariance",
            )
            .with_semantic_artifact_spec(graph_input_with_non_graph_obligations()),
        )
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed");

    let baseline_handoff =
        artifact_from_file_provenance(&baseline, "app/graph_instantiation.wui", 0)
            .graph_handoff()
            .expect("baseline declaration should lower to graph handoff");
    let baseline_root_handoff = root_page_artifact(&baseline)
        .graph_handoff()
        .expect("baseline bootstrap root page should lower to graph handoff");
    let enriched_handoff =
        artifact_from_file_provenance(&enriched, "app/graph_instantiation.wui", 0)
            .graph_handoff()
            .expect("enriched declaration should lower to graph handoff");
    let enriched_root_handoff = root_page_artifact(&enriched)
        .graph_handoff()
        .expect("enriched bootstrap root page should lower to graph handoff");
    let baseline_plan = UiGraphInstantiationPlan::admit_handoffs(
        &[baseline_root_handoff, baseline_handoff.clone()],
        &[],
    )
    .expect("baseline sealed handoff should admit graph instantiation");
    let enriched_plan = UiGraphInstantiationPlan::admit_handoffs(
        &[enriched_root_handoff, enriched_handoff.clone()],
        &[],
    )
    .expect("touch and measurement posture should not block graph instantiation");
    let baseline_entry = baseline_plan
        .node_entries()
        .iter()
        .find(|entry| entry.declaration_identity() == baseline_handoff.identity())
        .expect("baseline control handoff should admit one graph instantiation entry");
    let enriched_entry = enriched_plan
        .node_entries()
        .iter()
        .find(|entry| entry.declaration_identity() == enriched_handoff.identity())
        .expect("enriched control handoff should admit one graph instantiation entry");

    assert_eq!(
        baseline_entry.repeated_instance_basis().kind(),
        enriched_entry.repeated_instance_basis().kind()
    );
    assert_eq!(
        baseline_entry.topology_seed(),
        enriched_entry.topology_seed()
    );
    assert_eq!(
        baseline_entry.participation_seed(),
        enriched_entry.participation_seed()
    );
    assert_eq!(
        baseline_entry.attachment_posture(),
        enriched_entry.attachment_posture()
    );
    assert_eq!(
        baseline_entry.mount_eligibility_seed(),
        enriched_entry.mount_eligibility_seed()
    );
    assert_eq!(
        baseline_entry.core_index_contribution_seed(),
        enriched_entry.core_index_contribution_seed()
    );

    let baseline_commit = baseline_plan
        .commit_initial_generation(UiGraphWorldProfile::authoritative())
        .expect("baseline plan should commit coherently");
    let enriched_commit = enriched_plan
        .commit_initial_generation(UiGraphWorldProfile::authoritative())
        .expect("enriched plan should commit coherently");
    let baseline_graph = baseline_commit.graph();
    let enriched_graph = enriched_commit.graph();
    let baseline_node_identity = baseline_graph
        .lookup()
        .declaration_instances(baseline_handoff.identity())
        .value()
        .first()
        .copied()
        .expect("baseline graph node identity should exist");
    let enriched_node_identity = enriched_graph
        .lookup()
        .declaration_instances(enriched_handoff.identity())
        .value()
        .first()
        .copied()
        .expect("enriched graph node identity should exist");
    let baseline_node = baseline_graph
        .lookup()
        .graph_node(baseline_node_identity)
        .expect("baseline graph node should exist")
        .value();
    let enriched_node = enriched_graph
        .lookup()
        .graph_node(enriched_node_identity)
        .expect("enriched graph node should exist")
        .value();

    assert_eq!(baseline_graph.node_count(), enriched_graph.node_count());
    assert_eq!(
        baseline_node.attachment_posture(),
        enriched_node.attachment_posture()
    );
    assert_eq!(
        baseline_node.repeated_instance_basis().kind(),
        UiRepeatedInstanceBasisKind::DeclarationKeyed
    );
    assert_eq!(
        enriched_node.repeated_instance_basis().kind(),
        UiRepeatedInstanceBasisKind::DeclarationKeyed
    );
}
