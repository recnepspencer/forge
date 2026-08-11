#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum S1BlockingPredicate {
    MissingBackendTierMatrix,
    MissingDeferredGuaranteeMap,
    MissingTerminologyScanDigest,
    MissingForbiddenShortcutList,
    MissingHarnessReadinessRows,
    OverclaimedPhysicalPosturePresent,
    UnmappedDeferredGuaranteePresent,
    StaleAcceptedInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum S1BlockingPredicateStatus {
    Satisfied,
    Blocking,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct S1BlockingPredicateRow {
    pub(super) predicate: S1BlockingPredicate,
    pub(super) status: S1BlockingPredicateStatus,
}

impl S1BlockingPredicateRow {
    pub fn predicate(&self) -> S1BlockingPredicate {
        self.predicate
    }

    pub fn status(&self) -> S1BlockingPredicateStatus {
        self.status
    }
}
