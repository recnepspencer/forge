use crate::{
    ScopedPhysicalValidatorInput, WalFrameDamageDenial, WalFrameDamageDenialKind,
    WalFrameIntegrityCounters, WalTailIntegrityPosture,
};
use worth_store_physical_format::PhysicalScopeFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalFrameIntegrityInspectionRequest<'lease> {
    input: ScopedPhysicalValidatorInput<'lease>,
}

impl<'lease> WalFrameIntegrityInspectionRequest<'lease> {
    pub fn from_admitted_wal_frame(
        input: ScopedPhysicalValidatorInput<'lease>,
    ) -> Result<Self, WalFrameDamageDenial> {
        reject_non_wal_family(&input)?;
        Ok(Self { input })
    }

    pub(crate) const fn input(&self) -> &ScopedPhysicalValidatorInput<'lease> {
        &self.input
    }
}

fn reject_non_wal_family(
    input: &ScopedPhysicalValidatorInput<'_>,
) -> Result<(), WalFrameDamageDenial> {
    if input.family() == PhysicalScopeFamily::WalFrame {
        return Ok(());
    }
    Err(WalFrameDamageDenial::new(
        WalFrameDamageDenialKind::WrongPhysicalFamily,
        WalTailIntegrityPosture::UnknownTailIntegrity,
        WalFrameIntegrityCounters::start(),
    )
    .with_basis(input.admission().basis().clone()))
}
