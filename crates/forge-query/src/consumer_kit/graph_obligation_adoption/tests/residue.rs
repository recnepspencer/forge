use crate::{
    ForgeQueryGraphObligationConsumerKitErrorKind, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationResidueRow,
};

use super::adoption_fixture::{residue_row, residue_row_with_cap};

#[test]
fn residue_manifest_rejects_growth_over_cap() {
    let error = ForgeQueryGraphObligationResidueManifest::capped([
        ForgeQueryGraphObligationResidueRow::explicit(
            "manual selector comments",
            "forge-query",
            "phase-14",
            2,
            1,
            "covered selector replacement is incomplete",
            "delete manual selector comments",
            "remove",
        )
        .unwrap(),
    ])
    .unwrap_err();

    assert_eq!(
        error.kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::ResidueCapExceeded
    );
}

#[test]
fn residue_manifest_certifies_candidate_against_previous_without_growth() {
    let previous = ForgeQueryGraphObligationResidueManifest::capped([residue_row(1)]).unwrap();
    let candidate = ForgeQueryGraphObligationResidueManifest::capped([residue_row(0)]).unwrap();
    let certification =
        ForgeQueryGraphObligationResidueManifest::certify_candidate_against_previous(
            &previous, &candidate,
        )
        .unwrap();

    assert_eq!(certification.certified_row_count(), 1);
    assert_eq!(
        certification.previous_manifest_digest(),
        previous.manifest_digest()
    );
    assert_eq!(
        certification.candidate_manifest_digest(),
        candidate.manifest_digest()
    );
}

#[test]
fn residue_manifest_rejects_candidate_growth_or_contract_drift() {
    let previous = ForgeQueryGraphObligationResidueManifest::capped([residue_row(1)]).unwrap();
    let grown =
        ForgeQueryGraphObligationResidueManifest::capped([residue_row_with_cap(2, 2)]).unwrap();
    let growth_error =
        ForgeQueryGraphObligationResidueManifest::certify_candidate_against_previous(
            &previous, &grown,
        )
        .unwrap_err();

    assert_eq!(
        growth_error.kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::ResidueGrowthAfterIntroduction
    );

    let changed_contract = ForgeQueryGraphObligationResidueManifest::capped([
        ForgeQueryGraphObligationResidueRow::explicit(
            "manual selector comments",
            "forge-query",
            "phase-14",
            1,
            2,
            "covered selector replacement is incomplete",
            "delete manual selector comments",
            "remove",
        )
        .unwrap(),
    ])
    .unwrap();
    let drift_error = ForgeQueryGraphObligationResidueManifest::certify_candidate_against_previous(
        &previous,
        &changed_contract,
    )
    .unwrap_err();

    assert_eq!(
        drift_error.kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::ResidueContractDrift
    );
}
