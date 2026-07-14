use crate::{
    IntegrityEntryDenial, IntegrityEntryDenialKind, IntegrityEntryRequest, IntegrityEntryWitness,
    IntegrityInspectionLease,
};
use worth_store_contracts::PhysicalIntegrityReadinessPayload;

#[derive(Debug, PartialEq, Eq)]
pub struct IntegrityEntryAdmission {
    payload: PhysicalIntegrityReadinessPayload,
}

impl IntegrityEntryAdmission {
    pub fn from_physical_integrity_payload(
        payload: PhysicalIntegrityReadinessPayload,
    ) -> Result<Self, IntegrityEntryDenial> {
        payload.require_complete()?;
        Ok(Self { payload })
    }

    pub fn admit<'lease>(
        self,
        request: IntegrityEntryRequest<'lease>,
    ) -> Result<IntegrityInspectionLease<'lease>, IntegrityEntryDenial> {
        reject_missing_protected_view(request)?;
        Ok(IntegrityInspectionLease::new(
            request.protected_view(),
            IntegrityEntryWitness::mint(self.payload),
        ))
    }
}

fn reject_missing_protected_view(
    request: IntegrityEntryRequest<'_>,
) -> Result<(), IntegrityEntryDenial> {
    if request.protected_view().is_empty() {
        Err(IntegrityEntryDenial::new(
            IntegrityEntryDenialKind::MissingProtectedPhysicalByteView,
        ))
    } else {
        Ok(())
    }
}
