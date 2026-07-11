use crate::{
    authority::integrity_authority_basis_tokens::{
        allocation_class_token, checkpoint_adjacency_token, generation_report_token,
        owner_domain_token, reference_kind_token, root_posture_token, scope_family_token,
    },
    GenerationIntegrityReport, ManifestReferenceBasis, PhysicalScopeBasis,
};
use forge_foundational::canonicalization_api::lower_lane::basis::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalIntegerWidth,
};
use forge_store_aspect_native::StoreCanonicalBasisFamily;
use forge_store_physical_format::{
    ManifestMembershipProof, PhysicalGenerationOwner, PhysicalReference, PhysicalReferenceScope,
};

pub(crate) fn physical_scope_entries(
    authority_kind: &'static str,
    basis: &PhysicalScopeBasis,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = Vec::new();
    push_text(&mut entries, "authority-kind", authority_kind);
    push_text(
        &mut entries,
        "scope.family",
        scope_family_token(basis.scope()),
    );
    push_owner_entries(&mut entries, "scope.owner", basis.scope().owner());
    if let Some(reference) = basis.scope().reference() {
        push_reference_entries(&mut entries, "scope.reference", reference);
    }
    push_membership_entries(&mut entries, "manifest-membership", basis.membership());
    push_text(
        &mut entries,
        "root-posture",
        root_posture_token(basis.root_posture()),
    );
    push_text(
        &mut entries,
        "checkpoint-adjacency",
        checkpoint_adjacency_token(basis.checkpoint_adjacency()),
    );
    push_generation_report_entries(&mut entries, "generation-report", basis.generation_report());
    entries
}

pub(crate) fn push_manifest_reference_basis(
    entries: &mut Vec<CanonicalBasisEntry>,
    basis: &ManifestReferenceBasis,
) {
    if let Some(owner) = basis.root_owner() {
        push_owner_entries(entries, "manifest-reference.root-owner", owner);
    }
    push_u32(
        entries,
        "manifest-reference.physical-owner-count",
        basis.physical_owners().len() as u32,
    );
    for (index, owner) in basis.physical_owners().iter().copied().enumerate() {
        push_owner_entries(
            entries,
            &format!("manifest-reference.physical-owner.{index}"),
            owner,
        );
    }
    push_u32(
        entries,
        "manifest-reference.admitted-scope-count",
        basis.admitted_scopes().len() as u32,
    );
    for (index, scope) in basis.admitted_scopes().iter().copied().enumerate() {
        push_scope_entries(
            entries,
            &format!("manifest-reference.admitted-scope.{index}"),
            scope,
        );
    }
}

pub(crate) fn push_owner_entries(
    entries: &mut Vec<CanonicalBasisEntry>,
    prefix: &str,
    owner: PhysicalGenerationOwner,
) {
    push_text(
        entries,
        &format!("{prefix}.domain"),
        owner_domain_token(owner.domain()),
    );
    if let Some(value) = owner.segment_id() {
        push_u64(entries, &format!("{prefix}.segment-id"), value.get());
    }
    if let Some(value) = owner.page_id() {
        push_u64(entries, &format!("{prefix}.page-id"), value.get());
    }
    if let Some(value) = owner.extent_id() {
        push_u64(entries, &format!("{prefix}.extent-id"), value.get());
    }
    if let Some(value) = owner.slot() {
        push_u16(entries, &format!("{prefix}.slot"), value.get());
    }
    if let Some(value) = owner.root_reference() {
        push_u64(entries, &format!("{prefix}.root-reference"), value.get());
    }
    if let Some(value) = owner.allocation_class() {
        push_text(
            entries,
            &format!("{prefix}.allocation-class"),
            allocation_class_token(value),
        );
    }
    push_u64(
        entries,
        &format!("{prefix}.generation"),
        owner.generation().get(),
    );
}

fn push_scope_entries(
    entries: &mut Vec<CanonicalBasisEntry>,
    prefix: &str,
    scope: PhysicalReferenceScope,
) {
    push_text(
        entries,
        &format!("{prefix}.family"),
        scope_family_token(scope),
    );
    push_owner_entries(entries, &format!("{prefix}.owner"), scope.owner());
    if let Some(reference) = scope.reference() {
        push_reference_entries(entries, &format!("{prefix}.reference"), reference);
    }
}

fn push_reference_entries(
    entries: &mut Vec<CanonicalBasisEntry>,
    prefix: &str,
    reference: PhysicalReference,
) {
    push_text(
        entries,
        &format!("{prefix}.kind"),
        reference_kind_token(reference),
    );
    if let Some(value) = reference.segment_id() {
        push_u64(entries, &format!("{prefix}.segment-id"), value.get());
    }
    if let Some(value) = reference.page_id() {
        push_u64(entries, &format!("{prefix}.page-id"), value.get());
    }
    if let Some(value) = reference.extent_id() {
        push_u64(entries, &format!("{prefix}.extent-id"), value.get());
    }
    if let Some(value) = reference.slot() {
        push_u16(entries, &format!("{prefix}.slot"), value.get());
    }
    if let Some(value) = reference.root_reference() {
        push_u64(entries, &format!("{prefix}.root-reference"), value.get());
    }
    if let Some(value) = reference.allocation_class() {
        push_text(
            entries,
            &format!("{prefix}.allocation-class"),
            allocation_class_token(value),
        );
    }
    push_u64(
        entries,
        &format!("{prefix}.generation"),
        reference.generation().get(),
    );
}

fn push_membership_entries(
    entries: &mut Vec<CanonicalBasisEntry>,
    prefix: &str,
    membership: ManifestMembershipProof,
) {
    push_scope_entries(entries, &format!("{prefix}.scope"), membership.scope());
    push_owner_entries(
        entries,
        &format!("{prefix}.root-owner"),
        membership.root_owner(),
    );
}

fn push_generation_report_entries(
    entries: &mut Vec<CanonicalBasisEntry>,
    prefix: &str,
    report: GenerationIntegrityReport,
) {
    push_text(
        entries,
        &format!("{prefix}.kind"),
        generation_report_token(report),
    );
    push_owner_entries(
        entries,
        &format!("{prefix}.expected-owner"),
        report.expected_owner(),
    );
    push_owner_entries(
        entries,
        &format!("{prefix}.actual-owner"),
        report.actual_owner(),
    );
}

pub(crate) fn push_text(entries: &mut Vec<CanonicalBasisEntry>, locus: &str, value: &str) {
    entries.push(CanonicalBasisEntry::new(
        staging_domain(),
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::Future("physical-integrity-authority-field"),
        CanonicalBasisValue::ExactText(value.to_string().into()),
    ));
}

pub(crate) fn push_u16(entries: &mut Vec<CanonicalBasisEntry>, locus: &str, value: u16) {
    push_unsigned(
        entries,
        locus,
        CanonicalIntegerWidth::Bits16,
        u128::from(value),
    );
}

pub(crate) fn push_u32(entries: &mut Vec<CanonicalBasisEntry>, locus: &str, value: u32) {
    push_unsigned(
        entries,
        locus,
        CanonicalIntegerWidth::Bits32,
        u128::from(value),
    );
}

pub(crate) fn authority_domain(family: StoreCanonicalBasisFamily) -> CanonicalBasisDomain {
    match family {
        StoreCanonicalBasisFamily::WalFrameIntegrityEvidence => {
            CanonicalBasisDomain::Future("store.wal.frame.integrity.evidence")
        }
        _ => CanonicalBasisDomain::Future("store.physical.integrity.evidence"),
    }
}

fn push_u64(entries: &mut Vec<CanonicalBasisEntry>, locus: &str, value: u64) {
    push_unsigned(
        entries,
        locus,
        CanonicalIntegerWidth::Bits64,
        u128::from(value),
    );
}

fn push_unsigned(
    entries: &mut Vec<CanonicalBasisEntry>,
    locus: &str,
    width: CanonicalIntegerWidth,
    value: u128,
) {
    entries.push(CanonicalBasisEntry::new(
        staging_domain(),
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::Future("physical-integrity-authority-field"),
        CanonicalBasisValue::UnsignedInteger { width, value },
    ));
}

fn staging_domain() -> CanonicalBasisDomain {
    authority_domain(StoreCanonicalBasisFamily::PhysicalIntegrityEvidence)
}
