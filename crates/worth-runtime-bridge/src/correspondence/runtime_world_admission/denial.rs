/// A Runtime World correspondence admission failed before any component
/// effect. No fallback correspondence or equal-looking descriptor is valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeWorldCorrespondenceAdmissionDenial {
    ForeignBridgeRuntime {
        expected_runtime_key: u64,
        actual_runtime_key: u64,
    },
}
