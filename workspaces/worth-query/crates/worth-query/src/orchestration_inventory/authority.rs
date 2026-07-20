#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum WorthQueryOrchestrationBasisPosture {
    DeclarationEntry,
    ReadmissionAware,
    SignalCompatibilityRetained,
    PreviewSpecialized,
    CurrentTruthViewSpecialized,
    HistoricalTruthViewSpecialized,
    DeclarationScopedContribution,
    GroupedNeighborhoodDeclaration,
    Mixed,
}

impl WorthQueryOrchestrationBasisPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclarationEntry => "declaration_entry",
            Self::ReadmissionAware => "readmission_aware",
            Self::SignalCompatibilityRetained => "signal_compatibility_retained",
            Self::PreviewSpecialized => "preview_specialized",
            Self::CurrentTruthViewSpecialized => "current_truth_view_specialized",
            Self::HistoricalTruthViewSpecialized => "historical_truth_view_specialized",
            Self::DeclarationScopedContribution => "declaration_scoped_contribution",
            Self::GroupedNeighborhoodDeclaration => "grouped_neighborhood_declaration",
            Self::Mixed => "mixed",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum WorthQueryOrchestrationPolicyTenantPosture {
    InheritedPlatformPolicy,
    WorkflowPreviewAware,
}

impl WorthQueryOrchestrationPolicyTenantPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InheritedPlatformPolicy => "inherited_platform_policy",
            Self::WorkflowPreviewAware => "workflow_preview_aware",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum WorthQueryOrchestrationCollaborativeExtensionPosture {
    PlatformEntryReady,
    CollaborativePhasesReady,
}

impl WorthQueryOrchestrationCollaborativeExtensionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PlatformEntryReady => "platform_entry_ready",
            Self::CollaborativePhasesReady => "collaborative_phases_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct WorthQueryOrchestrationLowerAuthorityAttachment {
    relational: bool,
    signal: bool,
    runtime_bridge: bool,
    foundational_profile: bool,
}

impl WorthQueryOrchestrationLowerAuthorityAttachment {
    pub const fn new(
        relational: bool,
        signal: bool,
        runtime_bridge: bool,
        foundational_profile: bool,
    ) -> Self {
        Self {
            relational,
            signal,
            runtime_bridge,
            foundational_profile,
        }
    }

    pub const fn relational_bridge_signal() -> Self {
        Self::new(true, true, true, false)
    }

    pub const fn relational_bridge_signal_foundational() -> Self {
        Self::new(true, true, true, true)
    }

    pub fn includes_relational(self) -> bool {
        self.relational
    }

    pub fn includes_signal(self) -> bool {
        self.signal
    }

    pub fn includes_runtime_bridge(self) -> bool {
        self.runtime_bridge
    }

    pub fn includes_foundational_profile(self) -> bool {
        self.foundational_profile
    }

    pub fn as_str(self) -> &'static str {
        match (
            self.relational,
            self.signal,
            self.runtime_bridge,
            self.foundational_profile,
        ) {
            (true, true, true, true) => "query_over_relational_bridge_signal_foundational",
            (true, true, true, false) => "query_over_relational_bridge_signal",
            (true, true, false, false) => "query_over_relational_signal",
            (true, false, true, false) => "query_over_relational_bridge",
            (false, true, true, false) => "query_over_bridge_signal",
            (true, false, false, false) => "query_over_relational",
            (false, true, false, false) => "query_over_signal",
            (false, false, true, false) => "query_over_runtime_bridge",
            (false, false, false, true) => "query_over_foundational_profile",
            _ => "mixed_attachment",
        }
    }
}
