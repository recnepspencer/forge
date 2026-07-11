use forge_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalIntegerWidth, CanonicalizationRuleVersion,
};
use forge_foundational::{CanonicalBasisConstructionDenial, InternedString};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::StorePhysicalBoundaryWitness;
use forge_store_contracts::PhysicalAuthorityScope;

use crate::{
    PhysicalCellReuseDomain, PhysicalFrameKind, PhysicalHeaderDecodeWitness, PhysicalHeaderKind,
    PhysicalPageKind, PhysicalPublicationState,
};

const PAGE_HEADER_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("store.physical.page.header");
const PAGE_HEADER_FIELD_KIND: CanonicalBasisEntryKind =
    CanonicalBasisEntryKind::Future("store-page-header-field");
const PHYSICAL_BOUNDARY_WITNESS_KIND: CanonicalBasisEntryKind =
    CanonicalBasisEntryKind::Future("store-physical-boundary-witness");

pub type PhysicalPageHeaderCanonicalBasisOutcome =
    TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial>;

pub fn prepare_physical_page_header_canonical_basis(
    version: CanonicalizationRuleVersion,
    header: PhysicalHeaderDecodeWitness,
    boundary_witness: StorePhysicalBoundaryWitness,
) -> PhysicalPageHeaderCanonicalBasisOutcome {
    prepare_canonical_basis_sequence(
        version,
        PAGE_HEADER_DOMAIN,
        page_header_entries(header, boundary_witness),
    )
}

fn page_header_entries(
    header: PhysicalHeaderDecodeWitness,
    boundary_witness: StorePhysicalBoundaryWitness,
) -> Vec<CanonicalBasisEntry> {
    let owner = header.owner();
    let mut entries = vec![
        text_entry("source.kind", "store-page-header", PAGE_HEADER_FIELD_KIND),
        u64_entry(
            "header.kind.tag",
            u64::from(header.kind().tag()),
            PAGE_HEADER_FIELD_KIND,
        ),
        text_entry(
            "header.kind",
            header_kind_token(header.kind()),
            PAGE_HEADER_FIELD_KIND,
        ),
        u64_entry(
            "payload.offset",
            header.payload_offset() as u64,
            PAGE_HEADER_FIELD_KIND,
        ),
        u64_entry(
            "payload.length",
            u64::from(header.payload_length()),
            PAGE_HEADER_FIELD_KIND,
        ),
        text_entry(
            "publication",
            publication_token(header.publication()),
            PAGE_HEADER_FIELD_KIND,
        ),
        text_entry(
            "owner.domain",
            owner_domain_token(owner.domain()),
            PAGE_HEADER_FIELD_KIND,
        ),
        u64_entry(
            "owner.generation",
            owner.generation().get(),
            PAGE_HEADER_FIELD_KIND,
        ),
        text_entry(
            "boundary.authority.scope",
            authority_scope_token(boundary_witness.authority().authority_scope()),
            PHYSICAL_BOUNDARY_WITNESS_KIND,
        ),
        text_entry(
            "boundary.roadmap",
            boundary_witness.authority().roadmap_scope().roadmap(),
            PHYSICAL_BOUNDARY_WITNESS_KIND,
        ),
        text_entry(
            "boundary.sequence",
            boundary_witness.authority().roadmap_scope().sequence(),
            PHYSICAL_BOUNDARY_WITNESS_KIND,
        ),
    ];

    if let Some(segment) = owner.segment_id() {
        entries.push(u64_entry(
            "owner.segment",
            segment.get(),
            PAGE_HEADER_FIELD_KIND,
        ));
    }
    if let Some(page) = owner.page_id() {
        entries.push(u64_entry("owner.page", page.get(), PAGE_HEADER_FIELD_KIND));
    }
    if let Some(extent) = owner.extent_id() {
        entries.push(u64_entry(
            "owner.extent",
            extent.get(),
            PAGE_HEADER_FIELD_KIND,
        ));
    }
    if let Some(slot) = owner.slot() {
        entries.push(u64_entry(
            "owner.slot",
            u64::from(slot.get()),
            PAGE_HEADER_FIELD_KIND,
        ));
    }
    if let Some(root) = owner.root_reference() {
        entries.push(u64_entry("owner.root", root.get(), PAGE_HEADER_FIELD_KIND));
    }

    entries
}

fn text_entry(
    locus: impl Into<InternedString>,
    value: impl Into<InternedString>,
    kind: CanonicalBasisEntryKind,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        PAGE_HEADER_DOMAIN,
        CanonicalBasisLocus::Named(locus.into()),
        kind,
        CanonicalBasisValue::ExactText(value.into()),
    )
}

fn u64_entry(
    locus: impl Into<InternedString>,
    value: u64,
    kind: CanonicalBasisEntryKind,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        PAGE_HEADER_DOMAIN,
        CanonicalBasisLocus::Named(locus.into()),
        kind,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: u128::from(value),
        },
    )
}

fn header_kind_token(kind: PhysicalHeaderKind) -> &'static str {
    match kind {
        PhysicalHeaderKind::Page(PhysicalPageKind::DataPage) => "page.data",
        PhysicalHeaderKind::Page(PhysicalPageKind::ManifestPage) => "page.manifest",
        PhysicalHeaderKind::Frame(PhysicalFrameKind::RecordFrame) => "frame.record",
        PhysicalHeaderKind::Frame(PhysicalFrameKind::ExtentRecordFrame) => "frame.extent-record",
    }
}

fn publication_token(publication: PhysicalPublicationState) -> &'static str {
    match publication {
        PhysicalPublicationState::Unpublished => "unpublished",
        PhysicalPublicationState::Published => "published",
    }
}

fn owner_domain_token(domain: PhysicalCellReuseDomain) -> &'static str {
    match domain {
        PhysicalCellReuseDomain::SlotAllocation => "slot-allocation",
        PhysicalCellReuseDomain::ExtentAllocation => "extent-allocation",
        PhysicalCellReuseDomain::FreeSpaceReuse => "free-space-reuse",
        PhysicalCellReuseDomain::RootPublication => "root-publication",
        PhysicalCellReuseDomain::Page => "page",
        PhysicalCellReuseDomain::Segment => "segment",
    }
}

fn authority_scope_token(scope: PhysicalAuthorityScope) -> &'static str {
    match scope {
        PhysicalAuthorityScope::AspectNativeBoundaryVocabulary => "aspect-native-boundary",
        PhysicalAuthorityScope::PhysicalFoundationVocabulary => "physical-foundation",
        PhysicalAuthorityScope::PhysicalEvidenceExport => "physical-evidence-export",
        PhysicalAuthorityScope::PhysicalSubstrateReadiness => "physical-substrate-readiness",
    }
}
