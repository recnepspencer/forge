//! Public API boundary for `forge-query`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

pub use crate::authoring::{
    AspectFieldSelector, AuthoredBundleError, AuthoredBundleFailureClass, AuthoredResultShapeField,
    AuthoringError, AuthoringFailureClass, CollectionAuthoredQuery, CollectionAuthoredResultShape,
    CollectionQueryBuilder, CollectionResultShapeBuilder, DetailAuthoredQuery,
    DetailAuthoredResultShape, DetailQueryBuilder, DetailResultShapeBuilder, EqualityPredicate,
    GuidedAuthoringPath, IntegerComparisonOperator, IntegerComparisonPredicate, OrderingDirection,
    OrderingSelector, PredicateSelector, QueryFamily, ResultShapeFamily, RootEntityKey,
    ScalarPredicateValue, TraversalSelector,
};
pub use crate::binding::{
    BindingError, BindingFailureClass, IdentityBindingDescriptor, NonIdentityBindingMetadata,
    NonIdentityBindingMetadataKey, QueryBindingDescriptor, QueryBindingSlot, QueryBindingSubject,
};
pub use crate::canonicalization::{
    canonicalize_request, CanonicalOrderingEntry, CanonicalPredicateEntry,
    CanonicalPredicateFamily, CanonicalProjectionEntry, CanonicalQueryArtifact,
    CanonicalQueryBundle, CanonicalResultField, CanonicalResultShapeArtifact,
    CanonicalTraversalEntry, CanonicalizationFailureClass, QueryCanonicalizationError,
};
pub use crate::diagnostics::{
    CanonicalizationCounters, CanonicalizationReport, CanonicalizationWarning,
    CompatibilityEvidence, IdentityFreezeEvidence, NormalizationEvent,
};
pub use crate::identity::{
    CanonicalEquivalence, CanonicalQueryDigest, CanonicalResultShapeDigest, SchemaBasisDigest,
    ValidatedQueryDigest, ValidatedResultShapeDigest,
};
pub use crate::schema_view::{
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, SchemaRelationView,
};
pub use crate::typed::{
    TypedCollectionQuery, TypedCollectionQueryBuilder, TypedCollectionResultShape,
    TypedCollectionResultShapeBuilder, TypedDetailQuery, TypedDetailQueryBuilder,
    TypedDetailResultShape, TypedDetailResultShapeBuilder, TypedEqualityField,
    TypedGuidedAuthoringPath, TypedIntegerComparableField, TypedMembershipField,
    TypedOrderableField, TypedPresenceField, TypedProjectableField, TypedSchemaField,
    TypedSchemaRoot, TypedStringContainsField, TypedTraversalRelation,
};
pub use crate::validation::{
    validate_canonical_bundle, QueryValidationCounters, QueryValidationError,
    QueryValidationReport, ValidatedOrderingEntry, ValidatedOrderingSet, ValidatedPredicateEntry,
    ValidatedPredicateSet, ValidatedProjectionEntry, ValidatedQueryArtifact, ValidatedQueryBundle,
    ValidatedResultShapeArtifact, ValidatedResultShapeBinding, ValidatedTraversalEntry,
    ValidationEvent, ValidationFailureClass, ValidationRejectionMatrix, ValidationWarning,
};
