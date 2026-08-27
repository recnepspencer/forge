impl super::WorthUiNativeApplicationShell {
    pub(crate) fn reconstruct_current_presentation(
        &mut self,
        deadline_tick: u64,
        now_tick: u64,
    ) -> Result<crate::mounting::UiMountedFrameOutcome, ()> {
        let reconstructed_layouts = self
            .session
            .mounted
            .reconstruct_current_layouts()
            .map_err(|_| ())?;
        if reconstructed_layouts > 0 {
            let observed = self.runtime_derived_state_reconstruction.ok_or(())?;
            self.runtime_derived_state_reconstruction = Some(
                worth_ui_host_native::UiNativeClientDerivedStateReconstructionObservation::reported(
                    observed.class(),
                    observed.loss_count(),
                    observed.reconstruction_count().saturating_add(1),
                    observed.derived_items_lost(),
                    observed
                        .derived_items_reconstructed()
                        .saturating_add(u64::try_from(reconstructed_layouts).unwrap_or(u64::MAX)),
                ),
            );
        }
        let affected = self
            .session
            .mounted
            .current_publication()
            .and_then(|publication| publication.bindings().first().copied())
            .ok_or(())?;
        if affected == self.binding {
            self.rebind_native_surface_scale(self.scale_factor_milli)
                .map_err(|()| ())?;
        }
        self.settle_pending_native_viewport_measurements()
            .map_err(|_| ())?;
        let request = self.session.mounted_frame_request();
        let replacement = self.pending_surface_reconciliation.unwrap_or_else(|| {
            crate::mounting::UiMountedSurfaceReconciliationBinding::new(affected, self.binding)
        });
        let replacements = [replacement];
        let frame = self
            .session
            .prepare_mounted_reconstruction_frame_with_application_presentation(
                request,
                &replacements,
                |_| {},
            )
            .map_err(|_| ())?;
        let outcome = self
            .session
            .present_prepared_mounted_frame_for_reconciliation(
                frame,
                &replacements,
                worth_ui_host_contract::UiPresentationDeadline::at_tick(deadline_tick),
                now_tick,
            )
            .map_err(|_| ())?;
        let reconstructed_rasters = self.session.mounted.take_reconstructed_raster_cache_items();
        if reconstructed_rasters > 0 {
            self.record_runtime_derived_state_reconstruction(reconstructed_rasters)?;
        }
        self.settle_surface_reconciliation(&outcome);
        Ok(outcome)
    }

    pub(crate) fn require_current_layout_reconstruction(&mut self) -> Result<usize, ()> {
        if self.runtime_derived_state_reconstruction.is_some() {
            return Err(());
        }
        let lost = self
            .session
            .mounted
            .require_current_layout_reconstruction(self.binding)
            .map_err(|_| ())?;
        self.runtime_derived_state_reconstruction = Some(
            worth_ui_host_native::UiNativeClientDerivedStateReconstructionObservation::reported(
                worth_ui_host_native::UiNativeClientDerivedStateLossClass::MountedLayouts,
                1,
                0,
                u64::try_from(lost).unwrap_or(u64::MAX),
                0,
            ),
        );
        Ok(lost)
    }

    pub(crate) fn require_current_raster_cache_reconstruction(&mut self) -> Result<usize, ()> {
        if self.runtime_derived_state_reconstruction.is_some() {
            return Err(());
        }
        let lost = self
            .session
            .mounted
            .require_raster_cache_reconstruction(self.binding)
            .map_err(|_| ())?;
        self.runtime_derived_state_reconstruction = Some(
            worth_ui_host_native::UiNativeClientDerivedStateReconstructionObservation::reported(
                worth_ui_host_native::UiNativeClientDerivedStateLossClass::RasterCache,
                1,
                0,
                u64::try_from(lost).unwrap_or(u64::MAX),
                0,
            ),
        );
        Ok(lost)
    }

    fn record_runtime_derived_state_reconstruction(
        &mut self,
        reconstructed: usize,
    ) -> Result<(), ()> {
        let observed = self.runtime_derived_state_reconstruction.ok_or(())?;
        self.runtime_derived_state_reconstruction = Some(
            worth_ui_host_native::UiNativeClientDerivedStateReconstructionObservation::reported(
                observed.class(),
                observed.loss_count(),
                observed.reconstruction_count().saturating_add(1),
                observed.derived_items_lost(),
                observed
                    .derived_items_reconstructed()
                    .saturating_add(u64::try_from(reconstructed).unwrap_or(u64::MAX)),
            ),
        );
        Ok(())
    }

    pub(crate) const fn runtime_derived_state_reconstruction(
        &self,
    ) -> Option<worth_ui_host_native::UiNativeClientDerivedStateReconstructionObservation> {
        self.runtime_derived_state_reconstruction
    }
}
