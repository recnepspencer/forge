use crate::runtime::rebind::{decision_from_transition, UiIdentityLifecycleDecision};
pub use crate::runtime::WorthUiNodeLifecycleTransition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIdentityLifecyclePresence {
    Both,
    CandidateOnly,
    PredecessorOnly,
    Neither,
}

pub enum UiRebindPlanningBasisMutation {
    Session(crate::facade::WorthUiActiveApplicationSessionIdentity),
    PredecessorGeneration(
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    ),
    CandidateGeneration(
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    ),
}

pub fn identity_lifecycle_decision_for_certification(
    transition: WorthUiNodeLifecycleTransition,
    kind: crate::graph::UiGraphFactConsumerKind,
    presence: UiIdentityLifecyclePresence,
) -> Option<UiIdentityLifecycleDecision> {
    let (has_predecessor, has_candidate) = match presence {
        UiIdentityLifecyclePresence::Both => (true, true),
        UiIdentityLifecyclePresence::CandidateOnly => (false, true),
        UiIdentityLifecyclePresence::PredecessorOnly => (true, false),
        UiIdentityLifecyclePresence::Neither => (false, false),
    };
    decision_from_transition(kind, transition, has_predecessor, has_candidate)
}

pub trait UiResolvedIdentityLifecycleCertificationExt {
    fn known_consumer_keys_for_certification(&self) -> Box<[crate::graph::UiGraphFactConsumerKey]>;

    fn with_planning_basis_mutation_for_certification(
        self,
        mutation: UiRebindPlanningBasisMutation,
    ) -> Self;
}

impl UiResolvedIdentityLifecycleCertificationExt
    for crate::runtime::rebind::UiResolvedIdentityLifecycle
{
    fn known_consumer_keys_for_certification(&self) -> Box<[crate::graph::UiGraphFactConsumerKey]> {
        self.scope()
            .source_succession()
            .and_then(|succession| succession.identity_lifecycle_index())
            .map(|index| index.keys().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
            .into_boxed_slice()
    }

    fn with_planning_basis_mutation_for_certification(
        mut self,
        mutation: UiRebindPlanningBasisMutation,
    ) -> Self {
        match mutation {
            UiRebindPlanningBasisMutation::Session(session) => {
                self.replace_planning_session_for_certification(session)
            }
            UiRebindPlanningBasisMutation::PredecessorGeneration(generation) => {
                self.replace_planning_predecessor_for_certification(generation)
            }
            UiRebindPlanningBasisMutation::CandidateGeneration(generation) => {
                self.replace_planning_candidate_for_certification(generation)
            }
        }
        self
    }
}
