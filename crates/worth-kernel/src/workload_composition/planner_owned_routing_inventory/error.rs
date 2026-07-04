use std::error::Error;
use std::fmt::{Display, Formatter};

use super::classification::{PlannerOwnedRoutingDisplacedLane, PlannerOwnedRoutingDisposition};
use super::row::PlannerOwnedRoutingSurfaceIdentity;

#[derive(Debug)]
pub enum PlannerOwnedRoutingInventoryError {
    MissingRequiredSurface(PlannerOwnedRoutingSurfaceIdentity),
    DuplicateSurface(PlannerOwnedRoutingSurfaceIdentity),
    MissingLifecycleRole(&'static str),
    MissingReplacementLanePath(&'static str),
    MissingQueryGapKind(&'static str),
    UnexpectedQueryGapKind(&'static str),
    EmptyExitCondition(&'static str),
    MissingDisplacedLanePath(&'static str),
    MissingSourceToken {
        source_path: &'static str,
        token: &'static str,
    },
    MissingInventoryRowForCoveredSurface {
        lane: PlannerOwnedRoutingDisplacedLane,
        token: &'static str,
    },
    MissingCurrentAuthoritySource(&'static str),
    InvalidCurrentAuthoritySource {
        surface: &'static str,
        token: &'static str,
    },
    InvalidOrdinaryDisposition {
        surface: &'static str,
        disposition: PlannerOwnedRoutingDisposition,
    },
    SelfReplacingCapPath {
        surface: &'static str,
    },
    ExportParseFailure {
        source_path: &'static str,
        statement: String,
    },
    SourceReadFailure {
        source_path: &'static str,
        reason: String,
    },
}

impl Display for PlannerOwnedRoutingInventoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRequiredSurface(surface) => {
                write!(
                    f,
                    "missing required planner-owned routing surface: {surface:?}"
                )
            }
            Self::DuplicateSurface(surface) => {
                write!(f, "duplicate planner-owned routing surface: {surface:?}")
            }
            Self::MissingLifecycleRole(role) => {
                write!(
                    f,
                    "planner-owned routing inventory is missing lifecycle role `{role}`"
                )
            }
            Self::MissingReplacementLanePath(surface) => {
                write!(f, "surface `{surface}` is missing a replacement lane path")
            }
            Self::MissingQueryGapKind(surface) => {
                write!(
                    f,
                    "Query-gap surface `{surface}` is missing a Query-gap kind"
                )
            }
            Self::UnexpectedQueryGapKind(surface) => {
                write!(
                    f,
                    "non-Query-gap surface `{surface}` unexpectedly names a Query-gap kind"
                )
            }
            Self::EmptyExitCondition(surface) => {
                write!(
                    f,
                    "surface `{surface}` has an empty blocker or removal trigger"
                )
            }
            Self::MissingDisplacedLanePath(surface) => {
                write!(f, "surface `{surface}` is missing a displaced lane path")
            }
            Self::MissingSourceToken { source_path, token } => {
                write!(f, "source `{source_path}` does not contain token `{token}`")
            }
            Self::MissingInventoryRowForCoveredSurface { lane, token } => write!(
                f,
                "displaced lane {:?} exposes covered token `{}` without an inventory row",
                lane, token
            ),
            Self::MissingCurrentAuthoritySource(surface) => {
                write!(
                    f,
                    "surface `{surface}` is missing exact current authority sources"
                )
            }
            Self::InvalidCurrentAuthoritySource { surface, token } => write!(
                f,
                "surface `{surface}` names invalid current authority source token `{token}`"
            ),
            Self::InvalidOrdinaryDisposition {
                surface,
                disposition,
            } => write!(
                f,
                "ordinary surface `{surface}` has invalid disposition {:?}",
                disposition
            ),
            Self::SelfReplacingCapPath { surface } => write!(
                f,
                "capped residue surface `{surface}` reuses the replacement lane path instead of naming an exact surviving residue path"
            ),
            Self::ExportParseFailure {
                source_path,
                statement,
            } => write!(
                f,
                "failed to parse public export statement `{statement}` in `{source_path}`"
            ),
            Self::SourceReadFailure {
                source_path,
                reason,
            } => write!(f, "failed to read `{source_path}`: {reason}"),
        }
    }
}

impl Error for PlannerOwnedRoutingInventoryError {}
