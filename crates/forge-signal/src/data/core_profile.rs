use serde::{Deserialize, Serialize};

#[cfg(all(feature = "profile-compact", feature = "profile-standard"))]
compile_error!("forge-signal core profile features are mutually exclusive");
#[cfg(all(feature = "profile-compact", feature = "profile-extended"))]
compile_error!("forge-signal core profile features are mutually exclusive");
#[cfg(all(feature = "profile-standard", feature = "profile-extended"))]
compile_error!("forge-signal core profile features are mutually exclusive");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalCoreStorageProfile {
    Compact,
    Standard,
    Extended,
}

#[cfg(feature = "profile-compact")]
pub const CORE_STORAGE_PROFILE: SignalCoreStorageProfile = SignalCoreStorageProfile::Compact;
#[cfg(feature = "profile-standard")]
pub const CORE_STORAGE_PROFILE: SignalCoreStorageProfile = SignalCoreStorageProfile::Standard;
#[cfg(feature = "profile-extended")]
pub const CORE_STORAGE_PROFILE: SignalCoreStorageProfile = SignalCoreStorageProfile::Extended;

#[cfg(feature = "profile-compact")]
pub const CORE_STORAGE_PROFILE_ID: &str = "compact";
#[cfg(feature = "profile-standard")]
pub const CORE_STORAGE_PROFILE_ID: &str = "standard";
#[cfg(feature = "profile-extended")]
pub const CORE_STORAGE_PROFILE_ID: &str = "extended";

#[cfg(feature = "profile-compact")]
pub const MAX_ASPECTS: usize = 8;
#[cfg(feature = "profile-standard")]
pub const MAX_ASPECTS: usize = 8;
#[cfg(feature = "profile-extended")]
pub const MAX_ASPECTS: usize = 16;

#[cfg(feature = "profile-compact")]
pub type AspectMaskBits = u8;
#[cfg(feature = "profile-standard")]
pub type AspectMaskBits = u8;
#[cfg(feature = "profile-extended")]
pub type AspectMaskBits = u16;

#[cfg(feature = "profile-compact")]
pub const HOT_VEC_INLINE_CAPACITY: usize = 4;
#[cfg(feature = "profile-standard")]
pub const HOT_VEC_INLINE_CAPACITY: usize = 8;
#[cfg(feature = "profile-extended")]
pub const HOT_VEC_INLINE_CAPACITY: usize = 16;

#[cfg(feature = "profile-compact")]
pub type StableHashValue = u64;
#[cfg(feature = "profile-standard")]
pub type StableHashValue = u128;
#[cfg(feature = "profile-extended")]
pub type StableHashValue = u128;

#[cfg(feature = "profile-compact")]
pub const STABLE_HASH_WIDTH_BITS: usize = 64;
#[cfg(feature = "profile-standard")]
pub const STABLE_HASH_WIDTH_BITS: usize = 128;
#[cfg(feature = "profile-extended")]
pub const STABLE_HASH_WIDTH_BITS: usize = 128;
