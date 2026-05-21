use crate::construction::digest::digest_owned_parts;
use crate::construction::request::PrimitiveConstructionFamily;
use worth_geom::facade::{
    primitive_realization_exhaustion_witness_rows, PrimitiveConditioningWitness,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationExhaustionWitnessKind,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionRealizationExhaustionWitnessRow {
    witness_kind: PrimitiveRealizationExhaustionWitnessKind,
    family: PrimitiveConstructionFamily,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: PrimitiveStabilityClass,
    exhaustion_reason: PrimitiveRealizationExhaustionReason,
    conditioning_witness: PrimitiveConditioningWitness,
    row_digest: String,
}

impl PrimitiveConstructionRealizationExhaustionWitnessRow {
    fn from_geom(row: &worth_geom::facade::PrimitiveRealizationExhaustionWitnessRow) -> Self {
        let family = match row.witness_kind() {
            PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse => {
                PrimitiveConstructionFamily::RegularPyramid
            }
            PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse => {
                PrimitiveConstructionFamily::SimplexSolid
            }
            PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse => {
                PrimitiveConstructionFamily::SimplexSolid
            }
        };
        let row_digest = digest_owned_parts(&[
            row.witness_kind().as_str().to_string(),
            family.as_str().to_string(),
            row.exhaustion_report().report_digest().to_string(),
        ]);
        Self {
            witness_kind: row.witness_kind(),
            family,
            attempted_strategies: row.attempted_strategies().to_vec(),
            stability_class: row.exhaustion_report().stability_class(),
            exhaustion_reason: row.exhaustion_reason(),
            conditioning_witness: row.exhaustion_report().conditioning_witness().clone(),
            row_digest,
        }
    }

    pub fn witness_kind(&self) -> PrimitiveRealizationExhaustionWitnessKind {
        self.witness_kind
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    pub fn stability_class(&self) -> PrimitiveStabilityClass {
        self.stability_class
    }

    pub fn exhaustion_reason(&self) -> PrimitiveRealizationExhaustionReason {
        self.exhaustion_reason
    }

    pub fn conditioning_witness(&self) -> &PrimitiveConditioningWitness {
        &self.conditioning_witness
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionRealizationExhaustionWitnessReport {
    rows: Vec<PrimitiveConstructionRealizationExhaustionWitnessRow>,
    report_digest: String,
}

impl PrimitiveConstructionRealizationExhaustionWitnessReport {
    fn new(rows: Vec<PrimitiveConstructionRealizationExhaustionWitnessRow>) -> Self {
        let report_digest = digest_owned_parts(
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[PrimitiveConstructionRealizationExhaustionWitnessRow] {
        &self.rows
    }

    pub fn row_for(
        &self,
        witness_kind: PrimitiveRealizationExhaustionWitnessKind,
    ) -> Option<&PrimitiveConstructionRealizationExhaustionWitnessRow> {
        self.rows
            .iter()
            .find(|row| row.witness_kind() == witness_kind)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_realization_exhaustion_witness_report(
) -> PrimitiveConstructionRealizationExhaustionWitnessReport {
    PrimitiveConstructionRealizationExhaustionWitnessReport::new(
        primitive_realization_exhaustion_witness_rows()
            .iter()
            .map(PrimitiveConstructionRealizationExhaustionWitnessRow::from_geom)
            .collect(),
    )
}
