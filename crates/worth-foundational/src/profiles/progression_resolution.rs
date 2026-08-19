use worth_proof::{Artifact, AuthorityWitness, TransitionOutcome};

use super::progression::{
    classify_profile_narrowing_for_resolution, AdmittedFoundationalProfileArtifact,
    AdmittedFoundationalProfileSet, FoundationalProfileProgressionAuthority,
    FoundationalProfileProgressionDenial, FoundationalProfileProgressionOutcome,
    MaterializedFoundationalProfileArtifact, MaterializedFoundationalProfileSet,
    RequestedFoundationalProfileArtifact,
};
use super::resolution::{changed_resolution_families, FoundationalProfileResolutionLedger};
use super::FoundationalProfileSet;

pub fn admit_requested_foundational_profile_with_resolutions(
    requested: RequestedFoundationalProfileArtifact,
    admitted: FoundationalProfileSet,
    resolutions: FoundationalProfileResolutionLedger,
    _authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileProgressionOutcome<AdmittedFoundationalProfileArtifact> {
    let requested_profile = *requested.payload().requested();
    if let Err(denial) = classify_profile_narrowing_for_resolution(requested_profile, admitted) {
        return TransitionOutcome::denied(denial);
    }
    if let Err(denial) = validate_resolution_ledger(requested_profile, admitted, resolutions) {
        return TransitionOutcome::denied(denial);
    }

    TransitionOutcome::success(Artifact::new(AdmittedFoundationalProfileSet {
        requested: requested_profile,
        admitted,
        requested_to_admitted_resolutions: resolutions,
    }))
}

pub fn materialize_admitted_foundational_profile_with_resolutions(
    admitted: AdmittedFoundationalProfileArtifact,
    materialized: FoundationalProfileSet,
    resolutions: FoundationalProfileResolutionLedger,
    _authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileProgressionOutcome<MaterializedFoundationalProfileArtifact> {
    let admitted_payload = admitted.payload();
    if let Err(denial) =
        classify_profile_narrowing_for_resolution(*admitted_payload.admitted(), materialized)
    {
        return TransitionOutcome::denied(denial);
    }
    if let Err(denial) =
        validate_resolution_ledger(*admitted_payload.admitted(), materialized, resolutions)
    {
        return TransitionOutcome::denied(denial);
    }

    TransitionOutcome::success(Artifact::new(MaterializedFoundationalProfileSet {
        requested: *admitted_payload.requested(),
        admitted: *admitted_payload.admitted(),
        materialized,
        requested_to_admitted_resolutions: admitted_payload.requested_to_admitted_resolutions(),
        admitted_to_materialized_resolutions: resolutions,
    }))
}

fn validate_resolution_ledger(
    stronger: FoundationalProfileSet,
    weaker: FoundationalProfileSet,
    supplied: FoundationalProfileResolutionLedger,
) -> Result<(), FoundationalProfileProgressionDenial> {
    let expected = changed_resolution_families(stronger, weaker);
    if expected.len() != supplied.len() {
        return Err(
            FoundationalProfileProgressionDenial::ResolutionLedgerDoesNotMatchProfileChange,
        );
    }

    for record in expected.records() {
        let Some(actual) = supplied.get(record.family()) else {
            return Err(
                FoundationalProfileProgressionDenial::ResolutionLedgerDoesNotMatchProfileChange,
            );
        };
        if actual.relation() != record.relation() {
            return Err(
                FoundationalProfileProgressionDenial::ResolutionRelationMismatch(record.family()),
            );
        }
    }
    Ok(())
}
