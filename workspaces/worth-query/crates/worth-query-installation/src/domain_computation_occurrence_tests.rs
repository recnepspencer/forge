use crate::domain_computation_artifact_fixture::*;
use crate::facade::*;

#[test]
fn substitution_purpose_is_canonical_semantic_contract_data() {
    let identity = |occurrence| {
        base_builder()
            .occurrence(occurrence)
            .compatibility(active_compatibility())
            .finish()
            .unwrap()
            .identity()
            .as_str()
            .to_string()
    };
    let computational = WorthQueryArtifactOccurrenceContract::independent_per_execution()
        .permit(WorthQueryArtifactSubstitutionPurpose::ComputationalReuse);
    let duplicate_computational = computational
        .clone()
        .permit(WorthQueryArtifactSubstitutionPurpose::ComputationalReuse);
    let evidentiary = WorthQueryArtifactOccurrenceContract::independent_per_execution()
        .permit(WorthQueryArtifactSubstitutionPurpose::EvidentiarySubstitution);

    assert_eq!(identity(computational), identity(duplicate_computational));
    assert_ne!(
        identity(
            WorthQueryArtifactOccurrenceContract::independent_per_execution()
                .permit(WorthQueryArtifactSubstitutionPurpose::ComputationalReuse)
        ),
        identity(evidentiary)
    );
}

#[test]
fn occurrence_identity_policy_changes_contract_meaning() {
    let identity = |occurrence| {
        base_builder()
            .occurrence(occurrence)
            .compatibility(active_compatibility())
            .finish()
            .unwrap()
            .identity()
            .as_str()
            .to_string()
    };

    assert_ne!(
        identity(WorthQueryArtifactOccurrenceContract::independent_per_execution()),
        identity(WorthQueryArtifactOccurrenceContract::domain_minted_independent())
    );
}
