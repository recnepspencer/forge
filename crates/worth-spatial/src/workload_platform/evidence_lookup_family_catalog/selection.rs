use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

use super::declaration::EvidenceLookupFamilyDeclaration;
use super::stage_receipt_identity::EvidenceLookupStageReceiptFamilyIdentity;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupFamilyStageSelectionCounters {
    candidate_family_count: usize,
    receipt_family_match_count: usize,
    stage_match_count: usize,
}

impl EvidenceLookupFamilyStageSelectionCounters {
    pub const fn candidate_family_count(&self) -> usize {
        self.candidate_family_count
    }

    pub const fn receipt_family_match_count(&self) -> usize {
        self.receipt_family_match_count
    }

    pub const fn stage_match_count(&self) -> usize {
        self.stage_match_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupFamilyStageSelection {
    family_identities: Vec<String>,
    counters: EvidenceLookupFamilyStageSelectionCounters,
}

impl EvidenceLookupFamilyStageSelection {
    pub(crate) fn select(
        declarations: &[EvidenceLookupFamilyDeclaration],
        stage: WorkloadEvidenceStage,
        receipt_family: &EvidenceLookupStageReceiptFamilyIdentity,
    ) -> Self {
        let mut counters = EvidenceLookupFamilyStageSelectionCounters {
            candidate_family_count: declarations.len(),
            ..EvidenceLookupFamilyStageSelectionCounters::default()
        };
        let mut family_identities = Vec::new();
        for declaration in declarations {
            if declaration
                .stage_applicability()
                .stage_receipt_family_identity()
                != receipt_family
            {
                continue;
            }
            counters.receipt_family_match_count += 1;
            if declaration.stage_applicability().applies_to(stage) {
                counters.stage_match_count += 1;
                family_identities.push(declaration.identity().as_str().to_string());
            }
        }
        Self {
            family_identities,
            counters,
        }
    }

    pub fn family_identities(&self) -> &[String] {
        &self.family_identities
    }

    pub const fn counters(&self) -> &EvidenceLookupFamilyStageSelectionCounters {
        &self.counters
    }

    pub fn family_count(&self) -> usize {
        self.family_identities.len()
    }
}
