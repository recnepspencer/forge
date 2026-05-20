use super::vocabulary::{FoundationalPerformanceBudgetDecision, FoundationalPerformanceBudgetKind};
use crate::performance::claims::FoundationalPolicyAdmissionPerformanceClaim;
use crate::performance::{
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceWorkClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalPolicyAdmissionReceiptConstructionDenial {
    CompileTimeContractsCannotBecomePolicyReceipts,
    MissingBudgetDecisions,
    DuplicateBudgetKind,
    ZeroRequestedBudget,
    OverlappingDeniedAndWidenedWorkDisclosure,
    RejectedReceiptsMustNotAdmitBudget,
    VerifiedDeferredOrDebtReceiptsCannotWidenBudget,
    WidenedReceiptsMustExpandAtLeastOneBudget,
    WidenedReceiptsRequireWidenedWorkDisclosure,
    NonWidenedReceiptsMustNotDiscloseWidenedWork,
    RejectedReceiptsRequireDeniedWorkDisclosure,
    NonRejectedReceiptsMustNotDiscloseDeniedWork,
    StrongerEvidenceMustRemainCounterBackedExecution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalPolicyAdmissionReceipt {
    claim: FoundationalPolicyAdmissionPerformanceClaim,
    budget_decisions: Vec<FoundationalPerformanceBudgetDecision>,
    denied_work: Vec<FoundationalPerformanceWorkClass>,
    widened_work: Vec<FoundationalPerformanceWorkClass>,
    stronger_evidence_still_required: FoundationalPerformanceEvidenceStrength,
}

impl FoundationalPolicyAdmissionReceipt {
    pub fn claim(&self) -> &FoundationalPolicyAdmissionPerformanceClaim {
        &self.claim
    }

    pub fn budget_decisions(&self) -> &[FoundationalPerformanceBudgetDecision] {
        &self.budget_decisions
    }

    pub fn denied_work(&self) -> &[FoundationalPerformanceWorkClass] {
        &self.denied_work
    }

    pub fn widened_work(&self) -> &[FoundationalPerformanceWorkClass] {
        &self.widened_work
    }

    pub const fn stronger_evidence_still_required(
        &self,
    ) -> FoundationalPerformanceEvidenceStrength {
        self.stronger_evidence_still_required
    }

    pub fn boundary(&self) -> crate::performance::FoundationalPerformanceBoundary {
        self.claim.boundary()
    }

    pub fn evidence_strength(&self) -> crate::performance::FoundationalPerformanceEvidenceStrength {
        self.claim.evidence_strength()
    }

    pub fn included_work(&self) -> &[FoundationalPerformanceWorkClass] {
        self.claim.included_work()
    }

    pub fn excluded_work(&self) -> &[FoundationalPerformanceWorkClass] {
        self.claim.excluded_work()
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalPolicyAdmissionReceiptBuilder {
    claim: FoundationalPolicyAdmissionPerformanceClaim,
    budget_decisions: Vec<FoundationalPerformanceBudgetDecision>,
    denied_work: Vec<FoundationalPerformanceWorkClass>,
    widened_work: Vec<FoundationalPerformanceWorkClass>,
    stronger_evidence_still_required: FoundationalPerformanceEvidenceStrength,
}

impl FoundationalPolicyAdmissionReceiptBuilder {
    pub fn new(claim: FoundationalPolicyAdmissionPerformanceClaim) -> Self {
        Self {
            claim,
            budget_decisions: Vec::new(),
            denied_work: Vec::new(),
            widened_work: Vec::new(),
            stronger_evidence_still_required:
                FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt,
        }
    }

    pub fn budget_decision(
        mut self,
        kind: FoundationalPerformanceBudgetKind,
        requested_units: u32,
        admitted_units: u32,
    ) -> Self {
        self.budget_decisions
            .push(FoundationalPerformanceBudgetDecision::new(
                kind,
                requested_units,
                admitted_units,
            ));
        self
    }

    pub fn deny_work(mut self, work_class: FoundationalPerformanceWorkClass) -> Self {
        self.denied_work.push(work_class);
        self
    }

    pub fn widen_work(mut self, work_class: FoundationalPerformanceWorkClass) -> Self {
        self.widened_work.push(work_class);
        self
    }

    pub fn stronger_evidence_still_required(
        mut self,
        evidence_strength: FoundationalPerformanceEvidenceStrength,
    ) -> Self {
        self.stronger_evidence_still_required = evidence_strength;
        self
    }

    pub fn finish(
        mut self,
    ) -> Result<
        FoundationalPolicyAdmissionReceipt,
        FoundationalPolicyAdmissionReceiptConstructionDenial,
    > {
        if self.claim.evidence_strength()
            == FoundationalPerformanceEvidenceStrength::CompileTimeContract
        {
            return Err(
                FoundationalPolicyAdmissionReceiptConstructionDenial::CompileTimeContractsCannotBecomePolicyReceipts,
            );
        }

        if self.budget_decisions.is_empty() {
            return Err(
                FoundationalPolicyAdmissionReceiptConstructionDenial::MissingBudgetDecisions,
            );
        }

        self.budget_decisions
            .sort_by_key(|decision| decision.kind());
        if self
            .budget_decisions
            .windows(2)
            .any(|window| window[0].kind() == window[1].kind())
        {
            return Err(FoundationalPolicyAdmissionReceiptConstructionDenial::DuplicateBudgetKind);
        }

        canonicalize_work_classes(&mut self.denied_work);
        canonicalize_work_classes(&mut self.widened_work);
        if self
            .denied_work
            .iter()
            .any(|work_class| self.widened_work.contains(work_class))
        {
            return Err(
                FoundationalPolicyAdmissionReceiptConstructionDenial::OverlappingDeniedAndWidenedWorkDisclosure,
            );
        }

        if self.stronger_evidence_still_required
            != FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt
        {
            return Err(
                FoundationalPolicyAdmissionReceiptConstructionDenial::StrongerEvidenceMustRemainCounterBackedExecution,
            );
        }

        let fallback = self.claim.fallback_debt();
        let mut observed_widening = false;

        for decision in &self.budget_decisions {
            if decision.requested_units() == 0 {
                return Err(
                    FoundationalPolicyAdmissionReceiptConstructionDenial::ZeroRequestedBudget,
                );
            }
            observed_widening |= decision.admitted_units() > decision.requested_units();
        }

        match fallback {
            FoundationalPerformanceFallbackDebtPosture::Verified
            | FoundationalPerformanceFallbackDebtPosture::Deferred
            | FoundationalPerformanceFallbackDebtPosture::Debt
            | FoundationalPerformanceFallbackDebtPosture::FreshFreezeRebuildReadmissionRequired => {
                if observed_widening {
                    return Err(
                        FoundationalPolicyAdmissionReceiptConstructionDenial::VerifiedDeferredOrDebtReceiptsCannotWidenBudget,
                    );
                }
                if !self.widened_work.is_empty() {
                    return Err(
                        FoundationalPolicyAdmissionReceiptConstructionDenial::NonWidenedReceiptsMustNotDiscloseWidenedWork,
                    );
                }
                if !self.denied_work.is_empty() {
                    return Err(
                        FoundationalPolicyAdmissionReceiptConstructionDenial::NonRejectedReceiptsMustNotDiscloseDeniedWork,
                    );
                }
            }
            FoundationalPerformanceFallbackDebtPosture::Rejected => {
                if self.denied_work.is_empty() {
                    return Err(
                        FoundationalPolicyAdmissionReceiptConstructionDenial::RejectedReceiptsRequireDeniedWorkDisclosure,
                    );
                }
                if !self.widened_work.is_empty() {
                    return Err(
                        FoundationalPolicyAdmissionReceiptConstructionDenial::NonWidenedReceiptsMustNotDiscloseWidenedWork,
                    );
                }
                if self
                    .budget_decisions
                    .iter()
                    .any(|decision| decision.admitted_units() != 0)
                {
                    return Err(
                        FoundationalPolicyAdmissionReceiptConstructionDenial::RejectedReceiptsMustNotAdmitBudget,
                    );
                }
            }
            FoundationalPerformanceFallbackDebtPosture::WidenedWithExplicitDisclosure => {
                if !observed_widening {
                    return Err(
                        FoundationalPolicyAdmissionReceiptConstructionDenial::WidenedReceiptsMustExpandAtLeastOneBudget,
                    );
                }
                if self.widened_work.is_empty() {
                    return Err(
                        FoundationalPolicyAdmissionReceiptConstructionDenial::WidenedReceiptsRequireWidenedWorkDisclosure,
                    );
                }
                if !self.denied_work.is_empty() {
                    return Err(
                        FoundationalPolicyAdmissionReceiptConstructionDenial::NonRejectedReceiptsMustNotDiscloseDeniedWork,
                    );
                }
            }
        }

        Ok(FoundationalPolicyAdmissionReceipt {
            claim: self.claim,
            budget_decisions: self.budget_decisions,
            denied_work: self.denied_work,
            widened_work: self.widened_work,
            stronger_evidence_still_required: self.stronger_evidence_still_required,
        })
    }
}

fn canonicalize_work_classes(work_classes: &mut Vec<FoundationalPerformanceWorkClass>) {
    work_classes.sort();
    work_classes.dedup();
}
