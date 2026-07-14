use crate::profiles::FoundationalProfileSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalPerformanceReportRequest<Source> {
    pub source: Source,
    pub profile: FoundationalProfileSet,
    pub include_layout_intent: bool,
    pub include_contract_names: bool,
    pub include_counter_specs: bool,
    pub include_counter_rows: bool,
    pub include_supporting_evidence_rows: bool,
    pub include_budget_decisions: bool,
    pub include_denied_work: bool,
    pub include_widened_work: bool,
}
