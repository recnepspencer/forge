use worth_proof::{Artifact, AuthorityWitness, TransitionOutcome};

use super::super::basis::CanonicalBasisConstructionAuthority;
use super::super::{
    CanonicalBasisReadyArtifact, CanonicalBundleReadyArtifact,
    CanonicalDigestDerivationReadinessProofs, CanonicalDigestDerivationReady,
    CanonicalDigestInputShapeBound, CanonicalExportReadyArtifact, CanonicalRuleVersionBound,
};
use super::algorithm::{
    CanonicalDigestAlgorithmMetadata, CanonicalDomainBundleDigestAlgorithmSlot,
    CanonicalExportBundleDigestAlgorithmSlot, CanonicalSingleSequenceDigestAlgorithmSlot,
};
use super::derived::CanonicalDigestDerivationDenial;
use super::evidence::{
    CanonicalDigestBasisBundle, CanonicalDigestBasisSequence, CanonicalDigestDerivationInput,
    CanonicalDigestInputEvidence,
};
use super::material::canonical_digest_material;
use super::{CanonicalDigestWorkBudget, CanonicalDigestWorkEvidence};

pub type CanonicalDigestDerivationReadyArtifact = Artifact<
    CanonicalDigestDerivationReady,
    CanonicalDigestDerivationInput,
    CanonicalDigestDerivationReadinessProofs,
    worth_proof::FreshnessScopedBasis<
        worth_proof::CurrentValidity,
        worth_proof::AssumptionBasis<CanonicalDigestAlgorithmMetadata>,
    >,
>;

pub fn admit_canonical_sequence_digest_derivation(
    sequence: CanonicalBasisReadyArtifact,
    slot: CanonicalSingleSequenceDigestAlgorithmSlot,
) -> TransitionOutcome<CanonicalDigestDerivationReadyArtifact, CanonicalDigestDerivationDenial> {
    admit_canonical_sequence_digest_derivation_with_budget(
        sequence,
        slot,
        CanonicalDigestWorkBudget::standard(),
    )
}

pub fn admit_canonical_sequence_digest_derivation_with_budget(
    sequence: CanonicalBasisReadyArtifact,
    slot: CanonicalSingleSequenceDigestAlgorithmSlot,
    budget: CanonicalDigestWorkBudget,
) -> TransitionOutcome<CanonicalDigestDerivationReadyArtifact, CanonicalDigestDerivationDenial> {
    let (sequence, _proofs, _basis) = sequence.into_parts().into_parts();
    let evidence = CanonicalDigestInputEvidence::SingleSequence(CanonicalDigestBasisSequence::new(
        sequence.version().clone(),
        sequence.domain(),
        sequence.entries(),
        sequence.cost(),
    ));
    admit_canonical_digest_derivation(slot.into_metadata(), evidence, budget)
}

pub fn admit_canonical_bundle_digest_derivation(
    bundle: CanonicalBundleReadyArtifact,
    slot: CanonicalDomainBundleDigestAlgorithmSlot,
) -> TransitionOutcome<CanonicalDigestDerivationReadyArtifact, CanonicalDigestDerivationDenial> {
    admit_canonical_bundle_digest_derivation_with_budget(
        bundle,
        slot,
        CanonicalDigestWorkBudget::standard(),
    )
}

pub fn admit_canonical_bundle_digest_derivation_with_budget(
    bundle: CanonicalBundleReadyArtifact,
    slot: CanonicalDomainBundleDigestAlgorithmSlot,
    budget: CanonicalDigestWorkBudget,
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

    admit_canonical_digest_derivation(slot.into_metadata(), evidence, budget)
}

pub fn admit_canonical_export_digest_derivation(
    export: CanonicalExportReadyArtifact,
    slot: CanonicalExportBundleDigestAlgorithmSlot,
) -> TransitionOutcome<CanonicalDigestDerivationReadyArtifact, CanonicalDigestDerivationDenial> {
    admit_canonical_export_digest_derivation_with_budget(
        export,
        slot,
        CanonicalDigestWorkBudget::standard(),
    )
}

pub fn admit_canonical_export_digest_derivation_with_budget(
    export: CanonicalExportReadyArtifact,
    slot: CanonicalExportBundleDigestAlgorithmSlot,
    budget: CanonicalDigestWorkBudget,
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

    admit_canonical_digest_derivation(slot.into_metadata(), evidence, budget)
}

fn admit_canonical_digest_derivation(
    algorithm: CanonicalDigestAlgorithmMetadata,
    evidence: CanonicalDigestInputEvidence,
    budget: CanonicalDigestWorkBudget,
) -> TransitionOutcome<CanonicalDigestDerivationReadyArtifact, CanonicalDigestDerivationDenial> {
    if !algorithm.id().is_supported() {
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
    let entry_count = evidence.entry_count();
    if entry_count > budget.maximum_entry_count() {
        return TransitionOutcome::denied(CanonicalDigestDerivationDenial::EntryLimitExceeded {
            maximum: budget.maximum_entry_count(),
            actual: entry_count,
        });
    }
    let material =
        match canonical_digest_material(&algorithm, &evidence, budget.maximum_encoded_bytes()) {
            Ok(material) => material,
            Err(denial) => {
                return TransitionOutcome::denied(
                    CanonicalDigestDerivationDenial::EncodedByteLimitExceeded {
                        maximum: denial.maximum(),
                        attempted: denial.attempted(),
                    },
                )
            }
        };
    let work = CanonicalDigestWorkEvidence::new(
        entry_count,
        material.encoded_bytes(),
        material.allocation_bytes(),
    );

    let authority =
        AuthorityWitness::from_authority_marker(CanonicalBasisConstructionAuthority::new());
    let proofs = CanonicalDigestDerivationReadinessProofs::new(
        worth_proof::Proof::<
            CanonicalDigestInputShapeBound,
            CanonicalBasisConstructionAuthority,
        >::from_authority_witness(&authority),
        worth_proof::Proof::<CanonicalRuleVersionBound, CanonicalBasisConstructionAuthority>::from_authority_witness(
            &authority,
        ),
    );
    let input = CanonicalDigestDerivationInput::new(
        algorithm.clone(),
        evidence,
        material.into_bytes(),
        work,
    );

    TransitionOutcome::success(Artifact::with_proofs_and_current_basis(
        input, proofs, algorithm, authority,
    ))
}
