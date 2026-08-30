//! Typed structural denials for untrusted package-record intake.

use crate::package::{
    WorthQueryPortablePackageManifestVersion, WorthQueryPortablePackageRecordFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryPortablePackageReconstructionDenial {
    ApplicationSchemaReadmissionDenied {
        denial:
            worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationDenial,
    },
    ArtifactContractReadmissionDenied {
        denial: crate::domain_computation::WorthQueryPortableArtifactContractReadmissionDenial,
    },
    IllegalRecordOrdering {
        family: WorthQueryPortablePackageRecordFamily,
    },
    FreshPackageValidationDenied {
        denial: crate::package::WorthQueryPortablePackageValidationDenial,
    },
    FreshPackageExportDenied {
        denial: crate::package::WorthQueryPortablePackageExportDenial,
    },
    ManifestPackageIdentityMismatch {
        claimed: crate::package::WorthQueryPortableDomainPackageIdentity,
        recomputed: crate::package::WorthQueryPortableDomainPackageIdentity,
    },
    ExpectedPackageIdentityMismatch {
        expected: crate::package::WorthQueryPortableDomainPackageIdentity,
        recomputed: crate::package::WorthQueryPortableDomainPackageIdentity,
    },
    FreshManifestClosureMismatch,
    DerivedContractClosureMismatch {
        family: WorthQueryPortablePackageRecordFamily,
    },
    CanonicalQueryReadmissionDenied {
        operation_slot: String,
        denial: worth_query_declaration::facade::canonicalization::QueryCanonicalizationError,
    },
    DomainOperationIdentityMismatch {
        operation_slot: String,
    },
    NonCanonicalDomainOperationSemantics {
        operation_slot: String,
    },
    DomainIdentityCardinality {
        observed: u32,
    },
    UnsupportedManifestVersion {
        observed: WorthQueryPortablePackageManifestVersion,
        supported: WorthQueryPortablePackageManifestVersion,
    },
    RecordBudgetExceeded {
        declared: u32,
        maximum: u32,
    },
    DeclaredLogicalByteCeilingExceeded {
        declared: u64,
        maximum: u64,
    },
    DeclaredCanonicalWorkBudgetExceeded {
        declared: u64,
        maximum: u64,
    },
    CanonicalSourceExceedsLogicalExport {
        canonical_source_bytes: u64,
        logical_export_bytes: u64,
    },
    FamilyCountOverflow,
    FamilyCountMismatch {
        declared_family_total: u32,
        declared_record_count: u32,
    },
    RecordIndexMismatch {
        expected: u32,
        observed: u32,
    },
    RecordFamilyMismatch {
        canonical_index: u32,
        expected: WorthQueryPortablePackageRecordFamily,
        observed: WorthQueryPortablePackageRecordFamily,
    },
    RecordCountExceeded {
        declared: u32,
    },
    RecordCountIncomplete {
        declared: u32,
        observed: u32,
    },
    WorkObservationOverflow,
    LogicalByteBudgetExceeded {
        observed: u64,
        maximum: u64,
    },
    NestedEntryBudgetExceeded {
        observed: u64,
        maximum: u64,
    },
    CanonicalWorkBudgetExceeded {
        observed: u64,
        maximum: u64,
    },
}
