#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativePlatformProfileIdentity(&'static str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeMechanicsCapacities {
    pub retained_commands: u16,
    pub rectangle_commands: u16,
    pub text_commands: u16,
    pub damage_regions: u16,
    pub order_edits: u16,
    pub text_bytes: u32,
    pub readiness_owners: u8,
    pub causes_per_owner: u8,
    pub ready_owner_slots: u8,
    pub presentation_slots: u8,
    pub readback_slots: u8,
    pub readback_bytes: u32,
}

pub const WORTH_UI_NATIVE_PROFILE_MANIFEST: &str =
    include_str!("../profiles/worth-ui-windows-dx12-v1.toml");

impl UiNativePlatformProfileIdentity {
    pub const WORTH_UI_WINDOWS_DX12_V1: Self = Self("worth-ui-windows-dx12-v1");

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl UiNativeMechanicsCapacities {
    pub const QUALIFIED: Self = Self {
        retained_commands: 4_096,
        rectangle_commands: 2_048,
        text_commands: 2_048,
        damage_regions: 4_096,
        order_edits: 4_096,
        text_bytes: 1_048_576,
        readiness_owners: 8,
        causes_per_owner: 64,
        ready_owner_slots: 8,
        presentation_slots: 2,
        readback_slots: 4,
        readback_bytes: 16_777_216,
    };
}
