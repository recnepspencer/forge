use crate::runtime::ForgeQueryRuntimeFacadeFamily;

use super::super::error::{ForgeQuerySupportPinningError, ForgeQuerySupportPinningErrorKind};
use super::super::status::{ForgeQueryPinnedSupportStatus, ForgeQueryPinnedTeachingPosture};

pub(crate) fn admit_pinned_facade_family(
    family: &str,
) -> Result<ForgeQueryRuntimeFacadeFamily, ForgeQuerySupportPinningError> {
    match family {
        "read" => Ok(ForgeQueryRuntimeFacadeFamily::Read),
        "live" => Ok(ForgeQueryRuntimeFacadeFamily::Live),
        "computed" => Ok(ForgeQueryRuntimeFacadeFamily::Computed),
        "shared-read" => Ok(ForgeQueryRuntimeFacadeFamily::SharedRead),
        "submission" => Ok(ForgeQueryRuntimeFacadeFamily::Submission),
        "replay" => Ok(ForgeQueryRuntimeFacadeFamily::Replay),
        "effect" => Ok(ForgeQueryRuntimeFacadeFamily::Effect),
        "branch-preview" => Ok(ForgeQueryRuntimeFacadeFamily::BranchPreview),
        "write" => Ok(ForgeQueryRuntimeFacadeFamily::Write),
        "intent" => Ok(ForgeQueryRuntimeFacadeFamily::Intent),
        "inspect" => Ok(ForgeQueryRuntimeFacadeFamily::Inspect),
        "temporal" => Ok(ForgeQueryRuntimeFacadeFamily::Temporal),
        "async-resource" => Ok(ForgeQueryRuntimeFacadeFamily::AsyncResource),
        "mixed-cause-delivery" => Ok(ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery),
        "store-backed-execution" => Ok(ForgeQueryRuntimeFacadeFamily::StoreBackedExecution),
        "durable-artifacts" => Ok(ForgeQueryRuntimeFacadeFamily::DurableArtifacts),
        found => Err(ForgeQuerySupportPinningError::with_family(
            ForgeQuerySupportPinningErrorKind::InvalidFacadeFamily,
            "support pin facade family is not part of the pinned vocabulary",
            found,
        )),
    }
}

pub(crate) fn admit_pinned_status(
    status: &str,
) -> Result<ForgeQueryPinnedSupportStatus, ForgeQuerySupportPinningError> {
    match status {
        "supported" => Ok(ForgeQueryPinnedSupportStatus::Supported),
        "deferred-debt" => Ok(ForgeQueryPinnedSupportStatus::DeferredDebt),
        "unsupported" => Ok(ForgeQueryPinnedSupportStatus::Unsupported),
        found => Err(ForgeQuerySupportPinningError::with_found(
            ForgeQuerySupportPinningErrorKind::InvalidPinnedStatus,
            "support pin status is not part of the pinned vocabulary",
            found,
        )),
    }
}

pub(crate) fn admit_pinned_teaching_posture(
    posture: &str,
) -> Result<ForgeQueryPinnedTeachingPosture, ForgeQuerySupportPinningError> {
    match posture {
        "ordinary-runtime-dx" => Ok(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx),
        "visible-but-deferred" => Ok(ForgeQueryPinnedTeachingPosture::VisibleButDeferred),
        "visible-vocabulary-only" => Ok(ForgeQueryPinnedTeachingPosture::VisibleVocabularyOnly),
        "support-gate-only" => Ok(ForgeQueryPinnedTeachingPosture::SupportGateOnly),
        found => Err(ForgeQuerySupportPinningError::with_found(
            ForgeQuerySupportPinningErrorKind::InvalidPinnedTeachingPosture,
            "support pin teaching posture is not part of the pinned vocabulary",
            found,
        )),
    }
}
