use super::classification::DerivedInvalidationAuthorityDisposition;
use super::error::{
    DerivedInvalidationAuthorityInventoryError, DerivedInvalidationAuthorityInventoryErrorKind,
};
use super::row::DerivedInvalidationAuthorityInventoryRow;

pub struct DerivedInvalidationOrdinaryProofAdmission;

impl DerivedInvalidationOrdinaryProofAdmission {
    pub fn admit_inventory_row(
        row: &DerivedInvalidationAuthorityInventoryRow,
    ) -> Result<(), DerivedInvalidationAuthorityInventoryError> {
        if row.disposition()
            == DerivedInvalidationAuthorityDisposition::CertificationBootstrapResidue
        {
            return Err(DerivedInvalidationAuthorityInventoryError::new(
                DerivedInvalidationAuthorityInventoryErrorKind::CertificationResidueCannotSatisfyOrdinaryInvalidation {
                    surface: row.surface().to_string(),
                },
                format!(
                    "certification/bootstrap residue `{}` cannot satisfy ordinary invalidation proof",
                    row.surface()
                ),
            ));
        }
        Ok(())
    }
}
