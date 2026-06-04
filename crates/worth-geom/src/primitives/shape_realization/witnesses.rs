use super::exhaustion::{
    PrimitiveRealizationError, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationExhaustionReport,
};
use super::support::{
    realize_pyramid_support, realize_tetrahedron_support,
    realize_tetrahedron_support_with_altitude_component,
};
use super::PrimitiveRealizationStrategy;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveRealizationExhaustionWitnessKind {
    ZeroRadiusPyramidSupportCollapse,
    ZeroScaleSimplexSupportCollapse,
    AltitudeSqueezedSimplexSupportCollapse,
}

impl PrimitiveRealizationExhaustionWitnessKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ZeroRadiusPyramidSupportCollapse => "zero_radius_pyramid_support_collapse",
            Self::ZeroScaleSimplexSupportCollapse => "zero_scale_simplex_support_collapse",
            Self::AltitudeSqueezedSimplexSupportCollapse => {
                "altitude_squeezed_simplex_support_collapse"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRealizationExhaustionWitnessRow {
    witness_kind: PrimitiveRealizationExhaustionWitnessKind,
    exhaustion_report: PrimitiveRealizationExhaustionReport,
    row_digest: String,
}

impl PrimitiveRealizationExhaustionWitnessRow {
    pub fn witness_kind(&self) -> PrimitiveRealizationExhaustionWitnessKind {
        self.witness_kind
    }

    pub fn family(&self) -> &'static str {
        self.exhaustion_report.family()
    }

    pub fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        self.exhaustion_report.attempted_strategies()
    }

    pub fn exhaustion_reason(&self) -> PrimitiveRealizationExhaustionReason {
        self.exhaustion_report.exhaustion_reason()
    }

    pub fn exhaustion_report(&self) -> &PrimitiveRealizationExhaustionReport {
        &self.exhaustion_report
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub fn primitive_realization_exhaustion_witness_rows(
) -> Vec<PrimitiveRealizationExhaustionWitnessRow> {
    vec![
        zero_radius_pyramid_support_collapse(),
        zero_scale_simplex_support_collapse(),
        altitude_squeezed_simplex_support_collapse(),
    ]
}

fn zero_radius_pyramid_support_collapse() -> PrimitiveRealizationExhaustionWitnessRow {
    let error = realize_pyramid_support([0.0, 0.0, 0.0], 3, 0.0, 1.0)
        .expect_err("zero-radius pyramid should exhaust sanctioned support strategies");
    let exhaustion_report = match error {
        PrimitiveRealizationError::Exhausted(report) => report,
        PrimitiveRealizationError::Geometry(other) => {
            panic!("expected exhausted realization report, got geometry error: {other}")
        }
    };
    witness_row(
        PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse,
        exhaustion_report,
    )
}

fn zero_scale_simplex_support_collapse() -> PrimitiveRealizationExhaustionWitnessRow {
    let error = realize_tetrahedron_support([0.0, 0.0, 0.0], 0.0)
        .expect_err("zero-scale simplex should exhaust sanctioned support strategies");
    let exhaustion_report = match error {
        PrimitiveRealizationError::Exhausted(report) => report,
        PrimitiveRealizationError::Geometry(other) => {
            panic!("expected exhausted realization report, got geometry error: {other}")
        }
    };
    witness_row(
        PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse,
        exhaustion_report,
    )
}

fn altitude_squeezed_simplex_support_collapse() -> PrimitiveRealizationExhaustionWitnessRow {
    let error =
        realize_tetrahedron_support_with_altitude_component([0.0, 0.0, 0.0], 1.0e-240, 1.0e-280)
            .expect_err("altitude-squeezed simplex should exhaust sanctioned support strategies");
    let exhaustion_report = match error {
        PrimitiveRealizationError::Exhausted(report) => report,
        PrimitiveRealizationError::Geometry(other) => {
            panic!("expected exhausted realization report, got geometry error: {other}")
        }
    };
    witness_row(
        PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse,
        exhaustion_report,
    )
}

fn witness_row(
    witness_kind: PrimitiveRealizationExhaustionWitnessKind,
    exhaustion_report: PrimitiveRealizationExhaustionReport,
) -> PrimitiveRealizationExhaustionWitnessRow {
    let row_digest = digest_parts(&[
        witness_kind.as_str().to_string(),
        exhaustion_report.report_digest().to_string(),
    ]);
    PrimitiveRealizationExhaustionWitnessRow {
        witness_kind,
        exhaustion_report,
        row_digest,
    }
}

fn digest_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::WitnessIdentity, parts)
}
