use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ResourcePolicyKind {
    Retry,
    Timeout,
    Cancellation,
    StaleAfter,
    Supersession,
    Revalidation,
    Observation,
    OutputContinuity,
    Retention,
    Diagnostics,
    Replay,
}

impl ResourcePolicyKind {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Retry => "retry",
            Self::Timeout => "timeout",
            Self::Cancellation => "cancellation",
            Self::StaleAfter => "stale-after",
            Self::Supersession => "supersession",
            Self::Revalidation => "revalidation",
            Self::Observation => "observation",
            Self::OutputContinuity => "output-continuity",
            Self::Retention => "retention",
            Self::Diagnostics => "diagnostics",
            Self::Replay => "replay",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourcePolicyDescriptorId(u64);

impl ResourcePolicyDescriptorId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourcePolicyVersion {
    major: u16,
    minor: u16,
}

impl ResourcePolicyVersion {
    pub const INITIAL: Self = Self { major: 1, minor: 0 };

    pub fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    pub fn major(self) -> u16 {
        self.major
    }

    pub fn minor(self) -> u16 {
        self.minor
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourcePolicyDigest(String);

impl ResourcePolicyDigest {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourcePolicyCompatibilityPosture {
    ExactDescriptorMatch,
    CompatibleVersion,
    IncompatibleVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourcePolicySelectionBasis {
    BuiltInDefault,
    DeclaredBuiltIn,
    DeclaredName,
}

impl ResourcePolicySelectionBasis {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::BuiltInDefault => "built-in-default",
            Self::DeclaredBuiltIn => "declared-built-in",
            Self::DeclaredName => "declared-name",
        }
    }
}
