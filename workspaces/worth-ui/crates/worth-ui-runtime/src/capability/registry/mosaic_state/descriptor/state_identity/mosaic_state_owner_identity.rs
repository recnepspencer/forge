use crate::capability::{MosaicRegionKindId, MosaicStateOwnerScopeId, SurfaceId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicStateOwnerIdentity {
    MosaicRegionKind(MosaicRegionKindId),
    Surface(SurfaceId),
    RuntimeScope(MosaicStateOwnerScopeId),
    MissingForDiagnostics,
}

impl MosaicStateOwnerIdentity {
    pub fn mosaic_region_kind(id: MosaicRegionKindId) -> Self {
        Self::MosaicRegionKind(id)
    }

    pub fn surface(id: SurfaceId) -> Self {
        Self::Surface(id)
    }

    pub fn runtime_scope(scope: MosaicStateOwnerScopeId) -> Self {
        Self::RuntimeScope(scope)
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::MosaicRegionKind(id) => format!("mosaic_region_kind:{}", id.as_str()),
            Self::Surface(id) => format!("surface:{}", id.as_str()),
            Self::RuntimeScope(scope) => format!("runtime_scope:{}", scope.as_str()),
            Self::MissingForDiagnostics => "missing".to_string(),
        }
    }
}
