use crate::construction::digest::digest_owned_parts;
use crate::construction::outcome::{
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
use worth_geom::facade::{
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

use super::super::ordering::required_compound_adversarial_lane_name_set;
use super::lane_report::{
    PrimitiveConstructionCompoundAuthoringOrderRow, PrimitiveConstructionCompoundOrderLaneReport,
};
use super::rows::PrimitiveConstructionCompoundRow;
use super::schema::{
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundMotionKind,
    PrimitiveConstructionCompoundRowClass, PrimitiveConstructionCompoundTopologyClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundOrderingScenarioRow {
    scenario_id: String,
    canonical_row_digest: String,
    lane_count: usize,
    row_digest_stable: bool,
    topology_class: PrimitiveConstructionCompoundTopologyClass,
    topology_class_stable: bool,
    row_class: PrimitiveConstructionCompoundRowClass,
    row_class_stable: bool,
    realization_strategy: Option<PrimitiveRealizationStrategy>,
    realization_strategy_stable: bool,
    stability_class: Option<PrimitiveStabilityClass>,
    stability_class_stable: bool,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    exhaustion_reason_stable: bool,
    rejection_class: Option<PrimitiveConstructionRejectionClass>,
    rejection_class_stable: bool,
    rejection_locality: Option<PrimitiveConstructionRejectionLocality>,
    rejection_locality_stable: bool,
    motion_kind: Option<PrimitiveConstructionCompoundMotionKind>,
    motion_kind_stable: bool,
    grazing_kind: Option<PrimitiveConstructionCompoundGrazingKind>,
    grazing_kind_stable: bool,
    row_digest: String,
}

impl PrimitiveConstructionCompoundOrderingScenarioRow {
    pub fn from_canonical_and_lanes(
        canonical: &PrimitiveConstructionCompoundRow,
        lane_reports: &[PrimitiveConstructionCompoundOrderLaneReport],
    ) -> Self {
        let scenario_id = canonical.scenario_id().to_string();
        let lane_rows = lane_reports
            .iter()
            .filter_map(|lane| lane.row_for(&scenario_id))
            .collect::<Vec<_>>();
        let row_digest_stable = lane_rows
            .iter()
            .all(|row| row.row_digest() == canonical.row_digest());
        let topology_class = canonical.topology_class();
        let topology_class_stable = lane_rows
            .iter()
            .all(|row| row.topology_class() == topology_class);
        let row_class = canonical.row_class();
        let row_class_stable = lane_rows.iter().all(|row| row.row_class() == row_class);
        let realization_strategy = canonical.realization_strategy();
        let realization_strategy_stable = lane_rows
            .iter()
            .all(|row| row.realization_strategy() == realization_strategy);
        let stability_class = canonical.stability_class();
        let stability_class_stable = lane_rows
            .iter()
            .all(|row| row.stability_class() == stability_class);
        let exhaustion_reason = canonical.exhaustion_reason();
        let exhaustion_reason_stable = lane_rows
            .iter()
            .all(|row| row.exhaustion_reason() == exhaustion_reason);
        let rejection_class = canonical.rejection_class();
        let rejection_class_stable = lane_rows
            .iter()
            .all(|row| row.rejection_class() == rejection_class);
        let rejection_locality = canonical.rejection_locality();
        let rejection_locality_stable = lane_rows
            .iter()
            .all(|row| row.rejection_locality() == rejection_locality);
        let motion_kind = canonical.motion_kind();
        let motion_kind_stable = lane_rows.iter().all(|row| row.motion_kind() == motion_kind);
        let grazing_kind = canonical.grazing_kind();
        let grazing_kind_stable = lane_rows
            .iter()
            .all(|row| row.grazing_kind() == grazing_kind);
        let lane_count = lane_rows.len();
        let row_digest = digest_owned_parts(&[
            scenario_id.clone(),
            canonical.row_digest().to_string(),
            lane_count.to_string(),
            row_digest_stable.to_string(),
            topology_class.as_str().to_string(),
            topology_class_stable.to_string(),
            row_class.as_str().to_string(),
            row_class_stable.to_string(),
            realization_strategy
                .map(|value| value.as_str())
                .unwrap_or("none")
                .to_string(),
            realization_strategy_stable.to_string(),
            stability_class
                .map(|value| value.as_str())
                .unwrap_or("none")
                .to_string(),
            stability_class_stable.to_string(),
            exhaustion_reason
                .map(|value| value.as_str())
                .unwrap_or("none")
                .to_string(),
            exhaustion_reason_stable.to_string(),
            rejection_class
                .map(|value| value.as_str())
                .unwrap_or("none")
                .to_string(),
            rejection_class_stable.to_string(),
            rejection_locality
                .map(|value| value.as_str())
                .unwrap_or("none")
                .to_string(),
            rejection_locality_stable.to_string(),
            motion_kind
                .map(|value| value.as_str())
                .unwrap_or("none")
                .to_string(),
            motion_kind_stable.to_string(),
            grazing_kind
                .map(|value| value.as_str())
                .unwrap_or("none")
                .to_string(),
            grazing_kind_stable.to_string(),
        ]);
        Self {
            scenario_id,
            canonical_row_digest: canonical.row_digest().to_string(),
            lane_count,
            row_digest_stable,
            topology_class,
            topology_class_stable,
            row_class,
            row_class_stable,
            realization_strategy,
            realization_strategy_stable,
            stability_class,
            stability_class_stable,
            exhaustion_reason,
            exhaustion_reason_stable,
            rejection_class,
            rejection_class_stable,
            rejection_locality,
            rejection_locality_stable,
            motion_kind,
            motion_kind_stable,
            grazing_kind,
            grazing_kind_stable,
            row_digest,
        }
    }

    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    pub fn canonical_row_digest(&self) -> &str {
        &self.canonical_row_digest
    }

    pub fn lane_count(&self) -> usize {
        self.lane_count
    }

    pub fn row_digest_stable(&self) -> bool {
        self.row_digest_stable
    }

    pub fn topology_class(&self) -> PrimitiveConstructionCompoundTopologyClass {
        self.topology_class
    }

    pub fn topology_class_stable(&self) -> bool {
        self.topology_class_stable
    }

    pub fn row_class(&self) -> PrimitiveConstructionCompoundRowClass {
        self.row_class
    }

    pub fn row_class_stable(&self) -> bool {
        self.row_class_stable
    }

    pub fn realization_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        self.realization_strategy
    }

    pub fn realization_strategy_stable(&self) -> bool {
        self.realization_strategy_stable
    }

    pub fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.stability_class
    }

    pub fn stability_class_stable(&self) -> bool {
        self.stability_class_stable
    }

    pub fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.exhaustion_reason
    }

    pub fn exhaustion_reason_stable(&self) -> bool {
        self.exhaustion_reason_stable
    }

    pub fn rejection_class(&self) -> Option<PrimitiveConstructionRejectionClass> {
        self.rejection_class
    }

    pub fn rejection_class_stable(&self) -> bool {
        self.rejection_class_stable
    }

    pub fn rejection_locality(&self) -> Option<PrimitiveConstructionRejectionLocality> {
        self.rejection_locality
    }

    pub fn rejection_locality_stable(&self) -> bool {
        self.rejection_locality_stable
    }

    pub fn motion_kind(&self) -> Option<PrimitiveConstructionCompoundMotionKind> {
        self.motion_kind
    }

    pub fn motion_kind_stable(&self) -> bool {
        self.motion_kind_stable
    }

    pub fn grazing_kind(&self) -> Option<PrimitiveConstructionCompoundGrazingKind> {
        self.grazing_kind
    }

    pub fn grazing_kind_stable(&self) -> bool {
        self.grazing_kind_stable
    }

    pub fn stable_across_orders(&self) -> bool {
        self.row_digest_stable
            && self.topology_class_stable
            && self.row_class_stable
            && self.realization_strategy_stable
            && self.stability_class_stable
            && self.exhaustion_reason_stable
            && self.rejection_class_stable
            && self.rejection_locality_stable
            && self.motion_kind_stable
            && self.grazing_kind_stable
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundOrderingParityReport {
    authoring_order_rows: Vec<PrimitiveConstructionCompoundAuthoringOrderRow>,
    lane_reports: Vec<PrimitiveConstructionCompoundOrderLaneReport>,
    scenario_rows: Vec<PrimitiveConstructionCompoundOrderingScenarioRow>,
    normalized_matrix_digest: String,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionCompoundOrderingParityReport {
    pub fn new(lane_reports: Vec<PrimitiveConstructionCompoundOrderLaneReport>) -> Self {
        let required_lane_names = required_compound_adversarial_lane_name_set();
        let authoring_order_rows = lane_reports
            .iter()
            .map(PrimitiveConstructionCompoundOrderLaneReport::summary_row)
            .collect::<Vec<_>>();
        let lane_names = lane_reports
            .iter()
            .map(|row| row.lane_name())
            .collect::<std::collections::BTreeSet<_>>();
        let canonical_lane = lane_reports
            .iter()
            .find(|row| row.lane_name() == "canonical");
        let normalized_matrix_digest = canonical_lane
            .map(|row| row.normalized_matrix_digest().to_string())
            .unwrap_or_default();
        let scenario_rows = canonical_lane
            .map(|canonical_lane| {
                canonical_lane
                    .rows()
                    .iter()
                    .map(|row| {
                        PrimitiveConstructionCompoundOrderingScenarioRow::from_canonical_and_lanes(
                            row,
                            &lane_reports,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let lane_digests = lane_reports
            .iter()
            .map(|row| row.lane_digest())
            .collect::<std::collections::BTreeSet<_>>();
        let parity_verified = !lane_reports.is_empty()
            && lane_names == required_lane_names
            && lane_reports.iter().all(|row| row.parity_verified())
            && lane_digests.len() == lane_reports.len()
            && scenario_rows
                .iter()
                .all(|row| row.lane_count() == lane_reports.len() && row.stable_across_orders());
        let report_digest = digest_owned_parts(
            &lane_reports
                .iter()
                .map(|row| row.row_digest().to_string())
                .chain(scenario_rows.iter().map(|row| row.row_digest().to_string()))
                .chain(std::iter::once(normalized_matrix_digest.clone()))
                .chain(std::iter::once(parity_verified.to_string()))
                .collect::<Vec<_>>(),
        );
        Self {
            authoring_order_rows,
            lane_reports,
            scenario_rows,
            normalized_matrix_digest,
            parity_verified,
            report_digest,
        }
    }

    pub fn authoring_order_rows(&self) -> &[PrimitiveConstructionCompoundAuthoringOrderRow] {
        &self.authoring_order_rows
    }

    pub fn lane_reports(&self) -> &[PrimitiveConstructionCompoundOrderLaneReport] {
        &self.lane_reports
    }

    pub fn scenario_rows(&self) -> &[PrimitiveConstructionCompoundOrderingScenarioRow] {
        &self.scenario_rows
    }

    pub fn scenario_row_for(
        &self,
        scenario_id: &str,
    ) -> Option<&PrimitiveConstructionCompoundOrderingScenarioRow> {
        self.scenario_rows
            .iter()
            .find(|row| row.scenario_id() == scenario_id)
    }

    pub fn normalized_matrix_digest(&self) -> &str {
        &self.normalized_matrix_digest
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
