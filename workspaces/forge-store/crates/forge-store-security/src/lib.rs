#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TenantScopeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyVersion(pub u64);
