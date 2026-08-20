use worth_signal::facade::{AdmittedResourceRequest, ResourceCancellationReason};

use super::rejection::{
    BridgeAsyncRequestIdentityRejection, BridgeAsyncRequestIdentityRejectionKind,
};
use super::state::{
    retire_owned_async_declaration_for_lowering, retire_owned_resource_declaration_for_lowering,
    BridgeAsyncDeclarationRegistry, BridgeSignalRuntime,
};

pub(super) fn rollback_owned_resource_request(
    runtime: &mut BridgeSignalRuntime,
    registry: Option<&mut BridgeAsyncDeclarationRegistry>,
    installed_owned: bool,
    lowering: &str,
    admitted: AdmittedResourceRequest,
) -> Result<(), BridgeAsyncRequestIdentityRejection> {
    runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .map_err(signal_request_rejected)?;
    if installed_owned {
        retire_owned_resource_declaration_for_lowering(
            registry.expect("owned declaration retains its registry"),
            runtime,
            lowering,
        )
        .map_err(signal_request_rejected)?;
    }
    Ok(())
}

pub(super) fn rollback_owned_async_request(
    runtime: &mut BridgeSignalRuntime,
    registry: Option<&mut BridgeAsyncDeclarationRegistry>,
    installed_owned: bool,
    lowering: &str,
    admitted: AdmittedResourceRequest,
) -> Result<(), BridgeAsyncRequestIdentityRejection> {
    runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .map_err(signal_request_rejected)?;
    retire_owned_async_declaration(runtime, registry, installed_owned, lowering)
}

pub(super) fn retire_owned_async_declaration(
    runtime: &mut BridgeSignalRuntime,
    registry: Option<&mut BridgeAsyncDeclarationRegistry>,
    installed_owned: bool,
    lowering: &str,
) -> Result<(), BridgeAsyncRequestIdentityRejection> {
    if installed_owned {
        retire_owned_async_declaration_for_lowering(
            registry.expect("owned declaration retains its registry"),
            runtime,
            lowering,
        )
        .map_err(signal_request_rejected)?;
    }
    Ok(())
}

fn signal_request_rejected(
    error: worth_signal::facade::SignalError,
) -> BridgeAsyncRequestIdentityRejection {
    BridgeAsyncRequestIdentityRejection::new(
        BridgeAsyncRequestIdentityRejectionKind::SignalRequestAdmissionRejected,
        error.to_string(),
    )
}
