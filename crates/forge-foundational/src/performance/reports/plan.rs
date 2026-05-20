use crate::profiles::{DiagnosticRichnessProfile, FoundationalProfileSet, SupportPostureProfile};

use super::request::FoundationalPerformanceReportRequest;
use super::source::FoundationalPerformanceReportSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceReportSection {
    Claim,
    LayoutIntent,
    ContractNames,
    CounterSpecs,
    CounterRows,
    SupportingEvidenceRows,
    BudgetDecisions,
    DeniedWork,
    WidenedWork,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalPerformanceReportSectionDecisionCause {
    AlwaysPresent,
    Requested,
    NotRequested,
    UnavailableFromSource,
    ProfileElided,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformanceReportSectionDecision {
    section: FoundationalPerformanceReportSection,
    included: bool,
    cause: FoundationalPerformanceReportSectionDecisionCause,
}

impl FoundationalPerformanceReportSectionDecision {
    const fn included(
        section: FoundationalPerformanceReportSection,
        cause: FoundationalPerformanceReportSectionDecisionCause,
    ) -> Self {
        Self {
            section,
            included: true,
            cause,
        }
    }

    const fn excluded(
        section: FoundationalPerformanceReportSection,
        cause: FoundationalPerformanceReportSectionDecisionCause,
    ) -> Self {
        Self {
            section,
            included: false,
            cause,
        }
    }

    pub const fn section(&self) -> FoundationalPerformanceReportSection {
        self.section
    }

    pub const fn is_included(&self) -> bool {
        self.included
    }

    pub const fn cause(&self) -> FoundationalPerformanceReportSectionDecisionCause {
        self.cause
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalPerformanceReportMaterializationBoundary {
    ClaimInspectionOnly,
    ReportAssembly,
    SupportExpansion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalPerformanceReportPlan<Source> {
    pub(super) source: Source,
    pub(super) profile: FoundationalProfileSet,
    pub(super) boundary: FoundationalPerformanceReportMaterializationBoundary,
    pub(super) decisions: Vec<FoundationalPerformanceReportSectionDecision>,
}

impl<Source> FoundationalPerformanceReportPlan<Source> {
    pub const fn source(&self) -> &Source {
        &self.source
    }

    pub const fn profile(&self) -> FoundationalProfileSet {
        self.profile
    }

    pub const fn materialization_boundary(
        &self,
    ) -> FoundationalPerformanceReportMaterializationBoundary {
        self.boundary
    }

    pub fn section_decisions(&self) -> &[FoundationalPerformanceReportSectionDecision] {
        &self.decisions
    }

    pub fn included_sections(&self) -> Vec<FoundationalPerformanceReportSection> {
        self.decisions
            .iter()
            .filter(|decision| decision.is_included())
            .map(|decision| decision.section())
            .collect()
    }

    pub fn excluded_sections(&self) -> Vec<FoundationalPerformanceReportSectionDecision> {
        self.decisions
            .iter()
            .copied()
            .filter(|decision| !decision.is_included())
            .collect()
    }
}

pub fn plan_performance_report<Source>(
    request: FoundationalPerformanceReportRequest<Source>,
) -> FoundationalPerformanceReportPlan<Source>
where
    Source: FoundationalPerformanceReportSource,
{
    let mut decisions = vec![FoundationalPerformanceReportSectionDecision::included(
        FoundationalPerformanceReportSection::Claim,
        FoundationalPerformanceReportSectionDecisionCause::AlwaysPresent,
    )];

    push_optional(
        &mut decisions,
        FoundationalPerformanceReportSection::LayoutIntent,
        request.include_layout_intent,
        request.source.layout_intent_claim().is_some(),
        true,
    );
    push_optional(
        &mut decisions,
        FoundationalPerformanceReportSection::ContractNames,
        request.include_contract_names,
        !request.source.contract_names().is_empty(),
        true,
    );
    push_optional(
        &mut decisions,
        FoundationalPerformanceReportSection::CounterSpecs,
        request.include_counter_specs,
        !request.source.counter_specs().is_empty(),
        true,
    );
    push_optional(
        &mut decisions,
        FoundationalPerformanceReportSection::CounterRows,
        request.include_counter_rows,
        request
            .source
            .counter_rows()
            .is_some_and(|rows| !rows.is_empty()),
        true,
    );

    let support_allowed = request.profile.diagnostic_richness()
        != DiagnosticRichnessProfile::OperationalMinimal
        && request.profile.support_posture() != SupportPostureProfile::InternalOnly;
    push_optional(
        &mut decisions,
        FoundationalPerformanceReportSection::SupportingEvidenceRows,
        request.include_supporting_evidence_rows,
        !request.source.supporting_evidence_rows().is_empty(),
        support_allowed,
    );
    push_optional(
        &mut decisions,
        FoundationalPerformanceReportSection::BudgetDecisions,
        request.include_budget_decisions,
        request
            .source
            .budget_decisions()
            .is_some_and(|rows| !rows.is_empty()),
        true,
    );
    push_optional(
        &mut decisions,
        FoundationalPerformanceReportSection::DeniedWork,
        request.include_denied_work,
        request
            .source
            .denied_work()
            .is_some_and(|rows| !rows.is_empty()),
        true,
    );
    push_optional(
        &mut decisions,
        FoundationalPerformanceReportSection::WidenedWork,
        request.include_widened_work,
        request
            .source
            .widened_work()
            .is_some_and(|rows| !rows.is_empty()),
        true,
    );

    let includes_support = decisions.iter().any(|decision| {
        decision.is_included()
            && decision.section() == FoundationalPerformanceReportSection::SupportingEvidenceRows
    });
    let includes_additional_sections = decisions.iter().any(|decision| {
        decision.is_included() && decision.section() != FoundationalPerformanceReportSection::Claim
    });

    let boundary = if includes_support {
        FoundationalPerformanceReportMaterializationBoundary::SupportExpansion
    } else if includes_additional_sections {
        FoundationalPerformanceReportMaterializationBoundary::ReportAssembly
    } else {
        FoundationalPerformanceReportMaterializationBoundary::ClaimInspectionOnly
    };

    FoundationalPerformanceReportPlan {
        source: request.source,
        profile: request.profile,
        boundary,
        decisions,
    }
}

fn push_optional(
    decisions: &mut Vec<FoundationalPerformanceReportSectionDecision>,
    section: FoundationalPerformanceReportSection,
    requested: bool,
    available: bool,
    profile_allowed: bool,
) {
    let decision = if !requested {
        FoundationalPerformanceReportSectionDecision::excluded(
            section,
            FoundationalPerformanceReportSectionDecisionCause::NotRequested,
        )
    } else if !available {
        FoundationalPerformanceReportSectionDecision::excluded(
            section,
            FoundationalPerformanceReportSectionDecisionCause::UnavailableFromSource,
        )
    } else if !profile_allowed {
        FoundationalPerformanceReportSectionDecision::excluded(
            section,
            FoundationalPerformanceReportSectionDecisionCause::ProfileElided,
        )
    } else {
        FoundationalPerformanceReportSectionDecision::included(
            section,
            FoundationalPerformanceReportSectionDecisionCause::Requested,
        )
    };
    decisions.push(decision);
}
