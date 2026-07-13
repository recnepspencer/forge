use super::selection_issuance::{IssuedSelection, SelectionIssuedPayload};
use super::{
    AccessPlanSelectionDenied, SelectedBTreeLookup, SelectedBTreeReplayRecovery,
    SelectedDegradedExactScan, SelectedLsmCompaction, SelectedLsmLookup, SelectedLsmReplayRecovery,
    SelectedLsmRunPublication,
};

#[derive(Debug, PartialEq, Eq)]
pub struct AccessPlanSelectionOutcome {
    issued: IssuedSelection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessPlanSelectionView<'a> {
    BTreeLookup(&'a SelectedBTreeLookup),
    BTreeReplayRecovery(&'a SelectedBTreeReplayRecovery),
    LsmLookup(&'a SelectedLsmLookup),
    LsmRunPublication(&'a SelectedLsmRunPublication),
    LsmReplayRecovery(&'a SelectedLsmReplayRecovery),
    LsmCompaction(&'a SelectedLsmCompaction),
    Degraded(&'a SelectedDegradedExactScan),
    Denied(&'a AccessPlanSelectionDenied),
}

impl AccessPlanSelectionOutcome {
    pub(super) const fn from_issued(issued: IssuedSelection) -> Self {
        Self { issued }
    }

    pub fn view(&self) -> AccessPlanSelectionView<'_> {
        match self.issued.payload() {
            SelectionIssuedPayload::BTreeLookup(plan) => AccessPlanSelectionView::BTreeLookup(plan),
            SelectionIssuedPayload::BTreeReplayRecovery(plan) => {
                AccessPlanSelectionView::BTreeReplayRecovery(plan)
            }
            SelectionIssuedPayload::LsmLookup(plan) => AccessPlanSelectionView::LsmLookup(plan),
            SelectionIssuedPayload::LsmRunPublication(plan) => {
                AccessPlanSelectionView::LsmRunPublication(plan)
            }
            SelectionIssuedPayload::LsmReplayRecovery(plan) => {
                AccessPlanSelectionView::LsmReplayRecovery(plan)
            }
            SelectionIssuedPayload::LsmCompaction(plan) => {
                AccessPlanSelectionView::LsmCompaction(plan)
            }
            SelectionIssuedPayload::Degraded(plan) => AccessPlanSelectionView::Degraded(plan),
            SelectionIssuedPayload::Denied(denial) => AccessPlanSelectionView::Denied(denial),
        }
    }

    #[cfg(test)]
    pub(crate) fn case(&self) -> super::decision::AccessPlanSelectionCase {
        use super::decision::AccessPlanSelectionCase;

        match self.view() {
            AccessPlanSelectionView::BTreeLookup(plan) => match plan.operation() {
                super::BTreeLookupOperation::Point => AccessPlanSelectionCase::BTreePointLookup,
                super::BTreeLookupOperation::Range => AccessPlanSelectionCase::BTreeRangeLookup,
                super::BTreeLookupOperation::Prefix => AccessPlanSelectionCase::BTreePrefixLookup,
            },
            AccessPlanSelectionView::BTreeReplayRecovery(_) => {
                AccessPlanSelectionCase::BTreeReplayRecovery
            }
            AccessPlanSelectionView::LsmLookup(_) => AccessPlanSelectionCase::LsmLookup,
            AccessPlanSelectionView::LsmRunPublication(_) => {
                AccessPlanSelectionCase::LsmRunPublication
            }
            AccessPlanSelectionView::LsmReplayRecovery(_) => {
                AccessPlanSelectionCase::LsmReplayRecovery
            }
            AccessPlanSelectionView::LsmCompaction(_) => AccessPlanSelectionCase::LsmCompaction,
            AccessPlanSelectionView::Degraded(_) => AccessPlanSelectionCase::DegradedExactScan,
            AccessPlanSelectionView::Denied(_) => AccessPlanSelectionCase::Denied,
        }
    }

    #[cfg(test)]
    pub fn unwrap_err(self) -> AccessPlanSelectionDenied {
        match self.issued.into_payload() {
            SelectionIssuedPayload::Denied(denial) => denial,
            SelectionIssuedPayload::BTreeLookup(_)
            | SelectionIssuedPayload::BTreeReplayRecovery(_)
            | SelectionIssuedPayload::LsmLookup(_)
            | SelectionIssuedPayload::LsmRunPublication(_)
            | SelectionIssuedPayload::LsmReplayRecovery(_)
            | SelectionIssuedPayload::LsmCompaction(_)
            | SelectionIssuedPayload::Degraded(_) => {
                panic!("selection unexpectedly succeeded")
            }
        }
    }

    pub fn into_btree_lookup(self) -> Result<SelectedBTreeLookup, Self> {
        match self.issued.payload() {
            SelectionIssuedPayload::BTreeLookup(_) => {
                let SelectionIssuedPayload::BTreeLookup(plan) = self.issued.into_payload() else {
                    unreachable!()
                };
                Ok(plan)
            }
            _ => Err(self),
        }
    }

    pub fn into_btree_replay_recovery(self) -> Result<SelectedBTreeReplayRecovery, Self> {
        match self.issued.payload() {
            SelectionIssuedPayload::BTreeReplayRecovery(_) => {
                let SelectionIssuedPayload::BTreeReplayRecovery(plan) = self.issued.into_payload()
                else {
                    unreachable!()
                };
                Ok(plan)
            }
            _ => Err(self),
        }
    }

    pub fn into_lsm_lookup(self) -> Result<SelectedLsmLookup, Self> {
        match self.issued.payload() {
            SelectionIssuedPayload::LsmLookup(_) => {
                let SelectionIssuedPayload::LsmLookup(plan) = self.issued.into_payload() else {
                    unreachable!()
                };
                Ok(plan)
            }
            _ => Err(self),
        }
    }

    pub fn into_lsm_run_publication(self) -> Result<SelectedLsmRunPublication, Self> {
        match self.issued.payload() {
            SelectionIssuedPayload::LsmRunPublication(_) => {
                let SelectionIssuedPayload::LsmRunPublication(plan) = self.issued.into_payload()
                else {
                    unreachable!()
                };
                Ok(plan)
            }
            _ => Err(self),
        }
    }

    pub fn into_lsm_replay_recovery(self) -> Result<SelectedLsmReplayRecovery, Self> {
        match self.issued.payload() {
            SelectionIssuedPayload::LsmReplayRecovery(_) => {
                let SelectionIssuedPayload::LsmReplayRecovery(plan) = self.issued.into_payload()
                else {
                    unreachable!()
                };
                Ok(plan)
            }
            _ => Err(self),
        }
    }

    pub fn into_lsm_compaction(self) -> Result<SelectedLsmCompaction, Self> {
        match self.issued.payload() {
            SelectionIssuedPayload::LsmCompaction(_) => {
                let SelectionIssuedPayload::LsmCompaction(plan) = self.issued.into_payload() else {
                    unreachable!()
                };
                Ok(plan)
            }
            _ => Err(self),
        }
    }

    pub fn into_degraded(self) -> Result<SelectedDegradedExactScan, Self> {
        match self.issued.payload() {
            SelectionIssuedPayload::Degraded(_) => {
                let SelectionIssuedPayload::Degraded(plan) = self.issued.into_payload() else {
                    unreachable!()
                };
                Ok(plan)
            }
            _ => Err(self),
        }
    }
}
