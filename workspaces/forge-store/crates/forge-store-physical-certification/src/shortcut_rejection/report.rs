use std::collections::BTreeSet;

use crate::{ForbiddenShortcutKind, ForbiddenShortcutSet};

use super::{ShortcutRejectionBoundary, SyntheticHarnessShortcutDenialReceipt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntheticHarnessShortcutRejectionDenial {
    MissingRequiredShortcut(ForbiddenShortcutKind),
    MissingRequiredBoundary(ShortcutRejectionBoundary),
    EmptyShortcutDenialReport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntheticHarnessShortcutRejectionReport {
    receipts: Vec<SyntheticHarnessShortcutDenialReceipt>,
}

impl SyntheticHarnessShortcutRejectionReport {
    pub fn from_denied_shortcuts(
        receipts: impl IntoIterator<Item = SyntheticHarnessShortcutDenialReceipt>,
    ) -> Result<Self, SyntheticHarnessShortcutRejectionDenial> {
        let receipts = receipts.into_iter().collect::<Vec<_>>();
        if receipts.is_empty() {
            return Err(SyntheticHarnessShortcutRejectionDenial::EmptyShortcutDenialReport);
        }
        Self { receipts }.require_all_certification_shortcuts_denied()
    }

    pub fn require_all_certification_shortcuts_denied(
        self,
    ) -> Result<Self, SyntheticHarnessShortcutRejectionDenial> {
        let denied = self.denied_shortcut_set();
        for required in ForbiddenShortcutSet::physical_certification_baseline().iter() {
            if !denied.contains(&required) {
                return Err(
                    SyntheticHarnessShortcutRejectionDenial::MissingRequiredShortcut(required),
                );
            }
        }
        let denied_boundaries = self.denied_boundary_set();
        for required in REQUIRED_SHORTCUT_REJECTION_BOUNDARIES {
            if !denied_boundaries.contains(&required) {
                return Err(
                    SyntheticHarnessShortcutRejectionDenial::MissingRequiredBoundary(required),
                );
            }
        }
        Ok(self)
    }

    pub fn receipts(&self) -> &[SyntheticHarnessShortcutDenialReceipt] {
        &self.receipts
    }

    pub fn all_required_shortcuts_denied(&self) -> bool {
        let denied = self.denied_shortcut_set();
        let shortcuts_denied = ForbiddenShortcutSet::physical_certification_baseline()
            .iter()
            .all(|required| denied.contains(&required));
        shortcuts_denied && self.all_required_boundaries_denied()
    }

    pub fn all_required_boundaries_denied(&self) -> bool {
        let denied = self.denied_boundary_set();
        REQUIRED_SHORTCUT_REJECTION_BOUNDARIES
            .into_iter()
            .all(|required| denied.contains(&required))
    }

    fn denied_shortcut_set(&self) -> BTreeSet<ForbiddenShortcutKind> {
        self.receipts
            .iter()
            .map(|receipt| receipt.shortcut())
            .collect()
    }

    fn denied_boundary_set(&self) -> BTreeSet<ShortcutRejectionBoundary> {
        self.receipts
            .iter()
            .map(|receipt| receipt.boundary())
            .collect()
    }
}

const REQUIRED_SHORTCUT_REJECTION_BOUNDARIES: [ShortcutRejectionBoundary; 9] = [
    ShortcutRejectionBoundary::EvidenceLooseLog,
    ShortcutRejectionBoundary::ScenarioJsonAuthority,
    ShortcutRejectionBoundary::EvidenceTerminalProjection,
    ShortcutRejectionBoundary::EvidenceSameRunSelfComparison,
    ShortcutRejectionBoundary::FaultDeliveryPrivateMutation,
    ShortcutRejectionBoundary::OracleFixtureLabel,
    ShortcutRejectionBoundary::TranscriptCopiedFields,
    ShortcutRejectionBoundary::PlanProofProgressionSkipped,
    ShortcutRejectionBoundary::OracleTestSupportVerdict,
];
