#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopologyMilestoneNineAuthorityOccurrenceStatus {
    LedgeredWithinCap,
    UnledgeredOccurrence,
    ExceededLedgerCap,
    LedgeredButMissingFromScan,
}

impl WorthTopologyMilestoneNineAuthorityOccurrenceStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LedgeredWithinCap => "ledgered-within-cap",
            Self::UnledgeredOccurrence => "unledgered-occurrence",
            Self::ExceededLedgerCap => "exceeded-ledger-cap",
            Self::LedgeredButMissingFromScan => "ledgered-but-missing-from-scan",
        }
    }

    pub const fn is_violation(self) -> bool {
        matches!(
            self,
            Self::UnledgeredOccurrence | Self::ExceededLedgerCap | Self::LedgeredButMissingFromScan
        )
    }
}

pub(in crate::validator_invariant_catalog::milestone_nine_closeout::authority_occurrence_inventory) const fn occurrence_status(
    observed_count: usize,
    ledger_allowed_count: usize,
) -> WorthTopologyMilestoneNineAuthorityOccurrenceStatus {
    if observed_count == 0 && ledger_allowed_count > 0 {
        WorthTopologyMilestoneNineAuthorityOccurrenceStatus::LedgeredButMissingFromScan
    } else if observed_count > 0 && ledger_allowed_count == 0 {
        WorthTopologyMilestoneNineAuthorityOccurrenceStatus::UnledgeredOccurrence
    } else if observed_count > ledger_allowed_count {
        WorthTopologyMilestoneNineAuthorityOccurrenceStatus::ExceededLedgerCap
    } else {
        WorthTopologyMilestoneNineAuthorityOccurrenceStatus::LedgeredWithinCap
    }
}
