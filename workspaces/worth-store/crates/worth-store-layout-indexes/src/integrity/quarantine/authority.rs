use worth_store_physical_integrity::{DamageClassification, QuarantineRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::integrity) enum LayoutQuarantineAuthorityClass {
    DerivedProjectionDamage,
    AuthoritativeQuarantineRequired,
}

pub(in crate::integrity) fn classify_quarantine_authority(
    record: &QuarantineRecord,
) -> LayoutQuarantineAuthorityClass {
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
