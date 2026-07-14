use crate::{AmbiguousBoundaryDamage, PhysicalBoundaryLocalization, PhysicalScopeBasis};
use worth_store_physical_format::{PhysicalGenerationOwner, PhysicalReferenceScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuarantineLocalityBoundary {
    ExactPhysicalScope(PhysicalReferenceScope),
    BroaderPhysicalBoundary(PhysicalReferenceScope, PhysicalBoundaryLocalization),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalLocalityReport {
    boundary: QuarantineLocalityBoundary,
    owner: PhysicalGenerationOwner,
}

impl PhysicalLocalityReport {
    pub(crate) const fn exact_scope(basis: &PhysicalScopeBasis) -> Self {
        Self {
            boundary: QuarantineLocalityBoundary::ExactPhysicalScope(basis.scope()),
            owner: basis.scope().owner(),
        }
    }

    pub(crate) const fn exact_reference_scope(scope: PhysicalReferenceScope) -> Self {
        Self {
            boundary: QuarantineLocalityBoundary::ExactPhysicalScope(scope),
            owner: scope.owner(),
        }
    }

    pub(crate) const fn broader_boundary(
        basis: &PhysicalScopeBasis,
        damage: AmbiguousBoundaryDamage,
    ) -> Self {
        Self {
            boundary: QuarantineLocalityBoundary::BroaderPhysicalBoundary(
                basis.scope(),
                damage.boundary(),
            ),
            owner: basis.scope().owner(),
        }
    }

    pub const fn boundary(self) -> QuarantineLocalityBoundary {
        self.boundary
    }

    pub const fn owner(self) -> PhysicalGenerationOwner {
        self.owner
    }
}
