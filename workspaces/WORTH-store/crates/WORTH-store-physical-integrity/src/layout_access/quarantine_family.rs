use crate::{DamageClassification, QuarantineRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutQuarantineAuthorityClass {
    DerivedProjectionDamage,
    AuthoritativeQuarantineRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuarantineLayoutFamilyHome;

impl QuarantineLayoutFamilyHome {
    pub fn authority_class(&self, record: &QuarantineRecord) -> LayoutQuarantineAuthorityClass {
        match record.damage_classification() {
            DamageClassification::RebuildableDerivedDamage(_) => {
                LayoutQuarantineAuthorityClass::DerivedProjectionDamage
            }
            DamageClassification::IntactPhysicalBoundary(_)
            | DamageClassification::QuarantinedPhysicalDamage(_)
            | DamageClassification::UnrecoverableAuthorityDamage(_)
            | DamageClassification::IndeterminatePhysicalDamage(_) => {
                LayoutQuarantineAuthorityClass::AuthoritativeQuarantineRequired
            }
        }
    }
}

pub const fn quarantine_layout_family() -> QuarantineLayoutFamilyHome {
    QuarantineLayoutFamilyHome
}
