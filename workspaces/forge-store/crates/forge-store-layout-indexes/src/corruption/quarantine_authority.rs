use forge_store_physical_integrity::{DamageClassification, QuarantineRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LayoutQuarantineAuthorityClass {
    DerivedProjectionDamage,
    AuthoritativeQuarantineRequired,
}

pub(super) fn classify_quarantine_authority(
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
