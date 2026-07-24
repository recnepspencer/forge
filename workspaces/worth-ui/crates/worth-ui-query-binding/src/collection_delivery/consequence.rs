use super::{
    WorthUiCollectionAllocationEffect, WorthUiCollectionChangeCounters,
    WorthUiCollectionChangeInspection, WorthUiCollectionChangeSourceReference,
    WorthUiCollectionGraphEffect, WorthUiCollectionMeasurementEffect,
    WorthUiCollectionQueryWorkInspection, WorthUiCollectionResetReason,
};

/// Move-only UI source evidence minted from one exact applied Query patch.
///
/// There is intentionally no constructor or Query conversion API.
pub struct WorthUiCollectionChangeConsequence {
    inner: std::sync::Arc<WorthUiCollectionChangeConsequenceInner>,
}

pub(crate) struct WorthUiRetainedCollectionChangeConsequence {
    inner: std::sync::Arc<WorthUiCollectionChangeConsequenceInner>,
}

#[derive(Debug)]
struct WorthUiCollectionChangeConsequenceInner {
    installed_reference: crate::WorthUiInstalledQueryBindingReference,
    source: WorthUiCollectionChangeSourceReference,
    change_order: u64,
    kind: WorthUiCollectionChangeKind,
    inspection: WorthUiCollectionChangeInspection,
    ui_counters: WorthUiCollectionChangeCounters,
    query_work: WorthUiCollectionQueryWorkInspection,
}

#[derive(Debug, Eq, PartialEq)]
pub enum WorthUiCollectionChangeKind {
    Incremental(WorthUiCollectionIncrementalConsequence),
    Reset(WorthUiCollectionResetConsequence),
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct WorthUiCollectionIncrementalConsequence {
    graph: Vec<WorthUiCollectionGraphEffect>,
    measurement: Vec<WorthUiCollectionMeasurementEffect>,
    allocation: Vec<WorthUiCollectionAllocationEffect>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiCollectionResetConsequence {
    reason: WorthUiCollectionResetReason,
    fresh_execution_required: bool,
    maximum_replacement_rows: usize,
}

pub(crate) struct WorthUiCollectionChangeConsequenceParts {
    pub installed_reference: crate::WorthUiInstalledQueryBindingReference,
    pub source: WorthUiCollectionChangeSourceReference,
    pub change_order: u64,
    pub kind: WorthUiCollectionChangeKind,
    pub inspection: WorthUiCollectionChangeInspection,
    pub ui_counters: WorthUiCollectionChangeCounters,
    pub query_work: WorthUiCollectionQueryWorkInspection,
}

impl WorthUiCollectionChangeConsequence {
    pub(crate) fn new(parts: WorthUiCollectionChangeConsequenceParts) -> Self {
        Self {
            inner: std::sync::Arc::new(WorthUiCollectionChangeConsequenceInner {
                installed_reference: parts.installed_reference,
                source: parts.source,
                change_order: parts.change_order,
                kind: parts.kind,
                inspection: parts.inspection,
                ui_counters: parts.ui_counters,
                query_work: parts.query_work,
            }),
        }
    }

    pub(crate) fn retain(&self) -> WorthUiRetainedCollectionChangeConsequence {
        WorthUiRetainedCollectionChangeConsequence {
            inner: std::sync::Arc::clone(&self.inner),
        }
    }

    pub(crate) fn installed_reference(&self) -> &crate::WorthUiInstalledQueryBindingReference {
        &self.inner.installed_reference
    }

    pub fn source(&self) -> &WorthUiCollectionChangeSourceReference {
        &self.inner.source
    }

    pub fn change_order(&self) -> u64 {
        self.inner.change_order
    }

    pub fn kind(&self) -> &WorthUiCollectionChangeKind {
        &self.inner.kind
    }

    pub fn inspection(&self) -> WorthUiCollectionChangeInspection {
        self.inner.inspection
    }

    pub fn ui_counters(&self) -> WorthUiCollectionChangeCounters {
        self.inner.ui_counters
    }

    pub fn query_work(&self) -> WorthUiCollectionQueryWorkInspection {
        self.inner.query_work
    }
}

impl WorthUiRetainedCollectionChangeConsequence {
    pub(crate) fn matches(&self, consequence: &WorthUiCollectionChangeConsequence) -> bool {
        std::sync::Arc::ptr_eq(&self.inner, &consequence.inner)
    }

    pub(crate) fn handoff(&self) -> WorthUiCollectionChangeConsequence {
        WorthUiCollectionChangeConsequence {
            inner: std::sync::Arc::clone(&self.inner),
        }
    }
}

impl std::fmt::Debug for WorthUiCollectionChangeConsequence {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(formatter)
    }
}

impl WorthUiCollectionIncrementalConsequence {
    pub(crate) fn new(
        graph: Vec<WorthUiCollectionGraphEffect>,
        measurement: Vec<WorthUiCollectionMeasurementEffect>,
        allocation: Vec<WorthUiCollectionAllocationEffect>,
    ) -> Self {
        Self {
            graph,
            measurement,
            allocation,
        }
    }

    pub fn graph(&self) -> &[WorthUiCollectionGraphEffect] {
        &self.graph
    }

    pub fn measurement(&self) -> &[WorthUiCollectionMeasurementEffect] {
        &self.measurement
    }

    pub fn allocation(&self) -> &[WorthUiCollectionAllocationEffect] {
        &self.allocation
    }
}

impl WorthUiCollectionResetConsequence {
    pub(crate) fn new(
        reason: WorthUiCollectionResetReason,
        fresh_execution_required: bool,
        maximum_replacement_rows: usize,
    ) -> Self {
        Self {
            reason,
            fresh_execution_required,
            maximum_replacement_rows,
        }
    }

    pub fn reason(&self) -> WorthUiCollectionResetReason {
        self.reason
    }

    pub fn fresh_execution_required(&self) -> bool {
        self.fresh_execution_required
    }

    pub fn maximum_replacement_rows(&self) -> usize {
        self.maximum_replacement_rows
    }
}
