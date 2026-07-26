mod bounded_loader;
mod loaded_frame;
mod read_source;

pub(in crate::physical_runtime::record_serving) use super::frame_load_failure::{
    FrameLoadFailure, FrameLoadFailureKind, FrameLoadFaultCause,
};
pub(in crate::physical_runtime::record_serving) use bounded_loader::BoundedFrameLoader;
pub(in crate::physical_runtime::record_serving) use loaded_frame::LoadedPhysicalFrame;
pub(in crate::physical_runtime::record_serving::residency) use read_source::DirectFrameReadSource;
pub(in crate::physical_runtime::record_serving) use read_source::{
    CanonicalFrameReadSource, FrameLoadPort, ObservedArtifactLength,
};
