use forge_store_aspect_native::{
    canonical_basis_source_owner_for_family, certify_canonical_basis_field_role,
    StoreCanonicalBasisFamily, StoreCanonicalBasisFieldRole, StoreCanonicalBasisSourceDenial,
    STORE_CANONICAL_BASIS_SOURCE_OWNERS,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreCanonicalBasisInventoryRow {
    family: Option<StoreCanonicalBasisFamily>,
    family_name: &'static str,
    classifying_subsystem: &'static str,
}

impl StoreCanonicalBasisInventoryRow {
    pub const fn owned(
        family: StoreCanonicalBasisFamily,
        classifying_subsystem: &'static str,
    ) -> Self {
        Self {
            family: Some(family),
            family_name: family.canonical_basis_family_label(),
            classifying_subsystem,
        }
    }

    pub const fn unclassified(
        family_name: &'static str,
        classifying_subsystem: &'static str,
    ) -> Self {
        Self {
            family: None,
            family_name,
            classifying_subsystem,
        }
    }

    pub const fn family_name(&self) -> &'static str {
        self.family_name
    }

    pub const fn classifying_subsystem(&self) -> &'static str {
        self.classifying_subsystem
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
    DuplicateSourceOwner(StoreCanonicalBasisFamily),
    ForbiddenCanonicalBasisField(StoreCanonicalBasisFieldRole),
}

pub fn certify_store_canonical_basis_source_inventory(
) -> Result<(), StoreCanonicalBasisInventoryDenial> {
    certify_store_canonical_basis_source_rows(&current_store_canonical_basis_inventory())?;
    certify_source_owner_map_has_no_duplicates()?;
    certify_forbidden_fields_are_not_basis_sources()
}

pub fn current_store_canonical_basis_inventory() -> Vec<StoreCanonicalBasisInventoryRow> {
    StoreCanonicalBasisFamily::ALL
        .into_iter()
        .map(|family| {
            let subsystem = canonical_basis_source_owner_for_family(family)
                .map(|owner| owner.classifying_subsystem())
                .unwrap_or("unclassified Store evidence family");
            StoreCanonicalBasisInventoryRow::owned(family, subsystem)
        })
        .collect()
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
