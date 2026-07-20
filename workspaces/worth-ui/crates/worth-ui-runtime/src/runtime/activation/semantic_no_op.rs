use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;
use crate::runtime::{WorthUiPlanConstructionCounters, WorthUiPlanEquivalenceSummary};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiNoOpProvenancePosture {
    PriorAdmittedMappingPreserved,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiNoOpQueryPosture {
    ActiveBindingPreserved,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticNoOpWork {
    candidate_node_visit_count: usize,
    candidate_region_reuse_count: usize,
    candidate_region_construction_count: usize,
    exact_region_comparison_count: usize,
}

impl WorthUiSemanticNoOpWork {
    pub(crate) fn new(
        candidate_construction: WorthUiPlanConstructionCounters,
        exact_region_comparison_count: usize,
    ) -> Self {
        let regional = candidate_construction.regional_storage();
        Self {
            candidate_node_visit_count: candidate_construction.full_candidate_node_visit_count(),
            candidate_region_reuse_count: regional.reuse_count(),
            candidate_region_construction_count: regional.region_construction_count(),
            exact_region_comparison_count,
        }
    }

    pub fn candidate_node_visit_count(self) -> usize {
        self.candidate_node_visit_count
    }

    pub fn candidate_region_reuse_count(self) -> usize {
        self.candidate_region_reuse_count
    }

    pub fn candidate_region_construction_count(self) -> usize {
        self.candidate_region_construction_count
    }

    pub fn exact_region_comparison_count(self) -> usize {
        self.exact_region_comparison_count
    }

    pub fn activation_publication_count(self) -> usize {
        0
    }

    pub fn scheduler_transition_count(self) -> usize {
        0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticNoOpReceipt {
    candidate_generation: WorthUiPreparedApplicationGenerationIdentity,
    active_generation: WorthUiPreparedApplicationGenerationIdentity,
    equivalence: WorthUiPlanEquivalenceSummary,
    work: WorthUiSemanticNoOpWork,
    candidate_construction: WorthUiPlanConstructionCounters,
}

impl WorthUiSemanticNoOpReceipt {
    pub(crate) fn new(
        candidate_generation: WorthUiPreparedApplicationGenerationIdentity,
        active_generation: WorthUiPreparedApplicationGenerationIdentity,
        equivalence: WorthUiPlanEquivalenceSummary,
        candidate_construction: WorthUiPlanConstructionCounters,
    ) -> Self {
        Self {
            candidate_generation,
            active_generation,
            equivalence,
            work: WorthUiSemanticNoOpWork::new(
                candidate_construction,
                equivalence.exact_region_comparison_count(),
            ),
            candidate_construction,
        }
    }

    pub fn candidate_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.candidate_generation
    }

    pub fn active_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.active_generation
    }

    pub fn equivalence(&self) -> WorthUiPlanEquivalenceSummary {
        self.equivalence
    }

    pub fn work(&self) -> WorthUiSemanticNoOpWork {
        self.work
    }

    pub fn candidate_construction(&self) -> WorthUiPlanConstructionCounters {
        self.candidate_construction
    }

    pub fn provenance_posture(&self) -> WorthUiNoOpProvenancePosture {
        WorthUiNoOpProvenancePosture::PriorAdmittedMappingPreserved
    }

    pub fn query_posture(&self) -> WorthUiNoOpQueryPosture {
        WorthUiNoOpQueryPosture::ActiveBindingPreserved
    }
}
