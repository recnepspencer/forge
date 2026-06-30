use crate::workload_platform::evidence_lookup_family_catalog::{
    EvidenceLookupFamilyIndexPosture, EvidenceLookupFamilyIndexPostureKind,
};

use super::error::EvidenceLookupPlanSelectionError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupSelectedStrategyKind {
    SparseIndexedLookupPlan,
    BoundedDenseIndexedLookupPlan,
    DeclarationOnlyNoIndex,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupSelectedStrategy {
    kind: EvidenceLookupSelectedStrategyKind,
}

impl EvidenceLookupSelectedStrategy {
    pub(crate) fn from_index_posture(
        posture: &EvidenceLookupFamilyIndexPosture,
    ) -> Result<Self, EvidenceLookupPlanSelectionError> {
        let kind = match posture.kind() {
            EvidenceLookupFamilyIndexPostureKind::SparseLookupPlanRequired => {
                EvidenceLookupSelectedStrategyKind::SparseIndexedLookupPlan
            }
            EvidenceLookupFamilyIndexPostureKind::BoundedDenseLookupPlanRequired => {
                EvidenceLookupSelectedStrategyKind::BoundedDenseIndexedLookupPlan
            }
            EvidenceLookupFamilyIndexPostureKind::IndexNotRequiredForDeclarationOnly => {
                EvidenceLookupSelectedStrategyKind::DeclarationOnlyNoIndex
            }
        };
        Ok(Self { kind })
    }

    pub const fn kind(&self) -> EvidenceLookupSelectedStrategyKind {
        self.kind
    }

    pub const fn is_indexed_lookup_plan(&self) -> bool {
        matches!(
            self.kind,
            EvidenceLookupSelectedStrategyKind::SparseIndexedLookupPlan
                | EvidenceLookupSelectedStrategyKind::BoundedDenseIndexedLookupPlan
        )
    }
}
