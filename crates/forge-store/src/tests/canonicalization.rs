use crate::{
    authority::RawRuntimeCommitEnvelope,
    authority::{canonicalize, CURRENT_CANONICALIZATION_VERSION},
    StoreErrorKind,
};

use super::harness::fixtures::runtime::{create_entity, latest_envelope, runtime_with_demo_schema};

#[test]
fn unsupported_canonicalization_version_is_rejected() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let error = canonicalize(
        RawRuntimeCommitEnvelope::new(envelope),
        CURRENT_CANONICALIZATION_VERSION + 1,
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        &StoreErrorKind::UnsupportedCanonicalizationVersion
    );
}

#[test]
fn duplicate_parent_lists_are_rejected_as_noncanonical() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let mut envelope = latest_envelope(&runtime);
    envelope.commit.parents = vec![
        forge_relational::facade::history::CommitId(1),
        forge_relational::facade::history::CommitId(1),
    ];

    let error = canonicalize(
        RawRuntimeCommitEnvelope::new(envelope),
        CURRENT_CANONICALIZATION_VERSION,
    )
    .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::NonCanonicalEnvelope);
}
