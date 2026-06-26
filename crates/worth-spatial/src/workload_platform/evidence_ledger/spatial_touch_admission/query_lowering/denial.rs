use forge_query::facade::runtime::ForgeQueryGraphTouchDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialEvidenceQueryLoweringDenial {
    kind: SpatialEvidenceQueryLoweringDenialKind,
    detail: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialEvidenceQueryLoweringDenialKind {
    RawRowSubstitution,
    CopiedReceiptSubstitution,
    TopologyTouchedBasisSubstitution,
    QueryDescriptorSubstitution,
    LookupProductMismatch,
}

impl SpatialEvidenceQueryLoweringDenial {
    pub fn raw_row_substitution(surface: &str) -> Self {
        Self::new(
            SpatialEvidenceQueryLoweringDenialKind::RawRowSubstitution,
            format!("{surface} cannot lower to a Query descriptor without spatial touch authority"),
        )
    }

    pub fn copied_receipt_substitution(surface: &str) -> Self {
        Self::new(
            SpatialEvidenceQueryLoweringDenialKind::CopiedReceiptSubstitution,
            format!(
                "{surface} copied receipt fields cannot lower to a Query descriptor without spatial touch authority"
            ),
        )
    }

    pub fn topology_touched_basis_substitution(surface: &str) -> Self {
        Self::new(
            SpatialEvidenceQueryLoweringDenialKind::TopologyTouchedBasisSubstitution,
            format!(
                "{surface} topology touched-basis proof cannot lower spatial evidence without spatial touch authority"
            ),
        )
    }

    pub fn query_descriptor_substitution(digest: &str) -> Self {
        Self::new(
            SpatialEvidenceQueryLoweringDenialKind::QueryDescriptorSubstitution,
            format!(
                "Query descriptor {digest} cannot lower itself or reconstruct spatial touch authority"
            ),
        )
    }

    pub(super) fn lookup_product_mismatch(expected: &str, actual: &str) -> Self {
        Self::new(
            SpatialEvidenceQueryLoweringDenialKind::LookupProductMismatch,
            format!("expected lookup product for spatial touch {expected}, got {actual}"),
        )
    }

    fn new(kind: SpatialEvidenceQueryLoweringDenialKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> SpatialEvidenceQueryLoweringDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

pub fn deny_raw_row_as_spatial_query_lowering_authority(
    surface: &str,
) -> SpatialEvidenceQueryLoweringDenial {
    SpatialEvidenceQueryLoweringDenial::raw_row_substitution(surface)
}

pub fn deny_copied_receipt_fields_as_spatial_query_lowering_authority(
    surface: &str,
) -> SpatialEvidenceQueryLoweringDenial {
    SpatialEvidenceQueryLoweringDenial::copied_receipt_substitution(surface)
}

pub fn deny_topology_touched_basis_as_spatial_query_lowering_authority(
    surface: &str,
) -> SpatialEvidenceQueryLoweringDenial {
    SpatialEvidenceQueryLoweringDenial::topology_touched_basis_substitution(surface)
}

pub fn deny_query_descriptor_as_spatial_query_lowering_authority(
    descriptor: &ForgeQueryGraphTouchDescriptor,
) -> SpatialEvidenceQueryLoweringDenial {
    SpatialEvidenceQueryLoweringDenial::query_descriptor_substitution(
        descriptor.descriptor_digest(),
    )
}
