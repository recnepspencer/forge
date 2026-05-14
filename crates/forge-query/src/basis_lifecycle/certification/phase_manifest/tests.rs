use super::{
    basis_lifecycle_phase_artifact_manifest_digest, basis_lifecycle_phase_manifest,
    basis_lifecycle_typestate_transition_digest, BasisLifecyclePhaseArtifact,
};

#[test]
fn phase_manifest_names_every_closeout_artifact_in_order() {
    let manifest = basis_lifecycle_phase_manifest();
    let artifacts = manifest
        .rows()
        .iter()
        .map(|row| row.artifact())
        .collect::<Vec<_>>();

    assert_eq!(
        artifacts,
        vec![
            BasisLifecyclePhaseArtifact::RawIntent,
            BasisLifecyclePhaseArtifact::NormalizedIntent,
            BasisLifecyclePhaseArtifact::Eligibility,
            BasisLifecyclePhaseArtifact::AdmittedCapability,
            BasisLifecyclePhaseArtifact::ScopedBasis,
            BasisLifecyclePhaseArtifact::LowerRuntimeBinding,
            BasisLifecyclePhaseArtifact::UseReceipt,
            BasisLifecyclePhaseArtifact::SelfDescribingEnvelope,
            BasisLifecyclePhaseArtifact::CertificationBundle,
        ]
    );
    assert_eq!(
        basis_lifecycle_phase_artifact_manifest_digest(),
        manifest.manifest_digest()
    );
    assert_eq!(
        basis_lifecycle_typestate_transition_digest(),
        manifest.typestate_transition_digest()
    );
    assert_ne!(
        manifest.manifest_digest(),
        manifest.typestate_transition_digest()
    );
}

#[test]
fn phase_manifest_rows_bind_required_inputs_to_next_consumers() {
    let manifest = basis_lifecycle_phase_manifest();

    for row in manifest.rows() {
        assert!(
            !row.required_input().is_empty(),
            "{} must name the proof it consumes",
            row.artifact().as_str()
        );
        assert!(
            !row.next_consumer().is_empty(),
            "{} must name the next phase consumer",
            row.artifact().as_str()
        );
        assert!(
            row.enforcement_proof().starts_with("basis_lifecycle_"),
            "{} must bind to a concrete lifecycle proof",
            row.artifact().as_str()
        );
        assert!(!row.row_digest().is_empty());
    }
}

#[test]
fn phase_manifest_rejects_generic_row_summary_substitution() {
    let manifest = basis_lifecycle_phase_manifest();

    assert!(
        manifest
            .rows()
            .iter()
            .any(|row| row.producer() == "readmit_lower_runtime_evidence"),
        "manifest must include the lower-runtime readmission trust boundary"
    );
    assert!(
        manifest
            .rows()
            .iter()
            .any(|row| row.producer() == "SelfDescribingBasisEnvelope::from_receipt"),
        "manifest must include envelope materialization before certification"
    );
}
