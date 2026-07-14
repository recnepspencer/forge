#[test]
fn root_postures_are_mechanically_distinct() {
    assert_ne!(
        super::UiReplanRootPosture::RootPrimary,
        super::UiReplanRootPosture::CountedRootWiden {
            reason: super::UiReplanWidenReason::SharedAncestorRequirement,
        },
    );
    assert_ne!(
        super::UiReplanRootPosture::NotRoot,
        super::UiReplanRootPosture::RootPrimary,
    );
}
