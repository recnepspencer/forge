#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionFamilyPosture {
    Registered,
    RebuildRequired,
    Rejected,
}
