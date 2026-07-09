use worth_foundational::canonicalization_api::lower_lane::basis::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalIntegerWidth,
};
use worth_foundational::InternedString;
use worth_store_contracts::PhysicalAuthorityScope;
use worth_store_physical_format::{
    PhysicalCellReuseDomain, PhysicalHeaderDecodeWitness, PhysicalHeaderKind,
    PhysicalPublicationState,
};

use crate::StorePhysicalBoundaryWitness;

const PAGE_HEADER_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("store.physical.page.header");
const ASPECT_BOUNDARY_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("store.aspect.boundary.fact");
const ASPECT_PATCH_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("store.aspect.patch.boundary.fact");
const PAGE_HEADER_FIELD_KIND: CanonicalBasisEntryKind =
    CanonicalBasisEntryKind::Future("store-page-header-field");
const ASPECT_BOUNDARY_FIELD_KIND: CanonicalBasisEntryKind =
    CanonicalBasisEntryKind::Future("store-aspect-boundary-field");
const ASPECT_PATCH_FIELD_KIND: CanonicalBasisEntryKind =
    CanonicalBasisEntryKind::Future("store-aspect-patch-field");
const PHYSICAL_BOUNDARY_WITNESS_KIND: CanonicalBasisEntryKind =
    CanonicalBasisEntryKind::Future("store-physical-boundary-witness");

pub fn aspect_boundary_entries(
    foundational_entries: &[CanonicalBasisEntry],
    boundary_witness: StorePhysicalBoundaryWitness,
) -> Vec<CanonicalBasisEntry> {
    store_boundary_entries(
        ASPECT_BOUNDARY_DOMAIN,
        ASPECT_BOUNDARY_FIELD_KIND,
        "foundational.aspect-state",
        "foundational-aspect-state",
        foundational_entries,
        boundary_witness,
    )
}

pub fn aspect_patch_entries(
    foundational_entries: &[CanonicalBasisEntry],
    boundary_witness: StorePhysicalBoundaryWitness,
) -> Vec<CanonicalBasisEntry> {
    store_boundary_entries(
        ASPECT_PATCH_DOMAIN,
        ASPECT_PATCH_FIELD_KIND,
        "foundational.aspect-patch",
        "foundational-aspect-patch",
        foundational_entries,
        boundary_witness,
    )
}

pub fn page_header_entries(
    header: PhysicalHeaderDecodeWitness,
    boundary_witness: StorePhysicalBoundaryWitness,
) -> Vec<CanonicalBasisEntry> {
    let owner = header.owner();
    let mut entries = vec![
        text_entry(
            PAGE_HEADER_DOMAIN,
            PAGE_HEADER_FIELD_KIND,
            "source.kind",
            "store-page-header",
        ),
        u64_entry(
            PAGE_HEADER_DOMAIN,
            PAGE_HEADER_FIELD_KIND,
            "header.kind.tag",
            u64::from(header.kind().tag()),
        ),
        text_entry(
            PAGE_HEADER_DOMAIN,
            PAGE_HEADER_FIELD_KIND,
            "header.kind",
            header_kind_token(header.kind()),
        ),
        u64_entry(
            PAGE_HEADER_DOMAIN,
            PAGE_HEADER_FIELD_KIND,
            "payload.offset",
            header.payload_offset() as u64,
        ),
        u64_entry(
            PAGE_HEADER_DOMAIN,
            PAGE_HEADER_FIELD_KIND,
            "payload.length",
            u64::from(header.payload_length()),
        ),
        text_entry(
            PAGE_HEADER_DOMAIN,
            PAGE_HEADER_FIELD_KIND,
            "publication",
            publication_token(header.publication()),
        ),
        text_entry(
            PAGE_HEADER_DOMAIN,
            PAGE_HEADER_FIELD_KIND,
            "owner.domain",
            owner_domain_token(owner.domain()),
        ),
        u64_entry(
            PAGE_HEADER_DOMAIN,
            PAGE_HEADER_FIELD_KIND,
            "owner.generation",
            owner.generation().get(),
        ),
        text_entry(
            PAGE_HEADER_DOMAIN,
            PHYSICAL_BOUNDARY_WITNESS_KIND,
            "boundary.authority.scope",
            authority_scope_token(boundary_witness.authority().authority_scope()),
        ),
        text_entry(
            PAGE_HEADER_DOMAIN,
            PHYSICAL_BOUNDARY_WITNESS_KIND,
            "boundary.roadmap",
            boundary_witness.authority().roadmap_scope().roadmap(),
        ),
        text_entry(
            PAGE_HEADER_DOMAIN,
            PHYSICAL_BOUNDARY_WITNESS_KIND,
            "boundary.sequence",
            boundary_witness.authority().roadmap_scope().sequence(),
        ),
    ];

    if let Some(segment) = owner.segment_id() {
        entries.push(u64_entry(
            PAGE_HEADER_DOMAIN,
            PAGE_HEADER_FIELD_KIND,
            "owner.segment",
            segment.get(),
        ));
    }
    if let Some(page) = owner.page_id() {
        entries.push(u64_entry(
            PAGE_HEADER_DOMAIN,
            PAGE_HEADER_FIELD_KIND,
            "owner.page",
            page.get(),
        ));
    }
    if let Some(extent) = owner.extent_id() {
        entries.push(u64_entry(
            PAGE_HEADER_DOMAIN,
            PAGE_HEADER_FIELD_KIND,
            "owner.extent",
            extent.get(),
        ));
    }
    if let Some(slot) = owner.slot() {
        entries.push(u64_entry(
            PAGE_HEADER_DOMAIN,
            PAGE_HEADER_FIELD_KIND,
            "owner.slot",
            u64::from(slot.get()),
        ));
    }
    if let Some(root) = owner.root_reference() {
        entries.push(u64_entry(
            PAGE_HEADER_DOMAIN,
            PAGE_HEADER_FIELD_KIND,
            "owner.root",
            root.get(),
        ));
    }

    entries
}

fn store_boundary_entries(
    domain: CanonicalBasisDomain,
    store_field_kind: CanonicalBasisEntryKind,
    foundational_locus_prefix: &'static str,
    source_kind: &'static str,
    foundational_entries: &[CanonicalBasisEntry],
    boundary_witness: StorePhysicalBoundaryWitness,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = Vec::with_capacity(foundational_entries.len() + 4);

    entries.push(text_entry(
        domain,
        store_field_kind,
        "source.kind",
        source_kind,
    ));
    entries.extend(
        foundational_entries
            .iter()
            .enumerate()
            .map(|(ordinal, entry)| {
                let locus = namespaced_locus(foundational_locus_prefix, ordinal, entry.locus());
                CanonicalBasisEntry::new(
                    domain,
                    CanonicalBasisLocus::Named(locus),
                    entry.kind(),
                    entry.value().clone(),
                )
            }),
    );
    append_boundary_witness_entries(domain, &mut entries, boundary_witness);

    entries
}

fn append_boundary_witness_entries(
    domain: CanonicalBasisDomain,
    entries: &mut Vec<CanonicalBasisEntry>,
    boundary_witness: StorePhysicalBoundaryWitness,
) {
    entries.push(text_entry(
        domain,
        PHYSICAL_BOUNDARY_WITNESS_KIND,
        "physical.boundary.authority.scope",
        authority_scope_token(boundary_witness.authority().authority_scope()),
    ));
    entries.push(text_entry(
        domain,
        PHYSICAL_BOUNDARY_WITNESS_KIND,
        "physical.boundary.roadmap",
        boundary_witness.authority().roadmap_scope().roadmap(),
    ));
    entries.push(text_entry(
        domain,
        PHYSICAL_BOUNDARY_WITNESS_KIND,
        "physical.boundary.sequence",
        boundary_witness.authority().roadmap_scope().sequence(),
    ));
}

fn namespaced_locus(
    prefix: &'static str,
    ordinal: usize,
    source_locus: &CanonicalBasisLocus,
) -> InternedString {
    format!("{prefix}.{ordinal:04}.{}", locus_token(source_locus)).into()
}

fn locus_token(locus: &CanonicalBasisLocus) -> String {
    match locus {
        CanonicalBasisLocus::Root => "root".to_owned(),
        CanonicalBasisLocus::EntryOrdinal(ordinal) => format!("ordinal.{ordinal}"),
        CanonicalBasisLocus::Aspect(aspect) => format!("aspect.{}", aspect.as_str()),
        CanonicalBasisLocus::AspectField { aspect, path } => format!(
            "aspect-field.{}.{}",
            aspect.as_str(),
            field_path_token(path)
        ),
        CanonicalBasisLocus::Named(name) => interned_token("named", name),
    }
}

fn field_path_token(path: &worth_foundational::CanonicalFieldPath) -> String {
    path.fields()
        .iter()
        .map(|field| field.as_str())
        .collect::<Vec<_>>()
        .join(".")
}

fn interned_token(prefix: &'static str, value: &InternedString) -> String {
    match value {
        InternedString::Raw(raw) => format!("{prefix}.{raw}"),
        InternedString::Symbol(symbol) => format!("{prefix}.symbol.{}", symbol.0),
    }
}

fn text_entry(
    domain: CanonicalBasisDomain,
    kind: CanonicalBasisEntryKind,
    locus: impl Into<InternedString>,
    value: impl Into<InternedString>,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.into()),
        kind,
        CanonicalBasisValue::ExactText(value.into()),
    )
}

fn u64_entry(
    domain: CanonicalBasisDomain,
    kind: CanonicalBasisEntryKind,
    locus: impl Into<InternedString>,
    value: u64,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
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
        PhysicalHeaderKind::Page(page) => match page {
            worth_store_physical_format::PhysicalPageKind::DataPage => "page.data",
            worth_store_physical_format::PhysicalPageKind::ManifestPage => "page.manifest",
        },
        PhysicalHeaderKind::Frame(frame) => match frame {
            worth_store_physical_format::PhysicalFrameKind::RecordFrame => "frame.record",
            worth_store_physical_format::PhysicalFrameKind::ExtentRecordFrame => {
                "frame.extent-record"
            }
        },
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
