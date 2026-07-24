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

pub struct WorthUiCollectionChangeAdmissionStop {
    denial: WorthUiCollectionChangeAdmissionDenial,
    consequence: crate::WorthUiCollectionChangeConsequence,
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

impl std::fmt::Debug for WorthUiCollectionChangeAdmissionStop {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiCollectionChangeAdmissionStop")
            .field("denial", &self.denial)
            .field("consequence", &"returned sealed UI consequence")
            .finish()
    }
}
