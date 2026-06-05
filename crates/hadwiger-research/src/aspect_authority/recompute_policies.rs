#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerRecomputePolicy {
    RecomputeExactAspect,
    RecomputeDependencyClosure,
    ConservativeEscalationRequired,
    UnsupportedUntilCheckerAdmission,
}

impl HadwigerRecomputePolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RecomputeExactAspect => "recompute_exact_aspect",
            Self::RecomputeDependencyClosure => "recompute_dependency_closure",
            Self::ConservativeEscalationRequired => "conservative_escalation_required",
            Self::UnsupportedUntilCheckerAdmission => "unsupported_until_checker_admission",
        }
    }
}
