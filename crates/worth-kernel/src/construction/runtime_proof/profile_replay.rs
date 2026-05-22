use crate::construction::certification::profile::prepare_primitive_construction_policy_profile_row;
use crate::construction::certification::{
    prepare_primitive_construction_policy_profile_report, PrimitiveConstructionPolicyProfileCase,
    PrimitiveConstructionPolicyProfileRow,
};
use crate::construction::digest::digest_owned_parts;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPolicyProfileReplayParityReport {
    case: PrimitiveConstructionPolicyProfileCase,
    direct_row: PrimitiveConstructionPolicyProfileRow,
    replay_row: PrimitiveConstructionPolicyProfileRow,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPolicyProfileReplayParityReport {
    fn new(
        case: PrimitiveConstructionPolicyProfileCase,
        direct_row: PrimitiveConstructionPolicyProfileRow,
        replay_row: PrimitiveConstructionPolicyProfileRow,
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

    pub fn direct_row(&self) -> &PrimitiveConstructionPolicyProfileRow {
        &self.direct_row
    }

    pub fn replay_row(&self) -> &PrimitiveConstructionPolicyProfileRow {
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
pub enum PrimitiveConstructionPolicyProfileReplayParityError {
    MissingDirectRow(PrimitiveConstructionPolicyProfileCase),
}

impl std::fmt::Display for PrimitiveConstructionPolicyProfileReplayParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDirectRow(case) => {
                write!(f, "missing direct policy profile row for {case:?}")
            }
        }
    }
}

impl std::error::Error for PrimitiveConstructionPolicyProfileReplayParityError {}

pub fn prepare_primitive_construction_policy_profile_replay_parity_report(
    case: PrimitiveConstructionPolicyProfileCase,
) -> Result<
    PrimitiveConstructionPolicyProfileReplayParityReport,
    PrimitiveConstructionPolicyProfileReplayParityError,
> {
    let direct_report = prepare_primitive_construction_policy_profile_report();
    let direct_row = direct_report
        .row(case)
        .ok_or(PrimitiveConstructionPolicyProfileReplayParityError::MissingDirectRow(case))?
        .clone();
    let replay_row = prepare_primitive_construction_policy_profile_row(case);
    Ok(PrimitiveConstructionPolicyProfileReplayParityReport::new(
        case, direct_row, replay_row,
    ))
}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_policy_profile_replay_parity_report;
    use crate::construction::PrimitiveConstructionPolicyProfileCase;

    #[test]
    fn policy_profile_replay_parity_preserves_profile_posture_truth() {
        let report = prepare_primitive_construction_policy_profile_replay_parity_report(
            PrimitiveConstructionPolicyProfileCase::AggressiveSnap,
        )
        .expect("report");

        assert!(report.parity_verified());
        assert_eq!(
            report.direct_row().arbitration_posture(),
            report.replay_row().arbitration_posture()
        );
        assert_eq!(
            report.direct_row().representative_continuity_case(),
            report.replay_row().representative_continuity_case()
        );
    }
}
