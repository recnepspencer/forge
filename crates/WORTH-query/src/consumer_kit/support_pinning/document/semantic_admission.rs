use crate::runtime::WorthQueryRuntimeFacadeFamily;

use super::super::error::{WorthQuerySupportPinningError, WorthQuerySupportPinningErrorKind};
use super::super::status::{WorthQueryPinnedSupportStatus, WorthQueryPinnedTeachingPosture};

pub(crate) fn admit_pinned_facade_family(
    family: &str,
) -> Result<WorthQueryRuntimeFacadeFamily, WorthQuerySupportPinningError> {
    match family {
        "read" => Ok(WorthQueryRuntimeFacadeFamily::Read),
        "live" => Ok(WorthQueryRuntimeFacadeFamily::Live),
        "computed" => Ok(WorthQueryRuntimeFacadeFamily::Computed),
        "shared-read" => Ok(WorthQueryRuntimeFacadeFamily::SharedRead),
        "submission" => Ok(WorthQueryRuntimeFacadeFamily::Submission),
        "replay" => Ok(WorthQueryRuntimeFacadeFamily::Replay),
        "effect" => Ok(WorthQueryRuntimeFacadeFamily::Effect),
        "branch-preview" => Ok(WorthQueryRuntimeFacadeFamily::BranchPreview),
        "write" => Ok(WorthQueryRuntimeFacadeFamily::Write),
        "intent" => Ok(WorthQueryRuntimeFacadeFamily::Intent),
        "inspect" => Ok(WorthQueryRuntimeFacadeFamily::Inspect),
        "temporal" => Ok(WorthQueryRuntimeFacadeFamily::Temporal),
        "async-resource" => Ok(WorthQueryRuntimeFacadeFamily::AsyncResource),
        "mixed-cause-delivery" => Ok(WorthQueryRuntimeFacadeFamily::MixedCauseDelivery),
        "store-backed-execution" => Ok(WorthQueryRuntimeFacadeFamily::StoreBackedExecution),
        "durable-artifacts" => Ok(WorthQueryRuntimeFacadeFamily::DurableArtifacts),
        found => Err(WorthQuerySupportPinningError::with_family(
            WorthQuerySupportPinningErrorKind::InvalidFacadeFamily,
            "support pin facade family is not part of the pinned vocabulary",
            found,
        )),
    }
}

pub(crate) fn admit_pinned_status(
    status: &str,
) -> Result<WorthQueryPinnedSupportStatus, WorthQuerySupportPinningError> {
    match status {
        "supported" => Ok(WorthQueryPinnedSupportStatus::Supported),
        "deferred-debt" => Ok(WorthQueryPinnedSupportStatus::DeferredDebt),
        "unsupported" => Ok(WorthQueryPinnedSupportStatus::Unsupported),
        found => Err(WorthQuerySupportPinningError::with_found(
            WorthQuerySupportPinningErrorKind::InvalidPinnedStatus,
            "support pin status is not part of the pinned vocabulary",
            found,
        )),
    }
}

pub(crate) fn admit_pinned_teaching_posture(
    posture: &str,
) -> Result<WorthQueryPinnedTeachingPosture, WorthQuerySupportPinningError> {
    match posture {
        "ordinary-runtime-dx" => Ok(WorthQueryPinnedTeachingPosture::OrdinaryRuntimeDx),
        "visible-but-deferred" => Ok(WorthQueryPinnedTeachingPosture::VisibleButDeferred),
        "visible-vocabulary-only" => Ok(WorthQueryPinnedTeachingPosture::VisibleVocabularyOnly),
        "support-gate-only" => Ok(WorthQueryPinnedTeachingPosture::SupportGateOnly),
        found => Err(WorthQuerySupportPinningError::with_found(
            WorthQuerySupportPinningErrorKind::InvalidPinnedTeachingPosture,
            "support pin teaching posture is not part of the pinned vocabulary",
            found,
        )),
    }
}
