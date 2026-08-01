pub const PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME: &str =
    "component:platform.pulse.component.identity_target";
pub const PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT: [u32; 2] = [160, 96];
pub const PLATFORM_PULSE_BACKGROUND_LOGICAL_POINT: [u32; 2] = [16, 16];
pub const PLATFORM_PULSE_TARGET_LOGICAL_POINT: [u32; 2] = [52, 28];
pub const PLATFORM_PULSE_MAXIMUM_CAPTURE_SCALE: u32 = 4;
pub const PLATFORM_PULSE_MAXIMUM_PIXEL_BYTES: u64 = 983_040;
pub const PLATFORM_PULSE_TARGET_RGB: [u8; 3] = [0xf2, 0xcc, 0x60];
pub const PLATFORM_PULSE_CONFIRMATION_RGB: [u8; 3] = [0x6e, 0x40, 0xc9];
pub const PLATFORM_PULSE_VISIBLE_REGION_COUNT: u64 = 3;
pub const PLATFORM_PULSE_HIT_TEST_REGION_COUNT: u64 = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseVisualIdentityScenario {
    target_authored_name: &'static str,
    logical_extent: [u32; 2],
    background_logical_point: [u32; 2],
    target_logical_point: [u32; 2],
}

impl PlatformPulseVisualIdentityScenario {
    pub const fn canonical() -> Self {
        Self {
            target_authored_name: PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME,
            logical_extent: PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT,
            background_logical_point: PLATFORM_PULSE_BACKGROUND_LOGICAL_POINT,
            target_logical_point: PLATFORM_PULSE_TARGET_LOGICAL_POINT,
        }
    }

    pub const fn target_authored_name(self) -> &'static str {
        self.target_authored_name
    }

    pub const fn logical_extent(self) -> [u32; 2] {
        self.logical_extent
    }

    pub const fn background_logical_point(self) -> [u32; 2] {
        self.background_logical_point
    }

    pub const fn target_logical_point(self) -> [u32; 2] {
        self.target_logical_point
    }
}
