#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupFamilyIndexPostureKind {
    SparseLookupPlanRequired,
    BoundedDenseLookupPlanRequired,
    IndexNotRequiredForDeclarationOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupFamilyIndexPosture {
    kind: EvidenceLookupFamilyIndexPostureKind,
}

impl EvidenceLookupFamilyIndexPosture {
    pub(crate) const fn sparse_lookup_plan_required() -> Self {
        Self {
            kind: EvidenceLookupFamilyIndexPostureKind::SparseLookupPlanRequired,
        }
    }

    pub(crate) const fn bounded_dense_lookup_plan_required() -> Self {
        Self {
            kind: EvidenceLookupFamilyIndexPostureKind::BoundedDenseLookupPlanRequired,
        }
    }

    pub const fn kind(&self) -> EvidenceLookupFamilyIndexPostureKind {
        self.kind
    }
}
