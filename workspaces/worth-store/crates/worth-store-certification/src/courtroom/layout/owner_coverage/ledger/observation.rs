use std::collections::{BTreeMap, BTreeSet};

use super::LayoutOwnerFamily;
use crate::courtroom::layout::executed_evidence::{
    LayoutExecutedEvidenceKind, LayoutExecutedEvidenceReceipt,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct LayoutOwnerObservationLedger {
    observed: BTreeMap<LayoutOwnerFamily, BTreeSet<&'static str>>,
    duplicates: BTreeSet<(LayoutOwnerFamily, &'static str)>,
    executed_evidence: LayoutExecutedEvidenceReceipt,
}

impl LayoutOwnerObservationLedger {
    pub(in crate::courtroom::layout::owner_coverage) fn record(
        &mut self,
        family: LayoutOwnerFamily,
        case: &'static str,
    ) {
        if !self.observed.entry(family).or_default().insert(case) {
            self.duplicates.insert((family, case));
        }
    }

    pub(in crate::courtroom::layout::owner_coverage) fn record_executed_evidence(
        &mut self,
        evidence: LayoutExecutedEvidenceKind,
    ) {
        self.executed_evidence.record(evidence);
    }

    pub(in crate::courtroom::layout::owner_coverage) fn executed_evidence(
        &self,
    ) -> &LayoutExecutedEvidenceReceipt {
        &self.executed_evidence
    }

    pub(in crate::courtroom::layout::owner_coverage) fn observed(
        &self,
        family: LayoutOwnerFamily,
    ) -> BTreeSet<&'static str> {
        self.observed.get(&family).cloned().unwrap_or_default()
    }

    pub(in crate::courtroom::layout::owner_coverage) fn duplicates(
        &self,
    ) -> impl Iterator<Item = (LayoutOwnerFamily, &'static str)> + '_ {
        self.duplicates.iter().copied()
    }
}
