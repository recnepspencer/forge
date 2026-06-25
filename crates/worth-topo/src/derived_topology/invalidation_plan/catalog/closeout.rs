use std::collections::BTreeSet;

use serde::Serialize;

use super::{
    DerivedInvalidationFamilyCatalog, DerivedInvalidationFamilyCatalogError,
    DerivedInvalidationFamilyCatalogErrorKind, DerivedInvalidationFamilySourceCoverage,
    DerivedInvalidationPhaseThreeSeed, DerivedTopologyProductFamilyIdentity,
};
use crate::derived_topology::invalidation_plan::inventory::DerivedInvalidationAuthorityInventoryReport;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationFamilyCatalogCloseout {
    catalog: DerivedInvalidationFamilyCatalog,
    phase_three_seed: DerivedInvalidationPhaseThreeSeed,
}

impl DerivedInvalidationFamilyCatalogCloseout {
    pub fn close(
        catalog: DerivedInvalidationFamilyCatalog,
    ) -> Result<Self, DerivedInvalidationFamilyCatalogError> {
        validate_no_duplicates(&catalog)?;
        validate_required_families(&catalog)?;
        let phase_three_seed = DerivedInvalidationPhaseThreeSeed::from_catalog(&catalog);
        Ok(Self {
            catalog,
            phase_three_seed,
        })
    }

    pub const fn catalog(&self) -> &DerivedInvalidationFamilyCatalog {
        &self.catalog
    }

    pub const fn phase_three_seed(&self) -> &DerivedInvalidationPhaseThreeSeed {
        &self.phase_three_seed
    }

    pub fn validate_source_coverage(
        &self,
        inventory: &DerivedInvalidationAuthorityInventoryReport,
    ) -> Result<DerivedInvalidationFamilySourceCoverage, DerivedInvalidationFamilyCatalogError>
    {
        DerivedInvalidationFamilySourceCoverage::validate(self.catalog(), inventory)
    }

    pub fn require_family_query_support_present(
        &self,
        identity: DerivedTopologyProductFamilyIdentity,
    ) -> Result<(), DerivedInvalidationFamilyCatalogError> {
        let family = self.catalog.family(identity).ok_or_else(|| {
            DerivedInvalidationFamilyCatalogError::new(
                DerivedInvalidationFamilyCatalogErrorKind::MissingRequiredFamily {
                    family: identity.as_str(),
                },
                format!("derived product family `{}` is missing", identity.as_str()),
            )
        })?;
        if family.query_receipt_posture().requires_query_support() {
            return Err(DerivedInvalidationFamilyCatalogError::new(
                DerivedInvalidationFamilyCatalogErrorKind::QuerySupportRequired {
                    family: identity.as_str(),
                },
                format!(
                    "derived product family `{}` requires Query support before invalidation execution",
                    identity.as_str()
                ),
            ));
        }
        Ok(())
    }
}

fn validate_no_duplicates(
    catalog: &DerivedInvalidationFamilyCatalog,
) -> Result<(), DerivedInvalidationFamilyCatalogError> {
    let mut observed = BTreeSet::new();
    for family in catalog.families() {
        if !observed.insert(family.identity()) {
            return Err(DerivedInvalidationFamilyCatalogError::new(
                DerivedInvalidationFamilyCatalogErrorKind::DuplicateFamily {
                    family: family.identity().as_str(),
                },
                format!(
                    "derived product family `{}` appears more than once",
                    family.identity().as_str()
                ),
            ));
        }
    }
    Ok(())
}

fn validate_required_families(
    catalog: &DerivedInvalidationFamilyCatalog,
) -> Result<(), DerivedInvalidationFamilyCatalogError> {
    for required in DerivedTopologyProductFamilyIdentity::REQUIRED {
        if catalog.family(required).is_none() {
            return Err(DerivedInvalidationFamilyCatalogError::new(
                DerivedInvalidationFamilyCatalogErrorKind::MissingRequiredFamily {
                    family: required.as_str(),
                },
                format!(
                    "derived invalidation family catalog is missing `{}`",
                    required.as_str()
                ),
            ));
        }
    }
    Ok(())
}
