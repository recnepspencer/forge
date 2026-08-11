use super::super::support::digest;
use crate::storage_foundation::s0::*;

#[test]
fn phase1_required_artifacts_report_missing_and_counter_shape() {
    let set = S0RequiredArtifactSet::canonical();
    assert_eq!(
        set.canonical_artifact_dir(),
        "_docs/worth-store/artifacts/storage-foundation-s0"
    );
    let report = set.validate_present_artifacts([
        S0CanonicalArtifactSpec::new(
            S0ArtifactKind::BackendCapabilityMatrix,
            digest("schema:backend-capability-matrix"),
            S0ArtifactSchemaCompatibility::Compatible,
        ),
        S0CanonicalArtifactSpec::new(
            S0ArtifactKind::S0EvidenceBundle,
            digest("schema:evidence-bundle"),
            S0ArtifactSchemaCompatibility::Compatible,
        ),
    ]);

    assert_eq!(report.required_artifact_count(), 9);
    assert_eq!(report.present_artifact_count(), 2);
    assert_eq!(report.missing_required_artifact_count(), 7);
    assert_eq!(report.schema_incompatible_artifact_count(), 0);
    assert!(!report.is_complete());

    let complexity = S0ComplexityContractReport::from_contracts(
        S0RequiredArtifactSet::canonical_complexity_contracts(),
        S0RequiredArtifactSet::canonical_complexity_contracts()
            .into_iter()
            .map(|name| S0ComplexityContract::verified(name.as_str(), 0, 0)),
    );
    let counters = S0CounterSnapshot::from_artifact_and_complexity_reports(&report, &complexity);
    assert_eq!(counters.required_artifact_count(), 9);
    assert_eq!(counters.missing_required_artifact_count(), 7);
    assert_eq!(counters.complexity_contract_count(), 9);
    assert_eq!(counters.missing_complexity_contract_count(), 0);
    assert_eq!(counters.duplicate_complexity_contract_count(), 0);
    assert_eq!(counters.complexity_debt_count(), 0);
    assert_eq!(counters.evidence_ref_reresolution_count(), 0);
}

#[test]
fn phase1_missing_complexity_contracts_are_not_reported_as_zero() {
    let report = S0RequiredArtifactSet::canonical().validate_present_artifacts([]);
    let complexity = S0ComplexityContractReport::from_contracts(
        S0RequiredArtifactSet::canonical_complexity_contracts(),
        [S0ComplexityContract::verified(
            "s0_input_manifest_construction",
            0,
            0,
        )],
    );
    let counters = S0CounterSnapshot::from_artifact_and_complexity_reports(&report, &complexity);

    assert_eq!(complexity.missing_complexity_contract_count(), 8);
    assert_eq!(counters.missing_complexity_contract_count(), 8);
    assert!(counters.has_release_blocking_debt());
}

#[test]
fn phase1_duplicate_complexity_contracts_are_blocking_debt() {
    let complexity = S0ComplexityContractReport::from_contracts(
        S0RequiredArtifactSet::canonical_complexity_contracts(),
        [
            S0ComplexityContract::verified("s0_input_manifest_construction", 0, 0),
            S0ComplexityContract::verified("s0_input_manifest_construction", 1, 1),
        ],
    );
    let report = S0RequiredArtifactSet::canonical().validate_present_artifacts([]);
    let counters = S0CounterSnapshot::from_artifact_and_complexity_reports(&report, &complexity);

    assert_eq!(complexity.duplicate_complexity_contract_count(), 1);
    assert_eq!(counters.duplicate_complexity_contract_count(), 1);
    assert!(counters.has_release_blocking_debt());
}
