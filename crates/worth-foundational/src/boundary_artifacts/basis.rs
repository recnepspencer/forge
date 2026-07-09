use worth_proof::TransitionOutcome;

use super::materialization::{
    FoundationalBoundaryMaterializationAttachment, FoundationalBoundaryMaterializationBundle,
    FoundationalMaterializedBoundaryArtifact,
};
use super::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole,
    FoundationalBoundaryAvailability, FoundationalBoundaryDeliveryClass,
    FoundationalBoundaryMaterializationSeam, FoundationalBoundaryMaterializationSource,
};
use crate::canonicalization::{
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact,
    CanonicalBasisValue, CanonicalizationRuleVersion,
};

pub fn prepare_materialized_boundary_artifact_for_canonical_basis<Surface>(
    version: CanonicalizationRuleVersion,
    artifact: &FoundationalMaterializedBoundaryArtifact<Surface>,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::BoundaryArtifact,
        canonical_basis_for_materialized_boundary_artifact(artifact),
    )
}

pub fn prepare_materialized_boundary_bundle_for_canonical_basis<Primary, ReportRow>(
    version: CanonicalizationRuleVersion,
    bundle: &FoundationalBoundaryMaterializationBundle<Primary, ReportRow>,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::BoundaryArtifact,
        canonical_basis_for_materialized_boundary_bundle(bundle),
    )
}

pub fn foundational_boundary_canonical_basis_entries(
    ready: &CanonicalBasisReadyArtifact,
) -> &[CanonicalBasisEntry] {
    ready.payload().entries()
}

fn canonical_basis_for_materialized_boundary_artifact<Surface>(
    artifact: &FoundationalMaterializedBoundaryArtifact<Surface>,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = Vec::new();
    entries.push(text_entry("shape", "single-surface"));
    append_surface_entries("surface", artifact, &mut entries, true);
    entries
}

fn canonical_basis_for_materialized_boundary_bundle<Primary, ReportRow>(
    bundle: &FoundationalBoundaryMaterializationBundle<Primary, ReportRow>,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = Vec::new();
    entries.push(text_entry("shape", "coordinated-bundle"));
    entries.push(text_entry("bundle.source", source_token(bundle.source())));
    entries.push(text_entry("bundle.seam", seam_token(bundle.seam())));

    append_materialized_member_entries("member.primary", &bundle.primary, &mut entries, false);
    append_optional_member_entries("member.summary", bundle.summary(), &mut entries);
    append_optional_member_entries("member.report", bundle.report(), &mut entries);
    append_optional_member_entries("member.receipt", bundle.receipt(), &mut entries);

    entries
}

fn append_optional_member_entries<Surface>(
    prefix: &str,
    member: Option<&FoundationalMaterializedBoundaryArtifact<Surface>>,
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    entries.push(bool_entry(&format!("{prefix}.present"), member.is_some()));
    if let Some(member) = member {
        append_materialized_member_entries(prefix, member, entries, false);
    }
}

fn append_surface_entries<Surface>(
    prefix: &str,
    artifact: &FoundationalMaterializedBoundaryArtifact<Surface>,
    entries: &mut Vec<CanonicalBasisEntry>,
    include_source_and_seam: bool,
) {
    entries.push(text_entry(
        &format!("{prefix}.category"),
        category_token(artifact.category()),
    ));
    entries.push(text_entry(
        &format!("{prefix}.role"),
        role_token(artifact.role()),
    ));
    if include_source_and_seam {
        entries.push(text_entry(
            &format!("{prefix}.source"),
            source_token(artifact.source()),
        ));
        entries.push(text_entry(
            &format!("{prefix}.seam"),
            seam_token(artifact.seam()),
        ));
    }
    entries.push(text_entry(
        &format!("{prefix}.delivery-class"),
        delivery_class_token(artifact.disposition().delivery_class()),
    ));
    entries.push(text_entry(
        &format!("{prefix}.availability"),
        availability_token(artifact.disposition().availability()),
    ));
    append_attachment_entries(prefix, artifact.attachments(), entries);
}

fn append_materialized_member_entries<Surface>(
    prefix: &str,
    artifact: &FoundationalMaterializedBoundaryArtifact<Surface>,
    entries: &mut Vec<CanonicalBasisEntry>,
    include_source_and_seam: bool,
) {
    append_surface_entries(prefix, artifact, entries, include_source_and_seam);
}

fn append_attachment_entries(
    prefix: &str,
    attachments: &[FoundationalBoundaryMaterializationAttachment],
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    for attachment in attachments {
        entries.push(attachment_entry(
            &format!(
                "{prefix}.attachment.{}",
                attachment_point_token(attachment.point())
            ),
            attachment.is_included(),
        ));
    }
}

fn text_entry(locus: &str, value: &'static str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::BoundaryArtifact,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::BoundaryArtifact,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}

fn bool_entry(locus: &str, value: bool) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::BoundaryArtifact,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::BoundaryArtifact,
        CanonicalBasisValue::Bool(value),
    )
}

fn attachment_entry(locus: &str, included: bool) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::BoundaryArtifact,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::BoundaryAttachment,
        CanonicalBasisValue::Bool(included),
    )
}

fn category_token(value: FoundationalBoundaryArtifactCategory) -> &'static str {
    match value {
        FoundationalBoundaryArtifactCategory::Summary => "summary",
        FoundationalBoundaryArtifactCategory::Report => "report",
        FoundationalBoundaryArtifactCategory::Artifact => "artifact",
        FoundationalBoundaryArtifactCategory::Receipt => "receipt",
    }
}

fn role_token(value: FoundationalBoundaryArtifactRole) -> &'static str {
    match value {
        FoundationalBoundaryArtifactRole::AuthoritativeCurrent => "authoritative-current",
        FoundationalBoundaryArtifactRole::DerivedProjection => "derived-projection",
        FoundationalBoundaryArtifactRole::SupportOnly => "support-only",
        FoundationalBoundaryArtifactRole::PlannedWork => "planned-work",
        FoundationalBoundaryArtifactRole::ReceiptEvidence => "receipt-evidence",
    }
}

fn source_token(value: FoundationalBoundaryMaterializationSource) -> &'static str {
    match value {
        FoundationalBoundaryMaterializationSource::NativeAuthority => "native-authority",
        FoundationalBoundaryMaterializationSource::CompatibilityLowered => "compatibility-lowered",
        FoundationalBoundaryMaterializationSource::DerivedSupport => "derived-support",
    }
}

fn seam_token(value: FoundationalBoundaryMaterializationSeam) -> &'static str {
    match value {
        FoundationalBoundaryMaterializationSeam::BoundaryExchange => "boundary-exchange",
        FoundationalBoundaryMaterializationSeam::SupportMaterialization => {
            "support-materialization"
        }
        FoundationalBoundaryMaterializationSeam::PersistenceExport => "persistence-export",
    }
}

fn delivery_class_token(value: FoundationalBoundaryDeliveryClass) -> &'static str {
    match value {
        FoundationalBoundaryDeliveryClass::MustBeHot => "must-be-hot",
        FoundationalBoundaryDeliveryClass::CanDefer => "can-defer",
        FoundationalBoundaryDeliveryClass::ReconstructableFromRetainedBasis => {
            "reconstructable-from-retained-basis"
        }
    }
}

fn availability_token(value: FoundationalBoundaryAvailability) -> &'static str {
    match value {
        FoundationalBoundaryAvailability::Present => "present",
        FoundationalBoundaryAvailability::Deferred => "deferred",
        FoundationalBoundaryAvailability::Reconstructable => "reconstructable",
        FoundationalBoundaryAvailability::Unavailable => "unavailable",
    }
}

fn attachment_point_token(
    value: crate::boundary_artifacts::FoundationalBoundaryAttachmentPoint,
) -> &'static str {
    match value {
        crate::boundary_artifacts::FoundationalBoundaryAttachmentPoint::ProfileMeaning => {
            "profile-meaning"
        }
        crate::boundary_artifacts::FoundationalBoundaryAttachmentPoint::ProfileDecisions => {
            "profile-decisions"
        }
        crate::boundary_artifacts::FoundationalBoundaryAttachmentPoint::CanonicalBasis => {
            "canonical-basis"
        }
        crate::boundary_artifacts::FoundationalBoundaryAttachmentPoint::DiagnosticsAttachment => {
            "diagnostics-attachment"
        }
        crate::boundary_artifacts::FoundationalBoundaryAttachmentPoint::ProvenanceAttachment => {
            "provenance-attachment"
        }
        crate::boundary_artifacts::FoundationalBoundaryAttachmentPoint::PerformanceAccounting => {
            "performance-accounting"
        }
        crate::boundary_artifacts::FoundationalBoundaryAttachmentPoint::SameFamilyResolutionAttachment => {
            "same-family-resolution-attachment"
        }
    }
}
