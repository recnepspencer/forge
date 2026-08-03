use crate::domain_installation::{
    WorthQueryCollectionConsumerWindow, WorthQueryCollectionRowAccessDenial,
    WorthQueryCollectionRowHandle, WorthQueryNativeAccessKey,
};
use crate::projection_consumption::{ConsumedNativeValue, ConsumedNativeValueView};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryCollectionNativeAccessCounters {
    pub capability_checks: usize,
    pub window_row_checks: usize,
    pub selected_key_checks: usize,
    pub indexed_row_lookups: usize,
    pub native_facts_materialized: usize,
}

pub struct WorthQueryCollectionNativeFactAccess {
    row_identity: crate::memory_workspace::WorthQueryEntityIdentity,
    value: ConsumedNativeValue,
    counters: WorthQueryCollectionNativeAccessCounters,
}

impl WorthQueryCollectionConsumerWindow {
    pub fn native_value(
        &self,
        row: &WorthQueryCollectionRowHandle,
        key: &WorthQueryNativeAccessKey,
    ) -> Result<WorthQueryCollectionNativeFactAccess, WorthQueryCollectionRowAccessDenial> {
        let mut counters = WorthQueryCollectionNativeAccessCounters {
            capability_checks: 1,
            ..WorthQueryCollectionNativeAccessCounters::default()
        };
        if row.capability_identity() != self.window.capability_identity
            || row.capability_generation() != self.window.capability_generation
        {
            return Err(WorthQueryCollectionRowAccessDenial::ForeignRowHandle);
        }
        counters.window_row_checks += 1;
        if self.window.rows().get(row.ordinal()) != Some(row) {
            return Err(WorthQueryCollectionRowAccessDenial::RowNotInWindow);
        }
        counters.selected_key_checks += 1;
        if !self.index.selects_native_key(key) {
            return Err(WorthQueryCollectionRowAccessDenial::ForeignNativeAccessKey);
        }
        counters.indexed_row_lookups += 1;
        let value = self
            .index
            .native_value(row.entity_identity(), key)
            .ok_or(WorthQueryCollectionRowAccessDenial::RowNotInWindow)?;
        counters.native_facts_materialized += 1;
        Ok(WorthQueryCollectionNativeFactAccess {
            row_identity: row.entity_identity().clone(),
            value,
            counters,
        })
    }
}

impl WorthQueryCollectionNativeFactAccess {
    pub fn row_identity(&self) -> &crate::memory_workspace::WorthQueryEntityIdentity {
        &self.row_identity
    }

    pub fn native_value(&self) -> ConsumedNativeValueView<'_> {
        self.value.view()
    }

    pub const fn counters(&self) -> WorthQueryCollectionNativeAccessCounters {
        self.counters
    }
}
