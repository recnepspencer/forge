use crate::identity::data::VersionId;
use crate::storage::data::RecordLifecycleState;

pub(in super::super) fn lifecycle_storage_visible(lifecycle: RecordLifecycleState) -> bool {
    lifecycle != RecordLifecycleState::Reusable
}

pub(in super::super) fn historical_lifecycle(
    retired_at: Option<VersionId>,
    version_id: VersionId,
) -> RecordLifecycleState {
    if retired_at.is_some_and(|retired_at| retired_at <= version_id) {
        RecordLifecycleState::DeletedRetained
    } else {
        RecordLifecycleState::Live
    }
}
