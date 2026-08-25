use super::*;

#[test]
fn program_admission_is_bounded_and_component_semantic() {
    assert!(matches!(
        UiNativeApplicationProgram::new([]),
        Err(UiNativeApplicationProgramDenial::Empty)
    ));
    assert!(UiNativeComponentPresenceChange::new("row", true).is_err());
    let change = UiNativeComponentPresenceChange::new("component:app.row", false).unwrap();
    let frame = UiNativeApplicationFrame::with_component_presence([change]).unwrap();
    assert_eq!(
        UiNativeApplicationProgram::new([frame])
            .unwrap()
            .frames()
            .len(),
        1
    );
    assert!(UiNativeApplicationProgram::single_frame().closes_after_program());
    assert!(!UiNativeApplicationProgram::single_frame()
        .remain_open_until_external_close()
        .closes_after_program());

    let combined = UiNativeApplicationFrame::with_component_presence_and_semantic_text(
        [UiNativeComponentPresenceChange::new("component:app.row", true).unwrap()],
        [UiNativeComponentSemanticTextChange::new("component:app.row", "current").unwrap()],
    )
    .unwrap();
    assert_eq!(combined.component_presence().len(), 1);
    assert_eq!(combined.semantic_text().len(), 1);
}

#[test]
fn presented_source_capture_is_bounded_to_one_program_frame() {
    let captured = UiNativeApplicationFrame::present_current().capture_presented_source_pixels();
    assert!(UiNativeApplicationProgram::new([captured]).is_ok());

    let first = UiNativeApplicationFrame::present_current().capture_presented_source_pixels();
    let second = UiNativeApplicationFrame::present_current().capture_presented_source_pixels();
    assert!(matches!(
        UiNativeApplicationProgram::new([first, second]),
        Err(UiNativeApplicationProgramDenial::PresentedSourceCaptureCapacityExceeded)
    ));
}
