use crate::dedupe::classification::{classify_scope_mismatch, ScopeMismatchCase};
use crate::dedupe::evidence::BlobChunkDedupeCandidate;
use crate::{
    BlobChunkDedupeAdmissionDenial, BlobChunkDedupeAdmissionOutcome, BlobChunkDedupeCounterSnapshot,
};
use forge_proof::TransitionOutcome;

pub(crate) fn deny_missing_equivalence(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
    counters: BlobChunkDedupeCounterSnapshot,
) -> BlobChunkDedupeAdmissionOutcome {
    if let Some(mismatch) =
        classify_scope_mismatch(existing.security_metadata(), candidate.security_metadata())
    {
        return scope_mismatch_denial(mismatch, counters);
    }

    let counters = counters.record_digest_only_denial();
    TransitionOutcome::denied(
        BlobChunkDedupeAdmissionDenial::MissingFoundationalCanonicalEquivalence { counters },
    )
}

pub(crate) fn deny_scope_mismatch(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
    counters: BlobChunkDedupeCounterSnapshot,
) -> Option<BlobChunkDedupeAdmissionOutcome> {
    let mismatch =
        classify_scope_mismatch(existing.security_metadata(), candidate.security_metadata())?;
    Some(scope_mismatch_denial(mismatch, counters))
}

fn scope_mismatch_denial(
    mismatch: ScopeMismatchCase,
    counters: BlobChunkDedupeCounterSnapshot,
) -> BlobChunkDedupeAdmissionOutcome {
    match mismatch {
        ScopeMismatchCase::TenantScope { left, right } => TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossTenantScopeRequiresExplicitEquivalence {
                left,
                right,
                counters: counters.record_cross_scope_denial(),
            },
        ),
        ScopeMismatchCase::KeyScope { left, right } => TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossKeyScopeRequiresExplicitEquivalence {
                left,
                right,
                counters: counters.record_cross_scope_denial(),
            },
        ),
        ScopeMismatchCase::KeyVersionPosture => TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossScopeSecurityWitnessMismatch {
                counters: counters.record_stale_key_version_denial(),
            },
        ),
        ScopeMismatchCase::AuthenticityRequirement => TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossScopeSecurityWitnessMismatch {
                counters: counters.record_authenticity_mismatch_denial(),
            },
        ),
        ScopeMismatchCase::CustodyPosture => TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossScopeSecurityWitnessMismatch {
                counters: counters.record_custody_mismatch_denial(),
            },
        ),
        ScopeMismatchCase::FullWitness => TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossScopeSecurityWitnessMismatch {
                counters: counters.record_cross_scope_denial(),
            },
        ),
    }
}
