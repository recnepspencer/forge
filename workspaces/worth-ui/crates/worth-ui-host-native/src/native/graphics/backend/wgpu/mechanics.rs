use std::sync::Arc;

use crate::native::graphics::{UiNativeDeviceGeneration, UiNativeDeviceState};
use crate::native::presentation::UiNativePresentationSurface;

pub(crate) struct UiWgpuDeviceMechanics {
    pub(super) instance: wgpu::Instance,
    pub(super) adapter: wgpu::Adapter,
    pub(super) adapter_info: wgpu::AdapterInfo,
}

pub(crate) struct UiWgpuDeviceGenerationMechanics {
    pub(super) device: wgpu::Device,
    pub(super) queue: wgpu::Queue,
    pub(super) lost: Arc<std::sync::atomic::AtomicBool>,
}

#[cfg(test)]
impl UiWgpuDeviceGenerationMechanics {
    pub(in crate::native) fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        lost: Arc<std::sync::atomic::AtomicBool>,
    ) -> Self {
        Self {
            device,
            queue,
            lost,
        }
    }
}

pub(crate) struct UiWgpuSurfaceMechanics {
    pub(super) surface: wgpu::Surface<'static>,
    pub(super) retained_target: Option<wgpu::Texture>,
    pub(super) configuration: wgpu::SurfaceConfiguration,
}

pub(crate) struct UiWgpuSurfaceHandle(pub(super) wgpu::Surface<'static>);
pub(crate) struct UiWgpuRetainedTarget(pub(super) wgpu::Texture);

pub(crate) enum UiNativePreparedGraphicsRecovery {
    SurfaceOutdated,
    SurfaceLost(UiWgpuSurfaceHandle),
    DeviceLost {
        generation: Arc<UiNativeDeviceGeneration>,
        retained_target: UiWgpuRetainedTarget,
    },
}

impl UiNativeDeviceState {
    pub(crate) fn device(&self) -> &wgpu::Device {
        &self.generation.mechanics.device
    }

    pub(crate) fn queue(&self) -> &wgpu::Queue {
        &self.generation.mechanics.queue
    }

    pub(crate) fn generation(&self) -> Arc<UiNativeDeviceGeneration> {
        Arc::clone(&self.generation)
    }

    pub(crate) fn lost(&self) -> bool {
        self.generation
            .mechanics
            .lost
            .load(std::sync::atomic::Ordering::Acquire)
    }

    pub(crate) fn generation_identity(&self) -> u64 {
        self.generation.identity
    }

    pub(crate) fn adapter_info(&self) -> &wgpu::AdapterInfo {
        &self.mechanics.adapter_info
    }

    pub(crate) fn adapter_limits(&self) -> wgpu::Limits {
        self.mechanics.adapter.limits()
    }
}

impl UiNativeDeviceGeneration {
    pub(crate) fn device(&self) -> &wgpu::Device {
        &self.mechanics.device
    }
}

impl UiNativePresentationSurface {
    pub(crate) const fn has_retained_target(&self) -> bool {
        self.mechanics.retained_target.is_some()
    }

    pub(crate) fn retained_target(&self) -> &wgpu::Texture {
        self.mechanics
            .retained_target
            .as_ref()
            .expect("live presentation surface retains its source target")
    }

    pub(crate) const fn extent(&self) -> [u32; 2] {
        self.extent
    }

    pub(crate) const fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    pub(crate) const fn suspended(&self) -> bool {
        self.suspended
    }

    pub(crate) const fn generation(&self) -> u64 {
        self.generation
    }

    pub(crate) fn surface(&self) -> &wgpu::Surface<'static> {
        &self.mechanics.surface
    }

    pub(crate) fn configuration(&self) -> &wgpu::SurfaceConfiguration {
        &self.mechanics.configuration
    }

    pub(crate) fn configure(&self, device: &wgpu::Device) {
        if !self.suspended {
            self.mechanics
                .surface
                .configure(device, &self.mechanics.configuration);
        }
    }

    pub(crate) fn commit_basis(
        &mut self,
        scale_factor: f64,
        extent: [u32; 2],
        device: &wgpu::Device,
    ) {
        debug_assert!(!extent.contains(&0));
        self.mechanics.configuration.width = extent[0];
        self.mechanics.configuration.height = extent[1];
        self.scale_factor = scale_factor;
        self.extent = extent;
        self.suspended = false;
        self.configure(device);
    }

    pub(crate) fn replace_surface(
        &mut self,
        successor: UiWgpuSurfaceHandle,
    ) -> UiWgpuSurfaceHandle {
        UiWgpuSurfaceHandle(std::mem::replace(&mut self.mechanics.surface, successor.0))
    }

    pub(crate) fn replace_target(
        &mut self,
        successor: UiWgpuRetainedTarget,
    ) -> Option<UiWgpuRetainedTarget> {
        self.mechanics
            .retained_target
            .replace(successor.0)
            .map(UiWgpuRetainedTarget)
    }

    pub(crate) fn take_target(&mut self) -> Option<UiWgpuRetainedTarget> {
        self.mechanics
            .retained_target
            .take()
            .map(UiWgpuRetainedTarget)
    }
}
