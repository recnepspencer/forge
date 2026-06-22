use super::super::{
    ForgeQuerySessionLabel, ForgeQuerySessionLabelError, ForgeQuerySessionLabelSegment,
    ForgeQuerySessionNamespace,
};

#[test]
fn rejects_empty_namespace_segments_and_name_lists() {
    assert_eq!(
        ForgeQuerySessionNamespace::new("   "),
        Err(ForgeQuerySessionLabelError::EmptyNamespace)
    );
    assert_eq!(
        ForgeQuerySessionLabelSegment::new(" "),
        Err(ForgeQuerySessionLabelError::EmptyNameSegment)
    );
    assert_eq!(
        ForgeQuerySessionLabel::scoped(
            ForgeQuerySessionNamespace::new("worth-kernel").expect("namespace should build"),
            std::iter::empty::<ForgeQuerySessionLabelSegment>(),
        ),
        Err(ForgeQuerySessionLabelError::MissingNameSegments)
    );
    assert_eq!(
        ForgeQuerySessionLabel::scoped_strs("worth-kernel", ["preview", " "]),
        Err(ForgeQuerySessionLabelError::EmptyNameSegment)
    );
}
