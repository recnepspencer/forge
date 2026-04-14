use crate::facade::CanonicalizationCounters;

use super::digests::{
    canonical_bundle_digest_parts, digest_parts, hostile_expectation_key, parity_anchor_key,
    perturbation_class_key, rejection_bundle_digest_parts,
};
use super::model::{
    CanonicalCertificationBundle, CertificationBundleCompletenessReport, CertificationMatrix,
    CertificationPerturbationClass, CertificationRow, HostileLaneExpectation,
    MilestoneOneCertificationArtifact, ParityAnchor, RejectionCertificationRow,
};

impl CertificationMatrix {
    pub fn into_milestone_one_artifact(self) -> MilestoneOneCertificationArtifact {
        let bundle_completeness_report = self.bundle_completeness_report();
        let counter_snapshot = self.aggregate_counters();
        let certification_bundle_digest = digest_parts(&self.bundle_digest_parts());
        let coverage_matrix_digest = digest_parts(&self.coverage_digest_parts());

        MilestoneOneCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            bundle_completeness_report,
            counter_snapshot,
            matrix: self,
        }
    }

    fn bundle_completeness_report(&self) -> CertificationBundleCompletenessReport {
        let supported_lane_count = (self.rows.len() * 3) + (self.rejection_rows.len() * 2);
        let successful_lane_count = supported_lane_count;
        let zero_fallback_lane_count = self
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .chain(
                self.rejection_rows
                    .iter()
                    .flat_map(|row| [&row.control_lane, &row.parity_lane]),
            )
            .filter(|lane| lane.counter_snapshot.canonicalization_fallback_count == 0)
            .count();
        let all_lanes_emit_required_outputs =
            self.rows.iter().all(CertificationRow::has_required_outputs)
                && self
                    .rejection_rows
                    .iter()
                    .all(RejectionCertificationRow::has_required_outputs);
        let all_rows_have_hostile_coverage =
            self.rows.iter().all(CertificationRow::has_hostile_coverage)
                && self
                    .rejection_rows
                    .iter()
                    .all(RejectionCertificationRow::has_hostile_coverage);
        let covered_perturbation_classes = self.covered_perturbation_classes();
        let covers_all_mutation_sensitivity_classes = covered_perturbation_classes
            .contains(&CertificationPerturbationClass::ConstructionPath)
            && covered_perturbation_classes
                .contains(&CertificationPerturbationClass::MeaningChange)
            && covered_perturbation_classes
                .contains(&CertificationPerturbationClass::UnsupportedAuthoredForm)
            && covered_perturbation_classes
                .contains(&CertificationPerturbationClass::ForbiddenFallback);
        let covers_all_milestone_one_normative_scenarios = self.contains_row("detail-query-parity")
            && self.contains_row("result-shape-helper-composition")
            && self.contains_row("binding-descriptor-parity")
            && self.contains_row("collection-reordered-projection-parity")
            && self.contains_row("duplicate-clause-deduplication")
            && self.contains_row("semantic-distinction-boundary")
            && self.contains_row("unsupported-authored-query-family")
            && self.contains_row("forbidden-fallback-case");
        let offline_analysis_ready = all_lanes_emit_required_outputs
            && all_rows_have_hostile_coverage
            && zero_fallback_lane_count == supported_lane_count
            && covers_all_mutation_sensitivity_classes
            && covers_all_milestone_one_normative_scenarios;

        CertificationBundleCompletenessReport {
            canonical_row_count: self.rows.len(),
            rejection_row_count: self.rejection_rows.len(),
            supported_lane_count,
            successful_lane_count,
            zero_fallback_lane_count,
            covered_perturbation_classes,
            all_lanes_emit_required_outputs,
            all_rows_have_hostile_coverage,
            covers_all_mutation_sensitivity_classes,
            covers_all_milestone_one_normative_scenarios,
            offline_analysis_ready,
        }
    }

    fn aggregate_counters(&self) -> CanonicalizationCounters {
        self.rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .chain(
                self.rejection_rows
                    .iter()
                    .flat_map(|row| [&row.control_lane, &row.parity_lane]),
            )
            .fold(
                CanonicalizationCounters::default(),
                |mut aggregate, lane| {
                    aggregate.raw_clause_count += lane.counter_snapshot.raw_clause_count;
                    aggregate.normalized_clause_count +=
                        lane.counter_snapshot.normalized_clause_count;
                    aggregate.projection_entry_count +=
                        lane.counter_snapshot.projection_entry_count;
                    aggregate.traversal_clause_count +=
                        lane.counter_snapshot.traversal_clause_count;
                    aggregate.result_shape_field_count +=
                        lane.counter_snapshot.result_shape_field_count;
                    aggregate.binding_descriptor_count +=
                        lane.counter_snapshot.binding_descriptor_count;
                    aggregate.query_deduplication_count +=
                        lane.counter_snapshot.query_deduplication_count;
                    aggregate.result_shape_deduplication_count +=
                        lane.counter_snapshot.result_shape_deduplication_count;
                    aggregate.canonicalization_warning_count +=
                        lane.counter_snapshot.canonicalization_warning_count;
                    aggregate.canonicalization_fallback_count +=
                        lane.counter_snapshot.canonicalization_fallback_count;
                    aggregate
                },
            )
    }

    fn bundle_digest_parts(&self) -> Vec<String> {
        let mut parts = vec![format!("suite:{}", self.suite_name)];

        for row in &self.rows {
            parts.push(format!("row:{}", row.row_name));
            parts.extend(canonical_bundle_digest_parts(&row.control_lane, "control"));
            parts.extend(canonical_bundle_digest_parts(&row.hostile_lane, "hostile"));
            parts.extend(canonical_bundle_digest_parts(&row.parity_lane, "parity"));
        }

        for row in &self.rejection_rows {
            parts.push(format!("rejection-row:{}", row.row_name));
            parts.extend(canonical_bundle_digest_parts(&row.control_lane, "control"));
            parts.extend(rejection_bundle_digest_parts(&row.hostile_lane, "hostile"));
            parts.extend(canonical_bundle_digest_parts(&row.parity_lane, "parity"));
        }

        parts
    }

    fn coverage_digest_parts(&self) -> Vec<String> {
        let mut parts = vec![
            format!("suite:{}", self.suite_name),
            format!("canonical-rows:{}", self.rows.len()),
            format!("rejection-rows:{}", self.rejection_rows.len()),
        ];

        for row in &self.rows {
            parts.push(format!(
                "row:{}:{}:{}:{}",
                row.row_name,
                perturbation_class_key(row.perturbation_class),
                hostile_expectation_key(row.hostile_expectation),
                parity_anchor_key(row.parity_anchor)
            ));
        }

        for row in &self.rejection_rows {
            parts.push(format!(
                "rejection-row:{}:{}:control-hostile-parity",
                row.row_name,
                perturbation_class_key(row.perturbation_class)
            ));
            parts.push(format!(
                "rejection-class:{}",
                row.hostile_lane.failure_class
            ));
        }

        parts
    }

    fn covered_perturbation_classes(&self) -> Vec<CertificationPerturbationClass> {
        let mut classes: Vec<_> = self
            .rows
            .iter()
            .map(|row| row.perturbation_class)
            .chain(self.rejection_rows.iter().map(|row| row.perturbation_class))
            .collect();
        classes.sort();
        classes.dedup();
        classes
    }

    fn contains_row(&self, row_name: &str) -> bool {
        self.rows.iter().any(|row| row.row_name == row_name)
            || self
                .rejection_rows
                .iter()
                .any(|row| row.row_name == row_name)
    }
}

impl CertificationRow {
    fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.hostile_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
    }

    fn has_hostile_coverage(&self) -> bool {
        let hostile_relation = match self.hostile_expectation {
            HostileLaneExpectation::EquivalentToControl => {
                self.control_lane.query_digest == self.hostile_lane.query_digest
                    && self.control_lane.result_shape_digest
                        == self.hostile_lane.result_shape_digest
            }
            HostileLaneExpectation::DistinctFromControl => {
                self.control_lane.query_digest != self.hostile_lane.query_digest
                    || self.control_lane.result_shape_digest
                        != self.hostile_lane.result_shape_digest
            }
        };

        let parity_relation = match self.parity_anchor {
            ParityAnchor::Control => {
                self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_shape_digest == self.parity_lane.result_shape_digest
            }
            ParityAnchor::Hostile => {
                self.hostile_lane.query_digest == self.parity_lane.query_digest
                    && self.hostile_lane.result_shape_digest == self.parity_lane.result_shape_digest
            }
        };

        hostile_relation && parity_relation
    }
}

impl RejectionCertificationRow {
    fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
            && !self.hostile_lane.failure_class.is_empty()
            && !self.hostile_lane.failure_digest.is_empty()
    }

    fn has_hostile_coverage(&self) -> bool {
        self.control_lane.query_digest == self.parity_lane.query_digest
            && self.control_lane.result_shape_digest == self.parity_lane.result_shape_digest
    }
}

impl CanonicalCertificationBundle {
    fn has_required_outputs(&self) -> bool {
        !self.query_digest.is_empty()
            && !self.result_shape_digest.is_empty()
            && self.warning_count == self.canonicalization_report.warnings().len()
            && self.event_count == self.canonicalization_report.events().len()
            && self.canonicalization_report.identity_freeze().query_digest == self.query_digest
            && self
                .canonicalization_report
                .identity_freeze()
                .result_shape_digest
                == self.result_shape_digest
    }
}
