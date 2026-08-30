use worth_foundational::PhysicalArtifactFamily;

use super::{
    OfflineArtifactFamily, OfflineIndeterminatePhysicalReason, OfflineIntegrityReportCompleteness,
    OfflinePhysicalBlastRadius, OfflinePhysicalDamageCause, OfflinePhysicalFormatField,
    OfflineUnknownPhysicalReason, OfflineUnsupportedVersionAxis,
};

pub(crate) fn completeness(value: OfflineIntegrityReportCompleteness) -> &'static str {
    match value {
        OfflineIntegrityReportCompleteness::Complete => "complete",
        OfflineIntegrityReportCompleteness::BoundExhausted => "bound_exhausted",
        OfflineIntegrityReportCompleteness::Indeterminate => "indeterminate",
    }
}

pub(crate) fn family(value: OfflineArtifactFamily) -> &'static str {
    match value {
        OfflineArtifactFamily::Unrecognized => "unrecognized",
        OfflineArtifactFamily::Declared(PhysicalArtifactFamily::NamespaceIdentity) => {
            "namespace_identity"
        }
        OfflineArtifactFamily::Declared(PhysicalArtifactFamily::CurrentRootSelector) => {
            "current_root_selector"
        }
        OfflineArtifactFamily::Declared(PhysicalArtifactFamily::PreviousRootSelector) => {
            "previous_root_selector"
        }
        OfflineArtifactFamily::Declared(PhysicalArtifactFamily::RootManifest) => "root_manifest",
        OfflineArtifactFamily::Declared(_) => "outside_phase_3_root_slice",
    }
}

pub(crate) fn damage_cause(value: OfflinePhysicalDamageCause) -> &'static str {
    match value {
        OfflinePhysicalDamageCause::ChecksumMismatch => "checksum_mismatch",
        OfflinePhysicalDamageCause::Framing => "framing",
        OfflinePhysicalDamageCause::ScopeMismatch => "scope_mismatch",
        OfflinePhysicalDamageCause::Pointer => "pointer",
        OfflinePhysicalDamageCause::Truncation => "truncation",
        OfflinePhysicalDamageCause::MissingArtifact => "missing_artifact",
        OfflinePhysicalDamageCause::DuplicateIdentity => "duplicate_identity",
        OfflinePhysicalDamageCause::MalformedPayload => "malformed_payload",
    }
}

pub(crate) fn blast(value: OfflinePhysicalBlastRadius) -> &'static str {
    match value {
        OfflinePhysicalBlastRadius::Field => "field",
        OfflinePhysicalBlastRadius::Frame => "frame",
        OfflinePhysicalBlastRadius::Artifact => "artifact",
        OfflinePhysicalBlastRadius::ReachableRootSubtree => "reachable_root_subtree",
    }
}

pub(crate) fn format_field(value: OfflinePhysicalFormatField) -> &'static str {
    match value {
        OfflinePhysicalFormatField::Magic => "magic",
        OfflinePhysicalFormatField::EncodingVersion => "encoding_version",
        OfflinePhysicalFormatField::NamespaceVersion => "namespace_version",
        OfflinePhysicalFormatField::RecordLength => "record_length",
        OfflinePhysicalFormatField::FieldCount => "field_count",
        OfflinePhysicalFormatField::IdentityField => "identity_field",
        OfflinePhysicalFormatField::FamilyKind => "family_kind",
        OfflinePhysicalFormatField::EnvelopeSchema => "envelope_schema",
        OfflinePhysicalFormatField::FormatVersion => "format_version",
        OfflinePhysicalFormatField::PageSize => "page_size",
        OfflinePhysicalFormatField::ByteOrder => "byte_order",
        OfflinePhysicalFormatField::RootProtocol => "root_protocol",
        OfflinePhysicalFormatField::IntegrityAlgorithm => "integrity_algorithm",
        OfflinePhysicalFormatField::RecordIdentityWidth => "record_identity_width",
        OfflinePhysicalFormatField::HeaderLength => "header_length",
        OfflinePhysicalFormatField::PayloadLength => "payload_length",
        OfflinePhysicalFormatField::FrameIdentity => "frame_identity",
        OfflinePhysicalFormatField::Checksum => "checksum",
        OfflinePhysicalFormatField::StoreIdentity => "store_identity",
        OfflinePhysicalFormatField::SelectorRole => "selector_role",
        OfflinePhysicalFormatField::RootGeneration => "root_generation",
        OfflinePhysicalFormatField::LinkedSelector => "linked_selector",
        OfflinePhysicalFormatField::EmbeddedFormat => "embedded_format",
        OfflinePhysicalFormatField::ManifestGeneration => "manifest_generation",
        OfflinePhysicalFormatField::ManifestPointer => "manifest_pointer",
        OfflinePhysicalFormatField::Reserved => "reserved",
    }
}

pub(crate) fn unsupported_axis(value: OfflineUnsupportedVersionAxis) -> &'static str {
    match value {
        OfflineUnsupportedVersionAxis::NamespaceEncoding => "namespace_encoding",
        OfflineUnsupportedVersionAxis::NamespaceSchema => "namespace_schema",
        OfflineUnsupportedVersionAxis::EnvelopeSchema => "envelope_schema",
        OfflineUnsupportedVersionAxis::PhysicalRecordFormat => "physical_record_format",
        OfflineUnsupportedVersionAxis::PageSize => "page_size",
        OfflineUnsupportedVersionAxis::ByteOrder => "byte_order",
        OfflineUnsupportedVersionAxis::RootProtocol => "root_protocol",
        OfflineUnsupportedVersionAxis::IntegrityAlgorithm => "integrity_algorithm",
        OfflineUnsupportedVersionAxis::RecordIdentityWidth => "record_identity_width",
    }
}

pub(crate) fn unknown(value: OfflineUnknownPhysicalReason) -> &'static str {
    match value {
        OfflineUnknownPhysicalReason::UnrecognizedFile => "unrecognized_file",
        OfflineUnknownPhysicalReason::UnrecognizedDirectory => "unrecognized_directory",
        OfflineUnknownPhysicalReason::UnrecognizedOtherEntry => "unrecognized_other_entry",
        OfflineUnknownPhysicalReason::SelectorUnavailable => "selector_unavailable",
        OfflineUnknownPhysicalReason::RootNotAddressed => "root_not_addressed",
        OfflineUnknownPhysicalReason::StoreIdentityUnavailable => "store_identity_unavailable",
        OfflineUnknownPhysicalReason::FilesystemEntryUnavailable => "filesystem_entry_unavailable",
    }
}

pub(crate) fn indeterminate(value: OfflineIndeterminatePhysicalReason) -> &'static str {
    match value {
        OfflineIndeterminatePhysicalReason::SourceChanged => "source_changed",
        OfflineIndeterminatePhysicalReason::EntryBoundExceeded => "entry_bound_exceeded",
        OfflineIndeterminatePhysicalReason::ByteBoundExceeded => "byte_bound_exceeded",
        OfflineIndeterminatePhysicalReason::OpenFileBoundExceeded => "open_file_bound_exceeded",
        OfflineIndeterminatePhysicalReason::DepthBoundExceeded => "depth_bound_exceeded",
        OfflineIndeterminatePhysicalReason::SymlinkRefused => "symlink_refused",
        OfflineIndeterminatePhysicalReason::SymlinkBoundExceeded => "symlink_bound_exceeded",
        OfflineIndeterminatePhysicalReason::ElapsedBoundExceeded => "elapsed_bound_exceeded",
        OfflineIndeterminatePhysicalReason::PhysicalIdentityUnavailable => {
            "physical_identity_unavailable"
        }
        OfflineIndeterminatePhysicalReason::IoFailure => "io_failure",
    }
}
