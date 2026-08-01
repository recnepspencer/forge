#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCollectionChangeStagingReceipt {
    source: crate::WorthUiCollectionChangeSourceReference,
    change_order: u64,
    counters: crate::WorthUiCollectionChangeCounters,
    query_work: crate::WorthUiCollectionQueryWorkInspection,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCollectionChangePublicationReceipt {
    published_change_count: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiOperationLiveChangeObservation {
    staged_change_count: usize,
    admitted_change_count: usize,
    next_change_order: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCollectionChangeAdmissionDenial {
    QueryNotInstalled,
    ForeignInstalledReference,
    ResourceNotRetained,
    StaleOrForeignConsequence,
    AlreadyAdmitted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCollectionChangeHandoffRetryDenial {
    QueryNotInstalled,
    ForeignInstalledReference,
    ResourceNotRetained,
    NoUnpublishedChange,
    AlreadyAdmittedToFrameworkTurn,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCollectionChangePublicationDenial {
    QueryNotInstalled,
    ForeignInstalledReference,
    ResourceNotRetained,
    StaleOrForeignAdmission,
    AdmissionNotActive,
}

/// Affine Query-owner authority for one exact admitted collection change.
///
/// Only Query binding admission can mint this token. It must be consumed by
/// exact publication or withdrawal; possession alone does not publish state.
#[must_use = "an admitted Query change must be published or withdrawn"]
pub struct WorthUiAdmittedCollectionChangePublication {
    consequence: crate::WorthUiCollectionChangeConsequence,
    receipt: WorthUiCollectionChangeStagingReceipt,
}

pub struct WorthUiCollectionChangePublicationStop {
    denial: WorthUiCollectionChangePublicationDenial,
    admission: Box<WorthUiAdmittedCollectionChangePublication>,
}

pub struct WorthUiCollectionChangeAdmissionStop {
    denial: WorthUiCollectionChangeAdmissionDenial,
    consequence: crate::WorthUiCollectionChangeConsequence,
}

/// Move-only Query-owned consequence after validation against the exact
/// retained operation-live resource.
///
/// Validation does not stage or publish the consequence. Runtime observation
/// may retain this handoff without mutating Query binding truth.
pub struct WorthUiValidatedCollectionChangeObservation {
    consequence: crate::WorthUiCollectionChangeConsequence,
    receipt: WorthUiCollectionChangeStagingReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiOperationLiveSourceRefreshOutcome {
    NoSemanticDelivery,
    Staged(WorthUiCollectionChangeStagingReceipt),
}

#[derive(Debug)]
pub enum WorthUiOperationLiveSourceRefreshStop {
    Progression(Box<crate::WorthUiOperationLiveRefreshError>),
    Publication(Box<WorthUiCollectionChangeAdmissionStop>),
}

impl WorthUiCollectionChangeStagingReceipt {
    pub(crate) fn from_consequence(
        consequence: &crate::WorthUiCollectionChangeConsequence,
    ) -> Self {
        Self {
            source: consequence.source().clone(),
            change_order: consequence.change_order(),
            counters: consequence.ui_counters(),
            query_work: consequence.query_work(),
        }
    }

    pub fn source(&self) -> &crate::WorthUiCollectionChangeSourceReference {
        &self.source
    }

    pub fn change_order(&self) -> u64 {
        self.change_order
    }

    pub fn counters(&self) -> crate::WorthUiCollectionChangeCounters {
        self.counters
    }

    pub fn query_work(&self) -> crate::WorthUiCollectionQueryWorkInspection {
        self.query_work
    }
}

impl WorthUiAdmittedCollectionChangePublication {
    pub(crate) fn seal(
        consequence: crate::WorthUiCollectionChangeConsequence,
        receipt: WorthUiCollectionChangeStagingReceipt,
    ) -> Self {
        Self {
            consequence,
            receipt,
        }
    }

    pub fn source(&self) -> &crate::WorthUiCollectionChangeSourceReference {
        self.receipt.source()
    }

    pub fn change_order(&self) -> u64 {
        self.receipt.change_order()
    }

    pub fn counters(&self) -> crate::WorthUiCollectionChangeCounters {
        self.receipt.counters()
    }

    pub fn query_work(&self) -> crate::WorthUiCollectionQueryWorkInspection {
        self.receipt.query_work()
    }

    pub(crate) fn installed_reference(&self) -> &crate::WorthUiInstalledQueryBindingReference {
        self.consequence.installed_reference()
    }

    pub(crate) fn consequence(&self) -> &crate::WorthUiCollectionChangeConsequence {
        &self.consequence
    }

    pub(crate) fn into_consequence(self) -> crate::WorthUiCollectionChangeConsequence {
        self.consequence
    }
}

impl WorthUiCollectionChangePublicationStop {
    pub(crate) fn new(
        denial: WorthUiCollectionChangePublicationDenial,
        admission: WorthUiAdmittedCollectionChangePublication,
    ) -> Self {
        Self {
            denial,
            admission: Box::new(admission),
        }
    }

    pub const fn denial(&self) -> WorthUiCollectionChangePublicationDenial {
        self.denial
    }

    pub fn into_admission(self) -> WorthUiAdmittedCollectionChangePublication {
        *self.admission
    }
}

impl std::fmt::Debug for WorthUiAdmittedCollectionChangePublication {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiAdmittedCollectionChangePublication")
            .field("receipt", &self.receipt)
            .field("consequence", &"sealed Query consequence")
            .finish()
    }
}

impl std::fmt::Debug for WorthUiCollectionChangePublicationStop {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiCollectionChangePublicationStop")
            .field("denial", &self.denial)
            .field("admission", &self.admission)
            .finish()
    }
}

impl WorthUiCollectionChangePublicationReceipt {
    pub(crate) fn new(published_change_count: usize) -> Self {
        Self {
            published_change_count,
        }
    }

    pub fn published_change_count(self) -> usize {
        self.published_change_count
    }
}

impl WorthUiOperationLiveChangeObservation {
    pub(crate) fn new(
        staged_change_count: usize,
        admitted_change_count: usize,
        next_change_order: u64,
    ) -> Self {
        Self {
            staged_change_count,
            admitted_change_count,
            next_change_order,
        }
    }

    pub fn staged_change_count(self) -> usize {
        self.staged_change_count
    }

    pub fn admitted_change_count(self) -> usize {
        self.admitted_change_count
    }

    pub fn next_change_order(self) -> u64 {
        self.next_change_order
    }
}

impl WorthUiCollectionChangeAdmissionStop {
    pub(crate) fn new(
        denial: WorthUiCollectionChangeAdmissionDenial,
        consequence: crate::WorthUiCollectionChangeConsequence,
    ) -> Self {
        Self {
            denial,
            consequence,
        }
    }

    pub fn denial(&self) -> WorthUiCollectionChangeAdmissionDenial {
        self.denial
    }

    pub fn into_consequence(self) -> crate::WorthUiCollectionChangeConsequence {
        self.consequence
    }
}

impl WorthUiValidatedCollectionChangeObservation {
    pub(crate) fn seal(
        consequence: crate::WorthUiCollectionChangeConsequence,
        receipt: WorthUiCollectionChangeStagingReceipt,
    ) -> Self {
        Self {
            consequence,
            receipt,
        }
    }

    pub fn source(&self) -> &crate::WorthUiCollectionChangeSourceReference {
        self.receipt.source()
    }

    pub fn change_order(&self) -> u64 {
        self.receipt.change_order()
    }

    pub fn counters(&self) -> crate::WorthUiCollectionChangeCounters {
        self.receipt.counters()
    }

    pub fn query_work(&self) -> crate::WorthUiCollectionQueryWorkInspection {
        self.receipt.query_work()
    }

    pub fn into_consequence(self) -> crate::WorthUiCollectionChangeConsequence {
        self.consequence
    }
}

impl std::fmt::Debug for WorthUiValidatedCollectionChangeObservation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiValidatedCollectionChangeObservation")
            .field("receipt", &self.receipt)
            .field("consequence", &"sealed UI consequence")
            .finish()
    }
}

impl std::fmt::Debug for WorthUiCollectionChangeAdmissionStop {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiCollectionChangeAdmissionStop")
            .field("denial", &self.denial)
            .field("consequence", &"returned sealed UI consequence")
            .finish()
    }
}
