#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactFormatVersion(pub u32);
