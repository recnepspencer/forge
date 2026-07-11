use crate::courtroom::foundational::canonical_basis_source_registry::STORE_CANONICAL_BASIS_FAMILY_REGISTRY;
use crate::courtroom::foundational::canonical_basis_source_scan::certify_scanned_store_canonical_basis_families_are_registered;
use forge_store_aspect_native::{
    canonical_basis_source_owner_for_family, certify_canonical_basis_field_role,
    StoreCanonicalBasisFamily, StoreCanonicalBasisFieldRole, StoreCanonicalBasisSourceDenial,
    STORE_CANONICAL_BASIS_SOURCE_OWNERS,
};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreCanonicalBasisInventoryRow {
    family: Option<StoreCanonicalBasisFamily>,
    family_name: &'static str,
    source_path: &'static str,
    classifying_subsystem: &'static str,
}

impl StoreCanonicalBasisInventoryRow {
    pub const fn registered(
        family_name: &'static str,
        source_path: &'static str,
        family: StoreCanonicalBasisFamily,
        classifying_subsystem: &'static str,
    ) -> Self {
        Self {
            family: Some(family),
            family_name,
            source_path,
            classifying_subsystem,
        }
    }

    pub const fn unclassified(
        family_name: &'static str,
        source_path: &'static str,
        classifying_subsystem: &'static str,
    ) -> Self {
        Self {
            family: None,
            family_name,
            source_path,
            classifying_subsystem,
        }
    }

    pub const fn family_name(&self) -> &'static str {
        self.family_name
    }

    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn classifying_subsystem(&self) -> &'static str {
        self.classifying_subsystem
    }

    pub const fn family(&self) -> Option<StoreCanonicalBasisFamily> {
        self.family
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreCanonicalBasisInventoryDenial {
    MissingSourceOwner {
        family: StoreCanonicalBasisFamily,
        classifying_subsystem: &'static str,
    },
    UnclassifiedEvidenceFamily {
        family_name: &'static str,
        classifying_subsystem: &'static str,
    },
    ScannedUnregisteredEvidenceFamily {
        family_name: String,
        classifying_subsystem: String,
    },
    DuplicateSourceOwner(StoreCanonicalBasisFamily),
    ForbiddenCanonicalBasisField(StoreCanonicalBasisFieldRole),
}

pub fn certify_store_canonical_basis_source_inventory(
) -> Result<(), StoreCanonicalBasisInventoryDenial> {
    certify_store_canonical_basis_source_rows(&current_store_canonical_basis_inventory())?;
    certify_source_owner_map_has_no_duplicates()?;
    certify_forbidden_fields_are_not_basis_sources()
}

pub fn certify_scanned_store_canonical_basis_source_inventory(
    workspace_root: &Path,
    scope_roots: &[&str],
) -> Result<(), StoreCanonicalBasisInventoryDenial> {
    certify_store_canonical_basis_source_inventory()?;
    certify_scanned_store_canonical_basis_families_are_registered(
        workspace_root,
        scope_roots,
        STORE_CANONICAL_BASIS_FAMILY_REGISTRY,
    )
}

pub fn current_store_canonical_basis_inventory() -> Vec<StoreCanonicalBasisInventoryRow> {
    STORE_CANONICAL_BASIS_FAMILY_REGISTRY.to_vec()
}

pub fn certify_store_canonical_basis_source_rows(
    rows: &[StoreCanonicalBasisInventoryRow],
) -> Result<(), StoreCanonicalBasisInventoryDenial> {
    for row in rows {
        match row.family {
            Some(family) => {
                canonical_basis_source_owner_for_family(family)
                    .map_err(|denial| missing_owner_denial(denial, row.classifying_subsystem))?;
            }
            None => {
                return Err(
                    StoreCanonicalBasisInventoryDenial::UnclassifiedEvidenceFamily {
                        family_name: row.family_name,
                        classifying_subsystem: row.classifying_subsystem,
                    },
                );
            }
        }
    }

    Ok(())
}

fn certify_source_owner_map_has_no_duplicates() -> Result<(), StoreCanonicalBasisInventoryDenial> {
    for (index, owner) in STORE_CANONICAL_BASIS_SOURCE_OWNERS.iter().enumerate() {
        if STORE_CANONICAL_BASIS_SOURCE_OWNERS[index + 1..]
            .iter()
            .any(|candidate| candidate.family() == owner.family())
        {
            return Err(StoreCanonicalBasisInventoryDenial::DuplicateSourceOwner(
                owner.family(),
            ));
        }
    }

    Ok(())
}

fn certify_forbidden_fields_are_not_basis_sources() -> Result<(), StoreCanonicalBasisInventoryDenial>
{
    for field_role in [
        StoreCanonicalBasisFieldRole::TerminalProjection,
        StoreCanonicalBasisFieldRole::OperatorDisplay,
        StoreCanonicalBasisFieldRole::DocumentChecksum,
        StoreCanonicalBasisFieldRole::CompatibilityText,
        StoreCanonicalBasisFieldRole::DigestText,
        StoreCanonicalBasisFieldRole::RawJsonPayload,
    ] {
        let Err(StoreCanonicalBasisSourceDenial::ForbiddenFieldRole { .. }) =
            certify_canonical_basis_field_role(field_role)
        else {
            return Err(
                StoreCanonicalBasisInventoryDenial::ForbiddenCanonicalBasisField(field_role),
            );
        };
    }

    Ok(())
}

fn missing_owner_denial(
    denial: StoreCanonicalBasisSourceDenial,
    classifying_subsystem: &'static str,
) -> StoreCanonicalBasisInventoryDenial {
    match denial {
        StoreCanonicalBasisSourceDenial::MissingSourceOwner { family, .. } => {
            StoreCanonicalBasisInventoryDenial::MissingSourceOwner {
                family,
                classifying_subsystem,
            }
        }
        StoreCanonicalBasisSourceDenial::WrongNativeSourceKind { family, .. } => {
            StoreCanonicalBasisInventoryDenial::MissingSourceOwner {
                family,
                classifying_subsystem,
            }
        }
        StoreCanonicalBasisSourceDenial::ForbiddenFieldRole { field_role } => {
            StoreCanonicalBasisInventoryDenial::ForbiddenCanonicalBasisField(field_role)
        }
    }
}
