use worth_query::facade::{installed::observation, runtime};

use super::{
    UiLiveCollectionProjection, UiLiveCollectionProjectionCloseOutcome,
    UiLiveCollectionProjectionCloseReceipt, UiLiveCollectionProjectionCloseStop,
};

impl UiLiveCollectionProjection {
    pub fn close(
        self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> UiLiveCollectionProjectionCloseOutcome {
        let Self {
            binding,
            reference,
            lease,
            consumer,
            text_accesses,
            application_item_key_access,
            budget,
        } = self;
        match lease.dispose(workspace) {
            observation::WorthQuerySharedProjectionDisposalOutcome::Disposed(disposed) => {
                UiLiveCollectionProjectionCloseOutcome::Closed(
                    UiLiveCollectionProjectionCloseReceipt {
                        owner_terminal: disposed.release().owner_terminal(),
                        counters: disposed.release().counters(),
                    },
                )
            }
            observation::WorthQuerySharedProjectionDisposalOutcome::Stopped(stop) => {
                let (lease, query_error, counters) = stop.into_parts();
                UiLiveCollectionProjectionCloseOutcome::Stopped(Box::new(
                    UiLiveCollectionProjectionCloseStop {
                        live: Self {
                            binding,
                            reference,
                            lease,
                            consumer,
                            text_accesses,
                            application_item_key_access,
                            budget,
                        },
                        query_error,
                        counters,
                    },
                ))
            }
        }
    }
}
