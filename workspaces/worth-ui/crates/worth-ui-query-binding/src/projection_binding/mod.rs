mod admission;
mod binding;
mod collection_compatibility;
mod collection_live;
mod compatibility;
mod scalar_native_request_stop;
mod stop;

pub(crate) use admission::admit_collection_registration;
pub(crate) use admission::admit_scalar_registration;
pub use admission::{UiCollectionProjectionBindingAdmission, UiScalarProjectionBindingAdmission};
pub use binding::{UiCollectionProjectionBinding, UiProjectionBinding, UiScalarProjectionBinding};
pub use collection_compatibility::{
    UiCollectionProjectionReplacementOutcome, UiCollectionProjectionReplacementReceipt,
    UiCollectionProjectionReplacementStop,
};
pub use collection_live::{
    UiCollectionProjectionOpenOutcome, UiCollectionProjectionOpenReceipt,
    UiCollectionProjectionOpenStop, UiCollectionProjectionOpenStopKind,
    UiCollectionProjectionRefreshError, UiCollectionProjectionRefreshOutcome,
    UiCollectionProjectionRefreshReceipt, UiLiveCollectionProjection,
    UiLiveCollectionProjectionCloseOutcome, UiLiveCollectionProjectionCloseReceipt,
    UiLiveCollectionProjectionCloseStop,
};
pub use compatibility::{
    UiProjectionBindingCompatibilityProof, UiScalarProjectionReplacementOutcome,
    UiScalarProjectionReplacementReceipt, UiScalarProjectionReplacementStop,
};
pub use stop::{UiProjectionBindingStopKind, UiProjectionBindingStopReceipt};
