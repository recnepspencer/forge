use worth_proof::{Artifact, AuthorityWitness, TransitionOutcome};

use super::super::basis::CanonicalBasisConstructionAuthority;
use super::super::{
    CanonicalBasisBundle, CanonicalBundleReadyArtifact, CanonicalEquivalenceBasis,
    CanonicalExportReadinessProofs,
};
use super::bundle::{
    CanonicalExportBasisBundle, CanonicalExportBasisSequence, CanonicalExportBundle,
    CanonicalExportDebt, CanonicalExportHarnessSeed, CanonicalExportManifest,
    CanonicalExportManifestRow, CanonicalProducerShape,
};
use super::readmission::CanonicalExportReadyArtifact;

pub fn prepare_canonical_export_bundle(
    fixture_name: impl Into<String>,
    producer_shape: CanonicalProducerShape,
    equivalence_basis: CanonicalEquivalenceBasis,
    bundle: CanonicalBundleReadyArtifact,
) -> TransitionOutcome<CanonicalExportReadyArtifact> {
    let (bundle, _proofs, _basis) = bundle.into_parts().into_parts();
    let rule_version = bundle.version().clone();
    let export_sequences = export_sequences_from_ready_bundle(&bundle);
    let manifest_rows =
        manifest_rows_for_export_sequences(producer_shape, equivalence_basis, &export_sequences);
    let export_bundle = CanonicalExportBundle::new(
        CanonicalExportManifest::new(fixture_name, manifest_rows),
        CanonicalExportBasisBundle::new(rule_version.clone(), export_sequences),
        CanonicalExportHarnessSeed::new("canonical_basis_replay", "milestone_1_surfaces"),
        milestone_2_export_debt(),
    );

    let authority =
        AuthorityWitness::from_authority_marker(CanonicalBasisConstructionAuthority::new());
    let proofs = CanonicalExportReadinessProofs::new(
        worth_proof::Proof::from_authority_witness(&authority),
        worth_proof::Proof::from_authority_witness(&authority),
    );

    TransitionOutcome::success(Artifact::with_proofs_and_current_basis(
        export_bundle,
        proofs,
        rule_version,
        authority,
    ))
}

fn export_sequences_from_ready_bundle(
    bundle: &CanonicalBasisBundle,
) -> Vec<CanonicalExportBasisSequence> {
    bundle
        .sequences()
        .iter()
        .map(|sequence| {
            let payload = sequence.payload();
            CanonicalExportBasisSequence::from_ready_payload(
                payload.version().clone(),
                payload.domain(),
                payload.entries(),
                payload.cost(),
            )
        })
        .collect()
}

fn manifest_rows_for_export_sequences(
    producer_shape: CanonicalProducerShape,
    equivalence_basis: CanonicalEquivalenceBasis,
    export_sequences: &[CanonicalExportBasisSequence],
) -> Vec<CanonicalExportManifestRow> {
    export_sequences
        .iter()
        .map(|sequence| {
            CanonicalExportManifestRow::from_sequence(
                sequence.domain(),
                sequence.version().clone(),
                producer_shape,
                equivalence_basis,
                sequence.cost().entry_count(),
                sequence.cost(),
            )
        })
        .collect()
}

fn milestone_2_export_debt() -> Vec<CanonicalExportDebt> {
    vec![
        CanonicalExportDebt::FinalDigestPolicyDeferred,
        CanonicalExportDebt::RuntimeAdoptionParityDeferred,
        CanonicalExportDebt::LaterMilestoneDomainDeferred,
    ]
}
