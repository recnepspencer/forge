mod authority_adoption_rows;
mod evidence_rows;
mod installed_operation_rows;
mod model;
mod test_backend_rows;

use authority_adoption_rows::AUTHORITY_ADOPTION_ROWS;
use evidence_rows::EVIDENCE_ROWS;
use installed_operation_rows::INSTALLED_OPERATION_ROWS;
use test_backend_rows::TEST_BACKEND_ROWS;

pub use model::{
    WorthQueryConsumerResidueClass, WorthQueryConsumerResidueDetection,
    WorthQueryConsumerResidueRegistryRow,
};

const REGISTRY_ROW_COUNT: usize = TEST_BACKEND_ROWS.len()
    + EVIDENCE_ROWS.len()
    + AUTHORITY_ADOPTION_ROWS.len()
    + INSTALLED_OPERATION_ROWS.len();

const CONSUMER_RESIDUE_REGISTRY: [WorthQueryConsumerResidueRegistryRow; REGISTRY_ROW_COUNT] =
    assemble_registry();

pub fn worth_query_consumer_residue_registry() -> &'static [WorthQueryConsumerResidueRegistryRow] {
    &CONSUMER_RESIDUE_REGISTRY
}

pub fn worth_query_test_backend_residue_classes() -> Vec<WorthQueryConsumerResidueClass> {
    CONSUMER_RESIDUE_REGISTRY
        .iter()
        .map(WorthQueryConsumerResidueRegistryRow::class)
        .filter(|class| class.is_test_backend_residue())
        .collect()
}

pub(crate) fn registry_row_for_class(
    class: WorthQueryConsumerResidueClass,
) -> &'static WorthQueryConsumerResidueRegistryRow {
    CONSUMER_RESIDUE_REGISTRY
        .iter()
        .find(|row| row.class() == class)
        .expect("every consumer residue class must have a registry row")
}

const fn assemble_registry() -> [WorthQueryConsumerResidueRegistryRow; REGISTRY_ROW_COUNT] {
    let mut rows = [TEST_BACKEND_ROWS[0]; REGISTRY_ROW_COUNT];
    let mut target = 0;
    let mut source = 0;
    while source < TEST_BACKEND_ROWS.len() {
        rows[target] = TEST_BACKEND_ROWS[source];
        source += 1;
        target += 1;
    }
    source = 0;
    while source < EVIDENCE_ROWS.len() {
        rows[target] = EVIDENCE_ROWS[source];
        source += 1;
        target += 1;
    }
    source = 0;
    while source < AUTHORITY_ADOPTION_ROWS.len() {
        rows[target] = AUTHORITY_ADOPTION_ROWS[source];
        source += 1;
        target += 1;
    }
    source = 0;
    while source < INSTALLED_OPERATION_ROWS.len() {
        rows[target] = INSTALLED_OPERATION_ROWS[source];
        source += 1;
        target += 1;
    }
    rows
}

#[rustfmt::skip]
pub(super) const fn registry_row(class: WorthQueryConsumerResidueClass, detection: WorthQueryConsumerResidueDetection, detection_key: &'static str, explanation: &'static str, replacement_lane: &'static str) -> WorthQueryConsumerResidueRegistryRow {
    WorthQueryConsumerResidueRegistryRow::new(class, detection, detection_key, explanation, replacement_lane)
}
