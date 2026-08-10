use crate::frontier_planning::FrontierSurfaceDigest;
use worth_signal::facade::adapters::{
    FrontierExecutionSummary, FrontierPlan, FrontierWaveEntryPlan, FrontierWaveEntrySummary,
    FrontierWavePlan, FrontierWaveSummary, TouchedScopeSummary, TransitiveFrontierRoot,
};

use super::frontier_surface_model::SignalFrontierSurfaceEvidence;

impl SignalFrontierSurfaceEvidence {
    pub fn from_frontier_plan(plan: &FrontierPlan) -> Self {
        let mut parts = vec![
            format!("seed_count:{}", plan.seed_batch.as_slice().len()),
            format!("direct_wave_count:{}", plan.direct_waves.len()),
            format!("transitive_root_count:{}", plan.transitive_roots.len()),
            format!("predicted_seed_count:{}", plan.predicted.seed_count),
            format!("predicted_group_count:{}", plan.predicted.group_count),
            format!(
                "predicted_direct_wave_count:{}",
                plan.predicted.direct_wave_count
            ),
            format!(
                "predicted_transitive_wave_count:{}",
                plan.predicted.transitive_wave_count
            ),
        ];
        parts.extend(touched_scope_digest_parts(
            "touched_scope",
            &plan.touched_scope_summary,
        ));
        for (index, wave) in plan.direct_waves.iter().enumerate() {
            parts.extend(frontier_wave_plan_digest_parts(index, wave));
        }
        for (index, root) in plan.transitive_roots.iter().enumerate() {
            parts.extend(transitive_root_digest_parts(index, root));
        }

        Self::from_materialized_surface(
            FrontierSurfaceDigest::from_label(&parts.join("|")),
            predicted_frontier_breadth(plan),
            None,
        )
    }

    pub fn from_frontier_execution_summary(summary: &FrontierExecutionSummary) -> Self {
        let mut parts = vec![
            format!("seed_count:{}", summary.seed_count),
            format!("direct_wave_count:{}", summary.direct_waves.len()),
            format!("transitive_wave_count:{}", summary.transitive_waves.len()),
            format!(
                "counter_frontier_seed_count:{}",
                summary.counters.frontier_seed_count
            ),
            format!(
                "counter_frontier_group_count:{}",
                summary.counters.frontier_group_count
            ),
            format!(
                "counter_frontier_direct_wave_count:{}",
                summary.counters.frontier_direct_wave_count
            ),
            format!(
                "counter_frontier_transitive_wave_count:{}",
                summary.counters.frontier_transitive_wave_count
            ),
        ];
        parts.extend(touched_scope_digest_parts(
            "touched_scope",
            &summary.touched_scope_summary,
        ));
        for (index, wave) in summary.direct_waves.iter().enumerate() {
            parts.extend(frontier_wave_summary_digest_parts("direct", index, wave));
        }
        for (index, wave) in summary.transitive_waves.iter().enumerate() {
            parts.extend(frontier_wave_summary_digest_parts(
                "transitive",
                index,
                wave,
            ));
        }

        Self::from_materialized_surface(
            FrontierSurfaceDigest::from_label(&parts.join("|")),
            summary.seed_count as usize
                + summary.counters.frontier_direct_wave_count as usize
                + summary.counters.frontier_transitive_wave_count as usize,
            Some(realized_frontier_breadth(summary)),
        )
    }
}

fn predicted_frontier_breadth(plan: &FrontierPlan) -> usize {
    plan.predicted.seed_count as usize
        + plan.predicted.direct_wave_count as usize
        + plan.predicted.transitive_wave_count as usize
        + plan.predicted.partition_match_count as usize
        + plan.predicted.detail_match_count as usize
}

fn realized_frontier_breadth(summary: &FrontierExecutionSummary) -> usize {
    summary.seed_count as usize
        + summary
            .direct_waves
            .iter()
            .map(|wave| wave.entries.len())
            .sum::<usize>()
        + summary
            .transitive_waves
            .iter()
            .map(|wave| wave.entries.len())
            .sum::<usize>()
}

fn touched_scope_digest_parts(prefix: &str, scope: &TouchedScopeSummary) -> Vec<String> {
    let mut parts = vec![
        format!("{prefix}.seed_scopes:{}", scope.seed_scopes.len()),
        format!("{prefix}.inclusion_scopes:{}", scope.inclusion_scopes.len()),
        format!(
            "{prefix}.transitive_reached_scopes:{}",
            scope.transitive_reached_scopes.len()
        ),
        format!(
            "{prefix}.direct_dirty_scopes:{}",
            scope.direct_dirty_scopes.len()
        ),
        format!(
            "{prefix}.maybe_stale_scopes:{}",
            scope.maybe_stale_scopes.len()
        ),
        format!("{prefix}.touched_nodes:{}", scope.touched_nodes.len()),
        format!("{prefix}.touched_sources:{}", scope.touched_sources.len()),
    ];
    for (index, scope_entry) in scope.seed_scopes.iter().enumerate() {
        parts.push(format!("{prefix}.seed_scope[{index}]:{scope_entry:?}"));
    }
    for (index, scope_entry) in scope.inclusion_scopes.iter().enumerate() {
        parts.push(format!("{prefix}.inclusion_scope[{index}]:{scope_entry:?}"));
    }
    parts
}

fn frontier_wave_plan_digest_parts(index: usize, wave: &FrontierWavePlan) -> Vec<String> {
    let mut parts = vec![
        format!("direct_wave[{index}].wave_index:{}", wave.wave_index),
        format!("direct_wave[{index}].aspect:{}", wave.aspect.id()),
        format!("direct_wave[{index}].entry_count:{}", wave.entries.len()),
    ];
    for (entry_index, entry) in wave.entries.iter().enumerate() {
        parts.extend(frontier_wave_entry_plan_digest_parts(
            &format!("direct_wave[{index}].entry[{entry_index}]"),
            entry,
        ));
    }
    parts
}

fn frontier_wave_summary_digest_parts(
    label: &str,
    index: usize,
    wave: &FrontierWaveSummary,
) -> Vec<String> {
    let mut parts = vec![
        format!("{label}_wave[{index}].wave_index:{}", wave.wave_index),
        format!("{label}_wave[{index}].aspect:{}", wave.aspect.id()),
        format!("{label}_wave[{index}].entry_count:{}", wave.entries.len()),
    ];
    for (entry_index, entry) in wave.entries.iter().enumerate() {
        parts.extend(frontier_wave_entry_summary_digest_parts(
            &format!("{label}_wave[{index}].entry[{entry_index}]"),
            entry,
        ));
    }
    parts
}

fn frontier_wave_entry_plan_digest_parts(
    prefix: &str,
    entry: &FrontierWaveEntryPlan,
) -> Vec<String> {
    let mut parts = vec![
        format!(
            "{prefix}.node:{}:{}",
            entry.node.index(),
            entry.node.generation()
        ),
        format!("{prefix}.classification:{:?}", entry.classification),
        format!("{prefix}.inclusion_basis:{:?}", entry.inclusion_basis),
        format!(
            "{prefix}.narrowed_scope_count:{}",
            entry.narrowed_scopes.len()
        ),
    ];
    for (index, scope) in entry.narrowed_scopes.iter().enumerate() {
        parts.push(format!("{prefix}.scope[{index}]:{scope:?}"));
    }
    for (index, seed_ref) in entry.source_seed_refs.iter().enumerate() {
        parts.push(format!("{prefix}.seed_ref[{index}]:{seed_ref}"));
    }
    parts
}

fn frontier_wave_entry_summary_digest_parts(
    prefix: &str,
    entry: &FrontierWaveEntrySummary,
) -> Vec<String> {
    let mut parts = vec![
        format!(
            "{prefix}.node:{}:{}",
            entry.node.index(),
            entry.node.generation()
        ),
        format!("{prefix}.classification:{:?}", entry.classification),
        format!("{prefix}.inclusion_basis:{:?}", entry.inclusion_basis),
        format!(
            "{prefix}.narrowed_scope_count:{}",
            entry.narrowed_scopes.len()
        ),
    ];
    for (index, scope) in entry.narrowed_scopes.iter().enumerate() {
        parts.push(format!("{prefix}.scope[{index}]:{scope:?}"));
    }
    parts
}

fn transitive_root_digest_parts(index: usize, root: &TransitiveFrontierRoot) -> Vec<String> {
    let mut parts = vec![
        format!(
            "transitive_root[{index}].node:{}:{}",
            root.node.index(),
            root.node.generation()
        ),
        format!("transitive_root[{index}].aspect:{}", root.aspect.id()),
        format!(
            "transitive_root[{index}].classification:{:?}",
            root.classification
        ),
        format!(
            "transitive_root[{index}].narrowed_scope_count:{}",
            root.narrowed_scopes.len()
        ),
    ];
    for (scope_index, scope) in root.narrowed_scopes.iter().enumerate() {
        parts.push(format!(
            "transitive_root[{index}].scope[{scope_index}]:{scope:?}"
        ));
    }
    for (seed_ref_index, seed_ref) in root.source_seed_refs.iter().enumerate() {
        parts.push(format!(
            "transitive_root[{index}].seed_ref[{seed_ref_index}]:{seed_ref}"
        ));
    }
    parts
}
