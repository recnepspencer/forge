#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiSurfaceBindingCoordinatePosture {
    LogicalPoints,
    PhysicalPixels,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiSurfaceBindingProfile {
    device_scale_milli: u32,
    coordinate_posture: UiSurfaceBindingCoordinatePosture,
    native_resource_epoch: u64,
}

impl UiSurfaceBindingProfile {
    pub fn new(
        device_scale_milli: u32,
        coordinate_posture: UiSurfaceBindingCoordinatePosture,
        native_resource_epoch: u64,
    ) -> Result<Self, super::UiMountedIdentityDenial> {
        if device_scale_milli == 0 {
            return Err(super::UiMountedIdentityDenial::InvalidDeviceScale);
        }
        Ok(Self {
            device_scale_milli,
            coordinate_posture,
            native_resource_epoch,
        })
    }

    pub fn device_scale_milli(self) -> u32 {
        self.device_scale_milli
    }

    pub fn coordinate_posture(self) -> UiSurfaceBindingCoordinatePosture {
        self.coordinate_posture
    }

    pub fn native_resource_epoch(self) -> u64 {
        self.native_resource_epoch
    }
}
