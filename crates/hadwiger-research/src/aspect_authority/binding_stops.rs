use worth_query::facade::{
    WORTHQueryBindingAspectConflict, WORTHQueryBindingMissingRequiredAspect,
    WORTHQueryBindingRebindRequired, WORTHQueryBindingStale,
};

use super::aspect_kinds::HadwigerAspectKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AspectClosureStop {
    QueryStale {
        aspect_kind: HadwigerAspectKind,
        stop: WORTHQueryBindingStale,
    },
    QueryRebindRequired {
        aspect_kind: HadwigerAspectKind,
        stop: WORTHQueryBindingRebindRequired,
    },
    QueryMissingRequiredAspect {
        aspect_kind: HadwigerAspectKind,
        stop: WORTHQueryBindingMissingRequiredAspect,
    },
    QueryAspectConflict {
        aspect_kind: HadwigerAspectKind,
        stop: WORTHQueryBindingAspectConflict,
    },
    LocalClosureStop {
        aspect_kind: HadwigerAspectKind,
        reason: String,
    },
}

impl AspectClosureStop {
    pub fn query_stale(aspect_kind: HadwigerAspectKind, stop: WORTHQueryBindingStale) -> Self {
        Self::QueryStale { aspect_kind, stop }
    }

    pub fn query_rebind_required(
        aspect_kind: HadwigerAspectKind,
        stop: WORTHQueryBindingRebindRequired,
    ) -> Self {
        Self::QueryRebindRequired { aspect_kind, stop }
    }

    pub fn query_missing_required_aspect(
        aspect_kind: HadwigerAspectKind,
        stop: WORTHQueryBindingMissingRequiredAspect,
    ) -> Self {
        Self::QueryMissingRequiredAspect { aspect_kind, stop }
    }

    pub fn query_aspect_conflict(
        aspect_kind: HadwigerAspectKind,
        stop: WORTHQueryBindingAspectConflict,
    ) -> Self {
        Self::QueryAspectConflict { aspect_kind, stop }
    }

    pub fn local(aspect_kind: HadwigerAspectKind, reason: impl Into<String>) -> Self {
        Self::LocalClosureStop {
            aspect_kind,
            reason: reason.into(),
        }
    }

    pub fn is_query_owned(&self) -> bool {
        !matches!(self, Self::LocalClosureStop { .. })
    }

    pub fn aspect_kind(&self) -> HadwigerAspectKind {
        match self {
            Self::QueryStale { aspect_kind, .. }
            | Self::QueryRebindRequired { aspect_kind, .. }
            | Self::QueryMissingRequiredAspect { aspect_kind, .. }
            | Self::QueryAspectConflict { aspect_kind, .. }
            | Self::LocalClosureStop { aspect_kind, .. } => *aspect_kind,
        }
    }

    pub fn reason(&self) -> &str {
        match self {
            Self::QueryStale { stop, .. } => stop.reason(),
            Self::QueryRebindRequired { stop, .. } => stop.reason(),
            Self::QueryMissingRequiredAspect { stop, .. } => stop.reason(),
            Self::QueryAspectConflict { stop, .. } => stop.reason(),
            Self::LocalClosureStop { reason, .. } => reason,
        }
    }
}
