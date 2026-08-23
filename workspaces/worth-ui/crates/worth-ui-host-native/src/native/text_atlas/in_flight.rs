//! Native-owned text-atlas work retained between queue submission and
//! physical completion.

use worth_ui_host_contract::UiGlyphRasterTransactionPending;

use super::{UiNativeTextAtlasTransactionPlan, UiNativeTextAtlasUpload};

/// Move-only owner of one submitted atlas transaction.
///
/// The plan and raster uploads stay together until the host observes every
/// physical submission.  Neither candidate atlas state nor pin changes can be
/// committed while this owner is live.
pub(crate) struct UiNativeTextAtlasInFlight {
    payload: UiNativeTextAtlasInFlightPayload,
    pending: UiGlyphRasterTransactionPending,
    signal_token: crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken,
}

enum UiNativeTextAtlasInFlightPayload {
    Commit {
        plan: UiNativeTextAtlasTransactionPlan,
        uploads: Box<[UiNativeTextAtlasUpload]>,
    },
    Recovery,
}

impl UiNativeTextAtlasInFlight {
    pub(crate) fn new(
        plan: UiNativeTextAtlasTransactionPlan,
        uploads: Vec<UiNativeTextAtlasUpload>,
        host_session: u64,
        signal_token: crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken,
    ) -> Self {
        let pending = UiGlyphRasterTransactionPending::from_text_mechanics(
            plan.demand_identity,
            plan.candidate_generation.get(),
            plan.transaction_identity(),
            host_session,
        );
        Self {
            payload: UiNativeTextAtlasInFlightPayload::Commit {
                plan,
                uploads: uploads.into_boxed_slice(),
            },
            pending,
            signal_token,
        }
    }

    pub(crate) const fn recovery(
        pending: UiGlyphRasterTransactionPending,
        signal_token: crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken,
    ) -> Self {
        Self {
            payload: UiNativeTextAtlasInFlightPayload::Recovery,
            pending,
            signal_token,
        }
    }

    pub(crate) const fn pending(&self) -> UiGlyphRasterTransactionPending {
        self.pending
    }

    pub(crate) const fn signal_token(
        &self,
    ) -> crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken {
        self.signal_token
    }

    pub(crate) fn refresh_signal_token(
        &mut self,
        token: crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken,
    ) -> bool {
        let crate::native::physical_work_signal::UiNativePhysicalSignalWork::AtlasUpload(identity) =
            token.work()
        else {
            return false;
        };
        if identity.pending() != self.pending {
            return false;
        }
        self.signal_token = token;
        true
    }

    pub(crate) const fn observe(
        &self,
        status: crate::native::physical_work_signal::UiNativePhysicalSignalStatus,
    ) -> crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation {
        self.signal_token.observe(status)
    }

    pub(crate) const fn awaits_recovery(&self) -> bool {
        matches!(self.payload, UiNativeTextAtlasInFlightPayload::Recovery)
    }

    pub(crate) fn into_commit_parts(
        self,
    ) -> Option<(
        UiNativeTextAtlasTransactionPlan,
        Box<[UiNativeTextAtlasUpload]>,
    )> {
        match self.payload {
            UiNativeTextAtlasInFlightPayload::Commit { plan, uploads } => Some((plan, uploads)),
            UiNativeTextAtlasInFlightPayload::Recovery => None,
        }
    }
}
