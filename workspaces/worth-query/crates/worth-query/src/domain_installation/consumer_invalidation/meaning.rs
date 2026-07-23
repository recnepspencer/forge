#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConsumerInvalidationDisposition {
    LocalPatch,
    Reexecute,
    Rebind,
    Replace,
    Retire,
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryConsumerInvalidationCause {
    ResultStateChanged(Vec<crate::ordinary::live::WorthQueryManagedLiveDeliveryCauseKind>),
    CollectionMeaningChanged(Vec<crate::ordinary::live::WorthQueryManagedLiveDeliveryCauseKind>),
    CapabilityAuthorityChanged(Vec<crate::ordinary::live::WorthQueryManagedLiveDeliveryCauseKind>),
    LifecycleRetired(Vec<crate::ordinary::live::WorthQueryManagedLiveDeliveryCauseKind>),
    NativeNarrowingUnavailable(Vec<crate::ordinary::live::WorthQueryManagedLiveDeliveryCauseKind>),
    UnsupportedMeaning(Vec<crate::ordinary::live::WorthQueryManagedLiveDeliveryCauseKind>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConsumerInvalidationLocality {
    DeclaredNativeKeys,
    BoundCollection,
    WholeCapability,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConsumerInvalidationContinuation {
    NotApplicable,
    NotRequired,
    SnapshotCursor,
    LiveCursor,
}

impl WorthQueryConsumerInvalidationCause {
    pub fn delivery_causes(
        &self,
    ) -> &[crate::ordinary::live::WorthQueryManagedLiveDeliveryCauseKind] {
        match self {
            Self::ResultStateChanged(causes)
            | Self::CollectionMeaningChanged(causes)
            | Self::CapabilityAuthorityChanged(causes)
            | Self::LifecycleRetired(causes)
            | Self::NativeNarrowingUnavailable(causes)
            | Self::UnsupportedMeaning(causes) => causes,
        }
    }

    pub(crate) const fn canonical_name(&self) -> &'static str {
        match self {
            Self::ResultStateChanged(_) => "result-state-changed",
            Self::CollectionMeaningChanged(_) => "collection-meaning-changed",
            Self::CapabilityAuthorityChanged(_) => "capability-authority-changed",
            Self::LifecycleRetired(_) => "lifecycle-retired",
            Self::NativeNarrowingUnavailable(_) => "native-narrowing-unavailable",
            Self::UnsupportedMeaning(_) => "unsupported-meaning",
        }
    }
}

impl WorthQueryConsumerInvalidationDisposition {
    pub(crate) const fn canonical_name(self) -> &'static str {
        match self {
            Self::LocalPatch => "local-patch",
            Self::Reexecute => "reexecute",
            Self::Rebind => "rebind",
            Self::Replace => "replace",
            Self::Retire => "retire",
            Self::Unsupported => "unsupported",
        }
    }
}

impl WorthQueryConsumerInvalidationLocality {
    pub(crate) const fn canonical_name(self) -> &'static str {
        match self {
            Self::DeclaredNativeKeys => "declared-native-keys",
            Self::BoundCollection => "bound-collection",
            Self::WholeCapability => "whole-capability",
        }
    }
}

impl WorthQueryConsumerInvalidationContinuation {
    pub(crate) const fn canonical_name(self) -> &'static str {
        match self {
            Self::NotApplicable => "not-applicable",
            Self::NotRequired => "not-required",
            Self::SnapshotCursor => "snapshot-cursor",
            Self::LiveCursor => "live-cursor",
        }
    }
}
