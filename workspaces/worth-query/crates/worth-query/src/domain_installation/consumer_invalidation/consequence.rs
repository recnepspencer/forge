use super::{
    WorthQueryAdmittedConsumerInvalidation, WorthQueryConsumerInvalidationAuthority,
    WorthQueryConsumerInvalidationDelta, WorthQueryConsumerInvalidationDisposition,
};

pub struct WorthQueryConsumerConsequence<'admission, T> {
    delta: &'admission WorthQueryConsumerInvalidationDelta,
    _current_workspace: &'admission crate::runtime::WorthQueryWorkspace,
    admitted_disposition: WorthQueryConsumerInvalidationDisposition,
    consumer_authored: T,
}

pub struct WorthQueryConsumerConsequenceAdmissionStop<T> {
    kind: WorthQueryConsumerConsequenceAdmissionStopKind,
    required: WorthQueryConsumerInvalidationDisposition,
    requested: WorthQueryConsumerInvalidationDisposition,
    consumer_authored: T,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConsumerConsequenceAdmissionStopKind {
    ForeignOrStaleInvalidation,
    DispositionDowngrade,
}

impl<T> WorthQueryConsumerConsequenceAdmissionStop<T> {
    pub const fn kind(&self) -> WorthQueryConsumerConsequenceAdmissionStopKind {
        self.kind
    }

    pub const fn required_disposition(&self) -> WorthQueryConsumerInvalidationDisposition {
        self.required
    }

    pub const fn requested_disposition(&self) -> WorthQueryConsumerInvalidationDisposition {
        self.requested
    }

    pub fn into_consumer_authored(self) -> T {
        self.consumer_authored
    }
}

impl<T> WorthQueryConsumerConsequence<'_, T> {
    pub const fn authority(&self) -> &WorthQueryConsumerInvalidationAuthority {
        self.delta.authority()
    }

    pub const fn maintenance_ordinal(&self) -> u64 {
        self.delta.maintenance_ordinal()
    }

    pub const fn required_disposition(&self) -> WorthQueryConsumerInvalidationDisposition {
        self.delta.disposition()
    }

    pub const fn admitted_disposition(&self) -> WorthQueryConsumerInvalidationDisposition {
        self.admitted_disposition
    }

    pub const fn delta(&self) -> &WorthQueryConsumerInvalidationDelta {
        self.delta
    }

    pub const fn consumer_authored(&self) -> &T {
        &self.consumer_authored
    }

    pub fn into_consumer_authored(self) -> T {
        self.consumer_authored
    }
}

impl WorthQueryAdmittedConsumerInvalidation<'_> {
    pub fn attach_consumer_authored_consequence<'current, T>(
        &'current self,
        workspace: &'current crate::runtime::WorthQueryWorkspace,
        requested_disposition: WorthQueryConsumerInvalidationDisposition,
        consumer_authored: T,
    ) -> Result<
        WorthQueryConsumerConsequence<'current, T>,
        WorthQueryConsumerConsequenceAdmissionStop<T>,
    > {
        let required = self.delta().disposition();
        if !self.remains_current(workspace) {
            return Err(WorthQueryConsumerConsequenceAdmissionStop {
                kind: WorthQueryConsumerConsequenceAdmissionStopKind::ForeignOrStaleInvalidation,
                required,
                requested: requested_disposition,
                consumer_authored,
            });
        }
        if !consequence_preserves_or_widens(required, requested_disposition) {
            return Err(WorthQueryConsumerConsequenceAdmissionStop {
                kind: WorthQueryConsumerConsequenceAdmissionStopKind::DispositionDowngrade,
                required,
                requested: requested_disposition,
                consumer_authored,
            });
        }
        Ok(WorthQueryConsumerConsequence {
            delta: self.delta(),
            _current_workspace: workspace,
            admitted_disposition: requested_disposition,
            consumer_authored,
        })
    }
}

fn consequence_preserves_or_widens(
    required: WorthQueryConsumerInvalidationDisposition,
    requested: WorthQueryConsumerInvalidationDisposition,
) -> bool {
    consequence_rank(requested) >= consequence_rank(required)
}

fn consequence_rank(disposition: WorthQueryConsumerInvalidationDisposition) -> u8 {
    use WorthQueryConsumerInvalidationDisposition as Disposition;
    match disposition {
        Disposition::LocalPatch => 0,
        Disposition::Reexecute => 1,
        Disposition::Rebind => 2,
        Disposition::Replace => 3,
        Disposition::Retire => 4,
        Disposition::Unsupported => 5,
    }
}

#[cfg(test)]
mod tests {
    use super::{consequence_preserves_or_widens, WorthQueryConsumerInvalidationDisposition as D};

    #[test]
    fn every_disposition_preserves_itself_and_only_explicitly_widens() {
        let dispositions = [
            D::LocalPatch,
            D::Reexecute,
            D::Rebind,
            D::Replace,
            D::Retire,
            D::Unsupported,
        ];
        for (required_rank, required) in dispositions.iter().copied().enumerate() {
            for (requested_rank, requested) in dispositions.iter().copied().enumerate() {
                assert_eq!(
                    consequence_preserves_or_widens(required, requested),
                    requested_rank >= required_rank,
                    "{requested:?} was misclassified against required {required:?}",
                );
            }
        }
    }
}
