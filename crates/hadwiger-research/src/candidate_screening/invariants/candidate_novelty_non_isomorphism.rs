use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(F::CandidateNoveltyNonIsomorphism, "candidate_novelty_non_isomorphism", "Candidate novelty / non-isomorphism test", T::DiscoverySupport, A::DiscoveryMemory, "Relabeled or near-isomorphic candidates should be rejected or deprioritized before expensive compute.", "canonical graph, WL, spectral, symmetry, or geometric signatures match retained work", "canonicalization/fingerprint comparison record")
}
