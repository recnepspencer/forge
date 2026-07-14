use std::path::PathBuf;

use worth_query::facade::certification::certify_milestone_nine_twelve;
use worth_query::facade::consumer_kit::downstream_authority_adoption;
use worth_query::facade::foundation::{
    basis_lifecycle, emit_inspection_basis_receipt, emit_observation_basis_receipt,
    envelope_basis_use, readmit_lower_runtime_evidence, BasisUseReceiptKind,
    LowerRuntimeBasisEvidence,
};

#[test]
fn public_consumer_path_closes_milestone_with_real_worth_ui_adoption_evidence() {
    let repository_root = repository_root();
    let adoption = downstream_authority_adoption("worth-ui-milestone-nine-twelve")
        .required_root(
            repository_root.join("workspaces/worth-ui/crates/worth-ui-query-binding/src"),
        )
        .required_root(repository_root.join("workspaces/worth-ui/crates/worth-ui-runtime/src"))
        .evaluate()
        .expect("Worth UI authority adoption must be auditable");
    adoption.assert_adopted();

    let adoption_digest = adoption.manifest().source_inventory_digest().to_string();
    let bundle = certify_milestone_nine_twelve(adoption_digest.clone());

    assert!(bundle.is_closed(), "bundle: {bundle:#?}");
    assert_eq!(bundle.reference_consumer_adoption_digest(), adoption_digest);
    assert!(!bundle.intent_admission_certification_digest().is_empty());
    assert!(!bundle
        .projection_consumption_certification_digest()
        .is_empty());
}

#[test]
fn public_declarative_path_preserves_one_scoped_authority_chain() {
    let fluent = basis_lifecycle()
        .runtime_snapshot("consumer-generation", "runtime:consumer-generation")
        .observe()
        .expect("public declarative observation must admit");
    let explicit = basis_lifecycle()
        .runtime_snapshot("consumer-generation", "runtime:consumer-generation")
        .for_observation()
        .expect("public explicit observation intent must normalize")
        .admit()
        .expect("public explicit observation intent must admit")
        .scope();
    assert_eq!(fluent, explicit);

    let observation_bound = readmit_lower_runtime_evidence(
        fluent,
        LowerRuntimeBasisEvidence::from_runtime_bridge_facade(
            "runtime:consumer-generation",
            "consumer-observation-evidence",
            1,
        ),
    )
    .expect("matching observation evidence must readmit");
    let observation_receipt = emit_observation_basis_receipt(observation_bound);
    assert_eq!(observation_receipt.kind(), BasisUseReceiptKind::Observation);
    assert!(!observation_receipt.receipt_digest().is_empty());
    let observation_envelope = envelope_basis_use(observation_receipt);
    assert!(!observation_envelope.envelope_digest().is_empty());

    let inspection_basis = basis_lifecycle()
        .runtime_snapshot("consumer-generation", "runtime:consumer-generation")
        .inspect()
        .expect("public declarative inspection must admit");
    let inspection_bound = readmit_lower_runtime_evidence(
        inspection_basis,
        LowerRuntimeBasisEvidence::from_runtime_bridge_facade(
            "runtime:consumer-generation",
            "consumer-inspection-evidence",
            1,
        ),
    )
    .expect("matching inspection evidence must readmit");
    let inspection_receipt = emit_inspection_basis_receipt(inspection_bound);
    assert_eq!(inspection_receipt.kind(), BasisUseReceiptKind::Inspection);
    assert!(!inspection_receipt.receipt_digest().is_empty());
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("worth-query crate must live under repository crates/")
        .to_path_buf()
}
