use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;
use crate::graph::UiGraphFactIndexBasis;
use crate::runtime::observation::UiChangeClassificationBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAffectedScopeBasis {
    classification: UiChangeClassificationBasis,
    predecessor_graph: UiGraphFactIndexBasis,
    candidate_generation: WorthUiPreparedApplicationGenerationIdentity,
    candidate_graph: UiGraphFactIndexBasis,
}

impl UiAffectedScopeBasis {
    pub(crate) fn new(
        classification: UiChangeClassificationBasis,
        predecessor_graph: UiGraphFactIndexBasis,
        candidate_generation: WorthUiPreparedApplicationGenerationIdentity,
        candidate_graph: UiGraphFactIndexBasis,
    ) -> Self {
        Self {
            classification,
            predecessor_graph,
            candidate_generation,
            candidate_graph,
        }
    }

    pub const fn classification(&self) -> &UiChangeClassificationBasis {
        &self.classification
    }

    pub const fn predecessor_graph(&self) -> UiGraphFactIndexBasis {
        self.predecessor_graph
    }

    pub fn candidate_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.candidate_generation
    }

    pub const fn candidate_graph(&self) -> UiGraphFactIndexBasis {
        self.candidate_graph
    }

    pub fn has_distinct_candidate_generation(&self) -> bool {
        self.classification.predecessor_generation() != &self.candidate_generation
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn replace_session_for_certification(
        &mut self,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    ) {
        self.classification
            .replace_session_for_certification(session);
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn replace_predecessor_generation_for_certification(
        &mut self,
        generation: WorthUiPreparedApplicationGenerationIdentity,
    ) {
        self.classification
            .replace_predecessor_generation_for_certification(generation);
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn replace_candidate_generation_for_certification(
        &mut self,
        generation: WorthUiPreparedApplicationGenerationIdentity,
    ) {
        self.candidate_generation = generation;
    }
}
