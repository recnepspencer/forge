use crate::construction::certification::preview::prepare_primitive_construction_preview_row;
use crate::construction::certification::{
    prepare_primitive_construction_preview_surface_report, PrimitiveConstructionPreviewCase,
    PrimitiveConstructionPreviewRow, PrimitiveConstructionPreviewSurfaceReportError,
};
use crate::construction::digest::digest_owned_parts;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPreviewReplayParityReport {
    case: PrimitiveConstructionPreviewCase,
    direct_row: PrimitiveConstructionPreviewRow,
    replay_row: PrimitiveConstructionPreviewRow,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPreviewReplayParityReport {
    fn new(
        case: PrimitiveConstructionPreviewCase,
        direct_row: PrimitiveConstructionPreviewRow,
        replay_row: PrimitiveConstructionPreviewRow,
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

    pub fn case(&self) -> PrimitiveConstructionPreviewCase {
        self.case
    }

    pub fn direct_row(&self) -> &PrimitiveConstructionPreviewRow {
        &self.direct_row
    }

    pub fn replay_row(&self) -> &PrimitiveConstructionPreviewRow {
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
pub enum PrimitiveConstructionPreviewReplayParityError {
    Report(PrimitiveConstructionPreviewSurfaceReportError),
    MissingDirectRow(PrimitiveConstructionPreviewCase),
}

impl std::fmt::Display for PrimitiveConstructionPreviewReplayParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Report(error) => write!(f, "{error}"),
            Self::MissingDirectRow(case) => write!(f, "missing direct preview row for {case:?}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPreviewReplayParityError {}

pub fn prepare_primitive_construction_preview_replay_parity_report(
    case: PrimitiveConstructionPreviewCase,
) -> Result<
    PrimitiveConstructionPreviewReplayParityReport,
    PrimitiveConstructionPreviewReplayParityError,
> {
    let direct_report = prepare_primitive_construction_preview_surface_report()
        .map_err(PrimitiveConstructionPreviewReplayParityError::Report)?;
    let direct_row = direct_report
        .row(case)
        .ok_or(PrimitiveConstructionPreviewReplayParityError::MissingDirectRow(case))?
        .clone();
    let replay_row = prepare_primitive_construction_preview_row(case);
    Ok(PrimitiveConstructionPreviewReplayParityReport::new(
        case, direct_row, replay_row,
    ))
}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_preview_replay_parity_report;
    use crate::construction::PrimitiveConstructionPreviewCase;

    #[test]
    fn preview_replay_parity_preserves_profile_and_blocked_candidate_truth() {
        let report = prepare_primitive_construction_preview_replay_parity_report(
            PrimitiveConstructionPreviewCase::OverlapBlockedMerge,
        )
        .expect("report");

        assert!(report.parity_verified());
        assert_eq!(
            report.direct_row().blocked_candidates(),
            report.replay_row().blocked_candidates()
        );
        assert_eq!(
            report.direct_row().warnings(),
            report.replay_row().warnings()
        );
    }
}
