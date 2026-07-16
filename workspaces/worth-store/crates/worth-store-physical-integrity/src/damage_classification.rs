use crate::{
    AmbiguousBoundaryDamage, AuthorityDamageBoundary, IndeterminatePhysicalDamage,
    PhysicalLocalityReport, RebuildableDerivedDamage, UnrecoverableAuthorityDamage,
};
use worth_store_physical_format::PhysicalReferenceScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DamageClassification {
    IntactPhysicalBoundary(IntactPhysicalBoundary),
    RebuildableDerivedDamage(Box<RebuildableDerivedDamage>),
    QuarantinedPhysicalDamage(QuarantinedPhysicalDamage),
    UnrecoverableAuthorityDamage(UnrecoverableAuthorityDamage),
    IndeterminatePhysicalDamage(IndeterminatePhysicalDamage),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntactPhysicalBoundary {
    scope: PhysicalReferenceScope,
}

impl IntactPhysicalBoundary {
    pub(crate) const fn new(scope: PhysicalReferenceScope) -> Self {
        Self { scope }
    }

    pub const fn scope(self) -> PhysicalReferenceScope {
        self.scope
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuarantinedPhysicalDamage {
    locality: PhysicalLocalityReport,
    authority_boundary: Option<AuthorityDamageBoundary>,
    ambiguous_boundary: Option<AmbiguousBoundaryDamage>,
}

impl QuarantinedPhysicalDamage {
    pub(crate) const fn exact(locality: PhysicalLocalityReport) -> Self {
        Self {
            locality,
            authority_boundary: None,
            ambiguous_boundary: None,
        }
    }

    pub(crate) const fn ambiguous(
        locality: PhysicalLocalityReport,
        boundary: AmbiguousBoundaryDamage,
    ) -> Self {
        Self {
            locality,
            authority_boundary: None,
            ambiguous_boundary: Some(boundary),
        }
    }

    pub const fn locality(self) -> PhysicalLocalityReport {
        self.locality
    }

    pub const fn authority_boundary(self) -> Option<AuthorityDamageBoundary> {
        self.authority_boundary
    }

    pub const fn ambiguous_boundary(self) -> Option<AmbiguousBoundaryDamage> {
        self.ambiguous_boundary
    }
}
