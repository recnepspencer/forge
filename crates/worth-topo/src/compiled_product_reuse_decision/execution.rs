use schema::facade::platform::authority::compiled_product_semantic_graph::admit_compiled_product_reuse_decision_identity;

use super::counters::TopologyDerivedReuseDecisionCounters;
use super::decision::TopologyDerivedReuseDecision;
use super::denial::TopologyDerivedRebuildDenial;
use super::execution_input::TopologyDerivedReuseExecutionInput;
use super::mismatch_locus::TopologyDerivedReuseMismatchLocus;
use super::posture::TopologyDerivedReuseDecisionPosture;
use super::resolution::TopologyDerivedReuseResolution;

#[cfg(test)]
pub fn decide_topology_derived_reuse(
    current: &TopologyDerivedReuseExecutionInput,
    prior: &TopologyDerivedReuseExecutionInput,
) -> TopologyDerivedReuseDecision {
    execute_topology_derived_reuse(current, prior)
        .decision()
        .clone()
}

pub fn execute_topology_derived_reuse(
    current: &TopologyDerivedReuseExecutionInput,
    prior: &TopologyDerivedReuseExecutionInput,
) -> TopologyDerivedReuseResolution {
    let selected_comparison = current.compare_selected_equivalence(prior);
    let mismatch_loci = mismatch_loci(current, prior, &selected_comparison);
    let counters = TopologyDerivedReuseDecisionCounters::new(8, 3);
    let posture = decision_posture(current, &mismatch_loci);
    let rebuild_denial = build_rebuild_denial(current, &mismatch_loci, posture, counters);
    let reuse_decision_identity_digest =
        if posture == TopologyDerivedReuseDecisionPosture::ReuseAdmitted {
            current
                .compiled_product_identity()
                .zip(current.equivalence_policy_identity())
                .map(|(compiled_product_identity, equivalence_policy_identity)| {
                    admit_compiled_product_reuse_decision_identity(
                        compiled_product_identity,
                        equivalence_policy_identity,
                        "ordinary-reuse-admitted",
                    )
                    .expect("topology derived reuse decision identity")
                    .identity_digest()
                    .to_string()
                })
        } else {
            None
        };
    let shared_identity_basis_present =
        has_shared_identity_basis(current) && has_shared_identity_basis(prior);
    let family_identity_match = current.topology_compiled_product_family_identity()
        == prior.topology_compiled_product_family_identity()
        && current.topology_compiled_product_family_digest()
            == prior.topology_compiled_product_family_digest();
    let authority_identity_match = shared_identity_basis_present
        && family_identity_match
        && current.authority_snapshot_id() == prior.authority_snapshot_id()
        && current.truth_basis_digest_hex() == prior.truth_basis_digest_hex()
        && current.authority_truth_identity_digest() == prior.authority_truth_identity_digest()
        && current.authoritative_mutation_origin() == prior.authoritative_mutation_origin()
        && current.touched_aspect_count() == prior.touched_aspect_count();
    let branch_identity_match = current.authority_branch_id() == prior.authority_branch_id();
    let invalidation_target_match =
        current.triggered_invalidation_targets() == prior.triggered_invalidation_targets();
    let decision = TopologyDerivedReuseDecision::new(
        posture,
        reuse_decision_identity_digest,
        rebuild_denial,
        current
            .compiled_product_identity_digest()
            .map(str::to_string),
        current
            .equivalence_policy_identity_digest()
            .map(str::to_string),
        current
            .selected_equivalence_family_identity()
            .map(|identity| identity.as_str().to_string()),
        current
            .selected_equivalence_basis_identity_digest()
            .map(str::to_string),
        current
            .selected_compatibility_basis_identity_digest()
            .map(str::to_string),
        current
            .selected_reuse_basis_identity_digest()
            .map(str::to_string),
        counters,
        selected_comparison.comparison_supported,
        selected_comparison
            .unsupported_comparison_reason
            .as_deref()
            .map(str::to_string),
    );

    TopologyDerivedReuseResolution::new(
        decision,
        authority_identity_match,
        branch_identity_match,
        invalidation_target_match,
        selected_comparison.materialized_topology_digest_match,
        selected_comparison.interpreted_topology_digest_match,
        selected_comparison.derived_validation_digest_match,
        shared_identity_basis_present
            && family_identity_match
            && selected_comparison.equivalent_derived_meaning,
    )
}

fn build_rebuild_denial(
    current: &TopologyDerivedReuseExecutionInput,
    mismatch_loci: &[TopologyDerivedReuseMismatchLocus],
    posture: TopologyDerivedReuseDecisionPosture,
    counters: TopologyDerivedReuseDecisionCounters,
) -> Option<TopologyDerivedRebuildDenial> {
    if posture == TopologyDerivedReuseDecisionPosture::ReuseAdmitted {
        return None;
    }
    let denial_reason = match posture {
        TopologyDerivedReuseDecisionPosture::FreshRebuildRequired => {
            "topology-derived-fresh-rebuild-required"
        }
        TopologyDerivedReuseDecisionPosture::AdvisoryMatchRequiresRebuild => {
            "topology-derived-advisory-match-requires-rebuild"
        }
        TopologyDerivedReuseDecisionPosture::Denied => "topology-derived-reuse-denied",
        TopologyDerivedReuseDecisionPosture::ReuseAdmitted => unreachable!(),
    };
    current.compiled_product_identity().map(|compiled_product_identity| {
            TopologyDerivedRebuildDenial::new(
                schema::facade::platform::authority::compiled_product_semantic_graph::admit_compiled_product_rebuild_denial_identity(
                    compiled_product_identity,
                    denial_reason,
                )
                .expect("topology derived rebuild denial identity")
                .identity_digest()
                .to_string(),
                mismatch_loci.to_vec(),
                current.compiled_product_identity_digest().map(str::to_string),
                current.equivalence_policy_identity_digest().map(str::to_string),
                current
                    .selected_equivalence_family_identity()
                    .map(|identity| identity.as_str().to_string()),
                current
                    .selected_equivalence_basis_identity_digest()
                    .map(str::to_string),
                current
                    .selected_compatibility_basis_identity_digest()
                    .map(str::to_string),
                current.selected_reuse_basis_identity_digest().map(str::to_string),
                counters,
            )
        })
}

fn decision_posture(
    current: &TopologyDerivedReuseExecutionInput,
    mismatch_loci: &[TopologyDerivedReuseMismatchLocus],
) -> TopologyDerivedReuseDecisionPosture {
    if mismatch_loci.is_empty() {
        return TopologyDerivedReuseDecisionPosture::ReuseAdmitted;
    }
    if mismatch_loci == [TopologyDerivedReuseMismatchLocus::SelectedReuseBasisIdentity]
        && current
            .selected_compatibility_basis_identity_digest()
            .is_some()
    {
        return TopologyDerivedReuseDecisionPosture::AdvisoryMatchRequiresRebuild;
    }
    if mismatch_loci.iter().any(|locus| {
        matches!(
            locus,
            TopologyDerivedReuseMismatchLocus::MissingSelectedFamilyContract
                | TopologyDerivedReuseMismatchLocus::ComparatorContract
                | TopologyDerivedReuseMismatchLocus::TopologyCompiledProductFamilyIdentity
                | TopologyDerivedReuseMismatchLocus::TopologyCompiledProductFamilyDigest
                | TopologyDerivedReuseMismatchLocus::EquivalencePolicyIdentity
                | TopologyDerivedReuseMismatchLocus::SelectedEquivalenceFamilyIdentity
        )
    }) {
        return TopologyDerivedReuseDecisionPosture::Denied;
    }
    TopologyDerivedReuseDecisionPosture::FreshRebuildRequired
}

fn mismatch_loci(
    current: &TopologyDerivedReuseExecutionInput,
    prior: &TopologyDerivedReuseExecutionInput,
    selected_comparison: &crate::selected_equivalence_family::TopologySelectedEquivalenceComparisonReport,
) -> Vec<TopologyDerivedReuseMismatchLocus> {
    let mut loci = Vec::new();
    if current.selected_equivalence_family_identity().is_none()
        || prior.selected_equivalence_family_identity().is_none()
    {
        loci.push(TopologyDerivedReuseMismatchLocus::MissingSelectedFamilyContract);
    }
    if !selected_comparison.comparison_supported {
        loci.push(TopologyDerivedReuseMismatchLocus::ComparatorContract);
    }
    if current.topology_compiled_product_family_identity()
        != prior.topology_compiled_product_family_identity()
    {
        loci.push(TopologyDerivedReuseMismatchLocus::TopologyCompiledProductFamilyIdentity);
    }
    if current.topology_compiled_product_family_digest()
        != prior.topology_compiled_product_family_digest()
    {
        loci.push(TopologyDerivedReuseMismatchLocus::TopologyCompiledProductFamilyDigest);
    }
    if current.authority_truth_identity_digest() != prior.authority_truth_identity_digest() {
        loci.push(TopologyDerivedReuseMismatchLocus::AuthorityTruthIdentity);
    }
    if current.compiled_product_identity_digest() != prior.compiled_product_identity_digest() {
        loci.push(TopologyDerivedReuseMismatchLocus::CompiledProductIdentity);
    }
    if current.equivalence_policy_identity_digest() != prior.equivalence_policy_identity_digest() {
        loci.push(TopologyDerivedReuseMismatchLocus::EquivalencePolicyIdentity);
    }
    if current.selected_equivalence_family_identity()
        != prior.selected_equivalence_family_identity()
    {
        loci.push(TopologyDerivedReuseMismatchLocus::SelectedEquivalenceFamilyIdentity);
    }
    if current.selected_equivalence_basis_identity_digest()
        != prior.selected_equivalence_basis_identity_digest()
    {
        loci.push(TopologyDerivedReuseMismatchLocus::SelectedEquivalenceBasisIdentity);
    }
    if current.selected_reuse_basis_identity_digest()
        != prior.selected_reuse_basis_identity_digest()
    {
        loci.push(TopologyDerivedReuseMismatchLocus::SelectedReuseBasisIdentity);
    }
    if current.authority_branch_id() != prior.authority_branch_id() {
        loci.push(TopologyDerivedReuseMismatchLocus::BranchIdentity);
    }
    if current.triggered_invalidation_targets() != prior.triggered_invalidation_targets() {
        loci.push(TopologyDerivedReuseMismatchLocus::InvalidationTargets);
    }
    if !selected_comparison.materialized_topology_digest_match {
        loci.push(TopologyDerivedReuseMismatchLocus::MaterializedTopologyDigest);
    }
    if !selected_comparison.interpreted_topology_digest_match {
        loci.push(TopologyDerivedReuseMismatchLocus::InterpretedTopologyDigest);
    }
    if !selected_comparison.derived_validation_digest_match {
        loci.push(TopologyDerivedReuseMismatchLocus::DerivedValidationDigest);
    }
    loci
}

fn has_shared_identity_basis(input: &TopologyDerivedReuseExecutionInput) -> bool {
    input.authority_truth_identity().is_some()
        && input.compiled_product_identity().is_some()
        && input.equivalence_policy_identity().is_some()
}
