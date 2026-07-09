use worth_query::facade::{
    WorthQueryOrdinaryOutcome, WorthQueryOrdinaryPosture, WorthQueryOrdinaryPostureKind,
    WorthQueryOrdinaryRuntimePosture, WorthQueryOrdinaryRuntimePostureKind,
    WorthQueryRuntimeAsyncResultState, WorthQueryRuntimeAsyncResultStateKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeOutcomeSourceReference {
    QueryOrdinaryOutcome {
        kind: String,
        posture_kind: Option<WorthQueryOrdinaryPostureKind>,
    },
    QueryOrdinaryPosture {
        kind: WorthQueryOrdinaryPostureKind,
    },
    QueryOrdinaryRuntimePosture {
        kind: WorthQueryOrdinaryRuntimePostureKind,
        posture_digest: String,
    },
    QueryRuntimeAsyncResultState {
        kind: WorthQueryRuntimeAsyncResultStateKind,
        result_state_digest: String,
    },
}

impl RuntimeOutcomeSourceReference {
    pub fn from_query_ordinary_outcome<T>(outcome: &WorthQueryOrdinaryOutcome<T>) -> Self {
        match outcome {
            WorthQueryOrdinaryOutcome::Bound(_) => Self::QueryOrdinaryOutcome {
                kind: "bound".to_string(),
                posture_kind: None,
            },
            WorthQueryOrdinaryOutcome::Ambiguous(posture) => ordinary_outcome("ambiguous", posture),
            WorthQueryOrdinaryOutcome::AspectConflict(posture) => {
                ordinary_outcome("aspect_conflict", posture)
            }
            WorthQueryOrdinaryOutcome::AuthorityMismatch(posture) => {
                ordinary_outcome("authority_mismatch", posture)
            }
            WorthQueryOrdinaryOutcome::BasisMismatch(posture) => {
                ordinary_outcome("basis_mismatch", posture)
            }
            WorthQueryOrdinaryOutcome::Deferred(posture) => ordinary_outcome("deferred", posture),
            WorthQueryOrdinaryOutcome::Denied(posture) => ordinary_outcome("denied", posture),
            WorthQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture) => {
                ordinary_outcome("explicit_narrowing_required", posture)
            }
            WorthQueryOrdinaryOutcome::Failed(posture) => ordinary_outcome("failed", posture),
            WorthQueryOrdinaryOutcome::MissingRequiredAspect(posture) => {
                ordinary_outcome("missing_required_aspect", posture)
            }
            WorthQueryOrdinaryOutcome::RebindRequired(posture) => {
                ordinary_outcome("rebind_required", posture)
            }
            WorthQueryOrdinaryOutcome::Refused(posture) => ordinary_outcome("refused", posture),
            WorthQueryOrdinaryOutcome::Stale(posture) => ordinary_outcome("stale", posture),
            WorthQueryOrdinaryOutcome::Unavailable(posture) => {
                ordinary_outcome("unavailable", posture)
            }
            WorthQueryOrdinaryOutcome::Unsupported(posture) => {
                ordinary_outcome("unsupported", posture)
            }
            WorthQueryOrdinaryOutcome::WrongHandle(posture) => {
                ordinary_outcome("wrong_handle", posture)
            }
            WorthQueryOrdinaryOutcome::WrongWorld(posture) => {
                ordinary_outcome("wrong_world", posture)
            }
        }
    }

    pub fn from_query_ordinary_posture(posture: &WorthQueryOrdinaryPosture) -> Self {
        Self::QueryOrdinaryPosture {
            kind: posture.kind(),
        }
    }

    pub fn from_query_ordinary_posture_kind(kind: WorthQueryOrdinaryPostureKind) -> Self {
        Self::QueryOrdinaryPosture { kind }
    }

    pub fn from_query_ordinary_runtime_posture(posture: &WorthQueryOrdinaryRuntimePosture) -> Self {
        Self::QueryOrdinaryRuntimePosture {
            kind: posture.kind(),
            posture_digest: posture.posture_digest().to_string(),
        }
    }

    pub fn from_query_ordinary_runtime_posture_kind(
        kind: WorthQueryOrdinaryRuntimePostureKind,
    ) -> Self {
        Self::QueryOrdinaryRuntimePosture {
            kind,
            posture_digest: format!("kind-only:{}", kind.as_str()),
        }
    }

    pub fn from_query_async_result_state(state: &WorthQueryRuntimeAsyncResultState) -> Self {
        Self::QueryRuntimeAsyncResultState {
            kind: state.kind(),
            result_state_digest: state.result_state_for_reporting().to_string(),
        }
    }

    pub fn from_query_async_result_state_kind(kind: WorthQueryRuntimeAsyncResultStateKind) -> Self {
        Self::QueryRuntimeAsyncResultState {
            kind,
            result_state_digest: format!("kind-only:{}", kind.as_str()),
        }
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::QueryOrdinaryOutcome { kind, posture_kind } => {
                format!(
                    "query_ordinary_outcome:{}:{}",
                    kind,
                    posture_kind
                        .map(|kind| format!("{kind:?}"))
                        .unwrap_or_else(|| "none".to_string())
                )
            }
            Self::QueryOrdinaryPosture { kind } => {
                format!("query_ordinary_posture:{kind:?}")
            }
            Self::QueryOrdinaryRuntimePosture {
                kind,
                posture_digest,
            } => {
                format!(
                    "query_ordinary_runtime_posture:{}:{}:{}",
                    kind.as_str(),
                    posture_digest.len(),
                    posture_digest
                )
            }
            Self::QueryRuntimeAsyncResultState {
                kind,
                result_state_digest,
            } => {
                format!(
                    "query_runtime_async_result_state:{}:{}:{}",
                    kind.as_str(),
                    result_state_digest.len(),
                    result_state_digest
                )
            }
        }
    }
}

fn ordinary_outcome(
    kind: &str,
    posture: &WorthQueryOrdinaryPosture,
) -> RuntimeOutcomeSourceReference {
    RuntimeOutcomeSourceReference::QueryOrdinaryOutcome {
        kind: kind.to_string(),
        posture_kind: Some(posture.kind()),
    }
}
