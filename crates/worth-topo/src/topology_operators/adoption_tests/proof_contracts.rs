use std::collections::BTreeSet;

use forge_query::facade::consumer_kit::{
    ForgeQueryBoundaryAuditSourceSet, ForgeQueryGraphObligationInMemoryTestWorkspace,
    ForgeQueryGraphObligationLocalCeremonyAudit,
};
use forge_query::facade::{
    ForgeQueryGraphObligationOperatingWorldDescriptor, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchDescriptorKind, ForgeQueryGraphTouchLifecycleFamily,
    ForgeQueryGraphTouchReadVerb, ForgeQueryMutationFamily,
};

use super::super::{
    topology_operator_command_batch_equivalent_touch_descriptor,
    topology_operator_graph_obligation_adoption_proof,
    topology_operator_graph_obligation_local_ceremony_audit,
    topology_operator_graph_obligation_registration_declaration,
    topology_operator_graph_obligation_residue_manifest, topology_operator_legacy_guard_audit,
    topology_operator_local_guard_residue_inventory, topology_operator_local_guard_residue_total,
    topology_operator_relation_touch_descriptor, TOPOLOGY_OPERATOR_GRAPH_OBLIGATION_FAMILY,
    TOPOLOGY_OPERATOR_RELATION_COLLECTION, TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_OPERATION,
    TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_PATH,
};

#[test]
fn operator_catalog_builds_complete_adoption_proof() {
    let proof = topology_operator_graph_obligation_adoption_proof()
        .expect("operator catalog adoption proof should pass");

    assert_eq!(
        proof.manifest().consumer_name(),
        TOPOLOGY_OPERATOR_GRAPH_OBLIGATION_FAMILY
    );
    assert!(proof.local_ceremony_audit().is_evaluated());
    assert!(proof.local_ceremony_audit().is_clean());
    assert_eq!(proof.support_pin().row_count(), 2);
    assert_eq!(proof.residue_manifest().rows().len(), 6);
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 2);
    assert!(proof
        .in_memory_proof()
        .execution_statuses()
        .iter()
        .all(|status| status.as_str() == "diagnostic-only"));
}

#[test]
fn milestone_9_9_graph_obligation_operator_closeout_is_certifiable_by_query_kit() {
    let proof = topology_operator_graph_obligation_adoption_proof()
        .expect("operator graph obligation adoption proof");
    let residue = topology_operator_graph_obligation_residue_manifest()
        .expect("operator graph obligation residue manifest");

    assert_eq!(
        proof.manifest().consumer_name(),
        TOPOLOGY_OPERATOR_GRAPH_OBLIGATION_FAMILY
    );
    assert_eq!(
        proof.manifest().residue_manifest_digest(),
        residue.manifest_digest()
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 2);
    assert!(proof.local_ceremony_audit().is_evaluated());
    assert!(proof.local_ceremony_audit().is_clean());
    assert!(residue.rows().iter().all(|row| {
        !row.introduced_in().is_empty()
            && row.current_count() <= row.must_not_exceed_count()
            && !row.removal_trigger().is_empty()
    }));
}

#[test]
fn operator_touch_descriptor_is_real_mutation_not_read_family() {
    let descriptor = topology_operator_relation_touch_descriptor()
        .expect("topology operator touch descriptor should build");

    assert_eq!(
        descriptor.kind(),
        ForgeQueryGraphTouchDescriptorKind::AuthoritativeMutationBatch
    );
    assert_eq!(descriptor.update_command_count(), 1);
    assert_eq!(descriptor.declared_collection_count(), 1);
    assert!(descriptor.touches_collection(TOPOLOGY_OPERATOR_RELATION_COLLECTION));
    assert!(descriptor
        .touches_declared_aspect_operation(TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_OPERATION));
    assert!(descriptor.touches_aspect_path(TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_PATH));

    let row = descriptor.rows().first().expect("one operator touch row");
    assert_eq!(row.mutation_family(), ForgeQueryMutationFamily::Update);
    assert_eq!(
        row.lifecycle_family(),
        Some(ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetRetarget)
    );
    assert!(row.read_verb().is_none());
}

#[test]
fn operator_selection_rejects_read_and_wrong_lifecycle_false_fires() {
    let workspace = operator_adoption_workspace();
    let world = ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle();
    let read_descriptor = ForgeQueryGraphTouchDescriptor::read_family(
        TOPOLOGY_OPERATOR_RELATION_COLLECTION,
        [ForgeQueryGraphTouchReadVerb::ObservesCollection],
    )
    .expect("read descriptor should build");
    let wrong_lifecycle_descriptor = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        TOPOLOGY_OPERATOR_RELATION_COLLECTION,
        ForgeQueryMutationFamily::Update,
        Some(ForgeQueryGraphTouchLifecycleFamily::ExistingTargetFollowup),
        [TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_OPERATION],
        [TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_PATH],
    )
    .expect("wrong lifecycle descriptor should build");

    assert_eq!(
        workspace
            .prove_selection(&read_descriptor, &world)
            .selected_obligation_count(),
        0
    );
    assert_eq!(
        workspace
            .prove_selection(&wrong_lifecycle_descriptor, &world)
            .selected_obligation_count(),
        0
    );
}

#[test]
fn operator_command_batch_equivalent_descriptor_selects_same_obligations() {
    let workspace = operator_adoption_workspace();
    let world = ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle();
    let graph_descriptor = topology_operator_relation_touch_descriptor()
        .expect("graph composition descriptor should build");
    let command_batch_descriptor = topology_operator_command_batch_equivalent_touch_descriptor()
        .expect("command batch equivalent descriptor should build");

    let graph_selection = workspace.prove_selection(&graph_descriptor, &world);
    let command_selection = workspace.prove_selection(&command_batch_descriptor, &world);
    let graph_digests = graph_selection
        .selected_registration_digests()
        .collect::<BTreeSet<_>>();
    let command_digests = command_selection
        .selected_registration_digests()
        .collect::<BTreeSet<_>>();

    assert_eq!(graph_selection.selected_obligation_count(), 2);
    assert_eq!(graph_digests, command_digests);
}

#[test]
fn operator_catalog_residue_manifest_names_every_remaining_surface() {
    let manifest = topology_operator_graph_obligation_residue_manifest()
        .expect("operator residue manifest should build");

    let classes = manifest
        .rows()
        .iter()
        .map(|row| {
            assert_eq!(row.owner(), "worth-topo topology operator catalog");
            assert_eq!(row.introduced_in(), "forge-query-9.9-phase-17");
            if row.class() != "existing-entity-incoming-relation-count-mismatch-guards" {
                assert_eq!(row.current_count(), 1);
                assert_eq!(row.must_not_exceed_count(), 1);
            }
            assert!(!row.blocker().is_empty());
            assert!(!row.removal_trigger().is_empty());
            assert!(row.decision().contains("explicit residue"));
            row.class()
        })
        .collect::<BTreeSet<_>>();
    let local_guard_residue = manifest
        .rows()
        .iter()
        .find(|row| row.class() == "existing-entity-incoming-relation-count-mismatch-guards")
        .expect("manual incoming relation-count guards must be explicit residue");
    assert_eq!(local_guard_residue.current_count(), 6);
    assert_eq!(local_guard_residue.must_not_exceed_count(), 6);

    assert_eq!(
        classes,
        BTreeSet::from([
            "face-inner-loop-command-batch-operator",
            "existing-entity-incoming-relation-count-mismatch-guards",
            "milestone-one-reference-integrity-pack",
            "scalar-topology-mutation-fronts",
            "shell-membership-command-batch-operator",
            "wire-rehome-command-batch-operator",
        ])
    );
}

#[test]
fn operator_legacy_guard_audit_accounts_for_named_phase_seventeen_guard_residue() {
    let audit = topology_operator_legacy_guard_audit();
    let residue_rows = topology_operator_local_guard_residue_inventory();

    assert_eq!(
        audit.total_occurrence_count(),
        topology_operator_local_guard_residue_total()
    );
    assert_eq!(audit.rows().len(), residue_rows.len());
    assert!(audit.rows().iter().all(|row| {
        row.pattern() == "ExistingEntityIncomingRelationCountMismatch"
            && row.source_path().contains("local_rewrites")
            && row.occurrence_count() > 0
    }));
    assert_eq!(
        audit
            .rows()
            .iter()
            .map(|row| (row.source_path(), row.occurrence_count()))
            .collect::<BTreeSet<_>>(),
        residue_rows
            .iter()
            .map(|row| (row.source_path(), row.occurrence_count()))
            .collect::<BTreeSet<_>>()
    );
}

#[test]
fn operator_local_ceremony_audit_is_real_and_currently_clean() {
    let audit = topology_operator_graph_obligation_local_ceremony_audit();

    assert!(audit.is_evaluated());
    assert_eq!(audit.evaluated_source_count(), 32);
    assert!(
        audit.is_clean(),
        "unexpected findings: {:?}",
        audit.findings()
    );
}

fn operator_adoption_workspace() -> ForgeQueryGraphObligationInMemoryTestWorkspace {
    let declaration = topology_operator_graph_obligation_registration_declaration()
        .expect("operator catalog registration declaration should build");
    ForgeQueryGraphObligationInMemoryTestWorkspace::from_registrations(
        declaration.registrations().to_vec(),
    )
    .expect("operator catalog registration declaration should build in-memory workspace")
}

#[test]
fn operator_local_ceremony_audit_rejects_seeded_bypass_patterns() {
    let audit = ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
        &ForgeQueryBoundaryAuditSourceSet::new("worth-topo").source(
            "seeded-local-ceremony",
            r#"
            fn bypass() {
                let _selector = ForgeQueryGraphTouchSelector::any_graph_touch();
                let _marker = "masked string literal: phase_chain";
            }
            "#,
        ),
    );

    assert!(!audit.is_clean());
    assert_eq!(audit.findings().len(), 1);
    assert_eq!(
        audit.findings()[0].pattern(),
        "ForgeQueryGraphTouchSelector::"
    );
}
