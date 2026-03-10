use crate::facade::AspectVersion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PrimaryAuditSurface {
    pub desk: AspectVersion,
    pub scenario: AspectVersion,
}

impl PrimaryAuditSurface {
    pub(super) fn new(desk: AspectVersion, scenario: AspectVersion) -> Self {
        Self { desk, scenario }
    }
}
