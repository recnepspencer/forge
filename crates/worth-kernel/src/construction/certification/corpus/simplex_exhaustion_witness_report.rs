use super::PrimitiveConstructionCorpusParameterRole;
use crate::construction::certification::realization::{
    prepare_primitive_construction_realization_exhaustion_witness_report,
    PrimitiveConstructionRealizationExhaustionWitnessReport,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::PrimitiveConstructionFamily;
use worth_geom::facade::{
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationExhaustionWitnessKind,
    PrimitiveRealizationStrategy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionSimplexExhaustionWitnessRow {
    witness_kind: PrimitiveRealizationExhaustionWitnessKind,
    linked_parameter_role: PrimitiveConstructionCorpusParameterRole,
    exhaustion_reason: PrimitiveRealizationExhaustionReason,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    lower_layer_row_digest: String,
    row_digest: String,
}

impl PrimitiveConstructionSimplexExhaustionWitnessRow {
    pub fn witness_kind(&self) -> PrimitiveRealizationExhaustionWitnessKind {
        self.witness_kind
    }

    pub fn linked_parameter_role(&self) -> PrimitiveConstructionCorpusParameterRole {
        self.linked_parameter_role
    }

    pub fn exhaustion_reason(&self) -> PrimitiveRealizationExhaustionReason {
        self.exhaustion_reason
    }

    pub fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    pub fn lower_layer_row_digest(&self) -> &str {
        &self.lower_layer_row_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionSimplexRealizationExhaustionWitnessReport {
    rows: Vec<PrimitiveConstructionSimplexExhaustionWitnessRow>,
    report_digest: String,
}

impl PrimitiveConstructionSimplexRealizationExhaustionWitnessReport {
    pub fn rows(&self) -> &[PrimitiveConstructionSimplexExhaustionWitnessRow] {
        &self.rows
    }

    pub fn row_for(
        &self,
        witness_kind: PrimitiveRealizationExhaustionWitnessKind,
    ) -> Option<&PrimitiveConstructionSimplexExhaustionWitnessRow> {
        self.rows
            .iter()
            .find(|row| row.witness_kind() == witness_kind)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_simplex_realization_exhaustion_witness_report(
) -> PrimitiveConstructionSimplexRealizationExhaustionWitnessReport {
    let lower = prepare_primitive_construction_realization_exhaustion_witness_report();
    let rows = [
        (
            PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse,
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
        ),
        (
            PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse,
            PrimitiveConstructionCorpusParameterRole::ExplicitExhaustion,
        ),
    ]
    .into_iter()
    .map(|(kind, role)| simplex_exhaustion_row(&lower, kind, role))
    .collect::<Vec<_>>();
    let report_digest = digest_owned_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    PrimitiveConstructionSimplexRealizationExhaustionWitnessReport {
        rows,
        report_digest,
    }
}

fn simplex_exhaustion_row(
    lower: &PrimitiveConstructionRealizationExhaustionWitnessReport,
    witness_kind: PrimitiveRealizationExhaustionWitnessKind,
    linked_parameter_role: PrimitiveConstructionCorpusParameterRole,
) -> PrimitiveConstructionSimplexExhaustionWitnessRow {
    let lower_row = lower
        .row_for(witness_kind)
        .expect("simplex lower-layer exhaustion witness row");
    assert_eq!(
        lower_row.family(),
        PrimitiveConstructionFamily::SimplexSolid
    );
    let row_digest = digest_owned_parts(&[
        witness_kind.as_str().to_string(),
        linked_parameter_role.as_str().to_string(),
        lower_row.row_digest().to_string(),
    ]);
    PrimitiveConstructionSimplexExhaustionWitnessRow {
        witness_kind,
        linked_parameter_role,
        exhaustion_reason: lower_row.exhaustion_reason(),
        attempted_strategies: lower_row.attempted_strategies().to_vec(),
        lower_layer_row_digest: lower_row.row_digest().to_string(),
        row_digest,
    }
}
