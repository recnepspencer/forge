use crate::{
    AmbiguousBoundaryDamage, AuthorityDamageBoundary, ChunkDamageLocality, ChunkIntegrityDenial,
    DamageClassification, DerivedDamageClassification, IndexPageIntegrityDenial,
    IndexPageIntegrityReport, IntactPhysicalBoundary, PageIntegrityReport,
    PhysicalBoundaryLocalization, PhysicalContainerIntegrityDenial, PhysicalLocalityReport,
    QuarantineSealDenial, QuarantineSealDenialKind, QuarantinedPhysicalDamage,
    UnrecoverableAuthorityDamage, WalFrameDamageDenial,
};
use worth_store_physical_format::PhysicalReferenceScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedQuarantineFinding {
    locality: PhysicalLocalityReport,
    damage_classification: DamageClassification,
}

impl ExecutedQuarantineFinding {
    pub fn authoritative_quarantine(scope: PhysicalReferenceScope) -> Self {
        let locality = PhysicalLocalityReport::exact_reference_scope(scope);
        let damage = QuarantinedPhysicalDamage::exact(locality);
        Self {
            locality,
            damage_classification: DamageClassification::QuarantinedPhysicalDamage(damage),
        }
    }

    pub fn unresolved_authority(
        scope: PhysicalReferenceScope,
        boundary: AuthorityDamageBoundary,
    ) -> Self {
        Self {
            locality: PhysicalLocalityReport::exact_reference_scope(scope),
            damage_classification: DamageClassification::UnrecoverableAuthorityDamage(
                UnrecoverableAuthorityDamage::new(boundary, Some(scope.owner())),
            ),
        }
    }

    pub fn intact_page(report: &PageIntegrityReport) -> Self {
        let locality = PhysicalLocalityReport::exact_scope(report.basis());
        Self {
            locality,
            damage_classification: DamageClassification::IntactPhysicalBoundary(
                IntactPhysicalBoundary::new(report.basis().scope()),
            ),
        }
    }

    pub fn from_container_denial(
        denial: &PhysicalContainerIntegrityDenial,
    ) -> Result<Self, QuarantineSealDenial> {
        let basis = denial.basis().ok_or_else(|| {
            QuarantineSealDenial::new(QuarantineSealDenialKind::MissingExecutedPhysicalBasis)
        })?;
        let locality = match denial.ambiguous_boundary_damage() {
            Some(damage) => PhysicalLocalityReport::broader_boundary(basis, damage),
            None => PhysicalLocalityReport::exact_scope(basis),
        };
        let damage = match denial.ambiguous_boundary_damage() {
            Some(ambiguous) => QuarantinedPhysicalDamage::ambiguous(locality, ambiguous),
            None => QuarantinedPhysicalDamage::exact(locality),
        };
        Ok(Self {
            locality,
            damage_classification: DamageClassification::QuarantinedPhysicalDamage(damage),
        })
    }

    pub fn from_wal_frame_denial(
        denial: &WalFrameDamageDenial,
    ) -> Result<Self, QuarantineSealDenial> {
        let basis = denial.basis().ok_or_else(|| {
            QuarantineSealDenial::new(QuarantineSealDenialKind::MissingExecutedPhysicalBasis)
        })?;
        let locality = PhysicalLocalityReport::exact_scope(basis);
        let damage = QuarantinedPhysicalDamage::exact(locality);
        Ok(Self {
            locality,
            damage_classification: DamageClassification::QuarantinedPhysicalDamage(damage),
        })
    }

    pub fn from_index_page_report(report: &IndexPageIntegrityReport) -> Self {
        match report.damage_classification() {
            DerivedDamageClassification::IntactIndexPage(boundary) => Self {
                locality: PhysicalLocalityReport::exact_reference_scope(boundary.scope()),
                damage_classification: DamageClassification::IntactPhysicalBoundary(
                    IntactPhysicalBoundary::new(boundary.scope()),
                ),
            },
            DerivedDamageClassification::RebuildableDerived(damage) => Self {
                locality: PhysicalLocalityReport::exact_reference_scope(damage.damaged_scope()),
                damage_classification: DamageClassification::RebuildableDerivedDamage(
                    damage.clone(),
                ),
            },
            DerivedDamageClassification::Indeterminate(damage) => Self {
                locality: PhysicalLocalityReport::exact_reference_scope(damage.scope()),
                damage_classification: DamageClassification::IndeterminatePhysicalDamage(
                    damage.clone(),
                ),
            },
            DerivedDamageClassification::UnrecoverableAuthority(damage) => Self {
                locality: PhysicalLocalityReport::exact_scope(report.derived_basis()),
                damage_classification: DamageClassification::UnrecoverableAuthorityDamage(
                    damage.clone(),
                ),
            },
        }
    }

    pub fn from_index_page_denial(
        denial: &IndexPageIntegrityDenial,
    ) -> Result<Self, QuarantineSealDenial> {
        if let Some(damage) = denial.indeterminate_damage() {
            return Ok(Self {
                locality: PhysicalLocalityReport::exact_reference_scope(damage.scope()),
                damage_classification: DamageClassification::IndeterminatePhysicalDamage(
                    damage.clone(),
                ),
            });
        }
        if let Some(damage) = denial.authority_damage() {
            let basis = denial.derived_basis().ok_or_else(|| {
                QuarantineSealDenial::new(QuarantineSealDenialKind::MissingExecutedPhysicalBasis)
            })?;
            return Ok(Self {
                locality: PhysicalLocalityReport::exact_scope(basis),
                damage_classification: DamageClassification::UnrecoverableAuthorityDamage(
                    damage.clone(),
                ),
            });
        }
        Err(QuarantineSealDenial::new(
            QuarantineSealDenialKind::MissingExecutedPhysicalBasis,
        ))
    }

    pub fn from_chunk_denial(denial: &ChunkIntegrityDenial) -> Result<Self, QuarantineSealDenial> {
        let basis = denial.basis().ok_or_else(|| {
            QuarantineSealDenial::new(QuarantineSealDenialKind::MissingExecutedPhysicalBasis)
        })?;
        let ambiguous_damage = match denial.damage_locality() {
            Some(ChunkDamageLocality::Unknown(_scope)) => Some(AmbiguousBoundaryDamage::new(
                PhysicalBoundaryLocalization::AmbiguousBoundary,
            )),
            _ => None,
        };
        let locality = match ambiguous_damage {
            Some(damage) => PhysicalLocalityReport::broader_boundary(basis, damage),
            None => PhysicalLocalityReport::exact_scope(basis),
        };
        let damage = match ambiguous_damage {
            Some(ambiguous) => QuarantinedPhysicalDamage::ambiguous(locality, ambiguous),
            None => QuarantinedPhysicalDamage::exact(locality),
        };
        Ok(Self {
            locality,
            damage_classification: DamageClassification::QuarantinedPhysicalDamage(damage),
        })
    }

    pub const fn locality(&self) -> PhysicalLocalityReport {
        self.locality
    }

    pub const fn damage_classification(&self) -> &DamageClassification {
        &self.damage_classification
    }
}
