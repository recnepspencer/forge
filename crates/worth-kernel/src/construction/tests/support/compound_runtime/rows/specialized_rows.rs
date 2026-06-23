use super::super::schema::{
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundMotionKind,
};
use crate::construction::tests::support::evidence_reports::sealed_report_identity;
use worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionCompoundMotionParityRow {
    scenario_id: String,
    motion_kind: PrimitiveConstructionCompoundMotionKind,
    motion_digest: String,
    row_digest: String,
}

impl PrimitiveConstructionCompoundMotionParityRow {
    pub fn new(
        scenario_id: String,
        motion_kind: PrimitiveConstructionCompoundMotionKind,
        motion_digest: String,
    ) -> Self {
        let row_digest = sealed_report_identity(
            "worth-kernel.construction.compound-parity",
            "motion-parity-row",
            |report| {
                report
                    .value_participating("scenario", scenario_id.clone())?
                    .shape_participating("motion-kind", motion_kind.as_str())?
                    .value_participating("motion", motion_digest.clone())
            },
        );
        Self {
            scenario_id,
            motion_kind,
            motion_digest,
            row_digest,
        }
    }

    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    pub fn motion_kind(&self) -> PrimitiveConstructionCompoundMotionKind {
        self.motion_kind
    }

    pub fn motion_digest(&self) -> &str {
        &self.motion_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionCompoundGrazingBoundaryRow {
    scenario_id: String,
    grazing_kind: PrimitiveConstructionCompoundGrazingKind,
    grazing_digest: String,
    row_digest: String,
}

impl PrimitiveConstructionCompoundGrazingBoundaryRow {
    pub fn new(
        scenario_id: String,
        grazing_kind: PrimitiveConstructionCompoundGrazingKind,
        grazing_digest: String,
    ) -> Self {
        let row_digest = sealed_report_identity(
            "worth-kernel.construction.compound-parity",
            "grazing-boundary-row",
            |report| {
                report
                    .value_participating("scenario", scenario_id.clone())?
                    .shape_participating("grazing-kind", grazing_kind.as_str())?
                    .value_participating("grazing", grazing_digest.clone())
            },
        );
        Self {
            scenario_id,
            grazing_kind,
            grazing_digest,
            row_digest,
        }
    }

    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    pub fn grazing_kind(&self) -> PrimitiveConstructionCompoundGrazingKind {
        self.grazing_kind
    }

    pub fn grazing_digest(&self) -> &str {
        &self.grazing_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionCompoundExhaustionWitnessParityRow {
    scenario_id: String,
    witness_kind: PrimitiveRealizationExhaustionWitnessKind,
    siege_row_digest: String,
    witness_row_digest: String,
    row_digest: String,
}

impl PrimitiveConstructionCompoundExhaustionWitnessParityRow {
    pub fn new(
        scenario_id: String,
        witness_kind: PrimitiveRealizationExhaustionWitnessKind,
        siege_row_digest: String,
        witness_row_digest: String,
    ) -> Self {
        let row_digest = sealed_report_identity(
            "worth-kernel.construction.compound-parity",
            "exhaustion-witness-row",
            |report| {
                report
                    .value_participating("scenario", scenario_id.clone())?
                    .shape_participating("witness-kind", witness_kind.as_str())?
                    .value_participating("siege-row", siege_row_digest.clone())?
                    .value_participating("witness-row", witness_row_digest.clone())
            },
        );
        Self {
            scenario_id,
            witness_kind,
            siege_row_digest,
            witness_row_digest,
            row_digest,
        }
    }

    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    pub fn witness_kind(&self) -> PrimitiveRealizationExhaustionWitnessKind {
        self.witness_kind
    }

    pub fn siege_row_digest(&self) -> &str {
        &self.siege_row_digest
    }

    pub fn witness_row_digest(&self) -> &str {
        &self.witness_row_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
