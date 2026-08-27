mod curve;
mod damage;
mod interruption;
mod receipt;
mod sampled_geometry;
mod sampling;
#[cfg(test)]
mod tests;
mod track_sampling;

pub(crate) use damage::{
    UiPresentationMotionDamage, UiPresentationMotionDamageRegion, UiPresentationSampledClipGeometry,
};
pub(crate) use receipt::{
    UiPresentationMotionInstallationReceipt, UiPresentationMotionSamplePosture,
    UiPresentationMotionSampleReceipt, UiPresentationMotionSamplingCost,
    UiPresentationMotionSamplingReceipt, UiPresentationMotionTerminalRequest,
    UiPresentationReducedMotionPosture,
};
pub(crate) use sampled_geometry::{
    UiPresentationGeometrySamplingDenial, UiPresentationSampledGeometry,
};
pub(crate) use sampling::{
    UiMountedMotionSampler, UiPreparedMotionSampling, UiPresentationMotionSamplingDenial,
};
