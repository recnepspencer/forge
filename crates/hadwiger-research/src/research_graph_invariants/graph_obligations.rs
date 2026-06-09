use super::catalog::ResearchGraphInvariantFamily;
use super::runtime_vocabulary as vocab;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ResearchGraphInvariantObligation {
    family: ResearchGraphInvariantFamily,
    required_by_kind: &'static str,
    satisfied_by_relation_kinds: Vec<&'static str>,
}

impl ResearchGraphInvariantObligation {
    pub(crate) fn new(
        family: ResearchGraphInvariantFamily,
        required_by_kind: &'static str,
        satisfied_by_relation_kinds: Vec<&'static str>,
    ) -> Self {
        Self {
            family,
            required_by_kind,
            satisfied_by_relation_kinds,
        }
    }

    pub fn family(&self) -> ResearchGraphInvariantFamily {
        self.family
    }

    pub fn required_by_kind(&self) -> &'static str {
        self.required_by_kind
    }

    pub fn satisfied_by_relation_kinds(&self) -> &[&'static str] {
        &self.satisfied_by_relation_kinds
    }

    pub(crate) fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}",
            self.family.as_str(),
            self.required_by_kind,
            self.satisfied_by_relation_kinds.join(",")
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchGraphInvariantObligationSet {
    obligations: Vec<ResearchGraphInvariantObligation>,
}

impl ResearchGraphInvariantObligationSet {
    pub(crate) fn new(mut obligations: Vec<ResearchGraphInvariantObligation>) -> Self {
        obligations.sort();
        obligations.dedup();
        Self { obligations }
    }

    pub fn rows(&self) -> &[ResearchGraphInvariantObligation] {
        &self.obligations
    }

    pub fn contains_family(&self, family: ResearchGraphInvariantFamily) -> bool {
        self.obligations
            .iter()
            .any(|obligation| obligation.family() == family)
    }

    pub fn len(&self) -> usize {
        self.obligations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.obligations.is_empty()
    }

    pub(crate) fn stable_token(&self) -> String {
        self.obligations
            .iter()
            .map(ResearchGraphInvariantObligation::stable_token)
            .collect::<Vec<_>>()
            .join("|")
    }
}

pub(crate) fn obligation_for_entity_kind(
    kind_label: &str,
) -> Option<ResearchGraphInvariantObligation> {
    match kind_label {
        label if label == vocab::FAILURE.label() => Some(ResearchGraphInvariantObligation::new(
            ResearchGraphInvariantFamily::FailureResidency,
            vocab::FAILURE.label(),
            vec![
                vocab::FAILURE_HAS_NEGATIVE_EVIDENCE.label(),
                vocab::FAILURE_AFFECTS_ARTIFACT.label(),
                vocab::FAILURE_HAS_SCOPE.label(),
                vocab::FAILURE_HAS_REACTIVATION_HINT.label(),
            ],
        )),
        label if label == vocab::HYPOTHESIS.label() => Some(ResearchGraphInvariantObligation::new(
            ResearchGraphInvariantFamily::HypothesisLifecycle,
            vocab::HYPOTHESIS.label(),
            vec![vocab::HYPOTHESIS_HAS_STATUS.label()],
        )),
        label if label == vocab::FRONTIER_STATE.label() => {
            Some(ResearchGraphInvariantObligation::new(
                ResearchGraphInvariantFamily::BranchPromotion,
                vocab::FRONTIER_STATE.label(),
                vec![vocab::FRONTIER_HAS_AUTHORITY_POSTURE.label()],
            ))
        }
        _ => None,
    }
}

pub(crate) fn suppression_obligation() -> ResearchGraphInvariantObligation {
    ResearchGraphInvariantObligation::new(
        ResearchGraphInvariantFamily::SuppressionRelation,
        vocab::EXPERIMENT_PLAN.label(),
        vec![
            vocab::PLAN_HAS_SUPPRESSION_PROOF.label(),
            vocab::PLAN_HAS_REACTIVATION_CONDITION.label(),
        ],
    )
}

pub(crate) fn executable_experiment_obligation() -> ResearchGraphInvariantObligation {
    ResearchGraphInvariantObligation::new(
        ResearchGraphInvariantFamily::ExecutableExperimentAdmission,
        vocab::EXPERIMENT_PLAN.label(),
        vec![vocab::PLAN_HAS_QUERY_READINESS_COUNTER.label()],
    )
}
