use super::*;
use std::collections::BTreeSet;

#[test]
fn intent_admission_certification_bundle_assembles_phase_six_foundation_artifacts() {
    let bundle = certify_intent_admission();
    let compile_fail_targets = forge_query_intent_admission_compile_fail_targets();
    let golden_transcripts = forge_query_intent_admission_golden_transcripts();
    let compile_fail_paths = compile_fail_targets
        .iter()
        .map(|target| target.path())
        .collect::<Vec<_>>();
    let golden_paths = golden_transcripts
        .iter()
        .map(|target| target.path())
        .collect::<Vec<_>>();
    let crate_doc_example_targets = forge_query_intent_admission_crate_doc_example_targets();
    let crate_doc_example_paths = crate_doc_example_targets
        .iter()
        .map(|target| target.path())
        .collect::<Vec<_>>();
    let required_outputs = forge_query_intent_admission_required_certification_outputs();
    let closeout_extension_outputs = forge_query_intent_admission_closeout_extension_outputs();
    let output_manifest = forge_query_intent_admission_certification_output_manifest();
    let output_names = bundle
        .outputs()
        .iter()
        .map(|output| output.name())
        .collect::<Vec<_>>();

    assert_eq!(
        bundle.family_inventory().rows(),
        forge_query_intent_admission_family_inventory().rows()
    );
    assert_eq!(
        bundle.coverage_inventory().rows(),
        forge_query_intent_admission_coverage_inventory().rows()
    );
    assert_eq!(
        bundle.support_matrix().rows(),
        forge_query_intent_admission_support_matrix().rows()
    );
    assert_eq!(
        bundle.public_boundary_audit().compile_fail_targets(),
        compile_fail_targets
    );
    assert_eq!(
        bundle.public_boundary_audit().golden_transcripts(),
        golden_transcripts
    );
    assert_eq!(
        bundle.public_boundary_audit().crate_doc_example_targets(),
        crate_doc_example_targets
    );
    assert_eq!(bundle.output_manifest(), output_manifest);
    assert_eq!(compile_fail_targets.len(), 36);
    assert_eq!(golden_transcripts.len(), 5);
    assert_eq!(crate_doc_example_targets.len(), 5);
    assert_eq!(output_manifest.len(), 53);
    assert_eq!(required_outputs.len(), 42);
    assert_eq!(closeout_extension_outputs.len(), 11);
    assert_eq!(
        compile_fail_paths.len(),
        compile_fail_paths
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len(),
        "compile-fail manifest must be duplicate-free"
    );
    assert_eq!(
        golden_paths.len(),
        golden_paths.iter().copied().collect::<BTreeSet<_>>().len(),
        "golden transcript manifest must be duplicate-free"
    );
    assert_eq!(
        output_names.len(),
        output_names.iter().copied().collect::<BTreeSet<_>>().len(),
        "certification output manifest must be duplicate-free"
    );
    assert_eq!(
        required_outputs.len(),
        required_outputs
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len(),
        "required certification outputs must be duplicate-free"
    );
    assert_eq!(
        closeout_extension_outputs.len(),
        closeout_extension_outputs
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len(),
        "closeout extension outputs must be duplicate-free"
    );
    assert_eq!(
        required_outputs
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .intersection(
                &closeout_extension_outputs
                    .iter()
                    .copied()
                    .collect::<BTreeSet<_>>()
            )
            .count(),
        0,
        "required certification outputs and closeout extensions must stay disjoint"
    );
    assert_eq!(
        crate_doc_example_paths.len(),
        crate_doc_example_paths
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len(),
        "crate-doc example manifest must be duplicate-free"
    );
    assert_eq!(output_names, output_manifest);
    assert_eq!(
        output_manifest
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        required_outputs
            .iter()
            .chain(closeout_extension_outputs.iter())
            .copied()
            .collect::<BTreeSet<_>>(),
        "final output manifest must be exactly the union of spec-required outputs and explicit closeout extensions"
    );
    assert!(compile_fail_paths.iter().all(|path| {
        path.starts_with("tests/ui/intent_admission/") && !path.contains("/golden/")
    }));
    assert!(golden_paths
        .iter()
        .all(|path| path.starts_with("tests/ui/intent_admission/golden/")));
    assert!(crate_doc_example_paths
        .iter()
        .all(|path| path.starts_with("tests/ui/intent_admission/docs/")));
    assert!(compile_fail_targets.iter().any(|target| {
        target
            .path()
            .ends_with("intent_admission_width_run_row_constructor_private.rs")
    }));
    assert!(golden_transcripts.iter().any(|transcript| {
        transcript
            .path()
            .ends_with("intent_admission_basis_projection_golden_transcript_compiles.rs")
    }));
    assert!(golden_transcripts.iter().any(|transcript| {
        transcript.path().ends_with(
            "intent_admission_read_mutation_inspection_routing_golden_transcript_compiles.rs",
        )
    }));
    assert!(crate_doc_example_targets.iter().any(|target| {
        target.label() == "read_mutation_inspection_routing"
            && target
                .path()
                .ends_with("intent_admission_doc_read_mutation_inspection_routing_compiles.rs")
    }));
    assert_eq!(
        bundle.output_digest("intent_golden_transcript_digest"),
        Some(bundle.public_boundary_audit().golden_transcript_digest())
    );
    assert_eq!(
        bundle
            .public_boundary_audit()
            .crate_doc_example_target_digest(),
        &crate::identity::hash_parts(
            &crate_doc_example_targets
                .iter()
                .map(|target| format!("{}:{}", target.label(), target.path()))
                .collect::<Vec<_>>()
        )
    );
    assert_eq!(
        bundle.output_digest("compile_fail_boundary_digest"),
        Some(
            bundle
                .public_boundary_audit()
                .compile_fail_boundary_digest()
        )
    );
    assert_eq!(
        bundle.output_digest("decision_phase_progression_digest"),
        Some(
            bundle
                .proof_shape_audit()
                .decision_phase_progression_digest()
        )
    );
    assert_eq!(
        bundle.output_digest("decision_proof_shape_digest"),
        Some(bundle.proof_shape_audit().decision_proof_shape_digest())
    );
    assert_eq!(
        bundle.output_digest("query_digest"),
        bundle
            .representative_output_report()
            .digest_for("query_digest")
    );
    assert_eq!(
        bundle.output_digest("failure_digest"),
        bundle
            .representative_output_report()
            .digest_for("failure_digest")
    );
    assert_eq!(
        bundle.output_digest("crate_doc_example_digest"),
        Some(bundle.doc_example_report().crate_doc_example_digest())
    );
    assert_eq!(
        bundle.output_digest("decision_oracle_digest"),
        Some(bundle.oracle_report().oracle_digest())
    );
    assert_eq!(
        bundle.output_digest("legacy_delegation_parity_digest"),
        Some(
            bundle
                .legacy_parity_report()
                .legacy_delegation_parity_digest()
        )
    );
    assert_eq!(
        bundle.output_digest("decision_support_traceability_digest"),
        Some(
            bundle
                .support_traceability_report()
                .decision_support_traceability_digest()
        )
    );
    assert_eq!(
        bundle.output_digest("counter_snapshot"),
        Some(bundle.counter_snapshot().digest())
    );
    assert_eq!(
        bundle.output_digest("seeded_sequence_digest"),
        Some(bundle.seeded_report().seeded_sequence_digest())
    );
    assert_eq!(
        bundle.output_digest("seed_replay_digest"),
        Some(bundle.seeded_report().seed_replay_digest())
    );
    assert_eq!(
        bundle.output_digest("seed_generator_class_digest"),
        Some(bundle.seeded_report().seed_generator_class_digest())
    );
    assert_eq!(
        bundle.output_digest("intent_topology_audit_digest"),
        Some(bundle.topology_audit().topology_digest())
    );
    assert_eq!(
        bundle.output_digest("representative_family_coverage_digest"),
        Some(
            bundle
                .representative_family_report()
                .representative_family_coverage_digest()
        )
    );
    assert!(!bundle.certification_bundle_digest().is_empty());
}
