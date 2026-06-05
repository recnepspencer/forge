use super::{
    candidate_evaluation::ReplacementCandidateEvaluation, continuity::BindingContinuityClass,
    motion_posture::MotionAwareBindingPosture, neighborhood::NeighborhoodBindingFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RebindingExplanation {
    neighborhood_family: NeighborhoodBindingFamily,
    continuity_class: BindingContinuityClass,
    motion_posture: MotionAwareBindingPosture,
    prior_identity: String,
    prior_site_identity: String,
    candidate_labels: Vec<String>,
    candidate_identities: Vec<String>,
    candidate_site_identities: Vec<String>,
    selected_candidate_identity: Option<String>,
    selected_candidate_label: Option<String>,
}

impl RebindingExplanation {
    pub(crate) fn from_evaluation(
        evaluation: &ReplacementCandidateEvaluation,
        motion_posture: MotionAwareBindingPosture,
    ) -> Self {
        let continuity_class = evaluation.continuity().continuity_class();
        let selected_candidate_identity = evaluation
            .continuity()
            .candidate_identity()
            .filter(|_| {
                matches!(
                    continuity_class,
                    BindingContinuityClass::Exact
                        | BindingContinuityClass::AuthoritativeSuccessor
                        | BindingContinuityClass::CorrespondenceOnly
                )
            })
            .map(ToOwned::to_owned);
        let selected_candidate_label = evaluation
            .continuity()
            .candidate_label()
            .filter(|_| selected_candidate_identity.is_some())
            .map(ToOwned::to_owned);
        Self {
            neighborhood_family: evaluation.neighborhood().family(),
            continuity_class,
            motion_posture,
            prior_identity: evaluation.prior_binding().identity().as_str().to_string(),
            prior_site_identity: evaluation.neighborhood().prior_site_identity().to_string(),
            candidate_labels: evaluation
                .neighborhood()
                .candidates()
                .iter()
                .map(|candidate| candidate.label().to_string())
                .collect(),
            candidate_identities: evaluation
                .neighborhood()
                .candidates()
                .iter()
                .map(|candidate| candidate.binding().identity().as_str().to_string())
                .collect(),
            candidate_site_identities: evaluation
                .neighborhood()
                .candidates()
                .iter()
                .map(|candidate| candidate.site_identity().to_string())
                .collect(),
            selected_candidate_identity,
            selected_candidate_label,
        }
    }

    pub fn neighborhood_family(&self) -> NeighborhoodBindingFamily {
        self.neighborhood_family
    }

    pub fn continuity_class(&self) -> BindingContinuityClass {
        self.continuity_class
    }

    pub fn motion_posture(&self) -> &MotionAwareBindingPosture {
        &self.motion_posture
    }

    pub fn prior_identity(&self) -> &str {
        &self.prior_identity
    }

    pub fn prior_site_identity(&self) -> &str {
        &self.prior_site_identity
    }

    pub fn candidate_identities(&self) -> &[String] {
        &self.candidate_identities
    }

    pub fn candidate_labels(&self) -> &[String] {
        &self.candidate_labels
    }

    pub fn candidate_site_identities(&self) -> &[String] {
        &self.candidate_site_identities
    }

    pub fn selected_candidate_identity(&self) -> Option<&str> {
        self.selected_candidate_identity.as_deref()
    }

    pub fn selected_candidate_label(&self) -> Option<&str> {
        self.selected_candidate_label.as_deref()
    }
}
