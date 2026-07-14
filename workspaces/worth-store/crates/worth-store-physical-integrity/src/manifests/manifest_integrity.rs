use crate::{
    admit_root_posture, allocation_map_report, deny_derived_override, ManifestExpectedReference,
    ManifestIntegrityCounters, ManifestIntegrityDenial, ManifestIntegrityDenialKind,
    ManifestIntegrityInspectionRequest, ManifestIntegrityReport, ManifestReferenceBasis,
    SegmentManifestIntegrityReport,
};
use worth_store_physical_format::{
    ManifestDiscoveryAuthority, ManifestDiscoveryDenial, ManifestDiscoveryDenialKind,
    PhysicalGenerationAuthority, PhysicalGenerationOwner, PhysicalReferenceAuthority,
    PhysicalReferenceDenialKind, PhysicalReferenceKind, PhysicalReferenceScope,
    PhysicalReferenceValidationWitness, PhysicalRootManifest,
};

pub struct ManifestIntegrityAuthority;

impl ManifestIntegrityAuthority {
    pub const fn new() -> Self {
        Self
    }

    pub fn inspect_manifest(
        self,
        request: ManifestIntegrityInspectionRequest,
    ) -> Result<ManifestIntegrityReport, ManifestIntegrityDenial> {
        let mut counters = ManifestIntegrityCounters::start().with_manifest_sections();
        let admitted_root = admit_root_posture(&request, counters)?;
        if let Some(admission) = request.backend_residue_fallback() {
            let discovery_denial = ManifestDiscoveryAuthority::for_canonical_physical_format()
                .reject_backend_residue(admitted_root.discovery(), admission);
            return Err(backend_residue_denial(discovery_denial, counters));
        }
        for expected_reference in request.expected_references() {
            counters = counters.with_reference_probe();
            validate_expected_manifest_reference(
                admitted_root.discovery(),
                *expected_reference,
                admitted_root.posture(),
                counters,
            )?;
        }
        if let Some(attempt) = request.derived_override_attempt() {
            return Err(deny_derived_override(
                admitted_root.report(),
                attempt,
                counters,
            ));
        }

        Ok(ManifestIntegrityReport::new(
            admitted_root.report().clone(),
            segment_report(admitted_root.root()),
            allocation_map_report(admitted_root.root(), counters),
            manifest_reference_basis(
                admitted_root.root(),
                admitted_root.report().root_owner(),
                request.expected_references(),
            ),
            counters,
        ))
    }
}

fn validate_expected_manifest_reference(
    discovery: worth_store_physical_format::ManifestDiscoveryReport<'_>,
    reference: ManifestExpectedReference,
    posture: worth_store_physical_format::RootManifestIntegrityPosture,
    counters: ManifestIntegrityCounters,
) -> Result<PhysicalReferenceValidationWitness, ManifestIntegrityDenial> {
    let authority = ManifestDiscoveryAuthority::for_canonical_physical_format();
    match reference {
        ManifestExpectedReference::PageSlot(admission) => authority
            .locate_page_slot(discovery, admission)
            .map_err(|denial| manifest_reference_denial(denial, posture, counters)),
        ManifestExpectedReference::Extent(admission) => authority
            .locate_extent(discovery, admission)
            .map_err(|denial| manifest_reference_denial(denial, posture, counters)),
        ManifestExpectedReference::FreeSpaceReuse(admission) => authority
            .validate_free_space_reuse(discovery, admission)
            .map_err(|denial| manifest_reference_denial(denial, posture, counters)),
    }
}

fn manifest_reference_denial(
    denial: ManifestDiscoveryDenial,
    posture: worth_store_physical_format::RootManifestIntegrityPosture,
    counters: ManifestIntegrityCounters,
) -> ManifestIntegrityDenial {
    match denial.kind() {
        ManifestDiscoveryDenialKind::MissingSegmentManifestMembership => reference_denial(
            ManifestIntegrityDenialKind::WrongSegmentId,
            denial,
            posture,
            counters,
        ),
        ManifestDiscoveryDenialKind::MissingExtentManifestMembership => reference_denial(
            ManifestIntegrityDenialKind::MismatchedExtentId,
            denial,
            posture,
            counters,
        ),
        ManifestDiscoveryDenialKind::MissingFreeSpaceManifestMembership => reference_denial(
            ManifestIntegrityDenialKind::DamagedAllocationMap,
            denial,
            posture,
            counters,
        ),
        ManifestDiscoveryDenialKind::MissingPageSlotManifestMembership => reference_denial(
            ManifestIntegrityDenialKind::WrongSegmentId,
            denial,
            posture,
            counters,
        ),
        ManifestDiscoveryDenialKind::ReferenceValidationDenied => {
            reference_validation_denial(denial, posture, counters)
        }
        ManifestDiscoveryDenialKind::BackendResidueDiscoverySource => {
            backend_residue_denial(denial, counters)
        }
    }
}

fn reference_validation_denial(
    denial: ManifestDiscoveryDenial,
    posture: worth_store_physical_format::RootManifestIntegrityPosture,
    counters: ManifestIntegrityCounters,
) -> ManifestIntegrityDenial {
    let Some(reference_denial) = denial.reference_denial() else {
        return reference_denial(
            ManifestIntegrityDenialKind::WrongSegmentId,
            denial,
            posture,
            counters,
        );
    };
    let kind = match reference_denial.kind() {
        PhysicalReferenceDenialKind::StaleSlotGeneration
        | PhysicalReferenceDenialKind::StaleExtentGeneration
        | PhysicalReferenceDenialKind::StaleFreeSpaceReuseGeneration => {
            ManifestIntegrityDenialKind::StaleManifestGeneration
        }
        PhysicalReferenceDenialKind::PlacementMismatch => match reference_denial.reference().kind()
        {
            PhysicalReferenceKind::ExtentBacked => ManifestIntegrityDenialKind::MismatchedExtentId,
            PhysicalReferenceKind::FreeSpaceReuse => {
                ManifestIntegrityDenialKind::DamagedAllocationMap
            }
            PhysicalReferenceKind::PageSlot | PhysicalReferenceKind::RootPublication => {
                ManifestIntegrityDenialKind::WrongSegmentId
            }
        },
        PhysicalReferenceDenialKind::WrongReferenceKind => {
            ManifestIntegrityDenialKind::WrongSegmentId
        }
        PhysicalReferenceDenialKind::StaleRootPublicationGeneration => {
            ManifestIntegrityDenialKind::RootGenerationMismatch
        }
    };
    ManifestIntegrityDenial::new(kind, posture, counters)
        .with_locality(reference_denial.reference().generation_owner())
}

fn reference_denial(
    kind: ManifestIntegrityDenialKind,
    denial: ManifestDiscoveryDenial,
    posture: worth_store_physical_format::RootManifestIntegrityPosture,
    counters: ManifestIntegrityCounters,
) -> ManifestIntegrityDenial {
    let manifest_denial = ManifestIntegrityDenial::new(kind, posture, counters);
    if let Some(reference) = denial.reference() {
        return manifest_denial.with_locality(reference.generation_owner());
    }
    manifest_denial
}

fn backend_residue_denial(
    denial: ManifestDiscoveryDenial,
    counters: ManifestIntegrityCounters,
) -> ManifestIntegrityDenial {
    let manifest_denial = ManifestIntegrityDenial::new(
        ManifestIntegrityDenialKind::BackendResidueFallback,
        worth_store_physical_format::RootManifestIntegrityPosture::ResidueRootRejected,
        counters.with_backend_residue_rejection(),
    );
    if let Some(reference) = denial.reference() {
        return manifest_denial.with_locality(reference.generation_owner());
    }
    manifest_denial
}

fn manifest_reference_basis(
    root: &PhysicalRootManifest,
    root_owner: Option<PhysicalGenerationOwner>,
    expected_references: &[ManifestExpectedReference],
) -> ManifestReferenceBasis {
    ManifestReferenceBasis::new(
        root_owner,
        canonical_manifest_owners(root),
        expected_references
            .iter()
            .filter_map(|reference| scope_for_expected_reference(*reference))
            .collect(),
    )
}

fn canonical_manifest_owners(root: &PhysicalRootManifest) -> Vec<PhysicalGenerationOwner> {
    let mut owners = Vec::with_capacity(
        1 + root.segments().len()
            + root.page_slots().len()
            + root.extents().len()
            + root.free_space().len(),
    );
    owners.push(root.root_publication().owner());
    owners.extend(root.segments().iter().map(|entry| entry.segment().owner()));
    owners.extend(
        root.page_slots()
            .iter()
            .map(|entry| entry.page_slot().owner()),
    );
    owners.extend(root.extents().iter().map(|entry| entry.extent().owner()));
    owners.extend(
        root.free_space()
            .iter()
            .map(|entry| entry.reuse_cell().owner()),
    );
    owners
}

fn scope_for_expected_reference(
    reference: ManifestExpectedReference,
) -> Option<PhysicalReferenceScope> {
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let admission = reference.admission();
    match reference {
        ManifestExpectedReference::PageSlot(_) => {
            let physical_reference = admission.reference();
            let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
                .page_cell(
                    physical_reference.segment_id()?,
                    physical_reference.page_id()?,
                )
                .with_page_generation(physical_reference.generation());
            Some(PhysicalReferenceScope::manifest_page(cell))
        }
        ManifestExpectedReference::Extent(_) => {
            let physical_reference = admission.reference();
            let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
                .extent_cell(
                    physical_reference.segment_id()?,
                    physical_reference.extent_id()?,
                )
                .with_extent_generation(physical_reference.generation());
            let validation = references.validate_extent(admission, cell).ok()?;
            Some(PhysicalReferenceScope::chunk_like(validation))
        }
        ManifestExpectedReference::FreeSpaceReuse(_) => None,
    }
}

fn segment_report(root: &PhysicalRootManifest) -> SegmentManifestIntegrityReport {
    SegmentManifestIntegrityReport::new(
        root.segments().len() as u32,
        root.page_slots().len() as u32,
        root.extents().len() as u32,
    )
}
