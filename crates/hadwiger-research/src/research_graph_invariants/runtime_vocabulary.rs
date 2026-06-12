#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ResearchGraphRuntimeKind {
    id: u32,
    label: &'static str,
}

impl ResearchGraphRuntimeKind {
    pub const fn new(id: u32, label: &'static str) -> Self {
        Self { id, label }
    }

    pub const fn id(self) -> u32 {
        self.id
    }

    pub const fn label(self) -> &'static str {
        self.label
    }
}

pub const FAILURE: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_101, "hadwiger.research_graph.failure");
pub const NEGATIVE_EVIDENCE: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_102, "hadwiger.research_graph.negative_evidence");
pub const AFFECTED_ARTIFACT: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_103, "hadwiger.research_graph.affected_artifact");
pub const FAILURE_SCOPE: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_104, "hadwiger.research_graph.failure_scope");
pub const REACTIVATION_HINT: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_105, "hadwiger.research_graph.reactivation_hint");
pub const EXPERIMENT_PLAN: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_106, "hadwiger.research_graph.experiment_plan");
pub const SUPPRESSION_PROOF: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_107, "hadwiger.research_graph.suppression_proof");
pub const REACTIVATION_CONDITION: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_108, "hadwiger.research_graph.reactivation_condition");
pub const HYPOTHESIS: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_109, "hadwiger.research_graph.hypothesis");
pub const FRONTIER_STATE: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_110, "hadwiger.research_graph.frontier_state");
pub const QUERY_READINESS_COUNTER: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_111, "hadwiger.research_graph.query_readiness_counter");

pub const FAILURE_HAS_NEGATIVE_EVIDENCE: ResearchGraphRuntimeKind = ResearchGraphRuntimeKind::new(
    8_201,
    "hadwiger.research_graph.failure.has_negative_evidence",
);
pub const FAILURE_AFFECTS_ARTIFACT: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_202, "hadwiger.research_graph.failure.affects_artifact");
pub const FAILURE_HAS_SCOPE: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_203, "hadwiger.research_graph.failure.has_scope");
pub const FAILURE_HAS_REACTIVATION_HINT: ResearchGraphRuntimeKind = ResearchGraphRuntimeKind::new(
    8_204,
    "hadwiger.research_graph.failure.has_reactivation_hint",
);
pub const PLAN_HAS_SUPPRESSION_PROOF: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_205, "hadwiger.research_graph.plan.has_suppression_proof");
pub const PLAN_HAS_REACTIVATION_CONDITION: ResearchGraphRuntimeKind = ResearchGraphRuntimeKind::new(
    8_206,
    "hadwiger.research_graph.plan.has_reactivation_condition",
);
pub const HYPOTHESIS_HAS_STATUS: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(8_207, "hadwiger.research_graph.hypothesis.has_status");
pub const FRONTIER_HAS_AUTHORITY_POSTURE: ResearchGraphRuntimeKind = ResearchGraphRuntimeKind::new(
    8_208,
    "hadwiger.research_graph.frontier.has_authority_posture",
);
pub const PLAN_HAS_QUERY_READINESS_COUNTER: ResearchGraphRuntimeKind =
    ResearchGraphRuntimeKind::new(
        8_209,
        "hadwiger.research_graph.plan.has_query_readiness_counter",
    );

pub const ENTITY_KINDS: [ResearchGraphRuntimeKind; 11] = [
    FAILURE,
    NEGATIVE_EVIDENCE,
    AFFECTED_ARTIFACT,
    FAILURE_SCOPE,
    REACTIVATION_HINT,
    EXPERIMENT_PLAN,
    SUPPRESSION_PROOF,
    REACTIVATION_CONDITION,
    HYPOTHESIS,
    FRONTIER_STATE,
    QUERY_READINESS_COUNTER,
];

pub const RELATION_KINDS: [ResearchGraphRuntimeKind; 9] = [
    FAILURE_HAS_NEGATIVE_EVIDENCE,
    FAILURE_AFFECTS_ARTIFACT,
    FAILURE_HAS_SCOPE,
    FAILURE_HAS_REACTIVATION_HINT,
    PLAN_HAS_SUPPRESSION_PROOF,
    PLAN_HAS_REACTIVATION_CONDITION,
    HYPOTHESIS_HAS_STATUS,
    FRONTIER_HAS_AUTHORITY_POSTURE,
    PLAN_HAS_QUERY_READINESS_COUNTER,
];
