use super::{
    UiAllocationReceiptEquivalenceBasis, UiAllocationReceiptGeneration,
    UiAllocationReceiptIdentity, UiAllocationReceiptReport, UiAllocationReplanTransaction,
};

/// Committed allocation truth. Only the post-planning receipt-commit lane may mint it.
#[derive(Clone, Debug, PartialEq)]
pub struct UiAllocationReceipt {
    committed_allocation: super::UiCommittedAllocation,
    identity: UiAllocationReceiptIdentity,
    generation: UiAllocationReceiptGeneration,
    equivalence_basis: UiAllocationReceiptEquivalenceBasis,
    report: UiAllocationReceiptReport,
    transaction: UiAllocationReplanTransaction,
    geometry_evidence: super::UiCommittedAllocationGeometryEvidence,
}

impl UiAllocationReceipt {
    pub(crate) fn operationally_matches(&self, other: &Self) -> bool {
        self.equivalence_basis
            .operationally_matches(&other.equivalence_basis)
            && self.geometry_evidence == other.geometry_evidence
            && self.resize_basis() == other.resize_basis()
            && self
                .committed_allocation
                .measurement_basis()
                .operationally_matches(other.committed_allocation.measurement_basis())
    }

    pub(in crate::runtime::allocation_receipt) fn from_candidate(
        candidate: &super::UiAllocationCandidate,
        reuse_verdict: super::UiAllocationReuseVerdict,
        transaction: UiAllocationReplanTransaction,
    ) -> Self {
        let identity = UiAllocationReceiptIdentity::from_candidate(candidate);
        let generation = UiAllocationReceiptGeneration::from_candidate(candidate);
        let report = UiAllocationReceiptReport::new(identity.clone(), generation, reuse_verdict)
            .apply_committed_transaction_freshness(&transaction);
        Self {
            geometry_evidence: super::UiCommittedAllocationGeometryEvidence::from_candidate(
                candidate,
            ),
            committed_allocation: super::UiCommittedAllocation::from_candidate(candidate),
            generation,
            equivalence_basis: UiAllocationReceiptEquivalenceBasis::from_candidate(candidate),
            report,
            transaction,
            identity,
        }
    }

    /// The only execution input emitted by receipt commit.
    pub(crate) fn committed_allocation(&self) -> &super::UiCommittedAllocation {
        &self.committed_allocation
    }
    pub fn identity(&self) -> &UiAllocationReceiptIdentity {
        &self.identity
    }
    pub fn generation(&self) -> UiAllocationReceiptGeneration {
        self.generation
    }
    pub fn equivalence_basis(&self) -> &UiAllocationReceiptEquivalenceBasis {
        &self.equivalence_basis
    }
    pub fn report(&self) -> &UiAllocationReceiptReport {
        &self.report
    }
    pub fn transaction(&self) -> &UiAllocationReplanTransaction {
        &self.transaction
    }
    pub fn geometry_evidence(&self) -> &super::UiCommittedAllocationGeometryEvidence {
        &self.geometry_evidence
    }
    pub fn lowering_input(
        &self,
    ) -> Result<
        super::UiCommittedAllocationLoweringInput,
        super::UiAllocationFreshnessConsumptionDenial,
    > {
        super::UiCommittedAllocationLoweringInput::from_receipt(self)
    }
    pub(super) fn attach_counter_report(&mut self, counters: super::UiAllocationCounterReport) {
        self.report.attach_counters(counters);
    }
    pub fn resize_basis(&self) -> Option<&crate::runtime::UiResizeAllocationPlanningBasis> {
        self.committed_allocation.resize_basis()
    }

    pub fn inspection_receipt(&self) -> crate::evidence::UiAllocationReceiptInspectionReceipt {
        crate::evidence::project_allocation_receipt_inspection(self)
    }

    pub fn truth_category(&self) -> crate::evidence::allocation::UiAllocationTruthCategory {
        crate::evidence::allocation::UiAllocationTruthCategory::CommittedReceipt
    }
}
