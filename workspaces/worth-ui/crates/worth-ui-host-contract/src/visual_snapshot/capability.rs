#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostCaptureCapability {
    Unsupported,
    GeometryOnly,
    Pixels {
        maximum_bytes: u64,
        exact_presentation_epoch: bool,
    },
}
