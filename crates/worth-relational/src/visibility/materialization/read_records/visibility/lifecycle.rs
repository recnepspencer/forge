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

pub(in super::super) fn historical_retired_at(
    retired_at: Option<VersionId>,
    version_id: VersionId,
) -> Option<VersionId> {
    retired_at.filter(|retired_at| *retired_at <= version_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn historical_retirement_does_not_reveal_a_future_transition() {
        assert_eq!(
            historical_retired_at(Some(VersionId(9)), VersionId(8)),
            None
        );
        assert_eq!(
            historical_retired_at(Some(VersionId(9)), VersionId(9)),
            Some(VersionId(9))
        );
    }
}
