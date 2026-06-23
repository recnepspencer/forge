use core::str::FromStr;

use super::capability_id_error::CapabilityIdError;
use super::capability_id_family::{define_capability_id_family, CapabilityId};

define_capability_id_family!(CommandId, CommandIdFamily);
define_capability_id_family!(ComponentId, ComponentIdFamily);
define_capability_id_family!(SurfaceId, SurfaceIdFamily);
define_capability_id_family!(MosaicRegionKindId, MosaicRegionKindIdFamily);
define_capability_id_family!(MosaicPlacementPolicyId, MosaicPlacementPolicyIdFamily);
define_capability_id_family!(MosaicSizingContractId, MosaicSizingContractIdFamily);
define_capability_id_family!(MosaicStateSlotId, MosaicStateSlotIdFamily);
define_capability_id_family!(MosaicStateOwnerScopeId, MosaicStateOwnerScopeIdFamily);
define_capability_id_family!(ViewBindingId, ViewBindingIdFamily);
define_capability_id_family!(RuntimeOutcomeProjectionId, RuntimeOutcomeProjectionIdFamily);
define_capability_id_family!(SettingId, SettingIdFamily);
define_capability_id_family!(TaskPresentationId, TaskPresentationIdFamily);
define_capability_id_family!(ThemeTokenId, ThemeTokenIdFamily);
define_capability_id_family!(AppearanceTokenId, AppearanceTokenIdFamily);
define_capability_id_family!(DensityTokenId, DensityTokenIdFamily);
define_capability_id_family!(IconId, IconIdFamily);
define_capability_id_family!(CommandProjectionId, CommandProjectionIdFamily);
define_capability_id_family!(PluginSlotId, PluginSlotIdFamily);
define_capability_id_family!(NativeCapabilityId, NativeCapabilityIdFamily);
