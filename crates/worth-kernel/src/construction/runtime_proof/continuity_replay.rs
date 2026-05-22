use crate::construction::certification::continuity::prepare_primitive_construction_continuity_row;
use crate::construction::certification::{
    prepare_primitive_construction_continuity_surface_report, PrimitiveConstructionContinuityCase,
    PrimitiveConstructionContinuityRow, PrimitiveConstructionContinuitySurfaceReportError,
};
use crate::construction::digest::digest_owned_parts;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionContinuityReplayParityReport {
    case: PrimitiveConstructionContinuityCase,
    direct_row: PrimitiveConstructionContinuityRow,
    replay_row: PrimitiveConstructionContinuityRow,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionContinuityReplayParityReport {
    fn new(
        case: PrimitiveConstructionContinuityCase,
        direct_row: PrimitiveConstructionContinuityRow,
        replay_row: PrimitiveConstructionContinuityRow,
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

    pub fn case(&self) -> PrimitiveConstructionContinuityCase {
        self.case
    }

    pub fn direct_row(&self) -> &PrimitiveConstructionContinuityRow {
        &self.direct_row
    }

    pub fn replay_row(&self) -> &PrimitiveConstructionContinuityRow {
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
pub enum PrimitiveConstructionContinuityReplayParityError {
    Report(PrimitiveConstructionContinuitySurfaceReportError),
    Replay(PrimitiveConstructionContinuitySurfaceReportError),
    MissingDirectRow(PrimitiveConstructionContinuityCase),
}

impl std::fmt::Display for PrimitiveConstructionContinuityReplayParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Report(error) => write!(f, "{error}"),
            Self::Replay(error) => write!(f, "{error}"),
            Self::MissingDirectRow(case) => write!(f, "missing direct continuity row for {case:?}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionContinuityReplayParityError {}

pub fn prepare_primitive_construction_continuity_replay_parity_report(
    case: PrimitiveConstructionContinuityCase,
) -> Result<
    PrimitiveConstructionContinuityReplayParityReport,
    PrimitiveConstructionContinuityReplayParityError,
> {
    let direct_report = prepare_primitive_construction_continuity_surface_report()
        .map_err(PrimitiveConstructionContinuityReplayParityError::Report)?;
    let direct_row = direct_report
        .row(case)
        .ok_or(PrimitiveConstructionContinuityReplayParityError::MissingDirectRow(case))?
        .clone();
    let replay_row = prepare_primitive_construction_continuity_row(case)
        .map_err(PrimitiveConstructionContinuityReplayParityError::Replay)?;
    Ok(PrimitiveConstructionContinuityReplayParityReport::new(
        case, direct_row, replay_row,
    ))
}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_continuity_replay_parity_report;
    use crate::construction::PrimitiveConstructionContinuityCase;

    #[test]
    fn continuity_replay_parity_preserves_blocked_pending_choice_truth() {
        let report = prepare_primitive_construction_continuity_replay_parity_report(
            PrimitiveConstructionContinuityCase::OverlapBlockedPendingChoice,
        )
        .expect("report");

        assert!(report.parity_verified());
        assert_eq!(
            report.direct_row().continuity_class(),
            report.replay_row().continuity_class()
        );
    }
}
