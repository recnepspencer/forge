use super::UiNativeEventLoopRunDenial;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct UiNativeEventLoopThreadObservation {
    pub(super) thread: std::thread::ThreadId,
    pub(super) matches_launch: bool,
}

pub(super) fn transition(
    slot: &mut Option<UiNativeEventLoopThreadObservation>,
    run_thread: std::thread::ThreadId,
    callback_thread: std::thread::ThreadId,
) -> Result<UiNativeEventLoopThreadObservation, UiNativeEventLoopRunDenial> {
    let observation = UiNativeEventLoopThreadObservation {
        thread: callback_thread,
        matches_launch: run_thread == callback_thread,
    };
    *slot = Some(observation);
    observation
        .matches_launch
        .then_some(observation)
        .ok_or(UiNativeEventLoopRunDenial::ApplicationDriver)
}
