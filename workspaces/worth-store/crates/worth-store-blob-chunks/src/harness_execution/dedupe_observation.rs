use crate::handoffs::BlobHarnessSecurityScopeClass;
use crate::{
    BlobChunkCanonicalComparisonBasis, BlobChunkDedupeAdmission, BlobChunkDedupeCandidate,
};

use super::scope_admission::integrity_proof_for_scope;

pub(super) fn observe_cross_scope_dedupe(
    case: &str,
    scope_class: BlobHarnessSecurityScopeClass,
) -> bool {
    if !matches!(scope_class, BlobHarnessSecurityScopeClass::CrossScopeDenied) {
        return false;
    }
    let existing = BlobChunkDedupeCandidate::from_integrity_proof(integrity_proof_for_scope(
        case,
        BlobHarnessSecurityScopeClass::ScopePreserving,
        b"dedupe-cross-scope",
    ));
    let other_case = format!("{case}.other");
    let candidate = BlobChunkDedupeCandidate::from_integrity_proof(integrity_proof_for_scope(
        &other_case,
        BlobHarnessSecurityScopeClass::CrossScopeDenied,
        b"dedupe-cross-scope",
    ));
    let equivalence = BlobChunkCanonicalComparisonBasis::from_candidates(&existing, &candidate)
        .expect("basis")
        .evaluate_foundational_equivalence()
        .expect("equivalence");
    matches!(
        BlobChunkDedupeAdmission::compare_candidates(existing, candidate)
            .with_foundational_canonical_equivalence(equivalence)
            .with_policy(crate::BlobChunkDedupePolicy::same_tenant_same_key_scope())
            .admit(),
        worth_proof::TransitionOutcome::Denied(_)
    )
}
