mod external_observation;
mod progression;
mod request;

pub(crate) use external_observation::{
    UiNativePhysicalSignalExternalBasis, UiNativePhysicalSignalExternalObservation,
    UiNativePhysicalSignalExternalOrigin, UiNativePhysicalSignalExternalStatus,
};
pub(crate) use request::{
    UiNativePhysicalSignalRequestToken, UiNativePhysicalSignalRoute, UiNativePhysicalSignalWork,
};
