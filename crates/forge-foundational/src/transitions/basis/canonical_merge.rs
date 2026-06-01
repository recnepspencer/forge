use crate::canonicalization::CanonicalBasisEntry;
use crate::transitions::{
    FoundationalMergeConflictLocus, FoundationalMergeVerdict, FoundationalMergeVerdictKind,
    FoundationalTransitionStrategyOwnershipClass,
};

use super::canonical_scope::admitted_scope_entries;
use super::canonical_shared::{bool_entry, text_entry, u64_entry};

pub(super) fn merge_verdict_entries<T>(
    verdict: &FoundationalMergeVerdict<T>,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        text_entry("merge.kind", merge_verdict_token(verdict.kind())),
        text_entry("merge.source_branch", verdict.source_branch().as_str()),
        text_entry("merge.target_branch", verdict.target_branch().as_str()),
        text_entry("merge.intent", "reconcile-into-target"),
        u64_entry(
            "merge.summary.source_scope_width",
            verdict.structural_summary().source_scope_width(),
        ),
        u64_entry(
            "merge.summary.target_scope_width",
            verdict.structural_summary().target_scope_width(),
        ),
        u64_entry(
            "merge.summary.touched_scope_width",
            verdict.structural_summary().touched_scope_width(),
        ),
        u64_entry(
            "merge.summary.conflict_check_width",
            verdict.structural_summary().conflict_check_width(),
        ),
    ];
    entries.extend(admitted_scope_entries(
        "merge.scope_evidence",
        verdict.scope_evidence(),
    ));
    append_strategy_and_basis_entries("merge", verdict, &mut entries);
    append_conflict_entries(verdict.conflict_loci(), &mut entries);
    entries.push(bool_entry(
        "merge.has_superseded_by_branch",
        verdict.superseded_by_branch().is_some(),
    ));
    if let Some(branch) = verdict.superseded_by_branch() {
        entries.push(text_entry("merge.superseded_by_branch", branch.as_str()));
    }
    entries
}

pub(super) fn append_strategy_and_basis_entries<T>(
    prefix: &str,
    verdict: &FoundationalMergeVerdict<T>,
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    entries.push(u64_entry(
        &format!("{prefix}.merge_basis.identity"),
        verdict.merge_basis().identity().basis_id().get(),
    ));
    entries.push(text_entry(
        &format!("{prefix}.merge_basis.family"),
        verdict.merge_basis().family().as_str(),
    ));
    entries.push(text_entry(
        &format!("{prefix}.merge_basis.version"),
        verdict.merge_basis().version().as_str(),
    ));
    entries.push(u64_entry(
        &format!("{prefix}.merge_base_selection_basis"),
        verdict.merge_base_selection_basis().basis_id().get(),
    ));
    entries.push(u64_entry(
        &format!("{prefix}.strategy.id"),
        verdict.strategy_identity().id().handle().get(),
    ));
    entries.push(text_entry(
        &format!("{prefix}.strategy.family"),
        verdict.strategy_identity().family().as_str(),
    ));
    entries.push(text_entry(
        &format!("{prefix}.strategy.semantic_name"),
        verdict.strategy_identity().semantic_name().as_str(),
    ));
    entries.push(text_entry(
        &format!("{prefix}.strategy.version"),
        verdict.strategy_identity().version().as_str(),
    ));
    entries.push(text_entry(
        &format!("{prefix}.strategy.ownership"),
        strategy_ownership_token(verdict.strategy_identity().ownership()),
    ));
    entries.push(u64_entry(
        &format!("{prefix}.strategy.contract_basis"),
        verdict.strategy_contract_basis().basis_id().get(),
    ));
    entries.push(u64_entry(
        &format!("{prefix}.strategy.basis"),
        verdict.strategy_basis().basis_id().get(),
    ));
    entries.push(bool_entry(
        &format!("{prefix}.has_comparison_basis"),
        verdict.comparison_basis().is_some(),
    ));
    if let Some(basis) = verdict.comparison_basis() {
        entries.push(u64_entry(
            &format!("{prefix}.comparison_basis"),
            basis.basis_id().get(),
        ));
        entries.push(text_entry(
            &format!("{prefix}.compared_against_branch"),
            basis.compared_against_branch().as_str(),
        ));
    }
    entries.push(bool_entry(
        &format!("{prefix}.has_correspondence_basis"),
        verdict.correspondence_basis().is_some(),
    ));
    if let Some(basis) = verdict.correspondence_basis() {
        entries.push(u64_entry(
            &format!("{prefix}.correspondence_basis"),
            basis.basis_id().get(),
        ));
    }
    entries.push(bool_entry(
        &format!("{prefix}.has_remap_basis"),
        verdict.remap_basis().is_some(),
    ));
    if let Some(basis) = verdict.remap_basis() {
        entries.push(u64_entry(
            &format!("{prefix}.remap_basis"),
            basis.basis_id().get(),
        ));
    }
}

fn append_conflict_entries(
    conflicts: &[FoundationalMergeConflictLocus],
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    let mut ordered = conflicts.to_vec();
    ordered.sort_by(|left, right| {
        left.category()
            .cmp(right.category())
            .then(left.source_detail().cmp(right.source_detail()))
            .then(left.target_detail().cmp(right.target_detail()))
    });
    entries.extend(ordered.iter().enumerate().flat_map(|(index, locus)| {
        [
            text_entry(
                &format!("merge.conflict.{index}.category"),
                locus.category(),
            ),
            text_entry(
                &format!("merge.conflict.{index}.source_detail"),
                locus.source_detail(),
            ),
            text_entry(
                &format!("merge.conflict.{index}.target_detail"),
                locus.target_detail(),
            ),
        ]
    }));
}

fn merge_verdict_token(kind: FoundationalMergeVerdictKind) -> &'static str {
    match kind {
        FoundationalMergeVerdictKind::Accepted => "accepted",
        FoundationalMergeVerdictKind::Advisory => "advisory",
        FoundationalMergeVerdictKind::Conflict => "conflict",
        FoundationalMergeVerdictKind::Denied => "denied",
        FoundationalMergeVerdictKind::Superseded => "superseded",
        FoundationalMergeVerdictKind::StaleBasis => "stale-basis",
    }
}

pub(super) fn strategy_ownership_token(
    ownership: FoundationalTransitionStrategyOwnershipClass,
) -> &'static str {
    match ownership {
        FoundationalTransitionStrategyOwnershipClass::RuntimeBuiltIn => "runtime-built-in",
        FoundationalTransitionStrategyOwnershipClass::CustomRegistered => "custom-registered",
        FoundationalTransitionStrategyOwnershipClass::CompatibilityLowered => {
            "compatibility-lowered"
        }
    }
}
