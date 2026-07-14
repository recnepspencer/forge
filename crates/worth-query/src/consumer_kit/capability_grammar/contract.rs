use crate::consumer_kit::WorthQueryDeclarativeCapabilityFamily as Family;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCapabilityFacadeNamespace {
    Read,
    Aggregate,
    Live,
    History,
    Comparison,
    Preview,
    Mutation,
    Workflow,
    Inspection,
    Domain,
}

impl WorthQueryCapabilityFacadeNamespace {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Read => "facade::read",
            Self::Aggregate => "facade::aggregate",
            Self::Live => "facade::live",
            Self::History => "facade::history",
            Self::Comparison => "facade::comparison",
            Self::Preview => "facade::preview",
            Self::Mutation => "facade::mutation",
            Self::Workflow => "facade::workflow",
            Self::Inspection => "facade::inspection",
            Self::Domain => "facade::domain",
        }
    }

    pub(crate) fn family(self) -> Family {
        match self {
            Self::Read => Family::Read,
            Self::Aggregate => Family::Aggregate,
            Self::Live => Family::Live,
            Self::History => Family::Historical,
            Self::Comparison => Family::Comparison,
            Self::Preview => Family::Preview,
            Self::Mutation => Family::Mutation,
            Self::Workflow => Family::Workflow,
            Self::Inspection => Family::Inspection,
            Self::Domain => Family::DomainExtension,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCapabilityTerminalVocabulary {
    Run,
    OpenAndClose,
}

impl WorthQueryCapabilityTerminalVocabulary {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::OpenAndClose => "open/close",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCapabilityOutcomeContract {
    Read,
    Aggregate,
    Live,
    Historical,
    Comparison,
    Preview,
    Mutation,
    Workflow,
    Inspection,
    Domain,
}

impl WorthQueryCapabilityOutcomeContract {
    pub(crate) fn family(self) -> Family {
        match self {
            Self::Read => Family::Read,
            Self::Aggregate => Family::Aggregate,
            Self::Live => Family::Live,
            Self::Historical => Family::Historical,
            Self::Comparison => Family::Comparison,
            Self::Preview => Family::Preview,
            Self::Mutation => Family::Mutation,
            Self::Workflow => Family::Workflow,
            Self::Inspection => Family::Inspection,
            Self::Domain => Family::DomainExtension,
        }
    }

    pub fn outcome(self) -> &'static str {
        match self {
            Self::Read => "WorthQueryReadOutcome",
            Self::Aggregate => "WorthQueryCountOutcome",
            Self::Live => "WorthQueryLiveOpenOutcome / WorthQueryManagedLiveCloseOutcome",
            Self::Historical => "WorthQueryHistoricalOutcome",
            Self::Comparison => "WorthQueryComparisonOutcome",
            Self::Preview => "WorthQueryPreviewOutcome",
            Self::Mutation => "WorthQueryMutationOutcome",
            Self::Workflow => "WorthQueryWorkflowOutcome",
            Self::Inspection => "WorthQueryInspectionOutcome",
            Self::Domain => "WorthQueryDomainOutcome",
        }
    }

    pub fn stop(self) -> &'static str {
        match self {
            Self::Read => "WorthQueryReadStop",
            Self::Aggregate => "WorthQueryCountDeclarationStop / WorthQueryReadStop",
            Self::Live => "WorthQueryLiveDeclarationStop / WorthQueryLiveOpenStop",
            Self::Historical => "WorthQueryHistoricalStop",
            Self::Comparison => "WorthQueryComparisonStop",
            Self::Preview => "WorthQueryPreviewStop",
            Self::Mutation => "WorthQueryMutationStop",
            Self::Workflow => "WorthQueryWorkflowStop",
            Self::Inspection => "WorthQueryInspectionStop",
            Self::Domain => "WorthQueryDomainStop",
        }
    }

    pub fn next_action(self) -> &'static str {
        match self {
            Self::Read | Self::Aggregate | Self::Live => "WorthQueryReadNextAction",
            Self::Historical => "WorthQueryHistoricalNextAction",
            Self::Comparison => "WorthQueryComparisonNextAction",
            Self::Preview => "WorthQueryPreviewNextAction",
            Self::Mutation => "WorthQueryMutationNextAction",
            Self::Workflow => "WorthQueryWorkflowNextAction",
            Self::Inspection => "WorthQueryInspectionNextAction",
            Self::Domain => "WorthQueryDomainNextAction",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryCapabilityTranscriptOwner {
    PhaseFiveReadExecution,
    PhaseSixManagedLive,
    PhaseSevenHistoricalComparison,
    PhaseEightWorkflowOrchestration,
    PhaseNineOutcomeInspection,
}

impl WorthQueryCapabilityTranscriptOwner {
    pub fn phase(self) -> u8 {
        match self {
            Self::PhaseFiveReadExecution => 5,
            Self::PhaseSixManagedLive => 6,
            Self::PhaseSevenHistoricalComparison => 7,
            Self::PhaseEightWorkflowOrchestration => 8,
            Self::PhaseNineOutcomeInspection => 9,
        }
    }

    pub(crate) fn owns(self, family: Family) -> bool {
        match self {
            Self::PhaseFiveReadExecution => matches!(family, Family::Read | Family::Aggregate),
            Self::PhaseSixManagedLive => family == Family::Live,
            Self::PhaseSevenHistoricalComparison => {
                matches!(family, Family::Historical | Family::Comparison)
            }
            Self::PhaseEightWorkflowOrchestration => matches!(
                family,
                Family::Preview | Family::Mutation | Family::Workflow | Family::DomainExtension
            ),
            Self::PhaseNineOutcomeInspection => family == Family::Inspection,
        }
    }
}
