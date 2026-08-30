use crate::native::UiNativeHostState;

pub(super) fn presentation_epoch(
    state: &mut UiNativeHostState,
    key: u64,
    attempt: u64,
    painted: bool,
) -> Option<worth_ui_host_contract::UiHostPresentationEpoch> {
    if painted {
        let epoch = worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(attempt);
        state.presentation_epochs.insert(key, epoch);
        return Some(epoch);
    }
    state.presentation_epochs.get(&key).copied()
}
