use worth_proof::TransitionOutcome;

use crate::canonicalization::{
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
};
use crate::performance::claims::FoundationalPerformanceClaimSurface;
use crate::performance::receipts::FoundationalCounterBackedPerformanceReceipt;
use crate::performance::{
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBundle,
    FoundationalPerformanceCounterRow, FoundationalPerformanceCounterSpec,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceSupportingEvidenceRow, FoundationalPerformanceWorkClass,
};

use super::support::{
    access_pattern_token, allocation_posture_token, append_observation_context_entries,
    boundary_token, breadth_locality_token, claim_text_entry, counter_integer_entry,
    counter_text_entry, evidence_strength_token, execution_temperature_token, fallback_debt_token,
    freshness_retention_token, layout_bool_entry, layout_intent_token, layout_text_entry,
    support_text_entry, work_class_token,
};

pub fn prepare_performance_bundle_for_canonical_basis<Claim>(
    version: CanonicalizationRuleVersion,
    bundle: &FoundationalPerformanceBundle<Claim>,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Performance,
        canonical_basis_for_performance_bundle(bundle),
    )
}

pub fn prepare_counter_backed_performance_receipt_for_canonical_basis<Claim>(
    version: CanonicalizationRuleVersion,
    receipt: &FoundationalCounterBackedPerformanceReceipt<Claim>,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Performance,
        canonical_basis_for_counter_backed_receipt(receipt),
    )
}

pub fn foundational_performance_canonical_basis_entries(
    ready: &CanonicalBasisReadyArtifact,
) -> &[CanonicalBasisEntry] {
    ready.payload().entries()
}

fn canonical_basis_for_performance_bundle<Claim>(
    bundle: &FoundationalPerformanceBundle<Claim>,
) -> Vec<CanonicalBasisEntry>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    let mut entries = vec![claim_text_entry("shape", "performance-bundle")];
    append_claim_entries(bundle.claim(), &mut entries);
    append_layout_entries(bundle.layout_intent_claim(), &mut entries);
    append_contract_entries(bundle.contract_names(), &mut entries);
    append_counter_spec_entries(bundle.counter_specs(), &mut entries);
    append_support_entries(bundle.supporting_evidence_rows(), &mut entries);
    entries
}

fn canonical_basis_for_counter_backed_receipt<Claim>(
    receipt: &FoundationalCounterBackedPerformanceReceipt<Claim>,
) -> Vec<CanonicalBasisEntry>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    let mut entries = canonical_basis_for_performance_bundle(receipt.bundle());
    entries[0] = claim_text_entry("shape", "counter-backed-performance-receipt");
    append_counter_row_entries(receipt.counter_rows(), &mut entries);
    entries
}

fn append_claim_entries<Claim>(claim: &Claim, entries: &mut Vec<CanonicalBasisEntry>)
where
    Claim: FoundationalPerformanceClaimSurface,
{
    append_claim_surface_entries(
        claim.boundary(),
        claim.evidence_strength(),
        claim.breadth_locality(),
        claim.access_pattern(),
        claim.execution_temperature(),
        claim.freshness_retention(),
        claim.fallback_debt(),
        claim.included_work(),
        claim.excluded_work(),
        claim.observation_context(),
        entries,
    );
}

pub(super) fn append_claim_surface_entries(
    boundary: FoundationalPerformanceBoundary,
    evidence_strength: FoundationalPerformanceEvidenceStrength,
    breadth_locality: FoundationalPerformanceBreadthLocalityPosture,
    access_pattern: FoundationalPerformanceAccessPatternPosture,
    execution_temperature: FoundationalPerformanceExecutionTemperature,
    freshness_retention: FoundationalPerformanceFreshnessRetentionPosture,
    fallback_debt: FoundationalPerformanceFallbackDebtPosture,
    included_work: &[FoundationalPerformanceWorkClass],
    excluded_work: &[FoundationalPerformanceWorkClass],
    observation_context: Option<
        &crate::performance::claims::FoundationalPerformanceObservationContext,
    >,
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    entries.push(claim_text_entry("claim.boundary", boundary_token(boundary)));
    entries.push(claim_text_entry(
        "claim.evidence_strength",
        evidence_strength_token(evidence_strength),
    ));
    entries.push(claim_text_entry(
        "claim.breadth_locality",
        breadth_locality_token(breadth_locality),
    ));
    entries.push(claim_text_entry(
        "claim.access_pattern",
        access_pattern_token(access_pattern),
    ));
    entries.push(claim_text_entry(
        "claim.execution_temperature",
        execution_temperature_token(execution_temperature),
    ));
    entries.push(claim_text_entry(
        "claim.freshness_retention",
        freshness_retention_token(freshness_retention),
    ));
    entries.push(claim_text_entry(
        "claim.fallback_debt",
        fallback_debt_token(fallback_debt),
    ));
    for (ordinal, work_class) in included_work.iter().enumerate() {
        entries.push(claim_text_entry(
            &format!("claim.included_work.{ordinal}"),
            work_class_token(*work_class),
        ));
    }
    for (ordinal, work_class) in excluded_work.iter().enumerate() {
        entries.push(claim_text_entry(
            &format!("claim.excluded_work.{ordinal}"),
            work_class_token(*work_class),
        ));
    }
    append_observation_context_entries(observation_context, entries);
}

pub(super) fn append_layout_entries(
    layout: Option<&crate::performance::FoundationalLayoutIntentClaim>,
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    entries.push(layout_bool_entry("layout.present", layout.is_some()));
    if let Some(layout) = layout {
        entries.push(layout_text_entry(
            "layout.intent",
            layout_intent_token(layout.layout_intent()),
        ));
        entries.push(layout_text_entry(
            "layout.allocation_posture",
            allocation_posture_token(layout.allocation_posture()),
        ));
    }
}

pub(super) fn append_contract_entries(
    contracts: &[crate::performance::FoundationalPerformanceContractName],
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    for (ordinal, contract) in contracts.iter().enumerate() {
        entries.push(counter_text_entry(
            &format!("contract.{ordinal}.name"),
            contract.as_str(),
        ));
    }
}

pub(super) fn append_counter_spec_entries(
    counter_specs: &[FoundationalPerformanceCounterSpec],
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    for (ordinal, counter_spec) in counter_specs.iter().enumerate() {
        let prefix = format!("counter_spec.{ordinal}");
        entries.push(counter_text_entry(
            &format!("{prefix}.name"),
            counter_spec.name().as_str(),
        ));
        entries.push(counter_text_entry(
            &format!("{prefix}.work_class"),
            work_class_token(counter_spec.work_class()),
        ));
        entries.push(counter_integer_entry(
            &format!("{prefix}.expected_exact_count"),
            counter_spec.expected_exact_count(),
        ));
    }
}

pub(super) fn append_counter_row_entries(
    counter_rows: &[FoundationalPerformanceCounterRow],
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    for (ordinal, counter_row) in counter_rows.iter().enumerate() {
        let prefix = format!("counter_row.{ordinal}");
        entries.push(counter_text_entry(
            &format!("{prefix}.name"),
            counter_row.name().as_str(),
        ));
        entries.push(counter_integer_entry(
            &format!("{prefix}.observed_count"),
            counter_row.observed_count(),
        ));
    }
}

pub(super) fn append_support_entries(
    support_rows: &[FoundationalPerformanceSupportingEvidenceRow],
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    for (ordinal, support_row) in support_rows.iter().enumerate() {
        let prefix = format!("support_row.{ordinal}");
        entries.push(support_text_entry(
            &format!("{prefix}.code"),
            support_row.code().as_str(),
        ));
        entries.push(support_text_entry(
            &format!("{prefix}.related_work"),
            work_class_token(support_row.related_work()),
        ));
    }
}
