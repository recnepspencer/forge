use crate::branch::OwnerRetirementWork;
use crate::identity::ProductUnpublishedOwnerEffectsIdentity;
use crate::recovery::{ProductUnpublishedCause, ProductUnpublishedNextAction};

/// One row per retained obligation the owner exposed instead of discarding.
/// A row is exposure only: it carries no capability to settle or delete the
/// obligation it describes.
#[derive(Debug, Clone)]
pub struct RuntimeWorldRetainedRecordReport {
    identity: ProductUnpublishedOwnerEffectsIdentity,
    cause: ProductUnpublishedCause,
    obligations: RetainedObligationSplit,
    next_actions: Vec<ProductUnpublishedNextAction>,
}

/// A retained record's own live obligation count, divided into the
/// component-scoped half and the composite-scoped remainder. It is one value
/// rather than two counts because the halves are only meaningful together:
/// they must sum to the record's own count, and nothing outside this type may
/// name a pair that does not.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RetainedObligationSplit {
    component: usize,
    composite: usize,
}

impl RetainedObligationSplit {
    /// Charge `component_charge` of a record's `live` obligations to the
    /// component half and leave the rest composite.
    ///
    /// Both retention postures charge exactly 2 component-scoped obligations,
    /// and a retained record's live count is 3 when it kept its successor
    /// installed and 2 when it released its history slot before the product
    /// reference moved, so the clamp is unreachable today: the charge equals
    /// the count in the smaller case and is one below it in the larger. It
    /// stays because the split must remain total: a future posture that
    /// charges more components than the record has obligations must report a
    /// zero composite half, never underflow.
    pub(crate) fn new(live: usize, component_charge: usize) -> Self {
        let component = component_charge.min(live);
        Self {
            component,
            composite: live - component,
        }
    }
}

impl RuntimeWorldRetainedRecordReport {
    pub(crate) fn new(
        identity: ProductUnpublishedOwnerEffectsIdentity,
        cause: ProductUnpublishedCause,
        obligations: RetainedObligationSplit,
        next_actions: Vec<ProductUnpublishedNextAction>,
    ) -> Self {
        Self {
            identity,
            cause,
            obligations,
            next_actions,
        }
    }

    pub const fn identity(&self) -> &ProductUnpublishedOwnerEffectsIdentity {
        &self.identity
    }

    pub const fn cause(&self) -> ProductUnpublishedCause {
        self.cause
    }

    pub const fn live_component_obligations(&self) -> usize {
        self.obligations.component
    }

    pub const fn live_composite_obligations(&self) -> usize {
        self.obligations.composite
    }

    pub fn next_actions(&self) -> &[ProductUnpublishedNextAction] {
        &self.next_actions
    }
}

/// Terminal artifact of a completed close. Enumerating a retained record is
/// exposure, never settlement, and never a discarded owner obligation.
#[derive(Debug)]
#[must_use = "a close report enumerates obligations the caller must still address"]
pub struct RuntimeWorldCloseReport {
    retained_records: Vec<RuntimeWorldRetainedRecordReport>,
    settled_records: usize,
    released_product_head_pins: usize,
    released_observation_pins: usize,
    released_history_pins: usize,
    released_unique_component_pins: usize,
    retired_owner_created_custody: usize,
    outstanding_owner_retirement_work: Vec<OwnerRetirementWork>,
}

/// The counted half of a close report. The drain fills every field from what
/// it actually released; no count is inferred from a budget limit.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct RuntimeWorldCloseReleaseCounts {
    pub(crate) settled_records: usize,
    pub(crate) released_product_head_pins: usize,
    pub(crate) released_observation_pins: usize,
    pub(crate) released_history_pins: usize,
    pub(crate) released_unique_component_pins: usize,
    pub(crate) retired_owner_created_custody: usize,
}

impl RuntimeWorldCloseReport {
    pub(crate) fn new(
        retained_records: Vec<RuntimeWorldRetainedRecordReport>,
        counts: RuntimeWorldCloseReleaseCounts,
        outstanding_owner_retirement_work: Vec<OwnerRetirementWork>,
    ) -> Self {
        let RuntimeWorldCloseReleaseCounts {
            settled_records,
            released_product_head_pins,
            released_observation_pins,
            released_history_pins,
            released_unique_component_pins,
            retired_owner_created_custody,
        } = counts;
        Self {
            retained_records,
            settled_records,
            released_product_head_pins,
            released_observation_pins,
            released_history_pins,
            released_unique_component_pins,
            retired_owner_created_custody,
            outstanding_owner_retirement_work,
        }
    }

    pub fn retained_records(&self) -> &[RuntimeWorldRetainedRecordReport] {
        &self.retained_records
    }

    pub const fn settled_records(&self) -> usize {
        self.settled_records
    }

    pub const fn released_product_head_pins(&self) -> usize {
        self.released_product_head_pins
    }

    pub const fn released_observation_pins(&self) -> usize {
        self.released_observation_pins
    }

    pub const fn released_history_pins(&self) -> usize {
        self.released_history_pins
    }

    pub const fn released_unique_component_pins(&self) -> usize {
        self.released_unique_component_pins
    }

    pub const fn retired_owner_created_custody(&self) -> usize {
        self.retired_owner_created_custody
    }

    pub fn outstanding_owner_retirement_work(&self) -> &[OwnerRetirementWork] {
        &self.outstanding_owner_retirement_work
    }
}
