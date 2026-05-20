use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::exhaustion::{
    PrimitiveRealizationError, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationExhaustionReport,
};
use super::support::realize_pyramid_support;
use super::PrimitiveRealizationStrategy;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveRealizationExhaustionWitnessKind {
    ZeroRadiusPyramidSupportCollapse,
}

impl PrimitiveRealizationExhaustionWitnessKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ZeroRadiusPyramidSupportCollapse => "zero_radius_pyramid_support_collapse",
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
    vec![zero_radius_pyramid_support_collapse()]
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
    let mut hasher = DefaultHasher::new();
    for part in parts {
        part.hash(&mut hasher);
    }
    format!("{:016x}", hasher.finish())
}
