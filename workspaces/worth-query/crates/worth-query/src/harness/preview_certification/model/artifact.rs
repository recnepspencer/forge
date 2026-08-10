use super::{
    digests::{bundle_digest_parts, coverage_digest_parts},
    PreviewCertificationMatrix,
};
use crate::harness::certification::digest_parts;
use crate::preview::{
    PreviewBindingCounters, PreviewComparisonCounters, PreviewExecutionCounters,
    PreviewLiveCounters,
};

use super::super::super::certification::RequiredAssertionClass;
use super::super::completeness::bundle_completeness_report;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewBundleCompletenessReport {
    pub canonical_row_count: usize,
    pub rejection_row_count: usize,
    pub supported_lane_count: usize,
    pub successful_lane_count: usize,
    pub zero_rediscovery_lane_count: usize,
    pub preview_live_composition_admitted_by_design: bool,
    pub covered_perturbation_classes: Vec<super::PreviewPerturbationClass>,
    pub all_lanes_emit_required_outputs: bool,
    pub all_rows_have_hostile_coverage: bool,
    pub unmet_required_rows: Vec<&'static str>,
    pub unmet_required_assertion_classes: Vec<RequiredAssertionClass>,
    pub covers_all_currently_implemented_normative_scenarios: bool,
    pub covers_full_milestone_five_point_two_spec_matrix: bool,
    pub offline_analysis_ready: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneFivePointTwoPreviewCertificationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub bundle_completeness_report: PreviewBundleCompletenessReport,
    pub binding_counter_snapshot: PreviewBindingCounters,
    pub execution_counter_snapshot: PreviewExecutionCounters,
    pub comparison_counter_snapshot: PreviewComparisonCounters,
    pub preview_live_counter_snapshot: PreviewLiveCounters,
    pub matrix: PreviewCertificationMatrix,
}

impl PreviewCertificationMatrix {
    pub fn into_milestone_five_point_two_artifact(
        self,
    ) -> MilestoneFivePointTwoPreviewCertificationArtifact {
        let bundle_completeness_report = bundle_completeness_report(&self);
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));
        let (
            binding_counter_snapshot,
            execution_counter_snapshot,
            comparison_counter_snapshot,
            preview_live_counter_snapshot,
        ) = self.aggregate_counters();

        MilestoneFivePointTwoPreviewCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            bundle_completeness_report,
            binding_counter_snapshot,
            execution_counter_snapshot,
            comparison_counter_snapshot,
            preview_live_counter_snapshot,
            matrix: self,
        }
    }

    fn aggregate_counters(
        &self,
    ) -> (
        PreviewBindingCounters,
        PreviewExecutionCounters,
        PreviewComparisonCounters,
        PreviewLiveCounters,
    ) {
        let mut aggregate_binding = self
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .chain(
                self.rejection_rows
                    .iter()
                    .flat_map(|row| [&row.control_lane, &row.parity_lane]),
            )
            .fold(PreviewBindingCounters::default(), |mut aggregate, lane| {
                aggregate.absorb(&lane.counters);
                aggregate
            });
        let mut aggregate_execution = self
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .chain(
                self.rejection_rows
                    .iter()
                    .flat_map(|row| [&row.control_lane, &row.parity_lane]),
            )
            .fold(
                PreviewExecutionCounters::default(),
                |mut aggregate, lane| {
                    aggregate.absorb(&lane.execution_counters);
                    aggregate
                },
            );
        let mut aggregate_comparison = self
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .filter_map(|lane| lane.comparison_counters.as_ref())
            .fold(
                PreviewComparisonCounters::default(),
                |mut aggregate, counters| {
                    aggregate.absorb(counters);
                    aggregate
                },
            );
        let mut aggregate_preview_live = self
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .chain(
                self.rejection_rows
                    .iter()
                    .flat_map(|row| [&row.control_lane, &row.parity_lane]),
            )
            .filter_map(|lane| lane.preview_live_counters.as_ref())
            .fold(PreviewLiveCounters::default(), |mut aggregate, counters| {
                aggregate.absorb(counters);
                aggregate
            });

        for rejection in self
            .rejection_rows
            .iter()
            .filter_map(|row| row.hostile_lane.counters.as_ref())
        {
            aggregate_binding.absorb(rejection);
        }

        for rejection in self
            .rejection_rows
            .iter()
            .filter_map(|row| row.hostile_lane.execution_counters.as_ref())
        {
            aggregate_execution.absorb(rejection);
        }

        for rejection in self
            .rejection_rows
            .iter()
            .filter_map(|row| row.hostile_lane.comparison_counters.as_ref())
        {
            aggregate_comparison.absorb(rejection);
        }

        for rejection in self
            .rejection_rows
            .iter()
            .filter_map(|row| row.hostile_lane.preview_live_counters.as_ref())
        {
            aggregate_preview_live.absorb(rejection);
        }

        (
            aggregate_binding,
            aggregate_execution,
            aggregate_comparison,
            aggregate_preview_live,
        )
    }
}
