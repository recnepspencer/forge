use std::collections::BTreeSet;

use forge_query::facade::ForgeQueryWorkspace;
use topology::certification::milestone_one_runtime_builder;
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};

use crate::construction::digest::digest_owned_parts;
use crate::construction::outcome::{
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
use crate::construction::request::PrimitiveConstructionFamily;
use crate::construction::tests::support::blocking_boundary::{
    self, PrimitiveConstructionBlockingBoundary,
};
use crate::construction::tests::support::corpus_cases::primitive_construction_corpus;
use crate::construction::tests::support::corpus_ordering::PrimitiveConstructionCorpusAuthoringOrderLane;
use crate::construction::tests::support::corpus_replay_generation::prepare_primitive_construction_corpus_replay_rows;
use crate::construction::tests::support::corpus_replay_row::{
    PrimitiveConstructionCorpusParameterRole, PrimitiveConstructionCorpusReplaySiegeRow,
};
use crate::construction::tests::support::runtime_truth::PrimitiveConstructionCertificationRuntimeTruth;

use super::digest::{
    prepare_authoring_order_lane_digest_rows, row_digest,
    PrimitiveConstructionCorpusAuthoringOrderLaneDigestRow,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionCorpusReplaySiegeReport {
    canonical_rows: Vec<PrimitiveConstructionCorpusReplaySiegeRow>,
    canonical_row_digests: Vec<String>,
    authoring_order_lanes: Vec<PrimitiveConstructionCorpusAuthoringOrderLaneDigestRow>,
}

impl PrimitiveConstructionCorpusReplaySiegeReport {
    pub(crate) fn new(
        canonical_rows: Vec<PrimitiveConstructionCorpusReplaySiegeRow>,
        canonical_row_digests: Vec<String>,
        authoring_order_lanes: Vec<PrimitiveConstructionCorpusAuthoringOrderLaneDigestRow>,
    ) -> Self {
        Self {
            canonical_rows,
            canonical_row_digests,
            authoring_order_lanes,
        }
    }

    pub(crate) fn scenario_ids(&self) -> Vec<String> {
        self.canonical_rows
            .iter()
            .map(|row| row.scenario_id().to_string())
            .collect()
    }

    pub(crate) fn rows(&self) -> &[PrimitiveConstructionCorpusReplaySiegeRow] {
        &self.canonical_rows
    }

    pub(crate) fn row_for(
        &self,
        family: PrimitiveConstructionFamily,
        parameter_role: PrimitiveConstructionCorpusParameterRole,
    ) -> Option<&PrimitiveConstructionCorpusReplaySiegeRow> {
        self.canonical_rows
            .iter()
            .find(|row| row.family() == family && row.parameter_role() == parameter_role)
    }

    pub(crate) fn accepted_count(&self) -> usize {
        self.canonical_rows
            .iter()
            .filter(|row| {
                matches!(
                    row.runtime_truth(),
                    PrimitiveConstructionCertificationRuntimeTruth::Admitted(_)
                )
            })
            .count()
    }

    pub(crate) fn rejected_count(&self) -> usize {
        self.canonical_rows.len() - self.accepted_count()
    }

    pub(crate) fn required_scenario_coverage_verified(&self) -> bool {
        let scenario_ids = self
            .canonical_rows
            .iter()
            .map(|row| row.scenario_id().to_string())
            .collect::<BTreeSet<_>>();
        let required_scenario_ids = primitive_construction_corpus()
            .into_iter()
            .map(|scenario| scenario.scenario_id.to_string())
            .collect::<BTreeSet<_>>();
        scenario_ids == required_scenario_ids
    }

    pub(crate) fn row_digest_uniqueness_verified(&self) -> bool {
        self.canonical_row_digests
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            .len()
            == self.canonical_row_digests.len()
    }

    pub(crate) fn lane_names(&self) -> Vec<String> {
        self.authoring_order_lanes
            .iter()
            .map(|lane| lane_name(lane).to_string())
            .collect()
    }

    pub(crate) fn authoring_order_lanes(
        &self,
    ) -> &[PrimitiveConstructionCorpusAuthoringOrderLaneDigestRow] {
        &self.authoring_order_lanes
    }

    pub(crate) fn row_digest(
        &self,
        row: &PrimitiveConstructionCorpusReplaySiegeRow,
    ) -> Option<&str> {
        self.canonical_rows
            .iter()
            .position(|candidate| {
                candidate.scenario_id() == row.scenario_id()
                    && candidate.family() == row.family()
                    && candidate.parameter_role() == row.parameter_role()
            })
            .map(|index| self.canonical_row_digests[index].as_str())
    }

    pub(crate) fn authoring_order_parity_verified(&self) -> bool {
        self.authoring_order_matrix_stability_verified()
    }

    pub(crate) fn authoring_order_lane_coverage_verified(&self) -> bool {
        let lane_names = self
            .authoring_order_lanes
            .iter()
            .map(|lane| lane_name(lane).to_string())
            .collect::<BTreeSet<_>>();
        let required_lane_names = PrimitiveConstructionCorpusAuthoringOrderLane::all()
            .into_iter()
            .map(|lane| lane.as_str().to_string())
            .collect::<BTreeSet<_>>();
        lane_names == required_lane_names
    }

    pub(crate) fn authoring_order_digest_uniqueness_verified(&self) -> bool {
        self.authoring_order_lanes
            .iter()
            .map(|lane| lane_digest_of(lane).to_string())
            .collect::<BTreeSet<_>>()
            .len()
            == self.authoring_order_lanes.len()
    }

    pub(crate) fn authoring_order_matrix_stability_verified(&self) -> bool {
        self.authoring_order_lanes
            .iter()
            .find(|lane| {
                lane_kind(lane) == PrimitiveConstructionCorpusAuthoringOrderLane::Canonical
            })
            .map(|canonical| {
                self.authoring_order_lanes.iter().all(|lane| {
                    normalized_matrix_digest_of(lane) == normalized_matrix_digest_of(canonical)
                })
            })
            .unwrap_or(false)
    }

    pub(crate) fn report_digest(&self) -> String {
        let mut parts = self.canonical_row_digests.clone();
        parts.push(format!("accepted-count:{}", self.accepted_count()));
        parts.push(format!("rejected-count:{}", self.rejected_count()));
        parts.extend(self.authoring_order_lanes.iter().flat_map(|lane| {
            [
                lane_name(lane).to_string(),
                lane_digest_of(lane).to_string(),
                normalized_matrix_digest_of(lane).to_string(),
            ]
        }));
        digest_owned_parts(&parts)
    }
}

pub(crate) fn prepare_primitive_construction_corpus_replay_siege_report(
    label: &str,
) -> PrimitiveConstructionCorpusReplaySiegeReport {
    let mut workspace = siege_workspace(label);
    let canonical_rows =
        prepare_primitive_construction_corpus_replay_rows(&mut workspace).expect("siege rows");
    let canonical_row_digests = canonical_rows
        .iter()
        .map(|row| row_digest(&mut workspace, row))
        .collect::<Result<Vec<_>, _>>()
        .expect("canonical row digests");
    let authoring_order_lanes =
        prepare_authoring_order_lane_digest_rows(&mut workspace, &canonical_rows)
            .expect("siege lanes");
    PrimitiveConstructionCorpusReplaySiegeReport::new(
        canonical_rows,
        canonical_row_digests,
        authoring_order_lanes,
    )
}

pub(crate) use prepare_primitive_construction_corpus_replay_siege_report as siege_report;

pub(crate) fn siege_workspace(label: &str) -> ForgeQueryWorkspace {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        format!("worth-kernel.{label}"),
    )
    .expect("workspace")
}

fn lane_kind(
    lane: &PrimitiveConstructionCorpusAuthoringOrderLaneDigestRow,
) -> PrimitiveConstructionCorpusAuthoringOrderLane {
    lane.0
}

fn lane_name(lane: &PrimitiveConstructionCorpusAuthoringOrderLaneDigestRow) -> &'static str {
    lane_kind(lane).as_str()
}

fn lane_digest_of(lane: &PrimitiveConstructionCorpusAuthoringOrderLaneDigestRow) -> &str {
    &lane.1
}

fn normalized_matrix_digest_of(
    lane: &PrimitiveConstructionCorpusAuthoringOrderLaneDigestRow,
) -> &str {
    &lane.2
}

pub(crate) fn row_birth_digest(row: &PrimitiveConstructionCorpusReplaySiegeRow) -> Option<&str> {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            Some(outcome.birth_truth_digest())
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(_) => None,
    }
}

pub(crate) fn row_realization_strategy(
    row: &PrimitiveConstructionCorpusReplaySiegeRow,
) -> Option<PrimitiveRealizationStrategy> {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            Some(outcome.realization_strategy())
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(_) => None,
    }
}

pub(crate) fn row_attempted_realization_strategies(
    row: &PrimitiveConstructionCorpusReplaySiegeRow,
) -> &[PrimitiveRealizationStrategy] {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            outcome.attempted_realization_strategies()
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            rejected.attempted_realization_strategies()
        }
    }
}

pub(crate) fn row_stability_class(
    row: &PrimitiveConstructionCorpusReplaySiegeRow,
) -> Option<PrimitiveStabilityClass> {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            Some(outcome.stability_class())
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            rejected.stability_class()
        }
    }
}

pub(crate) fn row_feature_conditioning_class(
    row: &PrimitiveConstructionCorpusReplaySiegeRow,
) -> Option<PrimitiveFeatureConditioningClass> {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            Some(outcome.feature_conditioning_class())
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            rejected.feature_conditioning_class()
        }
    }
}

pub(crate) fn row_support_normal_class(
    row: &PrimitiveConstructionCorpusReplaySiegeRow,
) -> Option<PrimitiveSupportNormalClass> {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            Some(outcome.support_normal_class())
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            rejected.support_normal_class()
        }
    }
}

pub(crate) fn row_normalization_disposition(
    row: &PrimitiveConstructionCorpusReplaySiegeRow,
) -> Option<PrimitiveNormalizationDisposition> {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            Some(outcome.normalization_disposition())
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            rejected.normalization_disposition()
        }
    }
}

pub(crate) fn row_exhaustion_reason(
    row: &PrimitiveConstructionCorpusReplaySiegeRow,
) -> Option<PrimitiveRealizationExhaustionReason> {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(_) => None,
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            rejected.exhaustion_reason()
        }
    }
}

pub(crate) fn row_rejection_class(
    row: &PrimitiveConstructionCorpusReplaySiegeRow,
) -> Option<PrimitiveConstructionRejectionClass> {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(_) => None,
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            Some(rejected.rejection_class())
        }
    }
}

pub(crate) fn row_rejection_locality(
    row: &PrimitiveConstructionCorpusReplaySiegeRow,
) -> Option<PrimitiveConstructionRejectionLocality> {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(_) => None,
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            Some(rejected.rejection_locality())
        }
    }
}

pub(crate) fn row_blocking_boundary(
    row: &PrimitiveConstructionCorpusReplaySiegeRow,
) -> Option<PrimitiveConstructionBlockingBoundary> {
    row_rejection_locality(row).map(blocking_boundary::blocking_boundary_for)
}

pub(crate) fn row_construction_breadth(row: &PrimitiveConstructionCorpusReplaySiegeRow) -> usize {
    match row.runtime_truth() {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            outcome.topology_fact_breadth()
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(_) => 0,
    }
}
