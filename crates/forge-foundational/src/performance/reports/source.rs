use crate::performance::basis::{
    FoundationalPerformanceContractName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceSupportingEvidenceRow,
};
use crate::performance::claims::FoundationalPerformanceClaimSurface;
use crate::performance::layouts::FoundationalLayoutIntentClaim;
use crate::performance::policy::FoundationalPerformanceBudgetDecision;
use crate::performance::primitives::FoundationalPerformanceWorkClass;
use crate::performance::{
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture,
};

use super::attachment::{
    FoundationalAttachedCounterBackedPerformanceReceipt, FoundationalAttachedPerformanceBundle,
    FoundationalAttachedPolicyAdmissionReceipt,
};
use super::targets::FoundationalPerformanceAttachmentTargetKind;

pub trait FoundationalPerformanceReportSource: Clone {
    fn target(&self) -> FoundationalPerformanceAttachmentTargetKind;
    fn boundary(&self) -> FoundationalPerformanceBoundary;
    fn evidence_strength(&self) -> FoundationalPerformanceEvidenceStrength;
    fn breadth_locality(&self) -> FoundationalPerformanceBreadthLocalityPosture;
    fn access_pattern(&self) -> FoundationalPerformanceAccessPatternPosture;
    fn execution_temperature(&self) -> FoundationalPerformanceExecutionTemperature;
    fn freshness_retention(&self) -> FoundationalPerformanceFreshnessRetentionPosture;
    fn fallback_debt(&self) -> FoundationalPerformanceFallbackDebtPosture;
    fn included_work(&self) -> &[FoundationalPerformanceWorkClass];
    fn excluded_work(&self) -> &[FoundationalPerformanceWorkClass];
    fn layout_intent_claim(&self) -> Option<&FoundationalLayoutIntentClaim>;
    fn contract_names(&self) -> &[FoundationalPerformanceContractName];
    fn counter_specs(&self) -> &[FoundationalPerformanceCounterSpec];
    fn counter_rows(&self) -> Option<&[FoundationalPerformanceCounterRow]>;
    fn supporting_evidence_rows(&self) -> &[FoundationalPerformanceSupportingEvidenceRow];
    fn budget_decisions(&self) -> Option<&[FoundationalPerformanceBudgetDecision]>;
    fn denied_work(&self) -> Option<&[FoundationalPerformanceWorkClass]>;
    fn widened_work(&self) -> Option<&[FoundationalPerformanceWorkClass]>;
}

impl<Claim> FoundationalPerformanceReportSource for FoundationalAttachedPerformanceBundle<Claim>
where
    Claim: FoundationalPerformanceClaimSurface + Clone + PartialEq + Eq,
{
    fn target(&self) -> FoundationalPerformanceAttachmentTargetKind {
        self.target()
    }

    fn boundary(&self) -> FoundationalPerformanceBoundary {
        self.bundle().claim().boundary()
    }

    fn evidence_strength(&self) -> FoundationalPerformanceEvidenceStrength {
        self.bundle().claim().evidence_strength()
    }

    fn breadth_locality(&self) -> FoundationalPerformanceBreadthLocalityPosture {
        self.bundle().claim().breadth_locality()
    }

    fn access_pattern(&self) -> FoundationalPerformanceAccessPatternPosture {
        self.bundle().claim().access_pattern()
    }

    fn execution_temperature(&self) -> FoundationalPerformanceExecutionTemperature {
        self.bundle().claim().execution_temperature()
    }

    fn freshness_retention(&self) -> FoundationalPerformanceFreshnessRetentionPosture {
        self.bundle().claim().freshness_retention()
    }

    fn fallback_debt(&self) -> FoundationalPerformanceFallbackDebtPosture {
        self.bundle().claim().fallback_debt()
    }

    fn included_work(&self) -> &[FoundationalPerformanceWorkClass] {
        self.bundle().claim().included_work()
    }

    fn excluded_work(&self) -> &[FoundationalPerformanceWorkClass] {
        self.bundle().claim().excluded_work()
    }

    fn layout_intent_claim(&self) -> Option<&FoundationalLayoutIntentClaim> {
        self.bundle().layout_intent_claim()
    }

    fn contract_names(&self) -> &[FoundationalPerformanceContractName] {
        self.bundle().contract_names()
    }

    fn counter_specs(&self) -> &[FoundationalPerformanceCounterSpec] {
        self.bundle().counter_specs()
    }

    fn counter_rows(&self) -> Option<&[FoundationalPerformanceCounterRow]> {
        None
    }

    fn supporting_evidence_rows(&self) -> &[FoundationalPerformanceSupportingEvidenceRow] {
        self.bundle().supporting_evidence_rows()
    }

    fn budget_decisions(&self) -> Option<&[FoundationalPerformanceBudgetDecision]> {
        None
    }

    fn denied_work(&self) -> Option<&[FoundationalPerformanceWorkClass]> {
        None
    }

    fn widened_work(&self) -> Option<&[FoundationalPerformanceWorkClass]> {
        None
    }
}

impl FoundationalPerformanceReportSource for FoundationalAttachedPolicyAdmissionReceipt {
    fn target(&self) -> FoundationalPerformanceAttachmentTargetKind {
        self.target()
    }

    fn boundary(&self) -> FoundationalPerformanceBoundary {
        self.receipt().claim().boundary()
    }

    fn evidence_strength(&self) -> FoundationalPerformanceEvidenceStrength {
        self.receipt().claim().evidence_strength()
    }

    fn breadth_locality(&self) -> FoundationalPerformanceBreadthLocalityPosture {
        self.receipt().claim().breadth_locality()
    }

    fn access_pattern(&self) -> FoundationalPerformanceAccessPatternPosture {
        self.receipt().claim().access_pattern()
    }

    fn execution_temperature(&self) -> FoundationalPerformanceExecutionTemperature {
        self.receipt().claim().execution_temperature()
    }

    fn freshness_retention(&self) -> FoundationalPerformanceFreshnessRetentionPosture {
        self.receipt().claim().freshness_retention()
    }

    fn fallback_debt(&self) -> FoundationalPerformanceFallbackDebtPosture {
        self.receipt().claim().fallback_debt()
    }

    fn included_work(&self) -> &[FoundationalPerformanceWorkClass] {
        self.receipt().claim().included_work()
    }

    fn excluded_work(&self) -> &[FoundationalPerformanceWorkClass] {
        self.receipt().claim().excluded_work()
    }

    fn layout_intent_claim(&self) -> Option<&FoundationalLayoutIntentClaim> {
        None
    }

    fn contract_names(&self) -> &[FoundationalPerformanceContractName] {
        &[]
    }

    fn counter_specs(&self) -> &[FoundationalPerformanceCounterSpec] {
        &[]
    }

    fn counter_rows(&self) -> Option<&[FoundationalPerformanceCounterRow]> {
        None
    }

    fn supporting_evidence_rows(&self) -> &[FoundationalPerformanceSupportingEvidenceRow] {
        &[]
    }

    fn budget_decisions(&self) -> Option<&[FoundationalPerformanceBudgetDecision]> {
        Some(self.receipt().budget_decisions())
    }

    fn denied_work(&self) -> Option<&[FoundationalPerformanceWorkClass]> {
        Some(self.receipt().denied_work())
    }

    fn widened_work(&self) -> Option<&[FoundationalPerformanceWorkClass]> {
        Some(self.receipt().widened_work())
    }
}

impl<Claim> FoundationalPerformanceReportSource
    for FoundationalAttachedCounterBackedPerformanceReceipt<Claim>
where
    Claim: FoundationalPerformanceClaimSurface + Clone + PartialEq + Eq,
{
    fn target(&self) -> FoundationalPerformanceAttachmentTargetKind {
        self.target()
    }

    fn boundary(&self) -> FoundationalPerformanceBoundary {
        self.receipt().bundle().claim().boundary()
    }

    fn evidence_strength(&self) -> FoundationalPerformanceEvidenceStrength {
        self.receipt().bundle().claim().evidence_strength()
    }

    fn breadth_locality(&self) -> FoundationalPerformanceBreadthLocalityPosture {
        self.receipt().bundle().claim().breadth_locality()
    }

    fn access_pattern(&self) -> FoundationalPerformanceAccessPatternPosture {
        self.receipt().bundle().claim().access_pattern()
    }

    fn execution_temperature(&self) -> FoundationalPerformanceExecutionTemperature {
        self.receipt().bundle().claim().execution_temperature()
    }

    fn freshness_retention(&self) -> FoundationalPerformanceFreshnessRetentionPosture {
        self.receipt().bundle().claim().freshness_retention()
    }

    fn fallback_debt(&self) -> FoundationalPerformanceFallbackDebtPosture {
        self.receipt().bundle().claim().fallback_debt()
    }

    fn included_work(&self) -> &[FoundationalPerformanceWorkClass] {
        self.receipt().bundle().claim().included_work()
    }

    fn excluded_work(&self) -> &[FoundationalPerformanceWorkClass] {
        self.receipt().bundle().claim().excluded_work()
    }

    fn layout_intent_claim(&self) -> Option<&FoundationalLayoutIntentClaim> {
        self.receipt().bundle().layout_intent_claim()
    }

    fn contract_names(&self) -> &[FoundationalPerformanceContractName] {
        self.receipt().bundle().contract_names()
    }

    fn counter_specs(&self) -> &[FoundationalPerformanceCounterSpec] {
        self.receipt().bundle().counter_specs()
    }

    fn counter_rows(&self) -> Option<&[FoundationalPerformanceCounterRow]> {
        Some(self.receipt().counter_rows())
    }

    fn supporting_evidence_rows(&self) -> &[FoundationalPerformanceSupportingEvidenceRow] {
        self.receipt().bundle().supporting_evidence_rows()
    }

    fn budget_decisions(&self) -> Option<&[FoundationalPerformanceBudgetDecision]> {
        None
    }

    fn denied_work(&self) -> Option<&[FoundationalPerformanceWorkClass]> {
        None
    }

    fn widened_work(&self) -> Option<&[FoundationalPerformanceWorkClass]> {
        None
    }
}
