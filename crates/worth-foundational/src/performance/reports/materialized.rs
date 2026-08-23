use crate::performance::basis::{
    FoundationalPerformanceContractName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceSupportingEvidenceRow,
};
use crate::performance::claims::FoundationalPerformanceObservationContext;
use crate::performance::layouts::FoundationalLayoutIntentClaim;
use crate::performance::policy::FoundationalPerformanceBudgetDecision;
use crate::performance::primitives::FoundationalPerformanceWorkClass;
use crate::performance::{
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture,
};
use crate::profiles::FoundationalProfileSet;

use super::plan::{
    FoundationalPerformanceReportMaterializationBoundary, FoundationalPerformanceReportPlan,
    FoundationalPerformanceReportSection, FoundationalPerformanceReportSectionDecision,
};
use super::source::FoundationalPerformanceReportSource;
use super::targets::FoundationalPerformanceAttachmentTargetKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalMaterializedPerformanceReport<Source> {
    source: Source,
    profile: FoundationalProfileSet,
    target: FoundationalPerformanceAttachmentTargetKind,
    claim_boundary: FoundationalPerformanceBoundary,
    evidence_strength: FoundationalPerformanceEvidenceStrength,
    breadth_locality: FoundationalPerformanceBreadthLocalityPosture,
    access_pattern: FoundationalPerformanceAccessPatternPosture,
    execution_temperature: FoundationalPerformanceExecutionTemperature,
    freshness_retention: FoundationalPerformanceFreshnessRetentionPosture,
    fallback_debt: FoundationalPerformanceFallbackDebtPosture,
    materialization_boundary: FoundationalPerformanceReportMaterializationBoundary,
    decisions: Vec<FoundationalPerformanceReportSectionDecision>,
    included_work: Vec<FoundationalPerformanceWorkClass>,
    excluded_work: Vec<FoundationalPerformanceWorkClass>,
    observation_context: Option<FoundationalPerformanceObservationContext>,
    layout_intent_claim: Option<FoundationalLayoutIntentClaim>,
    contract_names: Vec<FoundationalPerformanceContractName>,
    counter_specs: Vec<FoundationalPerformanceCounterSpec>,
    counter_rows: Vec<FoundationalPerformanceCounterRow>,
    supporting_evidence_rows: Vec<FoundationalPerformanceSupportingEvidenceRow>,
    budget_decisions: Vec<FoundationalPerformanceBudgetDecision>,
    denied_work: Vec<FoundationalPerformanceWorkClass>,
    widened_work: Vec<FoundationalPerformanceWorkClass>,
}

impl<Source> FoundationalMaterializedPerformanceReport<Source> {
    pub const fn source(&self) -> &Source {
        &self.source
    }

    pub const fn profile(&self) -> FoundationalProfileSet {
        self.profile
    }

    pub const fn target(&self) -> FoundationalPerformanceAttachmentTargetKind {
        self.target
    }

    pub const fn boundary(&self) -> FoundationalPerformanceBoundary {
        self.claim_boundary
    }

    pub const fn evidence_strength(&self) -> FoundationalPerformanceEvidenceStrength {
        self.evidence_strength
    }

    pub const fn breadth_locality(&self) -> FoundationalPerformanceBreadthLocalityPosture {
        self.breadth_locality
    }

    pub const fn access_pattern(&self) -> FoundationalPerformanceAccessPatternPosture {
        self.access_pattern
    }

    pub const fn execution_temperature(&self) -> FoundationalPerformanceExecutionTemperature {
        self.execution_temperature
    }

    pub const fn freshness_retention(&self) -> FoundationalPerformanceFreshnessRetentionPosture {
        self.freshness_retention
    }

    pub const fn fallback_debt(&self) -> FoundationalPerformanceFallbackDebtPosture {
        self.fallback_debt
    }

    pub const fn materialization_boundary(
        &self,
    ) -> FoundationalPerformanceReportMaterializationBoundary {
        self.materialization_boundary
    }

    pub fn section_decisions(&self) -> &[FoundationalPerformanceReportSectionDecision] {
        &self.decisions
    }

    pub fn included_work(&self) -> &[FoundationalPerformanceWorkClass] {
        &self.included_work
    }

    pub fn excluded_work(&self) -> &[FoundationalPerformanceWorkClass] {
        &self.excluded_work
    }

    pub fn observation_context(&self) -> Option<&FoundationalPerformanceObservationContext> {
        self.observation_context.as_ref()
    }

    pub const fn layout_intent_claim(&self) -> Option<&FoundationalLayoutIntentClaim> {
        self.layout_intent_claim.as_ref()
    }

    pub fn contract_names(&self) -> &[FoundationalPerformanceContractName] {
        &self.contract_names
    }

    pub fn counter_specs(&self) -> &[FoundationalPerformanceCounterSpec] {
        &self.counter_specs
    }

    pub fn counter_rows(&self) -> &[FoundationalPerformanceCounterRow] {
        &self.counter_rows
    }

    pub fn supporting_evidence_rows(&self) -> &[FoundationalPerformanceSupportingEvidenceRow] {
        &self.supporting_evidence_rows
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
}

impl<Source> FoundationalPerformanceReportPlan<Source>
where
    Source: FoundationalPerformanceReportSource,
{
    pub fn materialize(self) -> FoundationalMaterializedPerformanceReport<Source> {
        let FoundationalPerformanceReportPlan {
            source,
            profile,
            boundary,
            decisions,
        } = self;
        let include = |section| {
            decisions
                .iter()
                .any(|decision| decision.is_included() && decision.section() == section)
        };

        let claim_boundary = source.boundary();
        let evidence_strength = source.evidence_strength();
        let breadth_locality = source.breadth_locality();
        let access_pattern = source.access_pattern();
        let execution_temperature = source.execution_temperature();
        let freshness_retention = source.freshness_retention();
        let fallback_debt = source.fallback_debt();
        let included_work = source.included_work().to_vec();
        let excluded_work = source.excluded_work().to_vec();
        let observation_context = source.observation_context().cloned();
        let layout_intent_claim = if include(FoundationalPerformanceReportSection::LayoutIntent) {
            source.layout_intent_claim().cloned()
        } else {
            None
        };
        let contract_names = if include(FoundationalPerformanceReportSection::ContractNames) {
            source.contract_names().to_vec()
        } else {
            Vec::new()
        };
        let counter_specs = if include(FoundationalPerformanceReportSection::CounterSpecs) {
            source.counter_specs().to_vec()
        } else {
            Vec::new()
        };
        let counter_rows = if include(FoundationalPerformanceReportSection::CounterRows) {
            source
                .counter_rows()
                .map_or_else(Vec::new, ToOwned::to_owned)
        } else {
            Vec::new()
        };
        let supporting_evidence_rows =
            if include(FoundationalPerformanceReportSection::SupportingEvidenceRows) {
                source.supporting_evidence_rows().to_vec()
            } else {
                Vec::new()
            };
        let budget_decisions = if include(FoundationalPerformanceReportSection::BudgetDecisions) {
            source
                .budget_decisions()
                .map_or_else(Vec::new, ToOwned::to_owned)
        } else {
            Vec::new()
        };
        let denied_work = if include(FoundationalPerformanceReportSection::DeniedWork) {
            source
                .denied_work()
                .map_or_else(Vec::new, ToOwned::to_owned)
        } else {
            Vec::new()
        };
        let widened_work = if include(FoundationalPerformanceReportSection::WidenedWork) {
            source
                .widened_work()
                .map_or_else(Vec::new, ToOwned::to_owned)
        } else {
            Vec::new()
        };

        FoundationalMaterializedPerformanceReport {
            target: source.target(),
            profile,
            claim_boundary,
            evidence_strength,
            breadth_locality,
            access_pattern,
            execution_temperature,
            freshness_retention,
            fallback_debt,
            materialization_boundary: boundary,
            decisions,
            source,
            included_work,
            excluded_work,
            observation_context,
            layout_intent_claim,
            contract_names,
            counter_specs,
            counter_rows,
            supporting_evidence_rows,
            budget_decisions,
            denied_work,
            widened_work,
        }
    }
}
