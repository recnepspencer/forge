use std::path::Path;

use crate::docs_closeout::{WorthDocMetadata, WorthDocsCloseoutErrorKind};

#[test]
fn metadata_parser_rejects_unknown_keys() {
    let error = WorthDocMetadata::parse(
        Path::new("virtual.md"),
        "<!-- worth-doc\ncrate: worth-kernel\nkind: feature\nid: x\nunknown: y\n-->\n",
    )
    .expect_err("unknown metadata key should fail");

    assert_eq!(error.kind(), &WorthDocsCloseoutErrorKind::InvalidMetadata);
    assert!(error.detail().contains("unknown metadata key"));
}

#[test]
fn metadata_parser_rejects_invalid_bools() {
    let error = WorthDocMetadata::parse(
        Path::new("virtual.md"),
        "<!-- worth-doc\ncrate: worth-kernel\nkind: feature\nid: x\nquery_integration_required: maybe\n-->\n",
    )
    .expect_err("invalid bool should fail");

    assert_eq!(error.kind(), &WorthDocsCloseoutErrorKind::InvalidMetadata);
    assert!(error.detail().contains("must be `true` or `false`"));
}

#[test]
fn metadata_parser_rejects_duplicate_keys() {
    let error = WorthDocMetadata::parse(
        Path::new("virtual.md"),
        "<!-- worth-doc\ncrate: worth-kernel\ncrate: worth-topo\nkind: feature\nid: x\n-->\n",
    )
    .expect_err("duplicate metadata key should fail");

    assert_eq!(error.kind(), &WorthDocsCloseoutErrorKind::InvalidMetadata);
    assert!(error.detail().contains("duplicate metadata key `crate`"));
}

#[test]
fn metadata_parser_rejects_unknown_doc_kinds() {
    let error = WorthDocMetadata::parse(
        Path::new("virtual.md"),
        "<!-- worth-doc\ncrate: worth-kernel\nkind: guide\nid: x\n-->\n",
    )
    .expect_err("unknown doc kind should fail");

    assert_eq!(error.kind(), &WorthDocsCloseoutErrorKind::InvalidMetadata);
    assert!(error.detail().contains("unknown doc kind"));
}

#[test]
fn metadata_parser_rejects_missing_required_keys() {
    let error = WorthDocMetadata::parse(
        Path::new("virtual.md"),
        "<!-- worth-doc\ncrate: worth-kernel\nkind: feature\n-->\n",
    )
    .expect_err("missing required key should fail");

    assert_eq!(error.kind(), &WorthDocsCloseoutErrorKind::MissingMetadata);
    assert!(error
        .detail()
        .contains("missing required metadata key `id`"));
}

#[test]
fn metadata_parser_rejects_missing_metadata_block() {
    let error = WorthDocMetadata::parse(Path::new("virtual.md"), "# No Metadata\n")
        .expect_err("missing metadata block should fail");

    assert_eq!(error.kind(), &WorthDocsCloseoutErrorKind::MissingMetadata);
    assert!(error.detail().contains("missing `<!-- worth-doc` block"));
}

#[test]
fn metadata_parser_rejects_unterminated_metadata_blocks() {
    let error = WorthDocMetadata::parse(
        Path::new("virtual.md"),
        "<!-- worth-doc\ncrate: worth-kernel\nkind: feature\nid: x\n",
    )
    .expect_err("unterminated metadata block should fail");

    assert_eq!(error.kind(), &WorthDocsCloseoutErrorKind::InvalidMetadata);
    assert!(error.detail().contains("unterminated metadata block"));
}
