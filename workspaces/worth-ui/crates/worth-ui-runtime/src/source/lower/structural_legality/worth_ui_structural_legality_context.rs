use crate::capability::CapabilitySupportId;
use crate::capability::{
    AdmittedCapability, CapabilitySnapshot, CapabilitySupportPosture,
    MosaicPlacementPolicyDescriptor, MosaicPlacementPolicyId, MosaicRegionKindDescriptor,
    MosaicRegionKindId, MosaicSizingContractDescriptor, MosaicSizingContractId,
    MosaicStateSlotDescriptor, MosaicStateSlotId, SupportRequirement, SurfaceDescriptor, SurfaceId,
};
use crate::source::{
    WorthUiArtifactInputProvenance, WorthUiSourceModuleId, WorthUiStructuralLegalityDiagnostic,
    WorthUiStructuralLegalityDiagnosticCode, WorthUiStructuralLegalityMetrics,
};

pub(crate) struct WorthUiStructuralLegalityContext<'snapshot> {
    snapshot: &'snapshot CapabilitySnapshot,
    metrics: WorthUiStructuralLegalityMetrics,
}

pub(crate) type RegionResolution = (
    AdmittedCapability<MosaicRegionKindId>,
    MosaicRegionKindDescriptor,
);
pub(crate) type PlacementResolution = (
    AdmittedCapability<MosaicPlacementPolicyId>,
    MosaicPlacementPolicyDescriptor,
);
pub(crate) type SizingResolution = (
    AdmittedCapability<MosaicSizingContractId>,
    MosaicSizingContractDescriptor,
);
pub(crate) type StateResolution = (
    AdmittedCapability<MosaicStateSlotId>,
    MosaicStateSlotDescriptor,
);
pub(crate) type SurfaceResolution = (AdmittedCapability<SurfaceId>, SurfaceDescriptor);

const REGION_REFERENCE_CODES: [WorthUiStructuralLegalityDiagnosticCode; 4] = [
    WorthUiStructuralLegalityDiagnosticCode::MissingMosaicRegionReference,
    WorthUiStructuralLegalityDiagnosticCode::DeferredMosaicRegionReference,
    WorthUiStructuralLegalityDiagnosticCode::UnsupportedMosaicRegionReference,
    WorthUiStructuralLegalityDiagnosticCode::PlatformInternalMosaicRegionReference,
];
const PLACEMENT_REFERENCE_CODES: [WorthUiStructuralLegalityDiagnosticCode; 4] = [
    WorthUiStructuralLegalityDiagnosticCode::MissingMosaicPlacementPolicyReference,
    WorthUiStructuralLegalityDiagnosticCode::DeferredMosaicPlacementPolicyReference,
    WorthUiStructuralLegalityDiagnosticCode::UnsupportedMosaicPlacementPolicyReference,
    WorthUiStructuralLegalityDiagnosticCode::PlatformInternalMosaicPlacementPolicyReference,
];
const SIZING_REFERENCE_CODES: [WorthUiStructuralLegalityDiagnosticCode; 4] = [
    WorthUiStructuralLegalityDiagnosticCode::MissingMosaicSizingContractReference,
    WorthUiStructuralLegalityDiagnosticCode::DeferredMosaicSizingContractReference,
    WorthUiStructuralLegalityDiagnosticCode::UnsupportedMosaicSizingContractReference,
    WorthUiStructuralLegalityDiagnosticCode::PlatformInternalMosaicSizingContractReference,
];
const STATE_SLOT_REFERENCE_CODES: [WorthUiStructuralLegalityDiagnosticCode; 4] = [
    WorthUiStructuralLegalityDiagnosticCode::MissingMosaicStateSlotReference,
    WorthUiStructuralLegalityDiagnosticCode::DeferredMosaicStateSlotReference,
    WorthUiStructuralLegalityDiagnosticCode::UnsupportedMosaicStateSlotReference,
    WorthUiStructuralLegalityDiagnosticCode::PlatformInternalMosaicStateSlotReference,
];
const SURFACE_REFERENCE_CODES: [WorthUiStructuralLegalityDiagnosticCode; 4] = [
    WorthUiStructuralLegalityDiagnosticCode::MissingStructuralSurfaceReference,
    WorthUiStructuralLegalityDiagnosticCode::DeferredStructuralSurfaceReference,
    WorthUiStructuralLegalityDiagnosticCode::UnsupportedStructuralSurfaceReference,
    WorthUiStructuralLegalityDiagnosticCode::PlatformInternalStructuralSurfaceReference,
];

impl<'snapshot> WorthUiStructuralLegalityContext<'snapshot> {
    pub(crate) fn new(snapshot: &'snapshot CapabilitySnapshot) -> Self {
        Self {
            snapshot,
            metrics: WorthUiStructuralLegalityMetrics::default(),
        }
    }

    pub(crate) fn finish_metrics(self) -> WorthUiStructuralLegalityMetrics {
        self.metrics
    }

    pub(crate) fn resolve_region(
        &mut self,
        module_id: &WorthUiSourceModuleId,
        authored_text: &str,
        structural_locus: &str,
        provenance: &WorthUiArtifactInputProvenance,
    ) -> Result<RegionResolution, WorthUiStructuralLegalityDiagnostic> {
        let id = MosaicRegionKindId::new(authored_text).map_err(|_| {
            diagnostic(
                WorthUiStructuralLegalityDiagnosticCode::InvalidMosaicRegionReferenceId,
                module_id,
                authored_text,
                structural_locus,
                provenance,
            )
        })?;
        let lookup = self.snapshot.index().mosaic_regions().lookup(&id);
        self.metrics.record_lookup(lookup.counters());
        if let Some(descriptor) = lookup.into_value() {
            let posture = self
                .snapshot
                .support_catalog()
                .mosaic_region_posture(&id)
                .expect("support catalog should contain mosaic region ids");
            if posture.is_admitted() {
                let admitted = SupportRequirement::admitted()
                    .check(posture)
                    .expect("admitted mosaic region should satisfy admitted support requirement");
                return Ok((admitted, descriptor.clone()));
            }
            return Err(postured_diagnostic(
                Some(posture),
                module_id,
                authored_text,
                structural_locus,
                provenance,
                REGION_REFERENCE_CODES,
            ));
        }
        Err(postured_diagnostic(
            self.snapshot.support_catalog().mosaic_region_posture(&id),
            module_id,
            authored_text,
            structural_locus,
            provenance,
            REGION_REFERENCE_CODES,
        ))
    }

    pub(crate) fn resolve_placement(
        &mut self,
        module_id: &WorthUiSourceModuleId,
        authored_text: &str,
        structural_locus: &str,
        provenance: &WorthUiArtifactInputProvenance,
    ) -> Result<PlacementResolution, WorthUiStructuralLegalityDiagnostic> {
        let id = MosaicPlacementPolicyId::new(authored_text).map_err(|_| {
            diagnostic(
                WorthUiStructuralLegalityDiagnosticCode::InvalidMosaicPlacementPolicyReferenceId,
                module_id,
                authored_text,
                structural_locus,
                provenance,
            )
        })?;
        let lookup = self
            .snapshot
            .index()
            .mosaic_placement_policies()
            .lookup(&id);
        self.metrics.record_lookup(lookup.counters());
        if let Some(descriptor) = lookup.into_value() {
            let posture = self
                .snapshot
                .support_catalog()
                .mosaic_placement_posture(&id)
                .expect("support catalog should contain mosaic placement ids");
            if posture.is_admitted() {
                let admitted = SupportRequirement::admitted().check(posture).expect(
                    "admitted mosaic placement should satisfy admitted support requirement",
                );
                return Ok((admitted, descriptor.clone()));
            }
            return Err(postured_diagnostic(
                Some(posture),
                module_id,
                authored_text,
                structural_locus,
                provenance,
                PLACEMENT_REFERENCE_CODES,
            ));
        }
        Err(postured_diagnostic(
            self.snapshot
                .support_catalog()
                .mosaic_placement_posture(&id),
            module_id,
            authored_text,
            structural_locus,
            provenance,
            PLACEMENT_REFERENCE_CODES,
        ))
    }

    pub(crate) fn resolve_sizing(
        &mut self,
        module_id: &WorthUiSourceModuleId,
        authored_text: &str,
        structural_locus: &str,
        provenance: &WorthUiArtifactInputProvenance,
    ) -> Result<SizingResolution, WorthUiStructuralLegalityDiagnostic> {
        let id = MosaicSizingContractId::new(authored_text).map_err(|_| {
            diagnostic(
                WorthUiStructuralLegalityDiagnosticCode::InvalidMosaicSizingContractReferenceId,
                module_id,
                authored_text,
                structural_locus,
                provenance,
            )
        })?;
        let lookup = self.snapshot.index().mosaic_sizing_contracts().lookup(&id);
        self.metrics.record_lookup(lookup.counters());
        if let Some(descriptor) = lookup.into_value() {
            let posture = self
                .snapshot
                .support_catalog()
                .mosaic_sizing_posture(&id)
                .expect("support catalog should contain mosaic sizing ids");
            if posture.is_admitted() {
                let admitted = SupportRequirement::admitted()
                    .check(posture)
                    .expect("admitted mosaic sizing should satisfy admitted support requirement");
                return Ok((admitted, descriptor.clone()));
            }
            return Err(postured_diagnostic(
                Some(posture),
                module_id,
                authored_text,
                structural_locus,
                provenance,
                SIZING_REFERENCE_CODES,
            ));
        }
        Err(postured_diagnostic(
            self.snapshot.support_catalog().mosaic_sizing_posture(&id),
            module_id,
            authored_text,
            structural_locus,
            provenance,
            SIZING_REFERENCE_CODES,
        ))
    }

    pub(crate) fn resolve_state_slot(
        &mut self,
        module_id: &WorthUiSourceModuleId,
        authored_text: &str,
        structural_locus: &str,
        provenance: &WorthUiArtifactInputProvenance,
    ) -> Result<StateResolution, WorthUiStructuralLegalityDiagnostic> {
        let id = MosaicStateSlotId::new(authored_text).map_err(|_| {
            diagnostic(
                WorthUiStructuralLegalityDiagnosticCode::InvalidMosaicStateSlotReferenceId,
                module_id,
                authored_text,
                structural_locus,
                provenance,
            )
        })?;
        let lookup = self.snapshot.index().mosaic_state_slots().lookup(&id);
        self.metrics.record_lookup(lookup.counters());
        if let Some(descriptor) = lookup.into_value() {
            let posture = self
                .snapshot
                .support_catalog()
                .mosaic_state_slot_posture(&id)
                .expect("support catalog should contain mosaic state ids");
            if posture.is_admitted() {
                let admitted = SupportRequirement::admitted()
                    .check(posture)
                    .expect("admitted mosaic state should satisfy admitted support requirement");
                return Ok((admitted, descriptor.clone()));
            }
            return Err(postured_diagnostic(
                Some(posture),
                module_id,
                authored_text,
                structural_locus,
                provenance,
                STATE_SLOT_REFERENCE_CODES,
            ));
        }
        Err(postured_diagnostic(
            self.snapshot
                .support_catalog()
                .mosaic_state_slot_posture(&id),
            module_id,
            authored_text,
            structural_locus,
            provenance,
            STATE_SLOT_REFERENCE_CODES,
        ))
    }

    pub(crate) fn resolve_surface(
        &mut self,
        module_id: &WorthUiSourceModuleId,
        authored_text: &str,
        structural_locus: &str,
        provenance: &WorthUiArtifactInputProvenance,
    ) -> Result<SurfaceResolution, WorthUiStructuralLegalityDiagnostic> {
        let id = SurfaceId::new(authored_text).map_err(|_| {
            diagnostic(
                WorthUiStructuralLegalityDiagnosticCode::InvalidStructuralSurfaceReferenceId,
                module_id,
                authored_text,
                structural_locus,
                provenance,
            )
        })?;
        let lookup = self.snapshot.index().surfaces().lookup(&id);
        self.metrics.record_lookup(lookup.counters());
        if let Some(descriptor) = lookup.into_value() {
            let posture = self
                .snapshot
                .support_catalog()
                .surface_posture(&id)
                .expect("support catalog should contain surface ids");
            if posture.is_admitted() {
                let admitted = SupportRequirement::admitted()
                    .check(posture)
                    .expect("admitted surface should satisfy admitted support requirement");
                return Ok((admitted, descriptor.clone()));
            }
            return Err(postured_diagnostic(
                Some(posture),
                module_id,
                authored_text,
                structural_locus,
                provenance,
                SURFACE_REFERENCE_CODES,
            ));
        }
        Err(postured_diagnostic(
            self.snapshot.support_catalog().surface_posture(&id),
            module_id,
            authored_text,
            structural_locus,
            provenance,
            SURFACE_REFERENCE_CODES,
        ))
    }
}

fn postured_diagnostic<T: CapabilitySupportId>(
    posture: Option<CapabilitySupportPosture<T>>,
    module_id: &WorthUiSourceModuleId,
    authored_text: &str,
    structural_locus: &str,
    provenance: &WorthUiArtifactInputProvenance,
    diagnostic_codes: [WorthUiStructuralLegalityDiagnosticCode; 4],
) -> WorthUiStructuralLegalityDiagnostic {
    let [missing_code, deferred_code, unsupported_code, platform_internal_code] = diagnostic_codes;
    diagnostic(
        match posture {
            Some(posture) if posture.is_deferred() => deferred_code,
            Some(posture) if posture.is_unsupported() => unsupported_code,
            Some(posture) if posture.is_platform_internal() => platform_internal_code,
            _ => missing_code,
        },
        module_id,
        authored_text,
        structural_locus,
        provenance,
    )
}

fn diagnostic(
    code: WorthUiStructuralLegalityDiagnosticCode,
    module_id: &WorthUiSourceModuleId,
    authored_text: &str,
    structural_locus: &str,
    provenance: &WorthUiArtifactInputProvenance,
) -> WorthUiStructuralLegalityDiagnostic {
    WorthUiStructuralLegalityDiagnostic::new(
        code,
        module_id.clone(),
        authored_text,
        structural_locus,
        provenance.clone(),
    )
}
