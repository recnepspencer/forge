use worth_ui_host_contract::{
    UiGlyphRasterTransactionOutcome, UiGlyphRasterTransactionPending,
    UiGlyphRasterTransactionReceipt,
};

use super::UiNativeHostState;
use crate::native::physical_work_signal::{
    UiNativePhysicalSignalSettlement, UiNativePhysicalSignalStatus,
};
use crate::native::text_atlas::{
    UiNativeTextAtlasCensus, UiNativeTextAtlasInFlight, UiNativeTextAtlasPhysicalPosture,
};

impl UiNativeHostState {
    pub(crate) fn text_atlas_census(&self) -> UiNativeTextAtlasCensus {
        let physical = self
            .text_atlas_gpu
            .as_ref()
            .map(|gpu| UiNativeTextAtlasPhysicalPosture {
                alpha_pages: gpu.page_count(crate::native::text_atlas::UiNativeGpuAtlasKind::Alpha),
                color_pages: gpu.page_count(crate::native::text_atlas::UiNativeGpuAtlasKind::Color),
                staging_buffers: self.resources.current().atlas_staging_buffers,
                upload_submissions: gpu.pending_count(),
                in_flight_transactions: usize::from(self.text_atlas_in_flight.is_some()),
            })
            .unwrap_or_else(|| UiNativeTextAtlasPhysicalPosture {
                in_flight_transactions: usize::from(self.text_atlas_in_flight.is_some()),
                ..UiNativeTextAtlasPhysicalPosture::default()
            });
        UiNativeTextAtlasCensus::from_snapshot_with_posture(
            self.text_atlas.snapshot(),
            usize::from(self.text_atlas_recovery.is_some()),
            physical,
        )
    }

    pub(crate) fn progress_text_atlas_physical(
        &mut self,
        pending: UiGlyphRasterTransactionPending,
    ) -> bool {
        let Some(in_flight) = self.text_atlas_in_flight.as_ref() else {
            return false;
        };
        if in_flight.pending() != pending {
            return false;
        }
        let token = match self.physical_signal.take_ready_atlas_upload(pending) {
            Ok(token) => token,
            Err(()) => return false,
        };
        if !self
            .text_atlas_in_flight
            .as_mut()
            .is_some_and(|in_flight| in_flight.refresh_signal_token(token))
        {
            return false;
        }
        if self.physical_signal.token_uses_recovery(token)
            && self
                .text_atlas_in_flight
                .as_ref()
                .is_some_and(|in_flight| !in_flight.awaits_recovery())
        {
            let _ = self.quarantine_text_atlas_in_flight(pending, token);
        }
        let Some(observation) = self.poll_gpu_observation(pending) else {
            return false;
        };
        match self.physical_signal.reconcile(observation) {
            UiNativePhysicalSignalSettlement::Pending => {}
            UiNativePhysicalSignalSettlement::Completed => {
                let outcome = if self
                    .text_atlas_in_flight
                    .as_ref()
                    .is_some_and(UiNativeTextAtlasInFlight::awaits_recovery)
                {
                    if self.resolve_text_atlas_recovery(pending) {
                        recovery_outcome(pending)
                    } else {
                        stale_plan()
                    }
                } else {
                    self.commit_text_atlas_in_flight()
                };
                self.text_atlas_completion = Some((pending, outcome));
            }
            UiNativePhysicalSignalSettlement::Indeterminate => {
                let outcome = self.transition_pending_text_atlas_to_recovery(pending);
                self.text_atlas_completion = Some((pending, outcome));
            }
            UiNativePhysicalSignalSettlement::Rejected
            | UiNativePhysicalSignalSettlement::Stale => {}
        }
        true
    }

    pub(crate) fn complete_pending_text_atlas(
        &mut self,
        pending: UiGlyphRasterTransactionPending,
    ) -> UiGlyphRasterTransactionOutcome {
        if let Some(outcome) = self.take_text_atlas_completion(pending) {
            return outcome;
        }
        if self
            .text_atlas_in_flight
            .as_ref()
            .is_some_and(|in_flight| in_flight.pending() == pending)
        {
            let _ = self.progress_text_atlas_physical(pending);
            if let Some(outcome) = self.take_text_atlas_completion(pending) {
                return outcome;
            }
            return if self
                .text_atlas_in_flight
                .as_ref()
                .is_some_and(UiNativeTextAtlasInFlight::awaits_recovery)
            {
                recovery_outcome(pending)
            } else {
                UiGlyphRasterTransactionOutcome::Pending(pending)
            };
        }
        stale_plan()
    }

    fn take_text_atlas_completion(
        &mut self,
        pending: UiGlyphRasterTransactionPending,
    ) -> Option<UiGlyphRasterTransactionOutcome> {
        let matches = self
            .text_atlas_completion
            .as_ref()
            .is_some_and(|(completed, _)| *completed == pending);
        matches.then(|| {
            self.text_atlas_completion
                .take()
                .expect("matching completion remains retained")
                .1
        })
    }

    fn poll_gpu_observation(
        &mut self,
        pending: UiGlyphRasterTransactionPending,
    ) -> Option<crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation>
    {
        let Some(graphics) = self.graphics.as_ref() else {
            return self
                .text_atlas_in_flight
                .as_ref()
                .map(|in_flight| in_flight.observe(UiNativePhysicalSignalStatus::Completed));
        };
        let Some(gpu) = self.text_atlas_gpu.as_mut() else {
            return self
                .text_atlas_in_flight
                .as_ref()
                .map(|in_flight| in_flight.observe(UiNativePhysicalSignalStatus::Completed));
        };
        if !gpu.transaction_pending(pending.transaction()) {
            return None;
        }
        gpu.poll_transaction_observation(
            &graphics.device,
            &mut self.resources,
            pending.transaction(),
        )
    }

    pub(crate) fn transition_pending_text_atlas_to_recovery(
        &mut self,
        pending: UiGlyphRasterTransactionPending,
    ) -> UiGlyphRasterTransactionOutcome {
        if !self.current_text_atlas_commit_matches(pending) {
            return if self
                .text_atlas_in_flight
                .as_ref()
                .is_some_and(UiNativeTextAtlasInFlight::awaits_recovery)
            {
                recovery_outcome(pending)
            } else {
                stale_plan()
            };
        }
        let Ok(recovery_token) = self
            .physical_signal
            .transition_atlas_upload_to_recovery(pending)
        else {
            return stale_plan();
        };
        self.quarantine_text_atlas_in_flight(pending, recovery_token)
    }

    pub(crate) fn cancel_pending_text_atlas(
        &mut self,
        pending: UiGlyphRasterTransactionPending,
    ) -> UiGlyphRasterTransactionOutcome {
        if !self.current_text_atlas_commit_matches(pending) {
            return stale_plan();
        }
        let Ok(recovery_token) = self
            .physical_signal
            .cancel_atlas_upload_to_recovery(pending)
        else {
            return stale_plan();
        };
        self.quarantine_text_atlas_in_flight(pending, recovery_token)
    }

    pub(crate) fn supersede_pending_text_atlas(
        &mut self,
        pending: UiGlyphRasterTransactionPending,
    ) -> UiGlyphRasterTransactionOutcome {
        if !self.current_text_atlas_commit_matches(pending) {
            return stale_plan();
        }
        let Ok(recovery_token) = self
            .physical_signal
            .supersede_atlas_upload_to_recovery(pending)
        else {
            return stale_plan();
        };
        self.quarantine_text_atlas_in_flight(pending, recovery_token)
    }

    fn current_text_atlas_commit_matches(&self, pending: UiGlyphRasterTransactionPending) -> bool {
        self.text_atlas_in_flight
            .as_ref()
            .is_some_and(|in_flight| in_flight.pending() == pending && !in_flight.awaits_recovery())
    }

    fn quarantine_text_atlas_in_flight(
        &mut self,
        pending: UiGlyphRasterTransactionPending,
        recovery_token: crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken,
    ) -> UiGlyphRasterTransactionOutcome {
        let Some(in_flight) = self.text_atlas_in_flight.take() else {
            return stale_plan();
        };
        if in_flight.pending() != pending {
            self.text_atlas_in_flight = Some(in_flight);
            return stale_plan();
        }
        if in_flight.awaits_recovery() {
            self.text_atlas_in_flight = Some(in_flight);
            return recovery_outcome(pending);
        }
        let Some((plan, uploads)) = in_flight.into_commit_parts() else {
            unreachable!("recovery posture was handled above")
        };
        match self.text_atlas.settle(
            plan,
            &uploads,
            crate::native::text_atlas::UiNativeTextAtlasExternalOutcome::EffectsIndeterminate,
        ) {
            crate::native::text_atlas::UiNativeTextAtlasCommitOutcome::EffectsIndeterminate(
                recovery,
            ) => {
                self.text_atlas_recovery = Some(recovery);
                self.text_atlas_in_flight =
                    Some(UiNativeTextAtlasInFlight::recovery(pending, recovery_token));
                recovery_outcome(pending)
            }
            _ => {
                let _ = self.physical_signal.reconcile(recovery_token.observe(
                    crate::native::physical_work_signal::UiNativePhysicalSignalStatus::RejectedBeforeEffects,
                ));
                stale_plan()
            }
        }
    }

    pub(crate) fn commit_text_atlas_in_flight(&mut self) -> UiGlyphRasterTransactionOutcome {
        let Some(in_flight) = self.text_atlas_in_flight.take() else {
            return stale_plan();
        };
        let pending = in_flight.pending();
        let signal_token = in_flight.signal_token();
        let Some((plan, uploads)) = in_flight.into_commit_parts() else {
            self.text_atlas_in_flight =
                Some(UiNativeTextAtlasInFlight::recovery(pending, signal_token));
            return stale_plan();
        };
        match self.text_atlas.settle(
            plan,
            &uploads,
            crate::native::text_atlas::UiNativeTextAtlasExternalOutcome::Submitted,
        ) {
            crate::native::text_atlas::UiNativeTextAtlasCommitOutcome::Committed(receipt) => {
                UiGlyphRasterTransactionOutcome::Committed(
                    UiGlyphRasterTransactionReceipt::from_text_mechanics(
                        receipt.generation.get(),
                        receipt.misses,
                        receipt.hits,
                        receipt.evictions,
                        receipt.committed_pins,
                        receipt.staged_bytes,
                        receipt.physical_staged_bytes,
                        receipt.peak_entries,
                        receipt.peak_texel_bytes,
                    ),
                )
            }
            crate::native::text_atlas::UiNativeTextAtlasCommitOutcome::Denied(denial) => {
                UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(map_atlas_denial(denial))
            }
            crate::native::text_atlas::UiNativeTextAtlasCommitOutcome::EffectsIndeterminate(
                recovery,
            ) => {
                let generation = recovery.generation().get();
                self.text_atlas_recovery = Some(recovery);
                UiGlyphRasterTransactionOutcome::EffectsIndeterminate(
                    worth_ui_host_contract::UiGlyphRasterEffectsIndeterminate::from_text_mechanics(
                        pending.demand_identity(),
                        generation,
                    ),
                )
            }
        }
    }

    fn resolve_text_atlas_recovery(&mut self, pending: UiGlyphRasterTransactionPending) -> bool {
        let Some(in_flight) = self.text_atlas_in_flight.take() else {
            return false;
        };
        if in_flight.pending() != pending || !in_flight.awaits_recovery() {
            self.text_atlas_in_flight = Some(in_flight);
            return false;
        }
        let Some(recovery) = self.text_atlas_recovery.as_ref() else {
            self.text_atlas_in_flight = Some(in_flight);
            return false;
        };
        if !self.text_atlas.recovery_matches(recovery) {
            self.text_atlas_in_flight = Some(in_flight);
            return false;
        }
        if let Some(gpu) = self.text_atlas_gpu.take() {
            match gpu.try_close(&mut self.resources) {
                Ok(()) => {}
                Err(gpu) => {
                    self.text_atlas_gpu = Some(gpu);
                    self.text_atlas_in_flight = Some(in_flight);
                    return false;
                }
            }
        }
        let recovered = self
            .text_atlas_recovery
            .as_ref()
            .is_some_and(|recovery| self.text_atlas.recover(recovery));
        if !recovered {
            self.text_atlas_in_flight = Some(in_flight);
            return false;
        }
        self.text_atlas_recovery = None;
        true
    }
}

fn recovery_outcome(pending: UiGlyphRasterTransactionPending) -> UiGlyphRasterTransactionOutcome {
    UiGlyphRasterTransactionOutcome::EffectsIndeterminate(
        worth_ui_host_contract::UiGlyphRasterEffectsIndeterminate::from_text_mechanics(
            pending.demand_identity(),
            pending.generation(),
        ),
    )
}

fn stale_plan() -> UiGlyphRasterTransactionOutcome {
    UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(
        worth_ui_host_contract::UiGlyphRasterTransactionDenial::StalePlan,
    )
}

fn map_atlas_denial(
    denial: crate::native::text_atlas::UiNativeTextAtlasDenial,
) -> worth_ui_host_contract::UiGlyphRasterTransactionDenial {
    use crate::native::text_atlas::UiNativeTextAtlasDenial as Native;
    use worth_ui_host_contract::UiGlyphRasterTransactionDenial as Contract;
    match denial {
        Native::ReservationConflict => Contract::ReservationConflict,
        Native::GenerationExhausted => Contract::GenerationExhausted,
        Native::StalePlan => Contract::StalePlan,
        Native::StalePin => Contract::StalePin,
        Native::ReconstructionRequired => Contract::ReconstructionRequired,
        Native::PinnedCapacityExceeded => Contract::PinnedCapacityExceeded,
        Native::RasterGeometryMismatch => Contract::RasterGeometryMismatch,
        Native::RasterBatchMismatch | Native::UploadRejected => Contract::RasterBatchMismatch,
        Native::PageCapacityExceeded
        | Native::EntryCapacityExceeded
        | Native::TexelCapacityExceeded
        | Native::StagingCapacityExceeded
        | Native::GlyphExtentExceeded => Contract::CapacityExceeded,
        Native::MalformedDemand | Native::StaleDemand => Contract::MalformedDemand,
        Native::LivePinConflict | Native::StaleAffinity | Native::PinConflict => {
            Contract::StalePlan
        }
    }
}
