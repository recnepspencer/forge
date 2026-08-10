use worth_ui_host_contract::{
    UiMountedFrameCanonicalCore, UiMountedFrameIntegrity, UiMountedFrameManifest,
    UiMountedSurfaceBindingRequirement, UiSemanticSurfaceIdentity,
};

use super::{
    UiMountedFramePreparationDenial, UiMountedFrameReceipt, UiMountedSurfaceReceipt,
    UiPreparedMountedFrame, UiPreparedMountedFrameAdmission,
};

impl UiPreparedMountedFrame {
    pub(crate) fn admit(
        admission: UiPreparedMountedFrameAdmission,
    ) -> Result<Self, UiMountedFramePreparationDenial> {
        let UiPreparedMountedFrameAdmission {
            candidate,
            generation,
            manifest,
            graph_world,
            allocation_truth_revision,
            trace_source,
            reuse_contract,
        } = admission;
        if trace_source.generation() != &generation {
            return Err(UiMountedFramePreparationDenial::TraceSourceGenerationMismatch);
        }
        crate::mounting::validate_manifest(&manifest)?;
        let surfaces = manifest
            .surfaces()
            .iter()
            .map(|requirement| {
                let projection = candidate
                    .frame()
                    .view_for(requirement.binding())
                    .expect("validated manifest binding is present in finalized projection");
                Ok(UiMountedSurfaceReceipt {
                    requirement: *requirement,
                    projection,
                })
            })
            .collect::<Result<Vec<_>, UiMountedFramePreparationDenial>>()?;
        let canonical_core = UiMountedFrameCanonicalCore::new(
            candidate.frame().frame_identity(),
            candidate.frame().plan_digest(),
            graph_world,
            allocation_truth_revision,
            table_range_digest(&surfaces),
        );
        let integrity = UiMountedFrameIntegrity::derive(canonical_core, &manifest);
        if !integrity.verifies(canonical_core, &manifest) {
            return Err(UiMountedFramePreparationDenial::IntegrityMismatch);
        }
        let cost = candidate.frame().cost_report();
        let identity_trace_basis = candidate.frame().identity_trace_basis(trace_source);
        Ok(Self {
            candidate,
            generation,
            manifest,
            canonical_core,
            integrity,
            surfaces: surfaces.into_boxed_slice(),
            identity_trace_basis,
            cost,
            reuse_contract,
        })
    }

    pub fn generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.generation
    }

    pub fn manifest(&self) -> &UiMountedFrameManifest {
        &self.manifest
    }

    pub fn canonical_core(&self) -> UiMountedFrameCanonicalCore {
        self.canonical_core
    }

    pub fn integrity(&self) -> UiMountedFrameIntegrity {
        self.integrity
    }

    pub fn surfaces(&self) -> &[UiMountedSurfaceReceipt] {
        &self.surfaces
    }

    pub(in crate::mounting) fn presentation_changed_instances(
        &self,
    ) -> &[worth_ui_host_contract::UiMountedInstanceIdentity] {
        self.candidate.presentation_changed_instances()
    }

    pub(in crate::mounting) fn presentation_surface_changed(
        &self,
        surface: UiSemanticSurfaceIdentity,
    ) -> bool {
        self.candidate.presentation_surface_changed(surface)
    }

    pub fn receipt(&self) -> UiMountedFrameReceipt {
        UiMountedFrameReceipt {
            canonical_core: self.canonical_core,
            integrity: self.integrity,
            surface_count: self.surfaces.len(),
            cost: self.cost,
        }
    }

    pub fn cost_report(&self) -> crate::mounting::UiMountCostReport {
        self.cost
    }

    pub fn reuse_contract(&self) -> &crate::mounting::UiMountedFrameReuseContract {
        &self.reuse_contract
    }

    pub(crate) fn visual_region_basis(&self) -> crate::mounting::UiMountedVisualRegionBasis {
        crate::mounting::UiMountedVisualRegionBasis::new(
            self.candidate.frame().static_paint_rows(),
            self.candidate.frame().hit_test_rows(),
        )
    }

    pub(crate) fn identity_trace_basis(&self) -> &crate::mounting::UiMountedIdentityTraceBasis {
        &self.identity_trace_basis
    }

    pub fn is_unpublished(&self) -> bool {
        self.candidate.is_unpublished()
    }

    pub(crate) fn presented_receipt_basis(&self) -> &crate::mounting::UiMountedNodeReceiptBasis {
        self.candidate.presented_receipt_basis()
    }

    pub(crate) fn into_publication_parts(
        self,
    ) -> (
        crate::mounting::UiProjectedMountedFrameCandidate,
        UiMountedFrameManifest,
        UiMountedFrameCanonicalCore,
        crate::mounting::UiMountedFrameReuseContract,
    ) {
        (
            self.candidate,
            self.manifest,
            self.canonical_core,
            self.reuse_contract,
        )
    }
}

fn table_range_digest(surfaces: &[UiMountedSurfaceReceipt]) -> u64 {
    surfaces.iter().fold(0_u64, |digest, receipt| {
        let view = receipt.projection();
        let table_digest = [
            view.nodes().len(),
            view.clips().rows().len(),
            view.layers().rows().len(),
            view.hit_tests().rows().len(),
            view.paint_batches().rows().len(),
            view.spatial_batches().rows().len(),
            view.realtime_batches().rows().len(),
            view.resources().entries().len(),
        ]
        .into_iter()
        .fold(
            digest ^ view.binding().diagnostic_value(),
            |value, length| value.rotate_left(7) ^ u64::try_from(length).unwrap_or(u64::MAX),
        );
        table_digest.rotate_left(11)
            ^ view.filled_rects().rows().iter().fold(0_u64, |value, row| {
                value.rotate_left(9) ^ row.semantic_digest()
            })
            ^ view.hit_tests().rows().iter().fold(0_u64, |value, row| {
                value.rotate_left(9) ^ row.semantic_digest()
            })
    })
}

pub(crate) fn binding_requirement(
    binding: crate::mounting::UiSurfaceBindingIdentityView,
) -> UiMountedSurfaceBindingRequirement {
    UiMountedSurfaceBindingRequirement::with_baseline(
        binding.semantic_surface_identity(),
        binding.host_surface_identity(),
        binding.binding_generation(),
        binding.capability_observation_generation(),
        binding.capability_profile_digest(),
        binding.presentation_mode(),
        binding.baseline(),
    )
}
