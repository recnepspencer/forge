use super::{
    WorthQueryConsumerInvalidationCause, WorthQueryConsumerInvalidationContinuation,
    WorthQueryConsumerInvalidationDisposition, WorthQueryConsumerInvalidationLocality,
};

pub(crate) fn classify_disposition(
    impact: crate::domain_installation::WorthQueryImpactClass,
) -> Option<WorthQueryConsumerInvalidationDisposition> {
    use crate::domain_installation::WorthQueryImpactClass as Impact;
    use WorthQueryConsumerInvalidationDisposition as Disposition;
    match impact {
        Impact::UnaffectedOrSuppressed => None,
        Impact::ValuePatch => Some(Disposition::LocalPatch),
        Impact::MembershipSplice
        | Impact::ReorderOrRegroup
        | Impact::WindowShift
        | Impact::Reexecute => Some(Disposition::Reexecute),
        Impact::ExplicitRebind => Some(Disposition::Rebind),
        Impact::Replacement => Some(Disposition::Replace),
        Impact::Retirement => Some(Disposition::Retire),
        Impact::UnsupportedEscalation => Some(Disposition::Unsupported),
    }
}

pub(crate) fn cause_for(
    disposition: WorthQueryConsumerInvalidationDisposition,
    retained: &[crate::ordinary::live::WorthQueryManagedLiveDeliveryCauseKind],
) -> WorthQueryConsumerInvalidationCause {
    use WorthQueryConsumerInvalidationCause as Cause;
    use WorthQueryConsumerInvalidationDisposition as Disposition;
    let retained = retained.to_vec();
    match disposition {
        Disposition::LocalPatch => Cause::ResultStateChanged(retained),
        Disposition::Reexecute => Cause::CollectionMeaningChanged(retained),
        Disposition::Rebind | Disposition::Replace => Cause::CapabilityAuthorityChanged(retained),
        Disposition::Retire => Cause::LifecycleRetired(retained),
        Disposition::Unsupported => Cause::UnsupportedMeaning(retained),
    }
}

pub(crate) fn locality_for(
    disposition: WorthQueryConsumerInvalidationDisposition,
    collection: &crate::domain_installation::WorthQueryOperationCollectionContract,
) -> WorthQueryConsumerInvalidationLocality {
    use WorthQueryConsumerInvalidationDisposition as Disposition;
    match disposition {
        Disposition::LocalPatch => WorthQueryConsumerInvalidationLocality::DeclaredNativeKeys,
        Disposition::Reexecute => match collection {
            crate::domain_installation::WorthQueryOperationCollectionContract::NotCollection => {
                WorthQueryConsumerInvalidationLocality::WholeCapability
            }
            crate::domain_installation::WorthQueryOperationCollectionContract::Collection {
                ..
            } => WorthQueryConsumerInvalidationLocality::BoundCollection,
        },
        Disposition::Rebind
        | Disposition::Replace
        | Disposition::Retire
        | Disposition::Unsupported => WorthQueryConsumerInvalidationLocality::WholeCapability,
    }
}

pub(crate) fn continuation_for(
    collection: &crate::domain_installation::WorthQueryOperationCollectionContract,
) -> WorthQueryConsumerInvalidationContinuation {
    use crate::domain_installation::{
        WorthQueryOperationCollectionContract as Collection,
        WorthQueryOperationContinuationPosture as Continuation,
    };
    match collection {
        Collection::NotCollection => WorthQueryConsumerInvalidationContinuation::NotApplicable,
        Collection::Collection { continuation, .. } => match continuation {
            Continuation::NotRequired => WorthQueryConsumerInvalidationContinuation::NotRequired,
            Continuation::SnapshotCursor => {
                WorthQueryConsumerInvalidationContinuation::SnapshotCursor
            }
            Continuation::LiveCursor => WorthQueryConsumerInvalidationContinuation::LiveCursor,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_semantic_impact_has_one_non_downgradable_disposition() {
        use crate::domain_installation::WorthQueryImpactClass as Impact;
        use WorthQueryConsumerInvalidationDisposition as Disposition;
        let cases = [
            (Impact::UnaffectedOrSuppressed, None),
            (Impact::ValuePatch, Some(Disposition::LocalPatch)),
            (Impact::MembershipSplice, Some(Disposition::Reexecute)),
            (Impact::ReorderOrRegroup, Some(Disposition::Reexecute)),
            (Impact::WindowShift, Some(Disposition::Reexecute)),
            (Impact::Reexecute, Some(Disposition::Reexecute)),
            (Impact::ExplicitRebind, Some(Disposition::Rebind)),
            (Impact::Replacement, Some(Disposition::Replace)),
            (Impact::Retirement, Some(Disposition::Retire)),
            (
                Impact::UnsupportedEscalation,
                Some(Disposition::Unsupported),
            ),
        ];
        for (impact, expected) in cases {
            assert_eq!(classify_disposition(impact), expected);
        }
    }
}
