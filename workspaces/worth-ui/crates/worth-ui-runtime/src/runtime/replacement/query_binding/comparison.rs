use crate::runtime::replacement::query_binding::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture, WorthUiQueryBindingPostureDriftFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryBindingComparisonOutcome {
    PreserveMeaning,
    RebindRequired,
    MissingActiveBinding,
    MissingCandidateBinding,
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
    posture_drift_count: usize,
    affected_query_invalidation_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingComparisonEntry {
    identity: WorthUiQueryBindingIdentity,
    active_posture: Option<WorthUiQueryBindingPosture>,
    candidate_posture: Option<WorthUiQueryBindingPosture>,
    outcome: WorthUiQueryBindingComparisonOutcome,
    posture_drifts: Vec<WorthUiQueryBindingPostureDriftFamily>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingComparison {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    entries: Vec<WorthUiQueryBindingComparisonEntry>,
    counters: WorthUiQueryBindingComparisonCounters,
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
        posture_drift_count: usize,
    ) {
        self.bindings_compared += 1;
        self.posture_drift_count += posture_drift_count;
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

    pub fn posture_drift_count(&self) -> usize {
        self.posture_drift_count
    }

    pub fn affected_query_invalidation_count(&self) -> usize {
        self.affected_query_invalidation_count
    }
}

impl WorthUiQueryBindingComparisonEntry {
    pub(crate) fn new(
        identity: WorthUiQueryBindingIdentity,
        active_posture: Option<WorthUiQueryBindingPosture>,
        candidate_posture: Option<WorthUiQueryBindingPosture>,
        outcome: WorthUiQueryBindingComparisonOutcome,
        posture_drifts: Vec<WorthUiQueryBindingPostureDriftFamily>,
    ) -> Self {
        Self {
            identity,
            active_posture,
            candidate_posture,
            outcome,
            posture_drifts,
        }
    }

    pub fn identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.identity
    }

    pub fn active_posture(&self) -> Option<&WorthUiQueryBindingPosture> {
        self.active_posture.as_ref()
    }

    pub fn candidate_posture(&self) -> Option<&WorthUiQueryBindingPosture> {
        self.candidate_posture.as_ref()
    }

    pub fn outcome(&self) -> WorthUiQueryBindingComparisonOutcome {
        self.outcome
    }

    pub fn posture_drifts(&self) -> &[WorthUiQueryBindingPostureDriftFamily] {
        &self.posture_drifts
    }
}

impl WorthUiQueryBindingComparison {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        mut entries: Vec<WorthUiQueryBindingComparisonEntry>,
        counters: WorthUiQueryBindingComparisonCounters,
    ) -> Self {
        entries.sort_by(|left, right| left.identity().cmp(right.identity()));
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            entries,
            counters,
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
}
