#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewLaneEvaluationClass {
    ReadOnly,
    PromotionEligible,
}

impl PreviewLaneEvaluationClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::PromotionEligible => "promotion_eligible",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewLaneLifecycleState {
    Active,
    Admitted,
    Declared,
    Promoted,
    Discarded,
}

impl PreviewLaneLifecycleState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Admitted => "Admitted",
            Self::Declared => "Declared",
            Self::Promoted => "Promoted",
            Self::Discarded => "Discarded",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewCertificationLane {
    pub query_digest: String,
    pub result_shape_digest: String,
    pub preview_session_identity: String,
    pub evaluation_class: PreviewLaneEvaluationClass,
    pub lifecycle_state_kind: PreviewLaneLifecycleState,
    pub binding_digest: String,
    pub preview_execution_digest: String,
    pub comparison_eligibility_digest: String,
    pub workflow_foundation_digest: String,
    pub promotion_parity_digest: Option<String>,
    pub preview_live_digest: Option<String>,
    pub preview_live_subscription_digest: Option<String>,
    pub preview_live_family: Option<String>,
    pub counters: crate::preview::PreviewBindingCounters,
    pub execution_counters: crate::preview::PreviewExecutionCounters,
    pub comparison_counters: Option<crate::preview::PreviewComparisonCounters>,
    pub preview_live_counters: Option<crate::preview::PreviewLiveCounters>,
}
