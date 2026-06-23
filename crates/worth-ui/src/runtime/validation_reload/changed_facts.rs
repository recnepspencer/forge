use crate::runtime::{
    runtime_fact::WorthUiAuthoredStructuralRuntimeFactLowering, WorthUiAuthoredDeltaSummary,
    WorthUiRuntimeFactSet,
};

pub(super) fn derive_validation_changed_fact_mapping_receipt(
    authored_delta_summary: Option<WorthUiAuthoredDeltaSummary>,
) -> Option<crate::runtime::WorthUiValidationChangedFactMappingReceipt> {
    let authored_delta_summary = authored_delta_summary?;
    let rows = WorthUiAuthoredStructuralRuntimeFactLowering::from_authored_delta_summary(
        &authored_delta_summary,
    );
    let mut changed_facts = WorthUiRuntimeFactSet::empty();
    for row in &rows {
        changed_facts.extend(row.changed_facts().facts().cloned());
    }
    Some(
        crate::runtime::WorthUiValidationChangedFactMappingReceipt::new(
            authored_delta_summary,
            rows,
            changed_facts,
        ),
    )
}
