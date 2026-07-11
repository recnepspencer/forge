use crate::{
    ManifestIntegrityCounters, ManifestIntegrityDenial, ManifestIntegrityDenialKind,
    ManifestIntegrityInspectionRequest, ManifestRootIntegrityEvidence, RootManifestIntegrityReport,
};
use forge_store_physical_format::{
    ManifestDiscoveryAuthority, ManifestDiscoveryDenial, ManifestDiscoveryDenialKind,
    ManifestDiscoveryReport, ManifestMembershipProof, PhysicalGenerationAuthority,
    PhysicalReferenceAuthority, PhysicalReferenceDenialKind, PhysicalReferenceScope,
    PhysicalRootManifest, RootManifestIntegrityPosture, RootPublicationValidationWitness,
};

pub(crate) struct AdmittedManifestRoot<'a> {
    root: &'a PhysicalRootManifest,
    discovery: ManifestDiscoveryReport<'a>,
    report: RootManifestIntegrityReport,
}

impl<'a> AdmittedManifestRoot<'a> {
    pub(crate) const fn root(&self) -> &'a PhysicalRootManifest {
        self.root
    }

    pub(crate) const fn discovery(&self) -> ManifestDiscoveryReport<'a> {
        self.discovery
    }

    pub(crate) const fn report(&self) -> &RootManifestIntegrityReport {
        &self.report
    }

    pub(crate) const fn posture(&self) -> RootManifestIntegrityPosture {
        self.report.posture()
    }
}

pub(crate) fn admit_root_posture<'a>(
    request: &'a ManifestIntegrityInspectionRequest,
    counters: ManifestIntegrityCounters,
) -> Result<AdmittedManifestRoot<'a>, ManifestIntegrityDenial> {
    match request.root_evidence() {
        ManifestRootIntegrityEvidence::RootPublication {
            root,
            root_admission,
        } => {
            let discovery = ManifestDiscoveryAuthority::for_canonical_physical_format()
                .reopen_from_root(root, *root_admission)
                .map_err(|denial| root_discovery_denial(denial, counters))?;
            let validation = PhysicalReferenceAuthority::for_canonical_physical_format()
                .validate_root_publication(*root_admission, root.root_publication())
                .map_err(|denial| {
                    ManifestIntegrityDenial::new(
                        ManifestIntegrityDenialKind::RootGenerationMismatch,
                        RootManifestIntegrityPosture::RootGenerationMismatch,
                        counters,
                    )
                    .with_locality(denial.reference().generation_owner())
                })?;
            let report = RootManifestIntegrityReport::new(current_root_posture(root, validation));
            if !report.posture().admits_scope() {
                return Err(ManifestIntegrityDenial::new(
                    ManifestIntegrityDenialKind::RootGenerationMismatch,
                    RootManifestIntegrityPosture::WrongRootPosture,
                    counters,
                ));
            }
            Ok(AdmittedManifestRoot {
                root,
                discovery,
                report,
            })
        }
        ManifestRootIntegrityEvidence::MissingRoot => Err(ManifestIntegrityDenial::new(
            ManifestIntegrityDenialKind::MissingRootPage,
            RootManifestIntegrityPosture::MissingRoot,
            counters,
        )),
        ManifestRootIntegrityEvidence::RootDamage {
            posture,
            denial,
            locality,
        } => {
            let mut manifest_denial = ManifestIntegrityDenial::new(*denial, *posture, counters);
            if let Some(locality) = locality {
                manifest_denial = manifest_denial.with_locality(*locality);
            }
            Err(manifest_denial)
        }
        ManifestRootIntegrityEvidence::MultipleValidRoots { second, .. } => {
            Err(ManifestIntegrityDenial::new(
                ManifestIntegrityDenialKind::MultipleValidRoots,
                RootManifestIntegrityPosture::MultipleValidRoots,
                counters,
            )
            .with_locality(second.root_publication().owner()))
        }
    }
}

pub(crate) fn current_root_posture(
    root: &PhysicalRootManifest,
    root_validation: RootPublicationValidationWitness,
) -> RootManifestIntegrityPosture {
    if let Some(posture) = posture_from_first_page_membership(root) {
        return posture;
    }
    if let Some(posture) = posture_from_first_extent_membership(root) {
        return posture;
    }
    RootManifestIntegrityPosture::current_root_publication(root_validation)
}

fn posture_from_first_page_membership(
    root: &PhysicalRootManifest,
) -> Option<RootManifestIntegrityPosture> {
    let slot = root.page_slots().first()?.page_slot();
    let owner = PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(slot.segment_id(), slot.page_id())
        .with_page_generation(slot.generation());
    let scope = PhysicalReferenceScope::manifest_page(owner);
    ManifestMembershipProof::from_root(root, scope)
        .ok()
        .map(RootManifestIntegrityPosture::current_root_admitted)
}

fn posture_from_first_extent_membership(
    root: &PhysicalRootManifest,
) -> Option<RootManifestIntegrityPosture> {
    let extent = root.extents().first()?.extent();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let validation = references
        .validate_extent(references.admit_extent(extent), extent)
        .ok()?;
    let scope = PhysicalReferenceScope::chunk_like(validation);
    ManifestMembershipProof::from_root(root, scope)
        .ok()
        .map(RootManifestIntegrityPosture::current_root_admitted)
}

fn root_discovery_denial(
    denial: ManifestDiscoveryDenial,
    counters: ManifestIntegrityCounters,
) -> ManifestIntegrityDenial {
    if denial.kind() == ManifestDiscoveryDenialKind::ReferenceValidationDenied {
        let Some(reference_denial) = denial.reference_denial() else {
            return root_generation_mismatch(counters);
        };
        return match reference_denial.kind() {
            PhysicalReferenceDenialKind::StaleRootPublicationGeneration => {
                root_generation_mismatch(counters)
                    .with_locality(reference_denial.reference().generation_owner())
            }
            _ => root_generation_mismatch(counters)
                .with_locality(reference_denial.reference().generation_owner()),
        };
    }
    root_generation_mismatch(counters)
}

fn root_generation_mismatch(counters: ManifestIntegrityCounters) -> ManifestIntegrityDenial {
    ManifestIntegrityDenial::new(
        ManifestIntegrityDenialKind::RootGenerationMismatch,
        RootManifestIntegrityPosture::RootGenerationMismatch,
        counters,
    )
}
