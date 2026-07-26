use sha2::Sha256;

use crate::canonical_hash_encoding::hash_text_field;
use crate::domain_computation::*;

use super::super::WorthQueryPortableArtifactContract;
use super::vocabulary::*;

pub(super) fn hash_occurrence_and_evidence(
    hash: &mut Sha256,
    contract: &WorthQueryPortableArtifactContract,
) {
    hash_text_field(
        hash,
        "occurrence-policy",
        occurrence_policy(contract.occurrence.identity_policy()),
    );
    for purpose in contract.occurrence.permitted_substitutions() {
        hash_text_field(hash, "substitution-purpose", substitution(*purpose));
    }
    for (label, value) in [
        ("basis-family", contract.evidence.basis_family()),
        ("provenance-family", contract.evidence.provenance_family()),
        ("dependency-family", contract.evidence.dependency_family()),
        (
            "invalidation-family",
            contract.evidence.invalidation_family(),
        ),
        ("equivalence-family", contract.evidence.equivalence_family()),
    ] {
        hash_text_field(hash, label, value);
    }
}

pub(super) fn hash_reproducibility(
    hash: &mut Sha256,
    value: &WorthQueryArtifactReproducibilityContract,
) {
    hash_text_field(
        hash,
        "reproducibility-class",
        reproducibility(value.class()),
    );
    hash_text_field(hash, "determinism", determinism(value.determinism()));
    match value.comparison() {
        WorthQueryArtifactComparisonAuthority::NotDeclared => {
            hash_text_field(hash, "comparison", "not-declared")
        }
        WorthQueryArtifactComparisonAuthority::ExactCanonicalValue => {
            hash_text_field(hash, "comparison", "exact-canonical-value")
        }
        WorthQueryArtifactComparisonAuthority::CanonicalReduction { family } => {
            hash_text_field(hash, "comparison", "canonical-reduction");
            hash_text_field(hash, "comparison-family", family);
        }
        WorthQueryArtifactComparisonAuthority::RegisteredDomainComparator { family } => {
            hash_text_field(hash, "comparison", "domain-comparator");
            hash_text_field(hash, "comparison-family", family);
        }
        WorthQueryArtifactComparisonAuthority::RegisteredErrorBoundComparator { family } => {
            hash_text_field(hash, "comparison", "error-bound-comparator");
            hash_text_field(hash, "comparison-family", family);
        }
        WorthQueryArtifactComparisonAuthority::RegisteredDistributionTest { family } => {
            hash_text_field(hash, "comparison", "distribution-test");
            hash_text_field(hash, "comparison-family", family);
        }
        WorthQueryArtifactComparisonAuthority::NotComparable => {
            hash_text_field(hash, "comparison", "not-comparable")
        }
    }
    for dependency in value.environment_dependencies() {
        hash_text_field(hash, "environment-dependency", dependency);
    }
    for dependency in value.entropy_dependencies() {
        hash_text_field(hash, "entropy-dependency", dependency);
    }
}

pub(super) fn hash_search(hash: &mut Sha256, value: &WorthQueryCandidateSearchContract) {
    hash_optional(hash, "candidate-universe", value.universe_family());
    hash_optional(hash, "search-termination", value.termination_family());
    hash_optional(hash, "candidate-feasibility", value.feasibility_family());
    hash_optional(hash, "candidate-comparison", value.comparison_family());
    hash_optional(hash, "candidate-incumbent", value.incumbent_family());
    match value.search_posture() {
        WorthQueryCandidateSearchPosture::NotApplicable => {
            hash_text_field(hash, "search-posture", "not-applicable")
        }
        WorthQueryCandidateSearchPosture::Exhaustive => {
            hash_text_field(hash, "search-posture", "exhaustive")
        }
        WorthQueryCandidateSearchPosture::ProvenTopK { count } => {
            hash_text_field(hash, "search-posture", "proven-top-k");
            hash_text_field(hash, "top-k", &count.to_string());
        }
        WorthQueryCandidateSearchPosture::Bounded { bound_identity } => {
            hash_text_field(hash, "search-posture", "bounded");
            hash_text_field(hash, "search-bound", bound_identity);
        }
        WorthQueryCandidateSearchPosture::Sampled { sample_identity } => {
            hash_text_field(hash, "search-posture", "sampled");
            hash_text_field(hash, "sample-identity", sample_identity);
        }
        WorthQueryCandidateSearchPosture::Heuristic => {
            hash_text_field(hash, "search-posture", "heuristic")
        }
        WorthQueryCandidateSearchPosture::Incomplete => {
            hash_text_field(hash, "search-posture", "incomplete")
        }
    }
    match value.optimality_posture() {
        WorthQueryCandidateOptimalityPosture::NotApplicable => {
            hash_text_field(hash, "optimality-posture", "not-applicable")
        }
        WorthQueryCandidateOptimalityPosture::ProvenOptimal => {
            hash_text_field(hash, "optimality-posture", "proven-optimal")
        }
        WorthQueryCandidateOptimalityPosture::ProvenTopK { count } => {
            hash_text_field(hash, "optimality-posture", "proven-top-k");
            hash_text_field(hash, "optimality-top-k", &count.to_string());
        }
        WorthQueryCandidateOptimalityPosture::BoundedGap { bound_identity } => {
            hash_text_field(hash, "optimality-posture", "bounded-gap");
            hash_text_field(hash, "optimality-bound", bound_identity);
        }
        WorthQueryCandidateOptimalityPosture::BestInDeclaredSample { sample_identity } => {
            hash_text_field(hash, "optimality-posture", "best-in-declared-sample");
            hash_text_field(hash, "optimality-sample", sample_identity);
        }
        WorthQueryCandidateOptimalityPosture::ParetoForDeclaredSet { set_identity } => {
            hash_text_field(hash, "optimality-posture", "pareto-for-declared-set");
            hash_text_field(hash, "pareto-set", set_identity);
        }
        WorthQueryCandidateOptimalityPosture::FeasibleOnly => {
            hash_text_field(hash, "optimality-posture", "feasible-only")
        }
        WorthQueryCandidateOptimalityPosture::Unknown => {
            hash_text_field(hash, "optimality-posture", "unknown")
        }
    }
}

pub(super) fn hash_convergence(hash: &mut Sha256, value: &WorthQueryConvergenceContract) {
    match value {
        WorthQueryConvergenceContract::NotIterative => {
            hash_text_field(hash, "convergence", "not-iterative")
        }
        WorthQueryConvergenceContract::Iterative {
            progress_measure_family,
            comparator_family,
            repeated_state_family,
            incumbent,
            iteration_bound,
            oscillation,
        } => {
            hash_text_field(hash, "convergence", "iterative");
            hash_text_field(hash, "progress-measure", progress_measure_family);
            hash_text_field(hash, "convergence-comparator", comparator_family);
            hash_text_field(hash, "repeated-state-detector", repeated_state_family);
            hash_text_field(hash, "incumbent", incumbent_name(*incumbent));
            hash_text_field(hash, "iteration-bound", &iteration_bound.to_string());
            hash_text_field(hash, "oscillation", oscillation_name(*oscillation));
        }
    }
}

pub(super) fn hash_transformation(
    hash: &mut Sha256,
    value: &WorthQueryTransformationEvidenceContract,
) {
    match value {
        WorthQueryTransformationEvidenceContract::NotTransformation => {
            hash_text_field(hash, "transformation", "not-applicable")
        }
        WorthQueryTransformationEvidenceContract::Declared {
            source_occurrence,
            transformation,
            outcome,
        } => {
            hash_text_field(hash, "transformation", "declared");
            hash_text_field(
                hash,
                "source-occurrence-identity-family",
                source_occurrence.identity_family(),
            );
            hash_text_field(hash, "transformation-family", transformation.family());
            hash_text_field(
                hash,
                "transformation-version",
                &transformation.version().to_string(),
            );
            hash_text_field(
                hash,
                "correspondence",
                correspondence_name(outcome.correspondence()),
            );
            hash_text_field(hash, "disposition", disposition_name(outcome.disposition()));
            hash_text_field(hash, "error-posture", error_name(outcome.error()));
            hash_text_field(hash, "loss-posture", loss_name(outcome.loss()));
        }
    }
}
