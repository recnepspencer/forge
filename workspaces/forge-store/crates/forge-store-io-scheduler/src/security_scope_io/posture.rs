#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecureIoPostureRequirement {
    ScopePreserving,
    SecureFrameCompatible,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecureIoPosture {
    ScopePreserving,
    SecureFrameCompatible,
}
