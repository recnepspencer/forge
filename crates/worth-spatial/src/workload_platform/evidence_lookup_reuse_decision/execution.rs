use schema::facade::platform::authority::compiled_product_semantic_graph::admit_compiled_product_reuse_decision_identity;

use crate::workload_platform::evidence_lookup_index_product::{
    rebuild_required_identity, EvidenceLookupIndexProduct, EvidenceLookupIndexProductError,
};
use crate::workload_platform::selected_equivalence_family::SelectedSpatialEquivalenceFamily;

use super::counters::EvidenceLookupReuseDecisionCounters;
use super::decision::EvidenceLookupIndexReuseDecision;
use super::denial::EvidenceLookupIndexRebuildDenial;
use super::execution_input::EvidenceLookupIndexReuseExecutionInput;
use super::mismatch_locus::EvidenceLookupReuseMismatchLocus;
use super::posture::EvidenceLookupReuseDecisionPosture;
use super::resolution::EvidenceLookupIndexReuseResolution;

pub fn decide_evidence_lookup_index_reuse(
    current_input: &EvidenceLookupIndexReuseExecutionInput,
    prior_product: &EvidenceLookupIndexProduct,
) -> EvidenceLookupIndexReuseDecision {
    let mismatch_loci = mismatch_loci(current_input, prior_product);
    let posture = decision_posture(current_input.selected_equivalence_family(), &mismatch_loci);
    let denial = build_rebuild_denial(current_input, &mismatch_loci, posture);
    let reuse_decision_identity_digest =
        if posture == EvidenceLookupReuseDecisionPosture::ReuseAdmitted {
            Some(
                admit_compiled_product_reuse_decision_identity(
                    current_input.lowered_identity().compiled_product_identity(),
                    current_input
                        .lowered_identity()
                        .equivalence_policy_identity(),
                    "ordinary-reuse-admitted",
                )
                .expect("evidence lookup reuse decision identity")
                .identity_digest()
                .to_string(),
            )
        } else {
            None
        };

    EvidenceLookupIndexReuseDecision::new(
        posture,
        current_input
            .lowered_identity()
            .compiled_product_identity()
            .identity_digest()
            .to_string(),
        current_input
            .lowered_identity()
            .equivalence_policy_identity()
            .identity_digest()
            .to_string(),
        current_input
            .selected_equivalence_family()
            .family_identity(),
        current_input
            .selected_equivalence_family()
            .equivalence_basis_identity()
            .identity_digest()
            .to_string(),
        current_input
            .selected_equivalence_family()
            .compatibility_basis_identity()
            .identity_digest()
            .to_string(),
        current_input
            .selected_equivalence_family()
            .reuse_basis_identity()
            .identity_digest()
            .to_string(),
        reuse_decision_identity_digest,
        denial,
        EvidenceLookupReuseDecisionCounters::new(
            *current_input.counters(),
            8,
            current_input.raw_evidence_row_scan_count(),
            current_input.broad_receipt_scan_count(),
            current_input.caller_owned_evidence_work_count(),
        ),
    )
}

pub fn execute_evidence_lookup_index_reuse(
    decision: EvidenceLookupIndexReuseDecision,
    current_input: &EvidenceLookupIndexReuseExecutionInput,
    prior_product: &EvidenceLookupIndexProduct,
) -> Result<EvidenceLookupIndexReuseResolution, EvidenceLookupIndexProductError> {
    match decision.posture() {
        EvidenceLookupReuseDecisionPosture::ReuseAdmitted => {
            let reuse_decision_identity_digest = decision
                .reuse_decision_identity_digest()
                .map(str::to_string);
            let counters = *decision.product_counters();
            Ok(EvidenceLookupIndexReuseResolution::Reused {
                decision,
                product: EvidenceLookupIndexProduct::new(
                    current_input.lowered_identity(),
                    current_input.selected_equivalence_family(),
                    current_input.selected_plan_digest().to_string(),
                    current_input.spatial_touch_digest().to_string(),
                    current_input.stage_receipt_digest().to_string(),
                    current_input.evidence_ledger_basis_digest().to_string(),
                    current_input.topology_support_digest().to_string(),
                    current_input.query_support_digest().to_string(),
                    reuse_decision_identity_digest,
                    current_input.query_surface_contract_rows().to_vec(),
                    current_input.lifecycle_posture(),
                    current_input.disposal_posture(),
                    counters.reused_from(),
                    prior_product.rows().to_vec(),
                ),
            })
        }
        EvidenceLookupReuseDecisionPosture::FreshRebuildRequired
        | EvidenceLookupReuseDecisionPosture::AdvisoryMatchRequiresRebuild => {
            Ok(EvidenceLookupIndexReuseResolution::Rebuilt {
                product: EvidenceLookupIndexProduct::new(
                    current_input.lowered_identity(),
                    current_input.selected_equivalence_family(),
                    current_input.selected_plan_digest().to_string(),
                    current_input.spatial_touch_digest().to_string(),
                    current_input.stage_receipt_digest().to_string(),
                    current_input.evidence_ledger_basis_digest().to_string(),
                    current_input.topology_support_digest().to_string(),
                    current_input.query_support_digest().to_string(),
                    None,
                    current_input.query_surface_contract_rows().to_vec(),
                    current_input.lifecycle_posture(),
                    current_input.disposal_posture(),
                    *current_input.counters(),
                    current_input.rows().to_vec(),
                ),
                decision,
            })
        }
        EvidenceLookupReuseDecisionPosture::Denied => {
            let denial = decision
                .rebuild_denial()
                .expect("denied decision must carry rebuild denial")
                .clone();
            Ok(EvidenceLookupIndexReuseResolution::Denied { decision, denial })
        }
    }
}

fn build_rebuild_denial(
    current_input: &EvidenceLookupIndexReuseExecutionInput,
    mismatch_loci: &[EvidenceLookupReuseMismatchLocus],
    posture: EvidenceLookupReuseDecisionPosture,
) -> Option<EvidenceLookupIndexRebuildDenial> {
    if posture == EvidenceLookupReuseDecisionPosture::ReuseAdmitted {
        return None;
    }
    let denial_reason = match posture {
        EvidenceLookupReuseDecisionPosture::FreshRebuildRequired => {
            "evidence-lookup-index-fresh-rebuild-required"
        }
        EvidenceLookupReuseDecisionPosture::AdvisoryMatchRequiresRebuild => {
            "evidence-lookup-index-advisory-match-requires-rebuild"
        }
        EvidenceLookupReuseDecisionPosture::Denied => "evidence-lookup-index-reuse-denied",
        EvidenceLookupReuseDecisionPosture::ReuseAdmitted => unreachable!(),
    };
    let denial_identity = rebuild_required_identity(
        current_input.lowered_identity().compiled_product_identity(),
        denial_reason,
    );
    Some(EvidenceLookupIndexRebuildDenial::new(
        denial_identity.identity_digest().to_string(),
        mismatch_loci.to_vec(),
        current_input
            .selected_equivalence_family()
            .family_identity(),
        current_input
            .selected_equivalence_family()
            .equivalence_basis_identity()
            .identity_digest()
            .to_string(),
        current_input
            .selected_equivalence_family()
            .compatibility_basis_identity()
            .identity_digest()
            .to_string(),
        current_input
            .selected_equivalence_family()
            .reuse_basis_identity()
            .identity_digest()
            .to_string(),
        *current_input.counters(),
    ))
}

fn decision_posture(
    selected_family: &SelectedSpatialEquivalenceFamily,
    mismatch_loci: &[EvidenceLookupReuseMismatchLocus],
) -> EvidenceLookupReuseDecisionPosture {
    if mismatch_loci.is_empty() {
        return EvidenceLookupReuseDecisionPosture::ReuseAdmitted;
    }
    if mismatch_loci
        == [EvidenceLookupReuseMismatchLocus::SelectedReuseBasisIdentity]
        && matches!(
            selected_family.compatibility_posture(),
            crate::workload_platform::selected_equivalence_family::SpatialCompatibilityPosture::DistinctFromEquivalence
        )
    {
        return EvidenceLookupReuseDecisionPosture::AdvisoryMatchRequiresRebuild;
    }
    if mismatch_loci.iter().any(|locus| {
        matches!(
            locus,
            EvidenceLookupReuseMismatchLocus::EquivalencePolicyIdentity
                | EvidenceLookupReuseMismatchLocus::SelectedEquivalenceFamilyIdentity
                | EvidenceLookupReuseMismatchLocus::SelectedCompatibilityBasisIdentity
        )
    }) {
        return EvidenceLookupReuseDecisionPosture::Denied;
    }
    EvidenceLookupReuseDecisionPosture::FreshRebuildRequired
}

fn mismatch_loci(
    current_input: &EvidenceLookupIndexReuseExecutionInput,
    prior_product: &EvidenceLookupIndexProduct,
) -> Vec<EvidenceLookupReuseMismatchLocus> {
    let mut loci = Vec::new();
    if prior_product.spatial_touch_digest() != current_input.spatial_touch_digest() {
        loci.push(EvidenceLookupReuseMismatchLocus::SpatialTouchAuthorityDigest);
    }
    if prior_product.stage_receipt_digest() != current_input.stage_receipt_digest() {
        loci.push(EvidenceLookupReuseMismatchLocus::StageReceiptDigest);
    }
    if prior_product.evidence_ledger_basis_digest() != current_input.evidence_ledger_basis_digest()
    {
        loci.push(EvidenceLookupReuseMismatchLocus::EvidenceLedgerBasisDigest);
    }
    if prior_product.topology_support_digest() != current_input.topology_support_digest() {
        loci.push(EvidenceLookupReuseMismatchLocus::TopologySupportDigest);
    }
    if prior_product.query_support_digest() != current_input.query_support_digest() {
        loci.push(EvidenceLookupReuseMismatchLocus::QuerySupportDigest);
    }
    if prior_product.equivalence_policy_identity_digest()
        != current_input
            .lowered_identity()
            .equivalence_policy_identity()
            .identity_digest()
    {
        loci.push(EvidenceLookupReuseMismatchLocus::EquivalencePolicyIdentity);
    }
    if prior_product.selected_equivalence_family_identity()
        != current_input
            .selected_equivalence_family()
            .family_identity()
    {
        loci.push(EvidenceLookupReuseMismatchLocus::SelectedEquivalenceFamilyIdentity);
    }
    if prior_product.selected_equivalence_basis_identity_digest()
        != current_input
            .selected_equivalence_family()
            .equivalence_basis_identity()
            .identity_digest()
    {
        loci.push(EvidenceLookupReuseMismatchLocus::SelectedEquivalenceBasisIdentity);
    }
    if prior_product.selected_compatibility_basis_identity_digest()
        != current_input
            .selected_equivalence_family()
            .compatibility_basis_identity()
            .identity_digest()
    {
        loci.push(EvidenceLookupReuseMismatchLocus::SelectedCompatibilityBasisIdentity);
    }
    if prior_product.selected_reuse_basis_identity_digest()
        != current_input
            .selected_equivalence_family()
            .reuse_basis_identity()
            .identity_digest()
    {
        loci.push(EvidenceLookupReuseMismatchLocus::SelectedReuseBasisIdentity);
    }
    loci
}
