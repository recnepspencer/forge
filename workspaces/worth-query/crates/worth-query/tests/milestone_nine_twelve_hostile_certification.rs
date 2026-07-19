use worth_query::facade::foundation::{
    basis_lifecycle, emit_inspection_basis_receipt, emit_observation_basis_receipt,
    envelope_basis_use, readmit_lower_runtime_evidence, BasisUseReceiptKind,
    LowerRuntimeBasisEvidence,
};

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
