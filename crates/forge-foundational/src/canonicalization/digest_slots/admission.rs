use forge_proof::{Artifact, AuthorityWitness, TransitionOutcome};

use super::super::basis::CanonicalBasisConstructionAuthority;
use super::super::{
    CanonicalBasisReadyArtifact, CanonicalBundleReadyArtifact,
    CanonicalDigestDerivationReadinessProofs, CanonicalDigestDerivationReady,
    CanonicalDigestInputShapeBound, CanonicalExportReadyArtifact, CanonicalRuleVersionBound,
};
use super::algorithm::{
    CanonicalDigestAlgorithmId, CanonicalDigestAlgorithmMetadata,
    CanonicalDomainBundleDigestAlgorithmSlot, CanonicalExportBundleDigestAlgorithmSlot,
    CanonicalSingleSequenceDigestAlgorithmSlot,
};
use super::derived::CanonicalDigestDerivationDenial;
use super::evidence::{
    CanonicalDigestBasisBundle, CanonicalDigestBasisSequence, CanonicalDigestDerivationInput,
    CanonicalDigestInputEvidence,
};

pub type CanonicalDigestDerivationReadyArtifact = Artifact<
    CanonicalDigestDerivationReady,
    CanonicalDigestDerivationInput,
    CanonicalDigestDerivationReadinessProofs,
    forge_proof::FreshnessScopedBasis<
        forge_proof::CurrentValidity,
        forge_proof::AssumptionBasis<CanonicalDigestAlgorithmMetadata>,
    >,
>;

pub fn admit_canonical_sequence_digest_derivation(
    sequence: CanonicalBasisReadyArtifact,
    slot: CanonicalSingleSequenceDigestAlgorithmSlot,
) -> TransitionOutcome<CanonicalDigestDerivationReadyArtifact, CanonicalDigestDerivationDenial> {
    let (sequence, _proofs, _basis) = sequence.into_parts().into_parts();
    let evidence = CanonicalDigestInputEvidence::SingleSequence(CanonicalDigestBasisSequence::new(
        sequence.version().clone(),
        sequence.domain(),
        sequence.entries(),
        sequence.cost(),
    ));
    admit_canonical_digest_derivation(slot.into_metadata(), evidence)
}

pub fn admit_canonical_bundle_digest_derivation(
    bundle: CanonicalBundleReadyArtifact,
    slot: CanonicalDomainBundleDigestAlgorithmSlot,
) -> TransitionOutcome<CanonicalDigestDerivationReadyArtifact, CanonicalDigestDerivationDenial> {
    let (bundle, _proofs, _basis) = bundle.into_parts().into_parts();
    let sequences = bundle
        .sequences()
        .iter()
        .map(|sequence| {
            let payload = sequence.payload();
            CanonicalDigestBasisSequence::new(
                payload.version().clone(),
                payload.domain(),
                payload.entries(),
                payload.cost(),
            )
        })
        .collect();
    let evidence = CanonicalDigestInputEvidence::DomainBundle(CanonicalDigestBasisBundle::new(
        bundle.version().clone(),
        sequences,
    ));

    admit_canonical_digest_derivation(slot.into_metadata(), evidence)
}

pub fn admit_canonical_export_digest_derivation(
    export: CanonicalExportReadyArtifact,
    slot: CanonicalExportBundleDigestAlgorithmSlot,
) -> TransitionOutcome<CanonicalDigestDerivationReadyArtifact, CanonicalDigestDerivationDenial> {
    let (export, _proofs, _basis) = export.into_parts().into_parts();
    let sequences = export
        .bundle()
        .sequences()
        .iter()
        .map(|sequence| {
            CanonicalDigestBasisSequence::new(
                sequence.version().clone(),
                sequence.domain(),
                sequence.entries(),
                sequence.cost(),
            )
        })
        .collect();
    let evidence = CanonicalDigestInputEvidence::ExportBundle(CanonicalDigestBasisBundle::new(
        export.bundle().version().clone(),
        sequences,
    ));

    admit_canonical_digest_derivation(slot.into_metadata(), evidence)
}

fn admit_canonical_digest_derivation(
    algorithm: CanonicalDigestAlgorithmMetadata,
    evidence: CanonicalDigestInputEvidence,
) -> TransitionOutcome<CanonicalDigestDerivationReadyArtifact, CanonicalDigestDerivationDenial> {
    if algorithm.id() != &CanonicalDigestAlgorithmId::test_stable_fixture() {
        return TransitionOutcome::denied(CanonicalDigestDerivationDenial::UnsupportedAlgorithm);
    }
    if algorithm.rule_version() != evidence.version() {
        return TransitionOutcome::denied(CanonicalDigestDerivationDenial::RuleVersionMismatch);
    }
    if algorithm.input_shape() != evidence.input_shape() {
        return TransitionOutcome::denied(CanonicalDigestDerivationDenial::InputShapeMismatch);
    }
    if algorithm.input_domain() != evidence.input_domain() {
        return TransitionOutcome::denied(CanonicalDigestDerivationDenial::InputDomainMismatch);
    }

    let authority =
        AuthorityWitness::from_authority_marker(CanonicalBasisConstructionAuthority::new());
    let proofs = CanonicalDigestDerivationReadinessProofs::new(
        forge_proof::Proof::<
            CanonicalDigestInputShapeBound,
            CanonicalBasisConstructionAuthority,
        >::from_authority_witness(&authority),
        forge_proof::Proof::<CanonicalRuleVersionBound, CanonicalBasisConstructionAuthority>::from_authority_witness(
            &authority,
        ),
    );
    let input = CanonicalDigestDerivationInput::new(algorithm.clone(), evidence);

    TransitionOutcome::success(Artifact::with_proofs_and_current_basis(
        input, proofs, algorithm, authority,
    ))
}
