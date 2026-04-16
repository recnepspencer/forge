use crate::frontier_planning::{
    FrontierDisjointnessClass, FrontierPredictionDriftOutcome, FrontierRouteEvidence,
    FrontierSurfaceDigest, ParallelAdmissionEvidence, SerialFallbackBundleEvidence,
    SerialFallbackEvidence, SerialFallbackReason,
};
use forge_signal::facade::adapters::{
    FrontierExecutionSummary, FrontierPlan, FrontierWaveEntryPlan, FrontierWaveEntrySummary,
    FrontierWavePlan, FrontierWaveSummary, TouchedScopeSummary, TransitiveFrontierRoot,
};
use forge_signal::facade::specialist::{ParallelAdmissionReason, StageExecutionRecord};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalFrontierSurfaceEvidence {
    surface_digest: FrontierSurfaceDigest,
    predicted_breadth: usize,
    realized_breadth: Option<usize>,
}

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

        Self {
            surface_digest: FrontierSurfaceDigest::from_label(&parts.join("|")),
            predicted_breadth: predicted_frontier_breadth(plan),
            realized_breadth: None,
        }
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

        Self {
            surface_digest: FrontierSurfaceDigest::from_label(&parts.join("|")),
            predicted_breadth: summary.seed_count as usize
                + summary.counters.frontier_direct_wave_count as usize
                + summary.counters.frontier_transitive_wave_count as usize,
            realized_breadth: Some(realized_frontier_breadth(summary)),
        }
    }

    pub fn surface_digest(&self) -> &FrontierSurfaceDigest {
        &self.surface_digest
    }

    pub fn predicted_breadth(&self) -> usize {
        self.predicted_breadth
    }

    pub fn realized_breadth(&self) -> Option<usize> {
        self.realized_breadth
    }

    #[cfg_attr(not(test), allow(dead_code))]
    #[allow(dead_code)]
    pub(crate) fn to_parallel_admission_evidence(
        &self,
        basis_digest: &str,
        disjointness_class: FrontierDisjointnessClass,
    ) -> ParallelAdmissionEvidence {
        ParallelAdmissionEvidence::new(FrontierRouteEvidence::parallel_admission(
            basis_digest.to_string(),
            self.surface_digest.clone(),
            disjointness_class,
        ))
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn to_serial_fallback_evidence(
        &self,
        basis_digest: &str,
        reason: SerialFallbackReason,
        drift_outcome: FrontierPredictionDriftOutcome,
    ) -> SerialFallbackEvidence {
        SerialFallbackEvidence::new(FrontierRouteEvidence::serial_fallback(
            basis_digest.to_string(),
            self.surface_digest.clone(),
            reason,
            drift_outcome,
        ))
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn to_route_evidence_from_stage_record(
        &self,
        basis_digest: &str,
        stage: &StageExecutionRecord,
        _disjointness_class: FrontierDisjointnessClass,
    ) -> Result<SerialFallbackEvidence, SignalAdmissionEvidenceError> {
        let reason = stage
            .parallel_admission_reason
            .ok_or(SignalAdmissionEvidenceError::MissingParallelAdmissionReason)?;
        if is_parallel_admitted(reason) {
            return Err(SignalAdmissionEvidenceError::ParallelAdmissionRouteUnsupported);
        }

        Ok(self.to_serial_fallback_evidence(
            basis_digest,
            serial_fallback_reason_from_signal(reason),
            FrontierPredictionDriftOutcome::WithinBudget,
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SignalFrontierBundleEvidence {
    bundle_surface_digest: FrontierSurfaceDigest,
    route_evidences: Vec<SerialFallbackEvidence>,
}

impl SignalFrontierBundleEvidence {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn from_route_evidences(route_evidences: Vec<SerialFallbackEvidence>) -> Self {
        let mut parts = vec![format!("route_count:{}", route_evidences.len())];
        for (index, route) in route_evidences.iter().enumerate() {
            parts.push(format!(
                "route[{index}].surface:{}",
                route.surface_digest().as_str()
            ));
            parts.push(format!(
                "route[{index}].drift:{}",
                route.drift_outcome().as_str()
            ));
            parts.push(format!(
                "route[{index}].drift:{}",
                route.drift_outcome().as_str()
            ));
            parts.push(format!(
                "route[{index}].fallback:{}",
                route.reason().as_str()
            ));
        }

        Self {
            bundle_surface_digest: FrontierSurfaceDigest::from_label(&parts.join("|")),
            route_evidences,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn from_stage_records(
        basis_digest: &str,
        route_surfaces: &[SignalFrontierSurfaceEvidence],
        stages: &[StageExecutionRecord],
        disjointness_classes: &[FrontierDisjointnessClass],
    ) -> Result<Self, SignalAdmissionEvidenceError> {
        if route_surfaces.len() != stages.len() || stages.len() != disjointness_classes.len() {
            return Err(SignalAdmissionEvidenceError::RouteCountMismatch {
                surfaces: route_surfaces.len(),
                stages: stages.len(),
                disjointness_classes: disjointness_classes.len(),
            });
        }

        let mut route_evidences = Vec::with_capacity(stages.len());
        for ((surface, stage), class) in route_surfaces
            .iter()
            .zip(stages.iter())
            .zip(disjointness_classes.iter())
        {
            route_evidences.push(surface.to_route_evidence_from_stage_record(
                basis_digest,
                stage,
                class.clone(),
            )?);
        }

        Ok(Self::from_route_evidences(route_evidences))
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn bundle_surface_digest(&self) -> &FrontierSurfaceDigest {
        &self.bundle_surface_digest
    }

    #[allow(dead_code)]
    pub(crate) fn bind_to_basis(&self, basis_digest: &str) -> SerialFallbackBundleEvidence {
        SerialFallbackBundleEvidence::new(
            basis_digest.to_string(),
            self.bundle_surface_digest.clone(),
            self.route_evidences.clone(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SignalAdmissionEvidenceError {
    MissingParallelAdmissionReason,
    ParallelAdmissionRouteUnsupported,
    RouteCountMismatch {
        surfaces: usize,
        stages: usize,
        disjointness_classes: usize,
    },
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

#[cfg_attr(not(test), allow(dead_code))]
fn is_parallel_admitted(reason: ParallelAdmissionReason) -> bool {
    matches!(
        reason,
        ParallelAdmissionReason::AdmittedOperational
            | ParallelAdmissionReason::AdmittedDevelopment
            | ParallelAdmissionReason::AdmittedForensic
            | ParallelAdmissionReason::AdmittedProofSafeGroupedConcurrent
    )
}

#[cfg_attr(not(test), allow(dead_code))]
fn serial_fallback_reason_from_signal(reason: ParallelAdmissionReason) -> SerialFallbackReason {
    match reason {
        ParallelAdmissionReason::SerialExecutor => SerialFallbackReason::SerialExecutor,
        ParallelAdmissionReason::BelowMinStageWidth => SerialFallbackReason::BelowMinStageWidth,
        ParallelAdmissionReason::BelowPolicyWorkThreshold => {
            SerialFallbackReason::BelowPolicyWorkThreshold
        }
        ParallelAdmissionReason::ValidationHeavyStage => SerialFallbackReason::ValidationHeavyStage,
        ParallelAdmissionReason::BelowFullParallelThreshold => {
            SerialFallbackReason::BelowFullParallelThreshold
        }
        ParallelAdmissionReason::FullParallelUnsupportedByMutableEngine => {
            SerialFallbackReason::FullParallelUnsupportedByMutableEngine
        }
        ParallelAdmissionReason::AdmittedOperational
        | ParallelAdmissionReason::AdmittedDevelopment
        | ParallelAdmissionReason::AdmittedForensic
        | ParallelAdmissionReason::AdmittedProofSafeGroupedConcurrent => {
            SerialFallbackReason::DeterministicAdmissionDenied
        }
    }
}
