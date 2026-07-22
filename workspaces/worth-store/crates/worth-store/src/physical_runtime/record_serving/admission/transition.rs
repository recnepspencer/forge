use worth_proof::TransitionOutcome;

use crate::physical_runtime::MediaOwnedPhysicalRuntime;

use super::super::{
    admission::bootstrap::BootstrapTransitionFailure, PhysicalRecordInitialization,
    PhysicalRecordOpen, RecordAllocationFrontier, RecordBootstrapFailure,
    RecordServingAdmissionInspectionRequired, RecordServingAdmissionRebindRequired,
    RecordServingAdmissionStale, RecordStoreInitializationDenial, RecordStoreInitializationOutcome,
    RecordStoreOpenDenial, RecordStoreOpenOutcome, ServingPhysicalRuntime,
};
use super::{initialization, open as record_open};

pub(in crate::physical_runtime) fn initialize(
    runtime: MediaOwnedPhysicalRuntime,
    request: PhysicalRecordInitialization,
) -> RecordStoreInitializationOutcome {
    if let Err(reason) = request
        .residency
        .preflight_format(request.format, request.access)
    {
        return TransitionOutcome::denied(RecordStoreInitializationDenial::new(
            runtime,
            super::super::RecordBootstrapDenial::ResidencyUnavailable(reason),
        ))
        .into();
    }
    let frame_ports = match super::super::residency::frame_ports::RecordFramePorts::bounded(
        runtime.store_identity(),
        request.residency.limits(),
    ) {
        Ok(ports) => ports,
        Err(reason) => {
            return TransitionOutcome::denied(RecordStoreInitializationDenial::new(
                runtime,
                super::super::RecordBootstrapDenial::ResidencyUnavailable(reason),
            ))
            .into();
        }
    };
    let bootstrap = match initialization::initialize(runtime.record_serving_media(), request) {
        Ok(bootstrap) => bootstrap,
        Err(BootstrapTransitionFailure::Denied(reason)) => {
            return TransitionOutcome::denied(RecordStoreInitializationDenial::new(
                runtime, reason,
            ))
            .into();
        }
        Err(BootstrapTransitionFailure::Failed(cause)) => {
            return initialization_failed(runtime, cause)
        }
        Err(BootstrapTransitionFailure::Stale(reason)) => {
            return initialization_failed(
                runtime,
                RecordBootstrapFailure::PublishedRootStale(reason),
            );
        }
        Err(BootstrapTransitionFailure::RebindRequired(reason)) => {
            return initialization_failed(
                runtime,
                RecordBootstrapFailure::PublishedRootRebindRequired(reason),
            );
        }
    };
    match record_open::load_current_root(
        runtime.record_serving_media(),
        frame_ports.loader(),
        bootstrap,
    ) {
        Ok(state) => initialize_serving(runtime, state, frame_ports),
        Err(BootstrapTransitionFailure::Denied(reason)) => initialization_failed(
            runtime,
            RecordBootstrapFailure::PublishedRootReadmission(reason),
        ),
        Err(BootstrapTransitionFailure::Stale(reason)) => {
            initialization_failed(runtime, RecordBootstrapFailure::PublishedRootStale(reason))
        }
        Err(BootstrapTransitionFailure::RebindRequired(reason)) => initialization_failed(
            runtime,
            RecordBootstrapFailure::PublishedRootRebindRequired(reason),
        ),
        Err(BootstrapTransitionFailure::Failed(cause)) => initialization_failed(runtime, cause),
    }
}

pub(in crate::physical_runtime) fn open(
    runtime: MediaOwnedPhysicalRuntime,
    request: PhysicalRecordOpen,
) -> RecordStoreOpenOutcome {
    if let Err(reason) = request
        .residency
        .preflight_format(request.format, request.access)
    {
        return TransitionOutcome::denied(RecordStoreOpenDenial::new(
            runtime,
            super::super::RecordBootstrapDenial::ResidencyUnavailable(reason),
        ))
        .into();
    }
    let frame_ports = match super::super::residency::frame_ports::RecordFramePorts::bounded(
        runtime.store_identity(),
        request.residency.limits(),
    ) {
        Ok(ports) => ports,
        Err(reason) => {
            return TransitionOutcome::denied(RecordStoreOpenDenial::new(
                runtime,
                super::super::RecordBootstrapDenial::ResidencyUnavailable(reason),
            ))
            .into();
        }
    };
    let bootstrap = match record_open::open(
        runtime.record_serving_media(),
        frame_ports.loader(),
        request,
    ) {
        Ok(bootstrap) => bootstrap,
        Err(failure) => return open_failure(runtime, failure),
    };
    match record_open::load_current_root(
        runtime.record_serving_media(),
        frame_ports.loader(),
        bootstrap,
    ) {
        Ok(state) => open_serving(runtime, state, frame_ports),
        Err(failure) => open_failure(runtime, failure),
    }
}

fn initialize_serving(
    runtime: MediaOwnedPhysicalRuntime,
    state: super::super::RecordServingState,
    frame_ports: super::super::residency::frame_ports::RecordFramePorts,
) -> RecordStoreInitializationOutcome {
    let frontier = RecordAllocationFrontier::new(&state.free_space);
    let (termination, media, core) = runtime.into_record_serving_parts();
    core.progress_to_record_serving();
    TransitionOutcome::success(ServingPhysicalRuntime::from_admission(
        termination,
        media,
        core,
        state,
        frontier,
        frame_ports,
    ))
    .into()
}

fn open_serving(
    runtime: MediaOwnedPhysicalRuntime,
    state: super::super::RecordServingState,
    frame_ports: super::super::residency::frame_ports::RecordFramePorts,
) -> RecordStoreOpenOutcome {
    let frontier = RecordAllocationFrontier::new(&state.free_space);
    let (termination, media, core) = runtime.into_record_serving_parts();
    core.progress_to_record_serving();
    TransitionOutcome::success(ServingPhysicalRuntime::from_admission(
        termination,
        media,
        core,
        state,
        frontier,
        frame_ports,
    ))
    .into()
}

fn open_failure(
    runtime: MediaOwnedPhysicalRuntime,
    failure: BootstrapTransitionFailure,
) -> RecordStoreOpenOutcome {
    match failure {
        BootstrapTransitionFailure::Denied(reason) => {
            TransitionOutcome::denied(RecordStoreOpenDenial::new(runtime, reason)).into()
        }
        BootstrapTransitionFailure::Stale(reason) => {
            TransitionOutcome::stale(RecordServingAdmissionStale::new(runtime, reason)).into()
        }
        BootstrapTransitionFailure::RebindRequired(reason) => TransitionOutcome::rebind_required(
            RecordServingAdmissionRebindRequired::new(runtime, reason),
        )
        .into(),
        BootstrapTransitionFailure::Failed(cause) => {
            let identity = runtime.runtime_identity();
            TransitionOutcome::failed(RecordServingAdmissionInspectionRequired::new(
                identity,
                runtime.abort(),
                cause,
            ))
            .into()
        }
    }
}

fn initialization_failed(
    runtime: MediaOwnedPhysicalRuntime,
    cause: RecordBootstrapFailure,
) -> RecordStoreInitializationOutcome {
    let identity = runtime.runtime_identity();
    TransitionOutcome::failed(RecordServingAdmissionInspectionRequired::new(
        identity,
        runtime.abort(),
        cause,
    ))
    .into()
}
