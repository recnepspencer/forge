use worth_foundational::facade::{
    compare_canonical_basis, prepare_canonical_comparison, CanonicalComparisonOutcome,
    CanonicalEquivalenceBasis, CanonicalEquivalentBasis,
};

use crate::basis_lifecycle::BasisOperationLane;

use super::denial::{
    WorthQueryCompatibilityCounters, WorthQueryCompatibilityDenial,
    WorthQueryCompatibilityDenialKind,
};

pub(super) fn compare_admitted_bases<L: BasisOperationLane>(
    subject: &crate::basis_lifecycle::AdmittedBasisCapability<L>,
    candidate: &crate::basis_lifecycle::AdmittedBasisCapability<L>,
    counters: &mut WorthQueryCompatibilityCounters,
) -> Result<CanonicalEquivalentBasis, WorthQueryCompatibilityDenial> {
    let result = compare_one(
        subject.canonical_basis().clone(),
        candidate.canonical_basis().clone(),
        WorthQueryCompatibilityDenialKind::BasisMismatched,
        WorthQueryCompatibilityDenialKind::BasisUnsupported,
        "admitted operational bases differ canonically",
        counters,
    );
    result.map_err(|mut denial| {
        if denial
            .canonical_mismatch()
            .is_some_and(mismatch_is_lifecycle)
        {
            denial.set_kind(WorthQueryCompatibilityDenialKind::BasisLifecycle);
        }
        denial
    })
}

fn mismatch_is_lifecycle(mismatch: &worth_foundational::facade::CanonicalMismatchBasis) -> bool {
    matches!(
        mismatch.left_locus(),
        Some(worth_foundational::facade::CanonicalBasisLocus::Named(
            worth_foundational::facade::InternedString::Raw(name)
        )) if name == "lifecycle"
    )
}

fn compare_one(
    subject: worth_foundational::facade::CanonicalBasisReadyArtifact,
    candidate: worth_foundational::facade::CanonicalBasisReadyArtifact,
    mismatch_kind: WorthQueryCompatibilityDenialKind,
    unsupported_kind: WorthQueryCompatibilityDenialKind,
    detail: &'static str,
    counters: &mut WorthQueryCompatibilityCounters,
) -> Result<CanonicalEquivalentBasis, WorthQueryCompatibilityDenial> {
    counters.canonical_comparisons += 1;
    let prepared = prepare_canonical_comparison(
        CanonicalEquivalenceBasis::ExactCanonicalBasis,
        subject,
        candidate,
    )
    .into_result()
    .expect("retained Foundational bases are comparison-ready");
    match compare_canonical_basis(&prepared) {
        CanonicalComparisonOutcome::Equivalent(evidence) => Ok(evidence),
        CanonicalComparisonOutcome::Mismatched(mismatch) => Err(
            WorthQueryCompatibilityDenial::canonical(mismatch_kind, mismatch, detail, *counters),
        ),
        CanonicalComparisonOutcome::Unsupported(mismatch) => Err(
            WorthQueryCompatibilityDenial::canonical(unsupported_kind, mismatch, detail, *counters),
        ),
    }
}
