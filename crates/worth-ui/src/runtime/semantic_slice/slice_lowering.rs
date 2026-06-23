use std::collections::BTreeMap;

use crate::runtime::{
    WorthUiAdmittedRuntimeChangeEvidence, WorthUiRuntimeChangeFamily, WorthUiRuntimeFactFamily,
    WorthUiRuntimeFactId,
};

use super::{WorthUiSemanticSliceDescriptor, WorthUiSemanticSliceInventory};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSemanticSliceLoweringCause {
    ExactRuntimeFactFamily(WorthUiRuntimeFactFamily),
    CompositeRuntimeFactFamily(WorthUiRuntimeFactFamily),
    QueryOwnedPostureProjection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticChangedSliceRow {
    descriptor: &'static WorthUiSemanticSliceDescriptor,
    cause: WorthUiSemanticSliceLoweringCause,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthUiSemanticChangedSliceSet {
    rows: Vec<WorthUiSemanticChangedSliceRow>,
}

impl WorthUiSemanticChangedSliceSet {
    pub fn lower_runtime_change(
        inventory: &WorthUiSemanticSliceInventory,
        evidence: &WorthUiAdmittedRuntimeChangeEvidence,
    ) -> Self {
        let mut rows = BTreeMap::new();
        for family_row in evidence.family_rows() {
            for fact in family_row.changed_facts().facts().facts() {
                insert_rows_for_fact_family(inventory, fact, family_row.family(), &mut rows);
            }
        }
        Self {
            rows: rows.into_values().collect(),
        }
    }

    pub(crate) fn from_rows(rows: Vec<WorthUiSemanticChangedSliceRow>) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &[WorthUiSemanticChangedSliceRow] {
        &self.rows
    }

    pub fn contains_slice_id(&self, slice_id: crate::runtime::WorthUiSemanticSliceId) -> bool {
        self.rows
            .iter()
            .any(|row| row.descriptor().id() == slice_id)
    }
}

impl WorthUiSemanticChangedSliceRow {
    pub(crate) fn new(
        descriptor: &'static WorthUiSemanticSliceDescriptor,
        cause: WorthUiSemanticSliceLoweringCause,
    ) -> Self {
        Self { descriptor, cause }
    }

    pub fn descriptor(&self) -> &'static WorthUiSemanticSliceDescriptor {
        self.descriptor
    }

    pub fn cause(&self) -> WorthUiSemanticSliceLoweringCause {
        self.cause
    }
}

fn insert_rows_for_fact_family(
    inventory: &WorthUiSemanticSliceInventory,
    fact: &WorthUiRuntimeFactId,
    runtime_change_family: WorthUiRuntimeChangeFamily,
    rows: &mut BTreeMap<crate::runtime::WorthUiSemanticSliceId, WorthUiSemanticChangedSliceRow>,
) {
    let fact_family = fact.family();
    for descriptor in inventory.slices() {
        let cause = match descriptor.runtime_fact_mapping() {
            crate::runtime::WorthUiSemanticSliceFactMapping::Exact(family)
                if family == fact_family =>
            {
                Some(WorthUiSemanticSliceLoweringCause::ExactRuntimeFactFamily(
                    fact_family,
                ))
            }
            crate::runtime::WorthUiSemanticSliceFactMapping::Composite(families)
                if families.contains(&fact_family) =>
            {
                Some(WorthUiSemanticSliceLoweringCause::CompositeRuntimeFactFamily(fact_family))
            }
            crate::runtime::WorthUiSemanticSliceFactMapping::Gap => None,
            crate::runtime::WorthUiSemanticSliceFactMapping::Exact(_)
            | crate::runtime::WorthUiSemanticSliceFactMapping::Composite(_) => None,
        };
        if let Some(cause) = cause {
            rows.entry(descriptor.id())
                .or_insert_with(|| WorthUiSemanticChangedSliceRow::new(descriptor, cause));
        }
    }

    if runtime_change_family == WorthUiRuntimeChangeFamily::QueryBinding {
        // Query-owned exact families should lower through the same changed-fact
        // path as every other runtime fact. Gap slices are projected separately.
    }
}
