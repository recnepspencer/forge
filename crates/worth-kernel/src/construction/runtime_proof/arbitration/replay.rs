use crate::construction::certification::{
    prepare_primitive_construction_preserved_intent_resolution_report,
    PrimitiveConstructionPreservedIntentResolutionCase,
    PrimitiveConstructionPreservedIntentResolutionReportError,
    PrimitiveConstructionPreservedIntentResolutionRow,
};
use crate::construction::digest::digest_owned_parts;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionIntentArbitrationReplayParityReport {
    case: PrimitiveConstructionPreservedIntentResolutionCase,
    direct_row: PrimitiveConstructionPreservedIntentResolutionRow,
    replay_row: PrimitiveConstructionPreservedIntentResolutionRow,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionIntentArbitrationReplayParityReport {
    fn new(
        case: PrimitiveConstructionPreservedIntentResolutionCase,
        direct_row: PrimitiveConstructionPreservedIntentResolutionRow,
        replay_row: PrimitiveConstructionPreservedIntentResolutionRow,
    ) -> Self {
        let parity_verified = direct_row == replay_row;
        let report_digest = digest_owned_parts(&[
            format!("{case:?}"),
            direct_row.row_digest().to_string(),
            replay_row.row_digest().to_string(),
            parity_verified.to_string(),
        ]);
        Self {
            case,
            direct_row,
            replay_row,
            parity_verified,
            report_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionPreservedIntentResolutionCase {
        self.case
    }

    pub fn direct_row(&self) -> &PrimitiveConstructionPreservedIntentResolutionRow {
        &self.direct_row
    }

    pub fn replay_row(&self) -> &PrimitiveConstructionPreservedIntentResolutionRow {
        &self.replay_row
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionIntentArbitrationReplayParityError {
    PreservedReport(PrimitiveConstructionPreservedIntentResolutionReportError),
    MissingDirectRow(PrimitiveConstructionPreservedIntentResolutionCase),
    MissingReplayRow(PrimitiveConstructionPreservedIntentResolutionCase),
}

impl std::fmt::Display for PrimitiveConstructionIntentArbitrationReplayParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PreservedReport(error) => write!(f, "{error}"),
            Self::MissingDirectRow(case) => {
                write!(f, "missing direct preserved arbitration row for {case:?}")
            }
            Self::MissingReplayRow(case) => {
                write!(f, "missing replay preserved arbitration row for {case:?}")
            }
        }
    }
}

impl std::error::Error for PrimitiveConstructionIntentArbitrationReplayParityError {}

pub fn prepare_primitive_construction_intent_arbitration_replay_parity_report(
    case: PrimitiveConstructionPreservedIntentResolutionCase,
) -> Result<
    PrimitiveConstructionIntentArbitrationReplayParityReport,
    PrimitiveConstructionIntentArbitrationReplayParityError,
> {
    let direct_report = prepare_primitive_construction_preserved_intent_resolution_report()
        .map_err(PrimitiveConstructionIntentArbitrationReplayParityError::PreservedReport)?;
    let replay_report = prepare_primitive_construction_preserved_intent_resolution_report()
        .map_err(PrimitiveConstructionIntentArbitrationReplayParityError::PreservedReport)?;
    let direct_row = direct_report
        .row(case)
        .ok_or(PrimitiveConstructionIntentArbitrationReplayParityError::MissingDirectRow(case))?
        .clone();
    let replay_row = replay_report
        .row(case)
        .ok_or(PrimitiveConstructionIntentArbitrationReplayParityError::MissingReplayRow(case))?
        .clone();
    Ok(PrimitiveConstructionIntentArbitrationReplayParityReport::new(case, direct_row, replay_row))
}

#[cfg(test)]
#[path = "replay_tests.rs"]
mod tests;
