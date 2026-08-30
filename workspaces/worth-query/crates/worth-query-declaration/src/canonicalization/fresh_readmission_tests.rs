use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, DetailAuthoredResultShape, GuidedAuthoringPath,
    RawAuthoredQuery, RawAuthoredResultShape, ResultShapeFamily, RootEntityKey, TraversalSelector,
};
use crate::canonicalization::{
    canonicalize_request, validate_portable_query_bundle_freshly, CanonicalQueryBundle,
    QueryCanonicalizationError as Error, WorthQueryPortableCanonicalQueryReadmissionLimits,
};
use crate::identity::{CanonicalQueryDigest, CanonicalResultShapeDigest};

use super::super::portable_bundle::{
    WorthQueryPortableCanonicalQueryBundleRecord as Record,
    WorthQueryPortableCanonicalQueryTestMutation as Mutation,
};

#[test]
fn honest_projection_receives_fresh_query_authority() {
    let source = fixture("profile", "name", "name");
    let expected = source.clone();
    let reconstructed = readmit(Record::project(&source)).unwrap();

    assert_eq!(reconstructed, expected);
    assert_eq!(
        reconstructed.query().authority().digest(),
        reconstructed.query().digest()
    );
}

#[test]
fn forged_query_digest_cannot_mint_canonical_authority() {
    let record = Record::project_for_test(
        &fixture("profile", "name", "name"),
        Mutation::QueryDigest(CanonicalQueryDigest::from_parts(&["forged".to_string()])),
    );

    assert_eq!(
        readmit(record),
        Err(Error::DigestBasisInconsistency { artifact: "query" })
    );
}

#[test]
fn forged_result_shape_digest_is_rejected() {
    let record = Record::project_for_test(
        &fixture("profile", "name", "name"),
        Mutation::ResultShapeDigest(CanonicalResultShapeDigest::from_parts(&[
            "forged".to_string()
        ])),
    );

    assert_eq!(
        readmit(record),
        Err(Error::DigestBasisInconsistency {
            artifact: "result_shape",
        })
    );
}

#[test]
fn duplicate_query_and_result_entries_are_not_recanonicalized_silently() {
    let source = fixture("profile", "name", "name");
    let query_duplicate = Record::project_for_test(&source, Mutation::DuplicateProjection);
    assert_eq!(
        readmit(query_duplicate),
        Err(Error::InvalidCanonicalOrderingBasis {
            artifact: "query_projection",
        })
    );

    let result_duplicate = Record::project_for_test(&source, Mutation::DuplicateResultField);
    assert_eq!(
        readmit(result_duplicate),
        Err(Error::InvalidCanonicalOrderingBasis {
            artifact: "result_shape_fields",
        })
    );
}

#[test]
fn unprojected_result_source_is_rejected() {
    let foreign = Record::project(&fixture("details", "title", "title"));
    let record = Record::project_for_test(
        &fixture("profile", "name", "name"),
        Mutation::ReplaceFirstResultField(foreign.result_shape().fields()[0].clone()),
    );

    assert!(matches!(
        readmit(record),
        Err(Error::UnprojectedShapeField { .. })
    ));
}

#[test]
fn ambiguous_result_alias_and_family_mismatch_are_rejected() {
    let alias = Record::project_for_test(&wide_fixture(), Mutation::AliasSecondResultToFirst);
    assert!(matches!(
        readmit(alias),
        Err(Error::AmbiguousShapeAliasIdentity { .. })
    ));

    let family = Record::project_for_test(
        &fixture("profile", "name", "name"),
        Mutation::ResultShapeFamily(ResultShapeFamily::Collection),
    );
    assert!(matches!(
        readmit(family),
        Err(Error::QueryShapeFamilyMismatch { .. })
    ));
}

#[test]
fn invalid_report_counter_cannot_enter_a_fresh_bundle() {
    let record = Record::project_for_test(
        &fixture("profile", "name", "name"),
        Mutation::IncrementWarningCount,
    );

    assert!(matches!(
        readmit(record),
        Err(Error::BundleInvariantViolation { .. })
    ));
}

#[test]
fn zero_depth_traversal_is_rejected_before_authority() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("profile", "name").unwrap())
        .traverse(TraversalSelector::bounded("owner", 2).unwrap())
        .build()
        .unwrap();
    let shape = shape("profile", "name", "name");
    let source =
        canonicalize_request(GuidedAuthoringPath::pair_detail(query, shape).unwrap()).unwrap();
    let record = Record::project_for_test(&source, Mutation::FirstTraversalDepth(0));

    assert!(matches!(
        readmit(record),
        Err(Error::UnsupportedTraversalDepth { depth: 0, .. })
    ));
}

#[test]
fn caller_narrowed_entry_and_logical_width_budgets_fail_closed() {
    let source = fixture("profile", "name", "name");
    let entry_limited = validate_portable_query_bundle_freshly(
        Record::project(&source),
        WorthQueryPortableCanonicalQueryReadmissionLimits::new(1, u64::MAX),
    );
    assert!(matches!(
        entry_limited,
        Err(Error::PortableRecordEntryBudgetExceeded { maximum: 1, .. })
    ));

    let byte_limited = validate_portable_query_bundle_freshly(
        Record::project(&source),
        WorthQueryPortableCanonicalQueryReadmissionLimits::new(u32::MAX, 1),
    );
    assert!(matches!(
        byte_limited,
        Err(Error::PortableRecordLogicalBytesBudgetExceeded { maximum: 1, .. })
    ));
}

fn readmit(record: Record) -> Result<CanonicalQueryBundle, Error> {
    validate_portable_query_bundle_freshly(
        record,
        WorthQueryPortableCanonicalQueryReadmissionLimits::DEFAULT,
    )
}

fn fixture(aspect: &str, field: &str, delivered: &str) -> CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new(aspect, field).unwrap())
        .build()
        .unwrap();
    canonicalize_request(
        GuidedAuthoringPath::pair_detail(query, shape(aspect, field, delivered)).unwrap(),
    )
    .unwrap()
}

fn wide_fixture() -> CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("details", "title").unwrap())
        .project(AspectFieldSelector::new("profile", "name").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("details", "title", "title").unwrap())
        .field(AuthoredResultShapeField::new("profile", "name", "name").unwrap())
        .build()
        .unwrap();
    canonicalize_request(GuidedAuthoringPath::pair_detail(query, shape).unwrap()).unwrap()
}

fn shape(aspect: &str, field: &str, delivered: &str) -> DetailAuthoredResultShape {
    RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new(aspect, field, delivered).unwrap())
        .build()
        .unwrap()
}
