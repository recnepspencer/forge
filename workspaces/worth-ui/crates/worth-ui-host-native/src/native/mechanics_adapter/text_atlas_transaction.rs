//! Typed staged execution for one native text-atlas transaction.

use worth_ui_host_contract::{
    UiGlyphRasterDemandBatchView, UiGlyphRasterMissRasterizer, UiGlyphRasterPinTransitionView,
    UiGlyphRasterTransactionDenial, UiGlyphRasterTransactionOutcome,
    UiGlyphRasterTransactionPending,
};

use crate::native::text_atlas::{
    UiNativeTextAtlasExternalOutcome, UiNativeTextAtlasInFlight, UiNativeTextAtlasUpload,
};
use crate::native::UiNativeHostState;

use super::text_atlas_admission::{native_pin_transition, physical_capacity_denial};
use super::text_atlas_rasterization::rasterize_misses;
use super::text_atlas_settlement::{
    map_denial, reject_after_rasterization, reject_plan, settle_plan,
};
use super::text_atlas_upload::{CorrelatedGpuUploadObservation, UiNativeTextAtlasUploadPort};

pub(super) struct TextAtlasExecution<'work> {
    pub(super) state: &'work mut UiNativeHostState,
    pub(super) presentation_basis:
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis,
    pub(super) demands: &'work [UiGlyphRasterDemandBatchView<'work>],
    pub(super) pins: UiGlyphRasterPinTransitionView<'work>,
    pub(super) rasterizer: &'work mut dyn UiGlyphRasterMissRasterizer,
    pub(super) upload_port: &'work mut dyn UiNativeTextAtlasUploadPort,
}

struct AdmittedTransaction {
    plan: crate::native::text_atlas::UiNativeTextAtlasTransactionPlan,
    pending: Option<UiGlyphRasterTransactionPending>,
    signal_token: crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken,
}

struct ObservedTransaction {
    admitted: AdmittedTransaction,
    uploads: Vec<UiNativeTextAtlasUpload>,
    external: UiNativeTextAtlasExternalOutcome,
    settlement: crate::native::physical_work_signal::UiNativePhysicalSignalSettlement,
}

pub(super) fn perform(execution: TextAtlasExecution<'_>) -> UiGlyphRasterTransactionOutcome {
    let TextAtlasExecution {
        state,
        presentation_basis,
        demands,
        pins,
        rasterizer,
        upload_port,
    } = execution;
    let admitted = match admit(state, presentation_basis, demands, pins) {
        Ok(admitted) => admitted,
        Err(outcome) => return outcome,
    };
    let observed = match rasterize_and_observe(state, admitted, rasterizer, upload_port) {
        Ok(observed) => observed,
        Err(outcome) => return outcome,
    };
    settle_observed(state, presentation_basis.host_session_identity(), observed)
}

fn admit(
    state: &mut UiNativeHostState,
    presentation_basis: crate::native::physical_work_signal::UiNativePhysicalPresentationBasis,
    demands: &[UiGlyphRasterDemandBatchView<'_>],
    pins: UiGlyphRasterPinTransitionView<'_>,
) -> Result<AdmittedTransaction, UiGlyphRasterTransactionOutcome> {
    let physical_request = state
        .physical_signal
        .admit_atlas_planning(presentation_basis, demands, pins)
        .map_err(|_| reservation_conflict())?;
    let planning_token = state
        .physical_signal
        .take_ready_atlas_planning(physical_request)
        .map_err(|_| reservation_conflict())?;
    if state.text_atlas_in_flight.is_some() {
        reject_signal_before_effects(state, planning_token);
        return Err(reservation_conflict());
    }
    let transition = native_pin_transition(pins);
    let plan = match state.text_atlas.plan_many(demands, &transition) {
        Ok(plan) => plan,
        Err(denial) => {
            reject_signal_before_effects(state, planning_token);
            return Err(UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(
                map_denial(denial),
            ));
        }
    };
    if let Some(denial) = physical_capacity_denial(state, &plan) {
        reject_signal_before_effects(state, planning_token);
        return Err(reject_plan(state, plan, denial));
    }
    let physical_pending = pending_for(&plan, presentation_basis.host_session_identity());
    let signal_token = match state
        .physical_signal
        .bind_atlas_upload(planning_token, physical_pending)
    {
        Ok(token) => token,
        Err(()) => {
            return Err(reject_plan(
                state,
                plan,
                UiGlyphRasterTransactionDenial::ReservationConflict,
            ));
        }
    };
    Ok(AdmittedTransaction {
        pending: (!plan.miss_demands().is_empty()).then_some(physical_pending),
        plan,
        signal_token,
    })
}

fn rasterize_and_observe(
    state: &mut UiNativeHostState,
    admitted: AdmittedTransaction,
    rasterizer: &mut dyn UiGlyphRasterMissRasterizer,
    upload_port: &mut dyn UiNativeTextAtlasUploadPort,
) -> Result<ObservedTransaction, UiGlyphRasterTransactionOutcome> {
    let uploads = match rasterize_misses(&admitted.plan, rasterizer) {
        Ok(uploads) => uploads,
        Err(denial) => {
            reject_signal_after_rasterization(state, admitted.signal_token);
            return Err(reject_after_rasterization(state, admitted.plan, denial));
        }
    };
    if !admitted.plan.admits_uploads(&uploads) {
        reject_signal_after_rasterization(state, admitted.signal_token);
        return Err(reject_after_rasterization(
            state,
            admitted.plan,
            UiGlyphRasterTransactionDenial::RasterBatchMismatch,
        ));
    }
    let correlated = upload_outcome(state, &admitted, &uploads, upload_port);
    let settlement = state.physical_signal.reconcile(correlated.signal);
    let external = match correlated.external {
        Ok(external) => external,
        Err(_) if matches!(settlement, signal_settlement::Stale) => {
            UiNativeTextAtlasExternalOutcome::EffectsIndeterminate
        }
        Err(denial) => {
            return Err(reject_after_rasterization(state, admitted.plan, denial));
        }
    };
    Ok(ObservedTransaction {
        admitted,
        uploads,
        external,
        settlement,
    })
}

fn settle_observed(
    state: &mut UiNativeHostState,
    host_session: u64,
    observed: ObservedTransaction,
) -> UiGlyphRasterTransactionOutcome {
    let ObservedTransaction {
        admitted,
        uploads,
        external,
        settlement,
    } = observed;
    if matches!(settlement, signal_settlement::Stale) {
        return quarantine_stale_external_observation(state, admitted, uploads);
    }
    match (admitted.pending, external, settlement) {
        (
            Some(pending),
            UiNativeTextAtlasExternalOutcome::Submitted,
            signal_settlement::Pending,
        ) => settle_pending(state, host_session, admitted, uploads, pending, settlement),
        (Some(pending), UiNativeTextAtlasExternalOutcome::EffectsIndeterminate, _) => {
            settle_indeterminate(state, admitted, uploads, pending, settlement)
        }
        (_, external, _) => settle_completed(state, admitted, uploads, external, settlement),
    }
}

fn quarantine_stale_external_observation(
    state: &mut UiNativeHostState,
    admitted: AdmittedTransaction,
    uploads: Vec<UiNativeTextAtlasUpload>,
) -> UiGlyphRasterTransactionOutcome {
    let Some(pending) = admitted.pending else {
        return stale_after_rasterization(state, admitted.plan);
    };
    let Ok(recovery_token) = state
        .physical_signal
        .transition_atlas_upload_to_recovery(pending)
    else {
        return stale_after_rasterization(state, admitted.plan);
    };
    let outcome = settle_plan(
        state,
        admitted.plan,
        uploads,
        UiNativeTextAtlasExternalOutcome::EffectsIndeterminate,
    );
    state.text_atlas_in_flight = Some(UiNativeTextAtlasInFlight::recovery(pending, recovery_token));
    outcome
}

fn settle_pending(
    state: &mut UiNativeHostState,
    host_session: u64,
    admitted: AdmittedTransaction,
    uploads: Vec<UiNativeTextAtlasUpload>,
    pending: UiGlyphRasterTransactionPending,
    settlement: crate::native::physical_work_signal::UiNativePhysicalSignalSettlement,
) -> UiGlyphRasterTransactionOutcome {
    if !matches!(settlement, signal_settlement::Pending) {
        return stale_after_rasterization(state, admitted.plan);
    }
    state.text_atlas_in_flight = Some(UiNativeTextAtlasInFlight::new(
        admitted.plan,
        uploads,
        host_session,
        admitted.signal_token,
    ));
    UiGlyphRasterTransactionOutcome::Pending(pending)
}

fn settle_indeterminate(
    state: &mut UiNativeHostState,
    admitted: AdmittedTransaction,
    uploads: Vec<UiNativeTextAtlasUpload>,
    pending: UiGlyphRasterTransactionPending,
    settlement: crate::native::physical_work_signal::UiNativePhysicalSignalSettlement,
) -> UiGlyphRasterTransactionOutcome {
    if !matches!(settlement, signal_settlement::Indeterminate) {
        return stale_after_rasterization(state, admitted.plan);
    }
    let recovery_token = state
        .physical_signal
        .transition_atlas_upload_to_recovery(pending)
        .expect("admitted indeterminate upload transitions to Signal recovery");
    let outcome = settle_plan(
        state,
        admitted.plan,
        uploads,
        UiNativeTextAtlasExternalOutcome::EffectsIndeterminate,
    );
    state.text_atlas_in_flight = Some(UiNativeTextAtlasInFlight::recovery(pending, recovery_token));
    outcome
}

fn settle_completed(
    state: &mut UiNativeHostState,
    admitted: AdmittedTransaction,
    uploads: Vec<UiNativeTextAtlasUpload>,
    external: UiNativeTextAtlasExternalOutcome,
    settlement: crate::native::physical_work_signal::UiNativePhysicalSignalSettlement,
) -> UiGlyphRasterTransactionOutcome {
    if !matches!(settlement, signal_settlement::Completed) {
        return stale_after_rasterization(state, admitted.plan);
    }
    settle_plan(state, admitted.plan, uploads, external)
}

fn upload_outcome(
    state: &mut UiNativeHostState,
    admitted: &AdmittedTransaction,
    uploads: &[UiNativeTextAtlasUpload],
    upload_port: &mut dyn UiNativeTextAtlasUploadPort,
) -> CorrelatedGpuUploadObservation {
    if uploads.is_empty() {
        return CorrelatedGpuUploadObservation {
            external: Ok(UiNativeTextAtlasExternalOutcome::Submitted),
            signal: admitted.signal_token.observe(
                crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Completed,
            ),
        };
    }
    upload_port.upload(
        state,
        &admitted.plan,
        uploads,
        admitted.signal_token.external_basis(),
    )
}

fn pending_for(
    plan: &crate::native::text_atlas::UiNativeTextAtlasTransactionPlan,
    host_session: u64,
) -> UiGlyphRasterTransactionPending {
    UiGlyphRasterTransactionPending::from_text_mechanics(
        plan.demand_identity,
        plan.candidate_generation.get(),
        plan.transaction_identity(),
        host_session,
    )
}

fn reject_signal_before_effects(
    state: &mut UiNativeHostState,
    token: crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken,
) {
    let _ = state.physical_signal.reconcile(token.observe(
        crate::native::physical_work_signal::UiNativePhysicalSignalStatus::RejectedBeforeEffects,
    ));
}

fn reject_signal_after_rasterization(
    state: &mut UiNativeHostState,
    token: crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken,
) {
    let _ = state.physical_signal.reconcile(token.observe(
        crate::native::physical_work_signal::UiNativePhysicalSignalStatus::RejectedAfterRasterization,
    ));
}

fn stale_after_rasterization(
    state: &mut UiNativeHostState,
    plan: crate::native::text_atlas::UiNativeTextAtlasTransactionPlan,
) -> UiGlyphRasterTransactionOutcome {
    reject_after_rasterization(state, plan, UiGlyphRasterTransactionDenial::StalePlan)
}

fn reservation_conflict() -> UiGlyphRasterTransactionOutcome {
    UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(
        UiGlyphRasterTransactionDenial::ReservationConflict,
    )
}

mod signal_settlement {
    pub(super) use crate::native::physical_work_signal::UiNativePhysicalSignalSettlement::{
        Completed, Indeterminate, Pending, Stale,
    };
}
