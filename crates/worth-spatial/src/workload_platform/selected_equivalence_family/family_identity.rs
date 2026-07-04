#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpatialSelectedEquivalenceFamilyIdentity {
    EvidenceLookupSemanticParity,
    RetainedCancellationSemanticParity,
    RetainedReplaySemanticParity,
}

impl SpatialSelectedEquivalenceFamilyIdentity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceLookupSemanticParity => {
                "spatial.selected-equivalence.evidence-lookup-semantic-parity"
            }
            Self::RetainedCancellationSemanticParity => {
                "spatial.selected-equivalence.retained-cancellation-semantic-parity"
            }
            Self::RetainedReplaySemanticParity => {
                "spatial.selected-equivalence.retained-replay-semantic-parity"
            }
        }
    }

    pub const fn compiled_product_family_identity(
        self,
    ) -> crate::spatial_compiled_product_family::SpatialCompiledProductFamilyIdentity {
        match self {
            Self::EvidenceLookupSemanticParity => {
                crate::spatial_compiled_product_family::SpatialCompiledProductFamilyIdentity::EvidenceLookupDerivedSupport
            }
            Self::RetainedCancellationSemanticParity => {
                crate::spatial_compiled_product_family::SpatialCompiledProductFamilyIdentity::RetainedCancellationDerivedSupport
            }
            Self::RetainedReplaySemanticParity => {
                crate::spatial_compiled_product_family::SpatialCompiledProductFamilyIdentity::RetainedReplayDerivedSupport
            }
        }
    }
}
