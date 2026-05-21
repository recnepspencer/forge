use crate::construction::certification::corpus::compound::lane_report::{
    PrimitiveConstructionCompoundAuthoringOrderRow, PrimitiveConstructionCompoundOrderLaneReport,
};
use crate::construction::certification::corpus::compound::ordering_report::PrimitiveConstructionCompoundOrderingParityReport;
use crate::construction::certification::corpus::compound::parity::compound_parity_registry;
use crate::construction::certification::corpus::compound::rows::{
    PrimitiveConstructionCompoundExhaustionWitnessParityRow,
    PrimitiveConstructionCompoundGrazingBoundaryRow, PrimitiveConstructionCompoundMotionParityRow,
    PrimitiveConstructionCompoundRow,
};
use crate::construction::digest::digest_owned_parts;
use std::collections::{BTreeMap, BTreeSet};
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
                            && compound_parity_registry()
                                .motion_inventory()
                                .get(row.scenario_id())
                                .is_some_and(|kind| row.motion_kind() == *kind)
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
                            && compound_parity_registry()
                                .grazing_inventory()
                                .get(row.scenario_id())
                                .is_some_and(|kind| row.grazing_kind() == *kind)
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
    let registry = compound_parity_registry();
    let actual = rows
        .iter()
        .map(|row| (row.scenario_id().to_string(), row.motion_kind()))
        .collect::<BTreeMap<_, _>>();
    actual.len() == rows.len()
        && actual == *registry.motion_inventory()
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
    let registry = compound_parity_registry();
    let actual = rows
        .iter()
        .map(|row| (row.scenario_id().to_string(), row.grazing_kind()))
        .collect::<BTreeMap<_, _>>();
    actual.len() == rows.len()
        && actual == *registry.grazing_inventory()
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
    let registry = compound_parity_registry();
    let actual = rows
        .iter()
        .map(|row| (row.scenario_id().to_string(), row.witness_kind()))
        .collect::<BTreeMap<_, _>>();
    actual.len() == rows.len()
        && actual == *registry.exhaustion_inventory()
        && rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<BTreeSet<_>>()
            .len()
            == rows.len()
}
