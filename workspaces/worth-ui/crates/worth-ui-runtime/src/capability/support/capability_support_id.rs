use crate::capability::{
    CommandId, CommandProjectionId, ComponentId, IconId, MosaicPlacementPolicyId,
    MosaicRegionKindId, MosaicSizingContractId, MosaicStateSlotId, NativeCapabilityId,
    PluginSlotId, RuntimeOutcomeProjectionId, SettingId, SurfaceId, TaskPresentationId,
    ThemeTokenId, ViewBindingId,
};

/// Sealed marker for typed capability identities that can carry support posture.
pub trait CapabilitySupportId: sealed::Sealed {}

macro_rules! impl_capability_support_id {
    ($($id_type:ty),+ $(,)?) => {
        $(
            impl sealed::Sealed for $id_type {}
            impl CapabilitySupportId for $id_type {}
        )+
    };
}

impl_capability_support_id!(
    CommandId,
    ComponentId,
    SurfaceId,
    MosaicRegionKindId,
    MosaicPlacementPolicyId,
    MosaicSizingContractId,
    MosaicStateSlotId,
    ViewBindingId,
    RuntimeOutcomeProjectionId,
    SettingId,
    TaskPresentationId,
    ThemeTokenId,
    IconId,
    CommandProjectionId,
    PluginSlotId,
    NativeCapabilityId,
);

mod sealed {
    pub trait Sealed {}
}
