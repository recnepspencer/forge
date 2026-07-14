use super::super::{
    WorthQuerySessionLabel, WorthQuerySessionLabelError, WorthQuerySessionLabelSegment,
    WorthQuerySessionNamespace,
};

#[test]
fn rejects_empty_namespace_segments_and_name_lists() {
    assert_eq!(
        WorthQuerySessionNamespace::new("   "),
        Err(WorthQuerySessionLabelError::EmptyNamespace)
    );
    assert_eq!(
        WorthQuerySessionLabelSegment::new(" "),
        Err(WorthQuerySessionLabelError::EmptyNameSegment)
    );
    assert_eq!(
        WorthQuerySessionLabel::scoped(
            WorthQuerySessionNamespace::new("worth-kernel").expect("namespace should build"),
            std::iter::empty::<WorthQuerySessionLabelSegment>(),
        ),
        Err(WorthQuerySessionLabelError::MissingNameSegments)
    );
    assert_eq!(
        WorthQuerySessionLabel::scoped_strs("worth-kernel", ["preview", " "]),
        Err(WorthQuerySessionLabelError::EmptyNameSegment)
    );
}
