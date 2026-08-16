use crate::domain_installation::{
    WorthQueryBoundCollectionWindow, WorthQueryCollectionRowHandle,
    WorthQueryConsumerInvalidationAuthority, WorthQueryFoundationalInvalidationProjection,
    WorthQueryImpactClass, WorthQueryNativeAccessKey,
};
use crate::memory_workspace::WorthQueryEntityIdentity;
use crate::projection_consumption::{ConsumedNativeValue, ConsumedNativeValueView};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryCollectionDeliveryCounters {
    pub invalidation_authority_checks: usize,
    pub lease_checks: usize,
    pub generation_checks: usize,
    pub cursor_checks: usize,
    pub semantic_contract_checks: usize,
    pub pending_patch_checks: usize,
    pub prior_window_rows_visited: usize,
    pub fresh_window_rows_visited: usize,
    pub affected_identity_lookups: usize,
    pub entity_point_lookups: usize,
    pub ordering_index_updates: usize,
    pub operations_materialized: usize,
    pub native_facts_materialized: usize,
    pub full_collection_scans: usize,
    pub unrelated_consumer_scans: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCollectionDeliveryDenialKind {
    UnsupportedCollectionDelivery,
    ForeignOrStaleInvalidation,
    SourceMismatch,
    BindingMismatch,
    ResultShapeMismatch,
    CollectionContractMismatch,
    OrderingMismatch,
    BasisMismatch,
    NoSemanticWindowEffect,
    ForeignCollectionCapability,
    CapabilityGenerationMismatch,
    WindowContractMismatch,
    CursorMismatch,
    WrongLease,
    SupersededPatch,
    DuplicateOrReorderedDelivery,
    ResetPending,
}

#[derive(Debug)]
pub struct WorthQueryCollectionDeliveryDenial {
    kind: WorthQueryCollectionDeliveryDenialKind,
    counters: WorthQueryCollectionDeliveryCounters,
}

impl WorthQueryCollectionDeliveryDenial {
    pub(super) const fn new(
        kind: WorthQueryCollectionDeliveryDenialKind,
        counters: WorthQueryCollectionDeliveryCounters,
    ) -> Self {
        Self { kind, counters }
    }

    pub const fn kind(&self) -> WorthQueryCollectionDeliveryDenialKind {
        self.kind
    }

    pub const fn counters(&self) -> WorthQueryCollectionDeliveryCounters {
        self.counters
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCollectionResetReason {
    ReexecutionRequired,
    CapabilityRebindRequired,
    ReplacementRequired,
    RetirementRequired,
    UnsupportedIncrementalMeaning,
    UnappliedPriorPatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryCollectionResetCost {
    pub fresh_execution_required: bool,
    pub maximum_replacement_rows: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryCollectionPatchOperation {
    Insert {
        row: WorthQueryCollectionRowHandle,
        at: usize,
    },
    Remove {
        entity: WorthQueryEntityIdentity,
        from: usize,
    },
    Move {
        row: WorthQueryCollectionRowHandle,
        from: usize,
        to: usize,
    },
    Regroup {
        entity: WorthQueryEntityIdentity,
        from: Option<Vec<String>>,
        to: Option<Vec<String>>,
    },
    Update {
        row: WorthQueryCollectionRowHandle,
    },
    WindowShift {
        first_row: Option<WorthQueryEntityIdentity>,
    },
    ResultState {
        state: crate::domain_installation::WorthQueryOperationResultState,
    },
    Warnings {
        warnings: Vec<crate::domain_installation::WorthQueryCollectionWindowWarning>,
    },
    Continuation {
        continuation: crate::domain_installation::WorthQueryCollectionContinuation,
    },
    ResetRequired {
        reason: WorthQueryCollectionResetReason,
        cost: WorthQueryCollectionResetCost,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryCollectionPatchFact {
    row: WorthQueryEntityIdentity,
    key: WorthQueryNativeAccessKey,
    value: ConsumedNativeValue,
}

impl WorthQueryCollectionPatchFact {
    pub(super) fn new(
        row: WorthQueryEntityIdentity,
        key: WorthQueryNativeAccessKey,
        value: ConsumedNativeValue,
    ) -> Self {
        Self { row, key, value }
    }

    pub fn row_identity(&self) -> &WorthQueryEntityIdentity {
        &self.row
    }

    pub fn key(&self) -> &WorthQueryNativeAccessKey {
        &self.key
    }

    pub fn native_value(&self) -> ConsumedNativeValueView<'_> {
        self.value.view()
    }
}

pub struct WorthQueryCollectionPatch {
    pub(super) authority: WorthQueryConsumerInvalidationAuthority,
    pub(super) maintenance_ordinal: u64,
    pub(super) impact: WorthQueryImpactClass,
    pub(super) prior_cursor: crate::domain_installation::WorthQueryCollectionCursor,
    pub(super) next: WorthQueryBoundCollectionWindow,
    pub(super) operations: Vec<WorthQueryCollectionPatchOperation>,
    pub(super) facts: Vec<WorthQueryCollectionPatchFact>,
    pub(super) foundational_invalidation: WorthQueryFoundationalInvalidationProjection,
    pub(super) counters: WorthQueryCollectionDeliveryCounters,
    pub(super) index_delta: Option<super::index::WorthQueryCollectionIndexDelta>,
}

impl WorthQueryCollectionPatch {
    pub fn operations(&self) -> &[WorthQueryCollectionPatchOperation] {
        &self.operations
    }

    pub fn facts(&self) -> &[WorthQueryCollectionPatchFact] {
        &self.facts
    }

    pub const fn foundational_invalidation(&self) -> &WorthQueryFoundationalInvalidationProjection {
        &self.foundational_invalidation
    }

    pub const fn impact(&self) -> WorthQueryImpactClass {
        self.impact
    }

    pub const fn counters(&self) -> WorthQueryCollectionDeliveryCounters {
        self.counters
    }

    pub const fn maintenance_ordinal(&self) -> u64 {
        self.maintenance_ordinal
    }

    pub const fn authority(&self) -> &WorthQueryConsumerInvalidationAuthority {
        &self.authority
    }
}

pub enum WorthQueryCollectionDeliveryOutcome {
    Patch(WorthQueryCollectionPatch),
    NoDelivery(WorthQueryCollectionDeliveryDenial),
}

pub struct WorthQueryCollectionPatchApplicationReceipt {
    operations: Vec<WorthQueryCollectionPatchOperation>,
    facts: Vec<WorthQueryCollectionPatchFact>,
    foundational_invalidation: WorthQueryFoundationalInvalidationProjection,
    maintenance_ordinal: u64,
    counters: WorthQueryCollectionDeliveryCounters,
    reset_required: bool,
}

pub(crate) struct WorthQueryPerformedCollectionStateMutation {
    pub(crate) operations: Vec<WorthQueryCollectionPatchOperation>,
    pub(crate) facts: Vec<WorthQueryCollectionPatchFact>,
    pub(crate) counters: WorthQueryCollectionDeliveryCounters,
    pub(crate) rows: Vec<WorthQueryCollectionRowHandle>,
}

pub(crate) struct WorthQueryPendingCollectionStateMutation {
    pub(super) delta: super::index::WorthQueryCollectionIndexDelta,
    pub(super) next: WorthQueryBoundCollectionWindow,
    pub(super) next_maintenance_ordinal: u64,
}

pub(super) struct WorthQueryCollectionPatchApplicationParts {
    pub operations: Vec<WorthQueryCollectionPatchOperation>,
    pub facts: Vec<WorthQueryCollectionPatchFact>,
    pub foundational_invalidation: WorthQueryFoundationalInvalidationProjection,
    pub maintenance_ordinal: u64,
    pub counters: WorthQueryCollectionDeliveryCounters,
    pub reset_required: bool,
}

impl WorthQueryCollectionPatchApplicationReceipt {
    pub(super) fn new(parts: WorthQueryCollectionPatchApplicationParts) -> Self {
        Self {
            operations: parts.operations,
            facts: parts.facts,
            foundational_invalidation: parts.foundational_invalidation,
            maintenance_ordinal: parts.maintenance_ordinal,
            counters: parts.counters,
            reset_required: parts.reset_required,
        }
    }

    pub fn operations(&self) -> &[WorthQueryCollectionPatchOperation] {
        &self.operations
    }

    pub fn facts(&self) -> &[WorthQueryCollectionPatchFact] {
        &self.facts
    }

    pub const fn foundational_invalidation(&self) -> &WorthQueryFoundationalInvalidationProjection {
        &self.foundational_invalidation
    }

    pub const fn maintenance_ordinal(&self) -> u64 {
        self.maintenance_ordinal
    }

    pub const fn counters(&self) -> WorthQueryCollectionDeliveryCounters {
        self.counters
    }

    pub const fn reset_required(&self) -> bool {
        self.reset_required
    }
}
