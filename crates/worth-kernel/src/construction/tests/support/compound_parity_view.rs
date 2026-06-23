use crate::construction::tests::support::compound_lane_support::compound_report_digest;
use crate::construction::tests::support::compound_parity_support::{
    build_exhaustion_witness_parity_rows_from_siege, build_grazing_boundary_rows_from_siege,
    build_motion_parity_rows_from_siege, exact_exhaustion_inventory_matches,
    exact_grazing_inventory_matches, exact_motion_inventory_matches, exhaustion_parity_verified,
    exhaustion_report_digest, grazing_parity_verified, grazing_report_digest,
    motion_parity_verified, motion_report_digest,
};
use crate::construction::tests::support::compound_runtime::{
    compound_parity_registry, prepare_primitive_construction_compound_adversarial_lanes,
    PrimitiveConstructionCompoundAdversarialLanes,
    PrimitiveConstructionCompoundAdversarialSiegeError,
    PrimitiveConstructionCompoundExhaustionWitnessParityRow,
    PrimitiveConstructionCompoundGrazingBoundaryRow, PrimitiveConstructionCompoundMotionParityRow,
};
use crate::construction::tests::support::evidence_reports::sealed_report_identity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionCompoundParityVerificationMismatch {
    MotionProjectionDrift,
    GrazingProjectionDrift,
    ExhaustionProjectionDrift,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionCompoundParityVerificationFailure {
    siege: PrimitiveConstructionCompoundAdversarialLanes,
    motion_rows: Option<Vec<PrimitiveConstructionCompoundMotionParityRow>>,
    grazing_rows: Option<Vec<PrimitiveConstructionCompoundGrazingBoundaryRow>>,
    exhaustion_rows: Option<Vec<PrimitiveConstructionCompoundExhaustionWitnessParityRow>>,
    mismatches: Vec<PrimitiveConstructionCompoundParityVerificationMismatch>,
}

impl PrimitiveConstructionCompoundParityVerificationFailure {
    pub(crate) fn siege(&self) -> &PrimitiveConstructionCompoundAdversarialLanes {
        &self.siege
    }

    pub(crate) fn motion_scenario_ids(&self) -> Vec<String> {
        self.motion_rows
            .as_deref()
            .map(|rows| {
                rows.iter()
                    .map(|row| row.scenario_id().to_string())
                    .collect()
            })
            .unwrap_or_else(|| {
                compound_parity_registry()
                    .motion_inventory()
                    .keys()
                    .cloned()
                    .collect()
            })
    }

    pub(crate) fn grazing_expected_inventory_coverage_verified(&self) -> bool {
        self.grazing_rows
            .as_deref()
            .map(exact_grazing_inventory_matches)
            .unwrap_or(true)
    }

    pub(crate) fn exhaustion_expected_inventory_coverage_verified(&self) -> bool {
        self.exhaustion_rows
            .as_deref()
            .map(|rows| exact_exhaustion_inventory_matches(rows).inventory_matches)
            .unwrap_or(true)
    }

    pub(crate) fn mismatches(&self) -> &[PrimitiveConstructionCompoundParityVerificationMismatch] {
        &self.mismatches
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionCompoundParityView {
    siege: PrimitiveConstructionCompoundAdversarialLanes,
}

impl PrimitiveConstructionCompoundParityView {
    pub(crate) fn from_siege(siege: PrimitiveConstructionCompoundAdversarialLanes) -> Self {
        Self { siege }
    }

    pub(crate) fn siege(&self) -> &PrimitiveConstructionCompoundAdversarialLanes {
        &self.siege
    }

    pub(crate) fn motion_rows(&self) -> Vec<PrimitiveConstructionCompoundMotionParityRow> {
        build_motion_parity_rows_from_siege(self.siege())
            .expect("compound parity view must rederive canonical motion rows")
    }

    pub(crate) fn motion_row_for(
        &self,
        scenario_id: &str,
    ) -> Option<PrimitiveConstructionCompoundMotionParityRow> {
        self.motion_rows()
            .into_iter()
            .find(|row| row.scenario_id() == scenario_id)
    }

    pub(crate) fn motion_scenario_ids(&self) -> Vec<String> {
        self.motion_rows()
            .into_iter()
            .map(|row| row.scenario_id().to_string())
            .collect()
    }

    pub(crate) fn motion_expected_inventory_coverage_verified(&self) -> bool {
        exact_motion_inventory_matches(&self.motion_rows())
    }

    pub(crate) fn motion_parity_verified(&self) -> bool {
        motion_parity_verified(self.siege(), &self.motion_rows())
    }

    pub(crate) fn motion_report_digest(&self) -> String {
        motion_report_digest(self.siege(), &self.motion_rows())
    }

    pub(crate) fn grazing_rows(&self) -> Vec<PrimitiveConstructionCompoundGrazingBoundaryRow> {
        build_grazing_boundary_rows_from_siege(self.siege())
            .expect("compound parity view must rederive canonical grazing rows")
    }

    pub(crate) fn grazing_row_for(
        &self,
        scenario_id: &str,
    ) -> Option<PrimitiveConstructionCompoundGrazingBoundaryRow> {
        self.grazing_rows()
            .into_iter()
            .find(|row| row.scenario_id() == scenario_id)
    }

    pub(crate) fn grazing_scenario_ids(&self) -> Vec<String> {
        self.grazing_rows()
            .into_iter()
            .map(|row| row.scenario_id().to_string())
            .collect()
    }

    pub(crate) fn grazing_expected_inventory_coverage_verified(&self) -> bool {
        exact_grazing_inventory_matches(&self.grazing_rows())
    }

    pub(crate) fn grazing_parity_verified(&self) -> bool {
        grazing_parity_verified(self.siege(), &self.grazing_rows())
    }

    pub(crate) fn grazing_report_digest(&self) -> String {
        grazing_report_digest(self.siege(), &self.grazing_rows())
    }

    pub(crate) fn exhaustion_rows(
        &self,
    ) -> Vec<PrimitiveConstructionCompoundExhaustionWitnessParityRow> {
        build_exhaustion_witness_parity_rows_from_siege(self.siege())
            .expect("compound parity view must rederive canonical exhaustion rows")
    }

    pub(crate) fn exhaustion_row_for(
        &self,
        scenario_id: &str,
    ) -> Option<PrimitiveConstructionCompoundExhaustionWitnessParityRow> {
        self.exhaustion_rows()
            .into_iter()
            .find(|row| row.scenario_id() == scenario_id)
    }

    pub(crate) fn exhaustion_scenario_ids(&self) -> Vec<String> {
        self.exhaustion_rows()
            .into_iter()
            .map(|row| row.scenario_id().to_string())
            .collect()
    }

    pub(crate) fn exhaustion_expected_inventory_coverage_verified(&self) -> bool {
        exact_exhaustion_inventory_matches(&self.exhaustion_rows()).inventory_matches
    }

    pub(crate) fn exhaustion_siege_row_digest_uniqueness_verified(&self) -> bool {
        exact_exhaustion_inventory_matches(&self.exhaustion_rows())
            .siege_row_digest_uniqueness_verified
    }

    pub(crate) fn exhaustion_witness_row_digest_uniqueness_verified(&self) -> bool {
        exact_exhaustion_inventory_matches(&self.exhaustion_rows())
            .witness_row_digest_uniqueness_verified
    }

    pub(crate) fn exhaustion_parity_verified(&self) -> bool {
        exhaustion_parity_verified(self.siege(), &self.exhaustion_rows())
    }

    pub(crate) fn exhaustion_report_digest(&self) -> String {
        exhaustion_report_digest(self.siege(), &self.exhaustion_rows())
    }

    pub(crate) fn report_digest(&self) -> String {
        sealed_report_identity(
            "worth-kernel.construction.compound-parity",
            "compound-parity-view",
            |report| {
                report
                    .value_participating("compound-report", compound_report_digest(self.siege()))?
                    .value_participating("motion-report", self.motion_report_digest())?
                    .value_participating("grazing-report", self.grazing_report_digest())?
                    .value_participating("exhaustion-report", self.exhaustion_report_digest())
            },
        )
    }
}

pub(crate) fn prepare_compound_parity_view() -> Result<
    PrimitiveConstructionCompoundParityView,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    Ok(PrimitiveConstructionCompoundParityView::from_siege(
        prepare_primitive_construction_compound_adversarial_lanes()?,
    ))
}

pub(crate) fn verify_compound_parity_view(
    siege: PrimitiveConstructionCompoundAdversarialLanes,
    motion_rows: Vec<PrimitiveConstructionCompoundMotionParityRow>,
    grazing_rows: Vec<PrimitiveConstructionCompoundGrazingBoundaryRow>,
    exhaustion_rows: Vec<PrimitiveConstructionCompoundExhaustionWitnessParityRow>,
) -> Result<
    PrimitiveConstructionCompoundParityView,
    PrimitiveConstructionCompoundParityVerificationFailure,
> {
    let mut mismatches = Vec::new();
    if motion_rows
        != build_motion_parity_rows_from_siege(&siege)
            .expect("compound parity verification must derive canonical motion rows")
    {
        mismatches
            .push(PrimitiveConstructionCompoundParityVerificationMismatch::MotionProjectionDrift);
    }
    if grazing_rows
        != build_grazing_boundary_rows_from_siege(&siege)
            .expect("compound parity verification must derive canonical grazing rows")
    {
        mismatches
            .push(PrimitiveConstructionCompoundParityVerificationMismatch::GrazingProjectionDrift);
    }
    if exhaustion_rows
        != build_exhaustion_witness_parity_rows_from_siege(&siege)
            .expect("compound parity verification must derive canonical exhaustion rows")
    {
        mismatches.push(
            PrimitiveConstructionCompoundParityVerificationMismatch::ExhaustionProjectionDrift,
        );
    }
    if mismatches.is_empty() {
        return Ok(PrimitiveConstructionCompoundParityView::from_siege(siege));
    }
    Err(PrimitiveConstructionCompoundParityVerificationFailure {
        siege,
        motion_rows: mismatches
            .contains(
                &PrimitiveConstructionCompoundParityVerificationMismatch::MotionProjectionDrift,
            )
            .then_some(motion_rows),
        grazing_rows: mismatches
            .contains(
                &PrimitiveConstructionCompoundParityVerificationMismatch::GrazingProjectionDrift,
            )
            .then_some(grazing_rows),
        exhaustion_rows: mismatches
            .contains(
                &PrimitiveConstructionCompoundParityVerificationMismatch::ExhaustionProjectionDrift,
            )
            .then_some(exhaustion_rows),
        mismatches,
    })
}
