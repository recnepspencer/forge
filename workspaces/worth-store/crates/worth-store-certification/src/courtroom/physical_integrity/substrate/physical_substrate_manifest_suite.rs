use crate::{
    PhysicalManifestDiscoveryEvidenceReport, PhysicalManifestDiscoveryEvidenceRow,
    PhysicalSubstrateCertificationDenial,
};
use worth_store_physical_format::{
    AllocationClassKind, ManifestDiscoveryAuthority, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalRootReference, PhysicalSegmentId,
};

pub(crate) fn manifest_reports(
) -> Result<Vec<PhysicalManifestDiscoveryEvidenceReport>, PhysicalSubstrateCertificationDenial> {
    let (root, cells) = manifest_fixture()?;
    let discovery = ManifestDiscoveryAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let report = discovery
        .reopen_from_root(&root, references.admit_root_publication(cells.root))
        .map_err(|_| PhysicalSubstrateCertificationDenial::ManifestEvidenceRejected)?;
    let backend_residue =
        discovery.reject_backend_residue(report, references.admit_page_slot(cells.residue_slot));
    let stale_free = discovery
        .validate_free_space_reuse(
            report,
            references.admit_free_space_reuse(cells.old_free_space),
        )
        .err()
        .ok_or(PhysicalSubstrateCertificationDenial::ManifestEvidenceRejected)?;
    let stale_root = discovery
        .reopen_from_root(&root, references.admit_root_publication(cells.old_root))
        .err()
        .ok_or(PhysicalSubstrateCertificationDenial::ManifestEvidenceRejected)?;
    Ok(vec![
        PhysicalManifestDiscoveryEvidenceReport::from_manifest_report(
            PhysicalManifestDiscoveryEvidenceRow::RootManifestDiscovery,
            report,
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::ManifestEvidenceRejected)?,
        PhysicalManifestDiscoveryEvidenceReport::from_manifest_denial(
            PhysicalManifestDiscoveryEvidenceRow::BackendResidueRejected,
            backend_residue,
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::ManifestEvidenceRejected)?,
        PhysicalManifestDiscoveryEvidenceReport::from_manifest_denial(
            PhysicalManifestDiscoveryEvidenceRow::FreeSpaceReuseGenerationChanged,
            stale_free,
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::ManifestEvidenceRejected)?,
        PhysicalManifestDiscoveryEvidenceReport::from_manifest_denial(
            PhysicalManifestDiscoveryEvidenceRow::RootPublicationGenerationChanged,
            stale_root,
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::ManifestEvidenceRejected)?,
    ])
}

fn manifest_fixture() -> Result<
    (
        worth_store_physical_format::PhysicalRootManifest,
        ManifestFixtureCells,
    ),
    PhysicalSubstrateCertificationDenial,
> {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let root = generations
        .root_publication_cell(root_ref(1)?)
        .with_root_publication_generation(generation(5)?);
    let old_root = generations
        .root_publication_cell(root_ref(1)?)
        .with_root_publication_generation(generation(4)?);
    let current_free_space = generations
        .free_space_slot_cell(
            segment(7)?,
            page(3)?,
            slot(1)?,
            AllocationClassKind::OrdinaryRecordPage,
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::PhysicalCellRejected)?
        .with_free_space_generation(generation(3)?);
    let old_free_space = generations
        .free_space_slot_cell(
            segment(7)?,
            page(3)?,
            slot(1)?,
            AllocationClassKind::OrdinaryRecordPage,
        )
        .map_err(|_| PhysicalSubstrateCertificationDenial::PhysicalCellRejected)?
        .with_free_space_generation(generation(2)?);
    let residue_slot = generations
        .slot_cell(segment(7)?, page(4)?, slot(1)?)
        .with_slot_generation(generation(3)?);
    let manifest = worth_store_physical_format::PhysicalManifestUniverseBuilder::for_canonical_physical_format(root)
        .segment(
            generations
                .segment_cell(segment(7)?)
                .with_segment_generation(generation(1)?),
        )
        .free_space_reuse(current_free_space)
        .publish();
    Ok((
        manifest,
        ManifestFixtureCells {
            root,
            old_root,
            old_free_space,
            residue_slot,
        },
    ))
}

#[derive(Debug, Clone, Copy)]
struct ManifestFixtureCells {
    root: worth_store_physical_format::RootPublicationCell,
    old_root: worth_store_physical_format::RootPublicationCell,
    old_free_space: worth_store_physical_format::FreeSpaceReuseCell,
    residue_slot: worth_store_physical_format::SlotGenerationCell,
}

fn segment(value: u64) -> Result<PhysicalSegmentId, PhysicalSubstrateCertificationDenial> {
    PhysicalSegmentId::from_raw(value)
        .map_err(|_| PhysicalSubstrateCertificationDenial::PhysicalIdentifierRejected)
}

fn page(value: u64) -> Result<PhysicalPageId, PhysicalSubstrateCertificationDenial> {
    PhysicalPageId::from_raw(value)
        .map_err(|_| PhysicalSubstrateCertificationDenial::PhysicalIdentifierRejected)
}

fn slot(value: u16) -> Result<PhysicalRecordSlot, PhysicalSubstrateCertificationDenial> {
    PhysicalRecordSlot::from_raw(value)
        .map_err(|_| PhysicalSubstrateCertificationDenial::PhysicalIdentifierRejected)
}

fn root_ref(value: u64) -> Result<PhysicalRootReference, PhysicalSubstrateCertificationDenial> {
    PhysicalRootReference::from_raw(value)
        .map_err(|_| PhysicalSubstrateCertificationDenial::PhysicalIdentifierRejected)
}

fn generation(value: u64) -> Result<PhysicalGeneration, PhysicalSubstrateCertificationDenial> {
    PhysicalGeneration::from_raw(value)
        .map_err(|_| PhysicalSubstrateCertificationDenial::PhysicalIdentifierRejected)
}
