use crate::capability::CapabilitySnapshot;
use crate::runtime::WorthUiRuntimeFactSet;

use super::{
    WorthUiCapabilityReloadFamilyCounters, WorthUiCapabilityReloadFamilyKind,
    WorthUiComponentReloadReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiCapabilityFamilyDelta {
    family: WorthUiCapabilityReloadFamilyKind,
    snapshot: CapabilitySnapshot,
    counters: WorthUiCapabilityReloadFamilyCounters,
    changed_facts: WorthUiRuntimeFactSet,
    component_reload_receipt: Option<WorthUiComponentReloadReceipt>,
}

impl WorthUiCapabilityFamilyDelta {
    pub(crate) fn new(
        family: WorthUiCapabilityReloadFamilyKind,
        snapshot: CapabilitySnapshot,
        counters: WorthUiCapabilityReloadFamilyCounters,
        changed_facts: WorthUiRuntimeFactSet,
    ) -> Self {
        Self {
            family,
            snapshot,
            counters,
            changed_facts,
            component_reload_receipt: None,
        }
    }

    pub(crate) fn with_component_reload_receipt(
        family: WorthUiCapabilityReloadFamilyKind,
        snapshot: CapabilitySnapshot,
        counters: WorthUiCapabilityReloadFamilyCounters,
        changed_facts: WorthUiRuntimeFactSet,
        component_reload_receipt: Option<WorthUiComponentReloadReceipt>,
    ) -> Self {
        Self {
            family,
            snapshot,
            counters,
            changed_facts,
            component_reload_receipt,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthUiCapabilityReloadFamilyKind,
        CapabilitySnapshot,
        WorthUiCapabilityReloadFamilyCounters,
        WorthUiRuntimeFactSet,
        Option<WorthUiComponentReloadReceipt>,
    ) {
        (
            self.family,
            self.snapshot,
            self.counters,
            self.changed_facts,
            self.component_reload_receipt,
        )
    }
}
