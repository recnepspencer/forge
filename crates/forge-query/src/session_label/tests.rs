use super::{
    ForgeQuerySessionLabel, ForgeQuerySessionLabelError, ForgeQuerySessionLabelSegment,
    ForgeQuerySessionNamespace,
};
use crate::facade::runtime::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

#[test]
fn artifact_is_exported_through_runtime_facade_and_recomposes_from_typed_parts() {
    let label = ForgeQuerySessionLabel::scoped(
        ForgeQuerySessionNamespace::new("worth-kernel").expect("namespace should build"),
        [
            ForgeQuerySessionLabelSegment::new("temporal").expect("segment should build"),
            ForgeQuerySessionLabelSegment::new("preview").expect("segment should build"),
        ],
    )
    .expect("label should build");
    let recomposed =
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SessionLabelIdentity)
            .field_identity(
                ForgeQueryEvidenceTag::new("session_label_namespace"),
                "worth-kernel",
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("session_label_name_segment_count"),
                2,
            )
            .field_identity_sequence(
                ForgeQueryEvidenceTag::new("session_label_name_segments"),
                ["temporal", "preview"],
            )
            .seal();

    assert_eq!(
        label.identity_digest().scope(),
        ForgeQueryEvidenceScope::SessionLabelIdentity
    );
    assert_eq!(label.display(), "worth-kernel.temporal.preview");
    assert_eq!(label.identity_digest(), &recomposed);
}

#[test]
fn scoped_and_string_construction_paths_share_identity() {
    let typed = ForgeQuerySessionLabel::scoped(
        ForgeQuerySessionNamespace::new("worth-kernel").expect("namespace should build"),
        [
            ForgeQuerySessionLabelSegment::new("temporal").expect("segment should build"),
            ForgeQuerySessionLabelSegment::new("preview").expect("segment should build"),
        ],
    )
    .expect("label should build");
    let strings = ForgeQuerySessionLabel::scoped_strs("worth-kernel", ["temporal", "preview"])
        .expect("string label should build");

    assert_eq!(typed, strings);
    assert_eq!(typed.identity_digest(), strings.identity_digest());
    assert_eq!(
        typed.identity_digest().scope(),
        ForgeQuerySessionLabel::identity_scope()
    );
}

#[test]
fn display_is_projection_over_typed_parts() {
    let label = ForgeQuerySessionLabel::scoped(
        ForgeQuerySessionNamespace::new("worth-kernel").expect("namespace should build"),
        [
            ForgeQuerySessionLabelSegment::new("temporal").expect("segment should build"),
            ForgeQuerySessionLabelSegment::new("preview").expect("segment should build"),
        ],
    )
    .expect("label should build");

    assert_eq!(label.namespace().as_str(), "worth-kernel");
    assert_eq!(
        label
            .name_segments()
            .iter()
            .map(ForgeQuerySessionLabelSegment::as_str)
            .collect::<Vec<_>>(),
        vec!["temporal", "preview"]
    );
    assert_eq!(label.display(), "worth-kernel.temporal.preview");
    assert_eq!(label.to_string(), label.display());
}

#[test]
fn render_collisions_do_not_collapse_identity() {
    let left = ForgeQuerySessionLabel::scoped(
        ForgeQuerySessionNamespace::new("worth.kernel").expect("namespace should build"),
        [ForgeQuerySessionLabelSegment::new("preview").expect("segment should build")],
    )
    .expect("label should build");
    let right = ForgeQuerySessionLabel::scoped(
        ForgeQuerySessionNamespace::new("worth").expect("namespace should build"),
        [
            ForgeQuerySessionLabelSegment::new("kernel").expect("segment should build"),
            ForgeQuerySessionLabelSegment::new("preview").expect("segment should build"),
        ],
    )
    .expect("label should build");

    assert_eq!(left.display(), right.display());
    assert_ne!(left, right);
    assert_ne!(left.identity_digest(), right.identity_digest());
}

#[test]
fn ordered_segments_change_identity_and_digest() {
    let left = ForgeQuerySessionLabel::scoped_strs("worth-kernel", ["preview", "temporal"])
        .expect("label should build");
    let right = ForgeQuerySessionLabel::scoped_strs("worth-kernel", ["temporal", "preview"])
        .expect("label should build");

    assert_ne!(left, right);
    assert_ne!(left.identity_digest(), right.identity_digest());
}

#[test]
fn namespace_changes_identity_and_digest_even_with_same_segments() {
    let left = ForgeQuerySessionLabel::scoped_strs("worth-kernel", ["temporal", "preview"])
        .expect("label should build");
    let right = ForgeQuerySessionLabel::scoped_strs("worth-runtime", ["temporal", "preview"])
        .expect("label should build");

    assert_ne!(left, right);
    assert_ne!(left.identity_digest(), right.identity_digest());
}

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
