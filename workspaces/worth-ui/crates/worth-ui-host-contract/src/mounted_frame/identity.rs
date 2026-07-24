use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_MOUNTED_CONTRACT_IDENTITY: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedContractIdentityExhaustion;

macro_rules! opaque_identity {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u64);

        impl $name {
            #[doc(hidden)]
            pub fn mint_unbound() -> Result<Self, UiMountedContractIdentityExhaustion> {
                NEXT_MOUNTED_CONTRACT_IDENTITY
                    .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                        current.checked_add(1)
                    })
                    .map(Self)
                    .map_err(|_| UiMountedContractIdentityExhaustion)
            }

            pub const fn diagnostic_value(self) -> u64 {
                self.0
            }
        }
    };
}

opaque_identity!(UiSemanticSurfaceIdentity);
opaque_identity!(UiHostSurfaceIdentity);
opaque_identity!(UiSurfaceBindingGeneration);
opaque_identity!(UiMountIncarnation);
opaque_identity!(UiMountedInstanceIdentity);
opaque_identity!(UiMountedFrameIdentity);
opaque_identity!(UiMountedNodeReceiptIdentity);
opaque_identity!(UiMountedPresentationAttemptIdentity);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostSurfacePresentationMode {
    NativeDisplay,
    RecordOnly,
}
