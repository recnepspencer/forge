use super::{
    CoverageBasisKind, LayoutMaterializationSourceIdentity, LayoutWatermark, PhysicalCoverageBasis,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedCoverageBasis {
    source: LayoutMaterializationSourceIdentity,
    watermark: LayoutWatermark,
}

impl AdmittedCoverageBasis {
    pub(super) fn admit(
        source: LayoutMaterializationSourceIdentity,
        basis: &PhysicalCoverageBasis,
    ) -> Self {
        Self {
            source,
            watermark: basis.watermark(),
        }
    }

    pub const fn source(&self) -> &LayoutMaterializationSourceIdentity {
        &self.source
    }

    pub const fn basis_kind(&self) -> CoverageBasisKind {
        self.watermark.basis_kind()
    }

    pub const fn watermark(&self) -> LayoutWatermark {
        self.watermark
    }
}
