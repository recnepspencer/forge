use crate::construction::certification::corpus::closeout::{
    PrimitiveConstructionCorpusCloseoutGateStatus,
    PrimitiveConstructionCorpusRequiredScenarioInventory,
};
use crate::construction::certification::corpus::compound::lane_report::{
    PrimitiveConstructionCompoundAuthoringOrderRow, PrimitiveConstructionCompoundOrderLaneReport,
};
use crate::construction::certification::corpus::compound::ordering_report::PrimitiveConstructionCompoundOrderingParityReport;
use crate::construction::certification::corpus::compound::rows::{
    PrimitiveConstructionCompoundExhaustionWitnessParityRow,
    PrimitiveConstructionCompoundGrazingBoundaryRow, PrimitiveConstructionCompoundMotionParityRow,
    PrimitiveConstructionCompoundRow,
};
use crate::construction::digest::digest_owned_parts;
use std::collections::{BTreeMap, BTreeSet};

const EXPECTED_MOTION_ROWS: [(&str, super::schema::PrimitiveConstructionCompoundMotionKind); 3] = [
    (
        "sheet_patch_reorient_grazing_workplane",
        super::schema::PrimitiveConstructionCompoundMotionKind::Reorient,
    ),
    (
        "wire_open_endpoint_graze",
        super::schema::PrimitiveConstructionCompoundMotionKind::Offset,
    ),
    (
        "wire_open_motion_relocation",
        super::schema::PrimitiveConstructionCompoundMotionKind::Move,
    ),
];

const EXPECTED_GRAZING_ROWS: [(
    &str,
    super::schema::PrimitiveConstructionCompoundGrazingKind,
); 2] = [
    (
        "sheet_patch_reorient_grazing_workplane",
        super::schema::PrimitiveConstructionCompoundGrazingKind::NearFrameNormalAlignment,
    ),
    (
        "wire_open_endpoint_graze",
        super::schema::PrimitiveConstructionCompoundGrazingKind::NearReferenceAnchorDistance,
    ),
];

const EXPECTED_EXHAUSTION_ROWS: [(&str, worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind); 2] = [
    (
        "pyramid_semantic_exhaustion",
        worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse,
    ),
    (
        "simplex_world_collapsed_explicit_exhaustion",
        worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse,
    ),
];
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundAdversarialSiegeReport {
    lane_reports: Vec<PrimitiveConstructionCompoundOrderLaneReport>,
    authoring_order_rows: Vec<PrimitiveConstructionCompoundAuthoringOrderRow>,
    report_digest: String,
}

impl PrimitiveConstructionCompoundAdversarialSiegeReport {
    pub fn new(lane_reports: Vec<PrimitiveConstructionCompoundOrderLaneReport>) -> Self {
        let authoring_order_rows = lane_reports
            .iter()
            .map(PrimitiveConstructionCompoundOrderLaneReport::summary_row)
            .collect::<Vec<_>>();
        let mut parts = lane_reports
            .iter()
            .map(|lane| lane.row_digest().to_string())
            .collect::<Vec<_>>();
        parts.extend(
            authoring_order_rows
                .iter()
                .map(|row| row.row_digest().to_string()),
        );
        Self {
            lane_reports,
            authoring_order_rows,
            report_digest: digest_owned_parts(&parts),
        }
    }
    pub fn rows(&self) -> &[PrimitiveConstructionCompoundRow] {
        self.lane_reports
            .iter()
            .find(|lane| lane.lane_name() == "canonical")
            .map(PrimitiveConstructionCompoundOrderLaneReport::rows)
            .unwrap_or(&[])
    }
    pub fn authoring_order_rows(&self) -> &[PrimitiveConstructionCompoundAuthoringOrderRow] {
        &self.authoring_order_rows
    }
    pub fn lane_reports(&self) -> &[PrimitiveConstructionCompoundOrderLaneReport] {
        &self.lane_reports
    }
    pub fn authoring_order_parity_verified(&self) -> bool {
        PrimitiveConstructionCompoundOrderingParityReport::new(self.lane_reports.clone())
            .parity_verified()
    }
    pub fn row_for(&self, scenario_id: &str) -> Option<&PrimitiveConstructionCompoundRow> {
        self.rows()
            .iter()
            .find(|row| row.scenario_id() == scenario_id)
    }
    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundMotionParityReport {
    rows: Vec<PrimitiveConstructionCompoundMotionParityRow>,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionCompoundMotionParityReport {
    pub fn new(
        rows: Vec<PrimitiveConstructionCompoundMotionParityRow>,
        ordering: &PrimitiveConstructionCompoundOrderingParityReport,
    ) -> Self {
        let parity_verified = ordering.parity_verified()
            && exact_motion_inventory_matches(&rows)
            && rows.iter().all(|row| {
                ordering
                    .scenario_row_for(row.scenario_id())
                    .is_some_and(|scenario| {
                        scenario.stable_across_orders()
                            && scenario.motion_kind_stable()
                            && row.motion_kind()
                                == EXPECTED_MOTION_ROWS
                                    .iter()
                                    .find_map(|(scenario_id, kind)| {
                                        (*scenario_id == row.scenario_id()).then_some(*kind)
                                    })
                                    .expect("expected motion inventory is complete")
                    })
            });
        let mut parts = rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>();
        parts.push(parity_verified.to_string());
        Self {
            rows,
            parity_verified,
            report_digest: digest_owned_parts(&parts),
        }
    }
    pub fn rows(&self) -> &[PrimitiveConstructionCompoundMotionParityRow] {
        &self.rows
    }
    pub fn row_for(
        &self,
        scenario_id: &str,
    ) -> Option<&PrimitiveConstructionCompoundMotionParityRow> {
        self.rows
            .iter()
            .find(|row| row.scenario_id() == scenario_id)
    }
    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }
    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundGrazingBoundaryReport {
    rows: Vec<PrimitiveConstructionCompoundGrazingBoundaryRow>,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionCompoundGrazingBoundaryReport {
    pub fn new(
        rows: Vec<PrimitiveConstructionCompoundGrazingBoundaryRow>,
        ordering: &PrimitiveConstructionCompoundOrderingParityReport,
    ) -> Self {
        let parity_verified = ordering.parity_verified()
            && exact_grazing_inventory_matches(&rows)
            && rows.iter().all(|row| {
                ordering
                    .scenario_row_for(row.scenario_id())
                    .is_some_and(|scenario| {
                        scenario.stable_across_orders()
                            && scenario.grazing_kind_stable()
                            && row.grazing_kind()
                                == EXPECTED_GRAZING_ROWS
                                    .iter()
                                    .find_map(|(scenario_id, kind)| {
                                        (*scenario_id == row.scenario_id()).then_some(*kind)
                                    })
                                    .expect("expected grazing inventory is complete")
                    })
            });
        let mut parts = rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>();
        parts.push(parity_verified.to_string());
        Self {
            rows,
            parity_verified,
            report_digest: digest_owned_parts(&parts),
        }
    }
    pub fn rows(&self) -> &[PrimitiveConstructionCompoundGrazingBoundaryRow] {
        &self.rows
    }
    pub fn row_for(
        &self,
        scenario_id: &str,
    ) -> Option<&PrimitiveConstructionCompoundGrazingBoundaryRow> {
        self.rows
            .iter()
            .find(|row| row.scenario_id() == scenario_id)
    }
    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }
    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundMilestoneCloseoutReport {
    siege: PrimitiveConstructionCompoundAdversarialSiegeReport,
    parity: PrimitiveConstructionCompoundParityReport,
    requirements: PrimitiveConstructionCorpusRequiredScenarioInventory,
    gate: PrimitiveConstructionCorpusCloseoutGateStatus,
    report_digest: String,
}

impl PrimitiveConstructionCompoundMilestoneCloseoutReport {
    pub(crate) fn new(
        siege: PrimitiveConstructionCompoundAdversarialSiegeReport,
        parity: PrimitiveConstructionCompoundParityReport,
        requirements: PrimitiveConstructionCorpusRequiredScenarioInventory,
    ) -> Self {
        let required_rows_present =
            requirements.all_present(|scenario_id| siege.row_for(scenario_id));
        let gate = PrimitiveConstructionCorpusCloseoutGateStatus::new(
            &requirements,
            required_rows_present,
            parity.parity_verified(),
            [
                siege.report_digest().to_string(),
                parity.report_digest().to_string(),
            ],
        );
        let report_digest = digest_owned_parts(&[
            siege.report_digest().to_string(),
            parity.report_digest().to_string(),
            requirements.inventory_digest().to_string(),
            gate.gate_digest().to_string(),
        ]);
        Self {
            siege,
            parity,
            requirements,
            gate,
            report_digest,
        }
    }

    pub fn siege(&self) -> &PrimitiveConstructionCompoundAdversarialSiegeReport {
        &self.siege
    }

    pub fn motion(&self) -> &PrimitiveConstructionCompoundMotionParityReport {
        self.parity.motion()
    }

    pub fn parity(&self) -> &PrimitiveConstructionCompoundParityReport {
        &self.parity
    }

    pub fn grazing(&self) -> &PrimitiveConstructionCompoundGrazingBoundaryReport {
        self.parity.grazing()
    }

    pub fn required_scenarios(&self) -> &[String] {
        self.requirements.scenario_ids()
    }

    pub fn required_row_for(&self, scenario_id: &str) -> Option<&PrimitiveConstructionCompoundRow> {
        self.requirements
            .row_for(scenario_id, |required| self.siege.row_for(required))
    }

    pub fn required_rows_present(&self) -> bool {
        self.gate.required_rows_present()
    }

    pub fn closeout_gate_verified(&self) -> bool {
        self.gate.gate_verified()
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundExhaustionWitnessParityReport {
    rows: Vec<PrimitiveConstructionCompoundExhaustionWitnessParityRow>,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionCompoundExhaustionWitnessParityReport {
    pub fn new(
        rows: Vec<PrimitiveConstructionCompoundExhaustionWitnessParityRow>,
        ordering: &PrimitiveConstructionCompoundOrderingParityReport,
    ) -> Self {
        let parity_verified = ordering.parity_verified()
            && exact_exhaustion_inventory_matches(&rows)
            && rows.iter().all(|row| {
                ordering
                    .scenario_row_for(row.scenario_id())
                    .is_some_and(|scenario| {
                        scenario.stable_across_orders() && scenario.exhaustion_reason_stable()
                    })
            });
        let report_digest = digest_owned_parts(
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .chain(std::iter::once(parity_verified.to_string()))
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            parity_verified,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[PrimitiveConstructionCompoundExhaustionWitnessParityRow] {
        &self.rows
    }

    pub fn row_for(
        &self,
        scenario_id: &str,
    ) -> Option<&PrimitiveConstructionCompoundExhaustionWitnessParityRow> {
        self.rows
            .iter()
            .find(|row| row.scenario_id() == scenario_id)
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

fn exact_motion_inventory_matches(rows: &[PrimitiveConstructionCompoundMotionParityRow]) -> bool {
    let actual = rows
        .iter()
        .map(|row| (row.scenario_id().to_string(), row.motion_kind()))
        .collect::<BTreeMap<_, _>>();
    actual.len() == rows.len()
        && actual
            == EXPECTED_MOTION_ROWS
                .into_iter()
                .map(|(scenario_id, kind)| (scenario_id.to_string(), kind))
                .collect::<BTreeMap<_, _>>()
        && rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<BTreeSet<_>>()
            .len()
            == rows.len()
}

fn exact_grazing_inventory_matches(
    rows: &[PrimitiveConstructionCompoundGrazingBoundaryRow],
) -> bool {
    let actual = rows
        .iter()
        .map(|row| (row.scenario_id().to_string(), row.grazing_kind()))
        .collect::<BTreeMap<_, _>>();
    actual.len() == rows.len()
        && actual
            == EXPECTED_GRAZING_ROWS
                .into_iter()
                .map(|(scenario_id, kind)| (scenario_id.to_string(), kind))
                .collect::<BTreeMap<_, _>>()
        && rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<BTreeSet<_>>()
            .len()
            == rows.len()
}

fn exact_exhaustion_inventory_matches(
    rows: &[PrimitiveConstructionCompoundExhaustionWitnessParityRow],
) -> bool {
    let actual = rows
        .iter()
        .map(|row| (row.scenario_id().to_string(), row.witness_kind()))
        .collect::<BTreeMap<_, _>>();
    actual.len() == rows.len()
        && actual
            == EXPECTED_EXHAUSTION_ROWS
                .into_iter()
                .map(|(scenario_id, kind)| (scenario_id.to_string(), kind))
                .collect::<BTreeMap<_, _>>()
        && rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<BTreeSet<_>>()
            .len()
            == rows.len()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundParityReport {
    ordering: PrimitiveConstructionCompoundOrderingParityReport,
    motion: PrimitiveConstructionCompoundMotionParityReport,
    grazing: PrimitiveConstructionCompoundGrazingBoundaryReport,
    exhaustion: PrimitiveConstructionCompoundExhaustionWitnessParityReport,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionCompoundParityReport {
    pub fn new(
        ordering: PrimitiveConstructionCompoundOrderingParityReport,
        motion: PrimitiveConstructionCompoundMotionParityReport,
        grazing: PrimitiveConstructionCompoundGrazingBoundaryReport,
        exhaustion: PrimitiveConstructionCompoundExhaustionWitnessParityReport,
    ) -> Self {
        let parity_verified = ordering.parity_verified()
            && motion.parity_verified()
            && grazing.parity_verified()
            && exhaustion.parity_verified();
        let report_digest = digest_owned_parts(&[
            ordering.report_digest().to_string(),
            motion.report_digest().to_string(),
            grazing.report_digest().to_string(),
            exhaustion.report_digest().to_string(),
            parity_verified.to_string(),
        ]);
        Self {
            ordering,
            motion,
            grazing,
            exhaustion,
            parity_verified,
            report_digest,
        }
    }

    pub fn ordering(&self) -> &PrimitiveConstructionCompoundOrderingParityReport {
        &self.ordering
    }

    pub fn motion(&self) -> &PrimitiveConstructionCompoundMotionParityReport {
        &self.motion
    }

    pub fn grazing(&self) -> &PrimitiveConstructionCompoundGrazingBoundaryReport {
        &self.grazing
    }

    pub fn exhaustion(&self) -> &PrimitiveConstructionCompoundExhaustionWitnessParityReport {
        &self.exhaustion
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
