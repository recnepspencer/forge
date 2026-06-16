use forge_query::facade::{
    ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind,
    ForgeQueryOrdinaryRuntimePosture, ForgeQueryOrdinaryRuntimePostureKind,
    ForgeQueryRuntimeAsyncResultState, ForgeQueryRuntimeAsyncResultStateKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeOutcomeSourceReference {
    QueryOrdinaryOutcome {
        kind: String,
        posture_kind: Option<ForgeQueryOrdinaryPostureKind>,
    },
    QueryOrdinaryPosture {
        kind: ForgeQueryOrdinaryPostureKind,
    },
    QueryOrdinaryRuntimePosture {
        kind: ForgeQueryOrdinaryRuntimePostureKind,
        posture_digest: String,
    },
    QueryRuntimeAsyncResultState {
        kind: ForgeQueryRuntimeAsyncResultStateKind,
        result_state_digest: String,
    },
}

impl RuntimeOutcomeSourceReference {
    pub fn from_query_ordinary_outcome<T>(outcome: &ForgeQueryOrdinaryOutcome<T>) -> Self {
        match outcome {
            ForgeQueryOrdinaryOutcome::Bound(_) => Self::QueryOrdinaryOutcome {
                kind: "bound".to_string(),
                posture_kind: None,
            },
            ForgeQueryOrdinaryOutcome::Ambiguous(posture) => ordinary_outcome("ambiguous", posture),
            ForgeQueryOrdinaryOutcome::AspectConflict(posture) => {
                ordinary_outcome("aspect_conflict", posture)
            }
            ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture) => {
                ordinary_outcome("authority_mismatch", posture)
            }
            ForgeQueryOrdinaryOutcome::BasisMismatch(posture) => {
                ordinary_outcome("basis_mismatch", posture)
            }
            ForgeQueryOrdinaryOutcome::Deferred(posture) => ordinary_outcome("deferred", posture),
            ForgeQueryOrdinaryOutcome::Denied(posture) => ordinary_outcome("denied", posture),
            ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture) => {
                ordinary_outcome("explicit_narrowing_required", posture)
            }
            ForgeQueryOrdinaryOutcome::Failed(posture) => ordinary_outcome("failed", posture),
            ForgeQueryOrdinaryOutcome::MissingRequiredAspect(posture) => {
                ordinary_outcome("missing_required_aspect", posture)
            }
            ForgeQueryOrdinaryOutcome::RebindRequired(posture) => {
                ordinary_outcome("rebind_required", posture)
            }
            ForgeQueryOrdinaryOutcome::Refused(posture) => ordinary_outcome("refused", posture),
            ForgeQueryOrdinaryOutcome::Stale(posture) => ordinary_outcome("stale", posture),
            ForgeQueryOrdinaryOutcome::Unavailable(posture) => {
                ordinary_outcome("unavailable", posture)
            }
            ForgeQueryOrdinaryOutcome::Unsupported(posture) => {
                ordinary_outcome("unsupported", posture)
            }
            ForgeQueryOrdinaryOutcome::WrongHandle(posture) => {
                ordinary_outcome("wrong_handle", posture)
            }
            ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
                ordinary_outcome("wrong_world", posture)
            }
        }
    }

    pub fn from_query_ordinary_posture(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::QueryOrdinaryPosture {
            kind: posture.kind(),
        }
    }

    pub fn from_query_ordinary_posture_kind(kind: ForgeQueryOrdinaryPostureKind) -> Self {
        Self::QueryOrdinaryPosture { kind }
    }

    pub fn from_query_ordinary_runtime_posture(posture: &ForgeQueryOrdinaryRuntimePosture) -> Self {
        Self::QueryOrdinaryRuntimePosture {
            kind: posture.kind(),
            posture_digest: posture.posture_digest().to_string(),
        }
    }

    pub fn from_query_ordinary_runtime_posture_kind(
        kind: ForgeQueryOrdinaryRuntimePostureKind,
    ) -> Self {
        Self::QueryOrdinaryRuntimePosture {
            kind,
            posture_digest: format!("kind-only:{}", kind.as_str()),
        }
    }

    pub fn from_query_async_result_state(state: &ForgeQueryRuntimeAsyncResultState) -> Self {
        Self::QueryRuntimeAsyncResultState {
            kind: state.kind(),
            result_state_digest: state.result_state_for_reporting().to_string(),
        }
    }

    pub fn from_query_async_result_state_kind(kind: ForgeQueryRuntimeAsyncResultStateKind) -> Self {
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
    posture: &ForgeQueryOrdinaryPosture,
) -> RuntimeOutcomeSourceReference {
    RuntimeOutcomeSourceReference::QueryOrdinaryOutcome {
        kind: kind.to_string(),
        posture_kind: Some(posture.kind()),
    }
}
