use super::super::{callback_thread, UiNativeEventLoopRunDenial};

#[test]
fn callback_thread_transition_rejects_off_thread_run() {
    let run_owner = include_str!("../run.rs");
    let posture_owner = include_str!("../thread_posture.rs");
    assert!(!run_owner.contains("with_any_thread("));
    assert!(!run_owner.contains("builder.with_any_thread(true);"));
    assert!(posture_owner.contains("builder.with_any_thread(false);"));
    assert!(posture_owner.contains("builder.with_any_thread(true);"));
    let run_thread = std::thread::current().id();
    let mut observation = None;
    let lawful = callback_thread::transition(&mut observation, run_thread, run_thread).unwrap();
    assert_eq!(lawful.thread, run_thread);
    assert!(lawful.matches_launch);
    assert!(observation.is_some_and(|observed| observed.matches_launch));
    let other = std::thread::spawn(|| std::thread::current().id())
        .join()
        .unwrap();
    assert_eq!(
        callback_thread::transition(&mut observation, run_thread, other),
        Err(UiNativeEventLoopRunDenial::ApplicationDriver)
    );
    let hostile = observation.expect("hostile callback remains observed");
    assert_eq!(hostile.thread, other);
    assert!(!hostile.matches_launch);
}
