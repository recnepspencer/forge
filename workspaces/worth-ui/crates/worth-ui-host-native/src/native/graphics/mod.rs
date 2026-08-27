pub(crate) mod adapter_selection;
mod backend;
mod device;

#[cfg(test)]
pub(crate) use backend::QUALIFIED_DX12_PRESENTATION_SYSTEM;
pub(crate) use backend::{
    prepare_external_recovery, prepare_platform_graphics, prepare_replacement_target,
    UiNativeBackendDeviceGenerationMechanics, UiNativeBackendDeviceMechanics,
    UiNativeBackendRetainedTarget, UiNativeBackendSurfaceHandle, UiNativeBackendSurfaceMechanics,
    UiNativeGraphicsRecovery, UiNativePreparedGraphicsRecovery,
};
pub(crate) use device::{
    UiNativeDeviceGeneration, UiNativeDeviceOwners, UiNativeDeviceState, UiNativeOwnedDevice,
};
