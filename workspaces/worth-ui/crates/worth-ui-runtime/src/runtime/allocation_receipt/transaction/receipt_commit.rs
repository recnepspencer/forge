use super::{
    UiAllocationCandidate, UiAllocationLeafRemeasureWitness, UiAllocationPreviewCandidate,
    UiAllocationReceipt, UiAllocationReceiptCommitDenial, UiAllocationReceiptCommitOutcome,
    UiAllocationReceiptEquivalenceBasis, UiAllocationReceiptGeneration,
    UiAllocationReceiptIdentity, UiAllocationReuseDenial, UiAllocationReuseVerdict,
};

/// Runtime-owned reuse evaluation over admitted identity and generation fields only.
pub(crate) fn evaluate_allocation_receipt_reuse(
    candidate: &UiAllocationCandidate,
    previous: &UiAllocationReceipt,
) -> UiAllocationReuseVerdict {
    let identity = UiAllocationReceiptIdentity::from_candidate(candidate);
    if identity != *previous.identity() {
        return UiAllocationReuseVerdict::Denied(UiAllocationReuseDenial::ReceiptIdentityMismatch);
    }
    let generation = UiAllocationReceiptGeneration::from_candidate(candidate);
    if generation.neighborhood_generation() != previous.generation().neighborhood_generation()
        || generation.measurement_basis_generation()
            != previous.generation().measurement_basis_generation()
    {
        return UiAllocationReuseVerdict::Denied(UiAllocationReuseDenial::GenerationMismatch);
    }
    let equivalence_basis = UiAllocationReceiptEquivalenceBasis::from_candidate(candidate);
    if equivalence_basis != *previous.equivalence_basis() {
        return UiAllocationReuseVerdict::Denied(UiAllocationReuseDenial::EquivalenceBasisMismatch);
    }
    if generation.planning_evidence_digest() != previous.generation().planning_evidence_digest() {
        if let Some(witness) =
            UiAllocationLeafRemeasureWitness::from_admitted_leaf_difference(candidate, previous)
        {
            return UiAllocationReuseVerdict::StructureReuseLeafRemeasure(witness);
        }
        return UiAllocationReuseVerdict::Denied(UiAllocationReuseDenial::UnsupportedPartialReuse);
    }
    UiAllocationReuseVerdict::FullReuse
}

/// The sole production promotion from admitted planning to committed allocation truth.
#[cfg(test)]
pub(super) fn admit_allocation_receipt_candidate(
    candidate: &UiAllocationCandidate,
    previous: Option<&UiAllocationReceipt>,
) -> Result<UiAllocationReuseVerdict, UiAllocationReceiptCommitOutcome> {
    if !candidate.is_admitted() {
        return Err(UiAllocationReceiptCommitOutcome::denied(
            UiAllocationReceiptCommitDenial::candidate_planning(
                super::UiAllocationReceiptDenialReport::candidate_planning_denied(candidate),
            ),
        ));
    }

    let reuse_verdict = previous.map_or(UiAllocationReuseVerdict::NewCommit, |previous| {
        evaluate_allocation_receipt_reuse(candidate, previous)
    });
    match reuse_verdict {
        UiAllocationReuseVerdict::Denied(reason) => {
            return Err(UiAllocationReceiptCommitOutcome::denied(
                UiAllocationReceiptCommitDenial::reuse(
                    super::UiAllocationReceiptDenialReport::reuse_denied(candidate, reason),
                ),
            ));
        }
        partial @ UiAllocationReuseVerdict::StructureReuseLeafRemeasure(_) => {
            return Err(UiAllocationReceiptCommitOutcome::recompute_pending(
                super::UiAllocationReceiptReport::new(
                    UiAllocationReceiptIdentity::from_candidate(candidate),
                    UiAllocationReceiptGeneration::from_candidate(candidate),
                    partial,
                ),
            ));
        }
        UiAllocationReuseVerdict::NewCommit | UiAllocationReuseVerdict::FullReuse => {}
    }
    Ok(reuse_verdict)
}

/// Replacement activation owns a full candidate-generation transition. A
/// prior receipt in the same semantic scope is a reuse opportunity, not a
/// requirement that the new generation impersonate the prior receipt.
pub(super) fn admit_replacement_allocation_receipt_candidate(
    candidate: &UiAllocationCandidate,
    previous: Option<&UiAllocationReceipt>,
) -> Result<UiAllocationReuseVerdict, UiAllocationReceiptCommitOutcome> {
    if !candidate.is_admitted() {
        return Err(UiAllocationReceiptCommitOutcome::denied(
            UiAllocationReceiptCommitDenial::candidate_planning(
                super::UiAllocationReceiptDenialReport::candidate_planning_denied(candidate),
            ),
        ));
    }
    Ok(
        previous.map_or(UiAllocationReuseVerdict::NewCommit, |previous| {
            match evaluate_allocation_receipt_reuse(candidate, previous) {
                UiAllocationReuseVerdict::FullReuse => UiAllocationReuseVerdict::FullReuse,
                UiAllocationReuseVerdict::NewCommit
                | UiAllocationReuseVerdict::StructureReuseLeafRemeasure(_)
                | UiAllocationReuseVerdict::Denied(_) => UiAllocationReuseVerdict::NewCommit,
            }
        }),
    )
}

pub(super) fn commit_admitted_allocation_receipt(
    candidate: UiAllocationCandidate,
    reuse_verdict: UiAllocationReuseVerdict,
    transaction: super::UiAllocationReplanTransaction,
) -> UiAllocationReceipt {
    UiAllocationReceipt::from_candidate(&candidate, reuse_verdict, transaction)
}

/// Preview construction deliberately does not grant receipt construction authority.
pub(crate) fn project_allocation_preview(
    candidate: UiAllocationCandidate,
) -> UiAllocationPreviewCandidate {
    UiAllocationPreviewCandidate::from_candidate(candidate)
}
