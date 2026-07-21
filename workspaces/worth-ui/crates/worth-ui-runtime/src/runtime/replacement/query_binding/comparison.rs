use crate::runtime::replacement::query_binding::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingUiRequirements,
    WorthUiQueryBindingUiRequirementsDriftFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryBindingComparisonOutcome {
    PreserveMeaning,
    RebindRequired,
    MissingActiveBinding,
    MissingCandidateBinding,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiQueryBindingAuthorityDrift {
    InstalledAuthority,
    InstallationCurrentness,
    BindingIdentity,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiQueryBindingComparisonCounters {
    active_bindings_indexed: usize,
    candidate_bindings_indexed: usize,
    bindings_compared: usize,
    preserved_meaning_count: usize,
    rebind_required_count: usize,
    missing_active_binding_count: usize,
    missing_candidate_binding_count: usize,
    ui_requirement_drift_count: usize,
    affected_query_invalidation_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingComparisonEntry {
    identity: WorthUiQueryBindingIdentity,
    active_ui_requirements: Option<WorthUiQueryBindingUiRequirements>,
    candidate_ui_requirements: Option<WorthUiQueryBindingUiRequirements>,
    outcome: WorthUiQueryBindingComparisonOutcome,
    ui_requirement_drifts: Vec<WorthUiQueryBindingUiRequirementsDriftFamily>,
    authority_drifts: Vec<WorthUiQueryBindingAuthorityDrift>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingComparison {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    entries: Vec<WorthUiQueryBindingComparisonEntry>,
    counters: WorthUiQueryBindingComparisonCounters,
    exact_invalidations: Vec<crate::runtime::WorthUiQueryDependencyInvalidation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiQueryBindingComparisonDenial {
    NodePlanDigestMismatch {
        runtime_active_artifact_digest: u64,
        plan_active_artifact_digest: u64,
        admitted_candidate_artifact_digest: u64,
        plan_candidate_artifact_digest: u64,
    },
    NarrowingDigestMismatch {
        plan_active_artifact_digest: u64,
        narrowing_active_artifact_digest: u64,
        plan_candidate_artifact_digest: u64,
        narrowing_candidate_artifact_digest: u64,
    },
    AmbiguousNodeReplacementPlan,
}

impl WorthUiQueryBindingComparisonCounters {
    pub(crate) fn record_active_bindings_indexed(&mut self, count: usize) {
        self.active_bindings_indexed = count;
    }

    pub(crate) fn record_candidate_bindings_indexed(&mut self, count: usize) {
        self.candidate_bindings_indexed = count;
    }

    pub(crate) fn record_affected_query_invalidations(&mut self, count: usize) {
        self.affected_query_invalidation_count = count;
    }

    pub(crate) fn record_entry(
        &mut self,
        outcome: WorthUiQueryBindingComparisonOutcome,
        ui_requirement_drift_count: usize,
    ) {
        self.bindings_compared += 1;
        self.ui_requirement_drift_count += ui_requirement_drift_count;
        match outcome {
            WorthUiQueryBindingComparisonOutcome::PreserveMeaning => {
                self.preserved_meaning_count += 1;
            }
            WorthUiQueryBindingComparisonOutcome::RebindRequired => {
                self.rebind_required_count += 1;
            }
            WorthUiQueryBindingComparisonOutcome::MissingActiveBinding => {
                self.missing_active_binding_count += 1;
            }
            WorthUiQueryBindingComparisonOutcome::MissingCandidateBinding => {
                self.missing_candidate_binding_count += 1;
            }
        }
    }

    pub fn active_bindings_indexed(&self) -> usize {
        self.active_bindings_indexed
    }

    pub fn candidate_bindings_indexed(&self) -> usize {
        self.candidate_bindings_indexed
    }

    pub fn bindings_compared(&self) -> usize {
        self.bindings_compared
    }

    pub fn preserved_meaning_count(&self) -> usize {
        self.preserved_meaning_count
    }

    pub fn rebind_required_count(&self) -> usize {
        self.rebind_required_count
    }

    pub fn missing_active_binding_count(&self) -> usize {
        self.missing_active_binding_count
    }

    pub fn missing_candidate_binding_count(&self) -> usize {
        self.missing_candidate_binding_count
    }

    pub fn ui_requirement_drift_count(&self) -> usize {
        self.ui_requirement_drift_count
    }

    pub fn affected_query_invalidation_count(&self) -> usize {
        self.affected_query_invalidation_count
    }
}

impl WorthUiQueryBindingComparisonEntry {
    pub(crate) fn new(
        identity: WorthUiQueryBindingIdentity,
        active_ui_requirements: Option<WorthUiQueryBindingUiRequirements>,
        candidate_ui_requirements: Option<WorthUiQueryBindingUiRequirements>,
        outcome: WorthUiQueryBindingComparisonOutcome,
        ui_requirement_drifts: Vec<WorthUiQueryBindingUiRequirementsDriftFamily>,
        authority_drifts: Vec<WorthUiQueryBindingAuthorityDrift>,
    ) -> Self {
        Self {
            identity,
            active_ui_requirements,
            candidate_ui_requirements,
            outcome,
            ui_requirement_drifts,
            authority_drifts,
        }
    }

    pub fn identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.identity
    }

    pub fn active_ui_requirements(&self) -> Option<&WorthUiQueryBindingUiRequirements> {
        self.active_ui_requirements.as_ref()
    }

    pub fn candidate_ui_requirements(&self) -> Option<&WorthUiQueryBindingUiRequirements> {
        self.candidate_ui_requirements.as_ref()
    }

    pub fn outcome(&self) -> WorthUiQueryBindingComparisonOutcome {
        self.outcome
    }

    pub fn ui_requirement_drifts(&self) -> &[WorthUiQueryBindingUiRequirementsDriftFamily] {
        &self.ui_requirement_drifts
    }

    pub fn has_query_authority_drift(&self) -> bool {
        !self.authority_drifts.is_empty()
    }

    pub(crate) fn requires_ui_invalidation(&self) -> bool {
        self.outcome != WorthUiQueryBindingComparisonOutcome::PreserveMeaning
            || !self.ui_requirement_drifts.is_empty()
    }
}

impl WorthUiQueryBindingComparison {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        mut entries: Vec<WorthUiQueryBindingComparisonEntry>,
        counters: WorthUiQueryBindingComparisonCounters,
        mut exact_invalidations: Vec<crate::runtime::WorthUiQueryDependencyInvalidation>,
    ) -> Self {
        entries.sort_by(|left, right| left.identity().cmp(right.identity()));
        exact_invalidations.sort();
        exact_invalidations.dedup();
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            entries,
            counters,
            exact_invalidations,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn entries(&self) -> &[WorthUiQueryBindingComparisonEntry] {
        &self.entries
    }

    pub fn counters(&self) -> WorthUiQueryBindingComparisonCounters {
        self.counters
    }

    pub fn binding_for_view_binding_id(
        &self,
        view_binding_id: &str,
    ) -> Option<&WorthUiQueryBindingComparisonEntry> {
        self.entries
            .iter()
            .find(|entry| entry.identity().view_binding_id() == view_binding_id)
    }

    pub(crate) fn exact_invalidations(
        &self,
    ) -> &[crate::runtime::WorthUiQueryDependencyInvalidation] {
        &self.exact_invalidations
    }
}
