mod certification_expectation_rows;
mod invariant_registration_rows;
mod old_authority_region_rows;
mod operator_closeout_rows;
mod rule_registry_rows;
mod validator_facade_rows;

use super::source::WorthValidationAuthoritySource;
use crate::validation_authority_inventory::inventory_row::{
    WorthValidationAuthorityInventoryRow, WorthValidationAuthorityInventoryRowInput,
};

pub(in crate::validation_authority_inventory) fn current_validation_authority_rows(
) -> Vec<WorthValidationAuthorityInventoryRow> {
    let mut rows = Vec::new();
    validator_facade_rows::push_validator_facade_rows(&mut rows);
    rule_registry_rows::push_rule_registry_rows(&mut rows);
    invariant_registration_rows::push_invariant_registration_rows(&mut rows);
    certification_expectation_rows::push_certification_expectation_rows(&mut rows);
    operator_closeout_rows::push_operator_closeout_rows(&mut rows);
    old_authority_region_rows::push_old_authority_region_rows(&mut rows);
    rows
}

pub(in crate::validation_authority_inventory) fn required_validation_authority_sources(
) -> Vec<WorthValidationAuthoritySource> {
    current_validation_authority_rows()
        .into_iter()
        .map(|row| row.source())
        .collect()
}

fn row(input: WorthValidationAuthorityInventoryRowInput) -> WorthValidationAuthorityInventoryRow {
    WorthValidationAuthorityInventoryRow::from_input(input)
}
