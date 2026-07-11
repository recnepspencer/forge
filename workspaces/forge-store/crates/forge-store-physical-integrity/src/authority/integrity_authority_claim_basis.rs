use crate::{
    authority::integrity_authority_basis_entries::{
        authority_domain, physical_scope_entries, push_manifest_reference_basis,
        push_owner_entries, push_text, push_u16, push_u32,
    },
    authority::integrity_authority_basis_tokens::{
        boundary_localization_token, root_posture_token, wal_tail_posture_token,
    },
    authority::integrity_authority_counter_entries::{
        push_container_counters, push_manifest_counters, push_wal_counters,
    },
    CheckpointRecordIntegrityReport, FrameIntegrityReport, ManifestIntegrityReport,
    PageIntegrityReport, PhysicalBoundaryLocalization, PhysicalIntegrityEvidenceDenial,
    WalFrameIntegrityReport,
};
use forge_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisEntry, CanonicalizationRuleVersion,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreCanonicalBasisFamily, StoreDigestAuthority, StoreDigestEvidence,
};

const AUTHORITY_BASIS_VERSION: &str = "store.physical-integrity.authority-basis.v1";

pub fn page_authority_digest(
    report: &PageIntegrityReport,
) -> Result<StoreDigestEvidence, PhysicalIntegrityEvidenceDenial> {
    let mut entries = physical_scope_entries("page-authority", report.basis());
    push_container_counters(&mut entries, report.counters());
    push_u16(
        &mut entries,
        "slot-directory.slot-count",
        report.slot_directory().slot_count(),
    );
    push_u16(
        &mut entries,
        "slot-directory.occupied-slots",
        report.slot_directory().occupied_slots(),
    );
    push_u16(
        &mut entries,
        "slot-directory.free-or-reserved-slots",
        report.slot_directory().free_or_reserved_slots(),
    );
    derive_authority_digest(
        StoreCanonicalBasisFamily::PhysicalIntegrityEvidence,
        entries,
    )
}

pub fn frame_authority_digest(
    report: &FrameIntegrityReport,
) -> Result<StoreDigestEvidence, PhysicalIntegrityEvidenceDenial> {
    let mut entries = physical_scope_entries("frame-authority", report.basis());
    push_container_counters(&mut entries, report.counters());
    push_boundary_entries(&mut entries, "boundary", report.boundary());
    derive_authority_digest(
        StoreCanonicalBasisFamily::PhysicalIntegrityEvidence,
        entries,
    )
}

pub fn wal_frame_authority_digest(
    report: &WalFrameIntegrityReport,
) -> Result<StoreDigestEvidence, PhysicalIntegrityEvidenceDenial> {
    let mut entries = physical_scope_entries("wal-frame-authority", report.basis());
    push_text(
        &mut entries,
        "tail-posture",
        wal_tail_posture_token(report.tail_posture()),
    );
    push_wal_counters(&mut entries, report.counters());
    derive_authority_digest(
        StoreCanonicalBasisFamily::WalFrameIntegrityEvidence,
        entries,
    )
}

pub fn checkpoint_authority_digest(
    report: &CheckpointRecordIntegrityReport,
) -> Result<StoreDigestEvidence, PhysicalIntegrityEvidenceDenial> {
    let mut entries = physical_scope_entries("checkpoint-authority", report.basis());
    push_text(
        &mut entries,
        "tail-posture",
        wal_tail_posture_token(report.tail_posture()),
    );
    push_wal_counters(&mut entries, report.counters());
    derive_authority_digest(
        StoreCanonicalBasisFamily::PhysicalIntegrityEvidence,
        entries,
    )
}

pub fn manifest_authority_digest(
    report: &ManifestIntegrityReport,
) -> Result<StoreDigestEvidence, PhysicalIntegrityEvidenceDenial> {
    let mut entries = Vec::new();
    push_text(&mut entries, "authority-kind", "manifest-authority");
    push_text(
        &mut entries,
        "root-posture",
        root_posture_token(report.root().posture()),
    );
    if let Some(owner) = report.root().root_owner() {
        push_owner_entries(&mut entries, "root-owner", owner);
    }
    push_u32(
        &mut entries,
        "segment-manifest.segment-entries",
        report.segment().segment_entries(),
    );
    push_u32(
        &mut entries,
        "segment-manifest.page-slot-entries",
        report.segment().page_slot_entries(),
    );
    push_u32(
        &mut entries,
        "segment-manifest.extent-entries",
        report.segment().extent_entries(),
    );
    push_u32(
        &mut entries,
        "allocation-map.allocation-entries",
        report.allocation().allocation_entries(),
    );
    push_u32(
        &mut entries,
        "allocation-map.free-space-entries",
        report.allocation().free_space_entries(),
    );
    push_manifest_counters(&mut entries, report.counters());
    push_manifest_reference_basis(&mut entries, report.reference_basis());
    derive_authority_digest(
        StoreCanonicalBasisFamily::PhysicalIntegrityEvidence,
        entries,
    )
}

fn derive_authority_digest(
    family: StoreCanonicalBasisFamily,
    mut entries: Vec<CanonicalBasisEntry>,
) -> Result<StoreDigestEvidence, PhysicalIntegrityEvidenceDenial> {
    let domain = authority_domain(family);
    let version = CanonicalizationRuleVersion::new(AUTHORITY_BASIS_VERSION)
        .ok_or(PhysicalIntegrityEvidenceDenial::AuthorityDigestDenied)?;
    entries = entries
        .into_iter()
        .map(|entry| {
            CanonicalBasisEntry::new(
                domain,
                entry.locus().clone(),
                entry.kind(),
                entry.value().clone(),
            )
        })
        .collect();
    let basis = match prepare_canonical_basis_sequence(version, domain, entries) {
        TransitionOutcome::Success(basis) => basis,
        _ => return Err(PhysicalIntegrityEvidenceDenial::AuthorityDigestDenied),
    };
    let algorithm = forge_foundational::CanonicalDigestAlgorithmId::test_stable_fixture();

    match StoreDigestAuthority::for_native_basis(family, basis).derive(algorithm) {
        TransitionOutcome::Success(evidence) => Ok(evidence),
        _ => Err(PhysicalIntegrityEvidenceDenial::AuthorityDigestDenied),
    }
}

fn push_boundary_entries(
    entries: &mut Vec<CanonicalBasisEntry>,
    prefix: &str,
    boundary: PhysicalBoundaryLocalization,
) {
    push_text(entries, prefix, boundary_localization_token(boundary));
    if let PhysicalBoundaryLocalization::SlotState(slot) = boundary {
        push_u16(entries, &format!("{prefix}.slot"), slot.get());
    }
}
