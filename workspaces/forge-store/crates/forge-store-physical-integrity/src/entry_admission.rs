use crate::{
    IntegrityEntryDenial, IntegrityEntryDenialKind, IntegrityEntryRequest, IntegrityEntryWitness,
    IntegrityInspectionLease,
};
use forge_store_readiness::S3PhysicalIntegrityReadiness;

#[derive(Debug, PartialEq, Eq)]
pub struct IntegrityEntryAdmission {
    readiness: S3PhysicalIntegrityReadiness,
}

impl IntegrityEntryAdmission {
    pub fn from_s3_readiness(
        readiness: S3PhysicalIntegrityReadiness,
    ) -> Result<Self, IntegrityEntryDenial> {
        readiness.payload().require_complete()?;
        Ok(Self { readiness })
    }

    pub fn admit<'lease>(
        self,
        request: IntegrityEntryRequest<'lease>,
    ) -> Result<IntegrityInspectionLease<'lease>, IntegrityEntryDenial> {
        reject_missing_protected_view(request)?;
        Ok(IntegrityInspectionLease::new(
            request.protected_view(),
            IntegrityEntryWitness::mint(self.readiness),
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
