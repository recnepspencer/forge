use std::collections::BTreeSet;

use crate::consumer_kit::support_pinning::error::{
    ForgeQuerySupportPinningError, ForgeQuerySupportPinningErrorKind,
};
use crate::consumer_kit::support_pinning::evidence::{
    derive_support_pin_contract_identity, derive_support_pin_observed_row_identity,
    derive_support_pin_requirement_identity,
};
use crate::consumer_kit::support_pinning::observed_row::ForgeQueryObservedSupportPin;
use crate::consumer_kit::support_pinning::requirement::ForgeQuerySupportPinRequirement;
use crate::runtime::ForgeQueryRuntimeFacadeFamily;

pub(super) fn rebuild_contract_digest(
    consumer_name: &str,
    contract_schema_identity: &str,
    pinned_vocabulary_identity: &str,
    support_snapshot_schema_identity: &str,
    source_matrix_digest: &str,
    requirements: &[ForgeQuerySupportPinRequirement],
    observed_rows: &[ForgeQueryObservedSupportPin],
) -> String {
    let requirement_identities = requirements
        .iter()
        .map(derive_support_pin_requirement_identity)
        .collect::<Vec<_>>();
    let observed_identities = observed_rows
        .iter()
        .map(derive_support_pin_observed_row_identity)
        .collect::<Vec<_>>();
    derive_support_pin_contract_identity(
        consumer_name,
        contract_schema_identity,
        pinned_vocabulary_identity,
        support_snapshot_schema_identity,
        source_matrix_digest,
        &requirement_identities,
        &observed_identities,
    )
    .terminal_projection_for_reporting()
    .to_string()
}

pub(super) fn reject_duplicate_document_families(
    requirements: &[ForgeQuerySupportPinRequirement],
    observed_rows: &[ForgeQueryObservedSupportPin],
) -> Result<(), ForgeQuerySupportPinningError> {
    let mut required = BTreeSet::new();
    let mut observed = BTreeSet::new();
    for row in requirements {
        reject_family_insert(&mut required, row.family(), true)?;
    }
    for row in observed_rows {
        reject_family_insert(&mut observed, row.family(), false)?;
        if required.contains(&row.family()) {
            return Err(duplicate_family_error(
                ForgeQuerySupportPinningErrorKind::RequiredObservedFamilyConflict,
                row.family(),
            ));
        }
    }
    Ok(())
}

fn reject_family_insert(
    set: &mut BTreeSet<ForgeQueryRuntimeFacadeFamily>,
    family: ForgeQueryRuntimeFacadeFamily,
    required: bool,
) -> Result<(), ForgeQuerySupportPinningError> {
    if set.insert(family) {
        Ok(())
    } else if required {
        Err(duplicate_family_error(
            ForgeQuerySupportPinningErrorKind::DuplicateRequiredFamily,
            family,
        ))
    } else {
        Err(duplicate_family_error(
            ForgeQuerySupportPinningErrorKind::DuplicateObservedFamily,
            family,
        ))
    }
}

fn duplicate_family_error(
    kind: ForgeQuerySupportPinningErrorKind,
    family: ForgeQueryRuntimeFacadeFamily,
) -> ForgeQuerySupportPinningError {
    ForgeQuerySupportPinningError::with_family(
        kind,
        "support pin contract contains duplicate or conflicting family declarations",
        family.as_str(),
    )
}
