use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use worth_ui::facade::intent::{
    UiIntentExecutionAttempt, UiIntentExecutionCancellationContext, UiIntentExecutionPollContext,
    UiIntentExecutionProvider, UiIntentExecutionRequest, UiIntentProviderPoll,
    UiIntentProviderSettlement, UiIntentProviderStart, UiIntentProviderStop,
    UiIntentProviderVersion,
};

use crate::intent::operability::{ConsequenceIntent, ConsequenceOutcome};

pub(super) struct ConsequenceProvider {
    state: Arc<ConsequenceProviderState>,
}

#[derive(Clone)]
pub(super) struct ConsequenceProviderControl {
    state: Arc<ConsequenceProviderState>,
}

struct ConsequenceProviderState {
    next: Mutex<Option<ConsequenceOutcome>>,
    begin_calls: AtomicUsize,
    poll_calls: AtomicUsize,
}

struct ConsequenceAttempt {
    state: Arc<ConsequenceProviderState>,
}

impl ConsequenceProvider {
    pub(super) fn controlled() -> (Self, ConsequenceProviderControl) {
        let state = Arc::new(ConsequenceProviderState {
            next: Mutex::new(None),
            begin_calls: AtomicUsize::new(0),
            poll_calls: AtomicUsize::new(0),
        });
        (
            Self {
                state: Arc::clone(&state),
            },
            ConsequenceProviderControl { state },
        )
    }
}

impl UiIntentExecutionProvider<ConsequenceIntent> for ConsequenceProvider {
    const VERSION: UiIntentProviderVersion = UiIntentProviderVersion::stable(314);

    fn begin(
        &self,
        request: UiIntentExecutionRequest<ConsequenceIntent>,
    ) -> UiIntentProviderStart<ConsequenceIntent> {
        drop(request);
        self.state.begin_calls.fetch_add(1, Ordering::Relaxed);
        UiIntentProviderStart::Started(Box::new(ConsequenceAttempt {
            state: Arc::clone(&self.state),
        }))
    }
}

impl UiIntentExecutionAttempt<ConsequenceIntent> for ConsequenceAttempt {
    fn poll(
        &mut self,
        _context: UiIntentExecutionPollContext,
    ) -> UiIntentProviderPoll<ConsequenceIntent> {
        self.state.poll_calls.fetch_add(1, Ordering::Relaxed);
        let outcome = self
            .state
            .next
            .lock()
            .expect("consequence provider slot")
            .take()
            .expect("each completed attempt receives one typed product outcome");
        UiIntentProviderPoll::Settled(UiIntentProviderSettlement::Completed(outcome))
    }

    fn cancel(
        &mut self,
        _context: UiIntentExecutionCancellationContext,
    ) -> UiIntentProviderPoll<ConsequenceIntent> {
        UiIntentProviderPoll::Settled(UiIntentProviderSettlement::CancelledBeforeEffect(
            UiIntentProviderStop::stable("phase4.consequence.cancelled"),
        ))
    }
}

impl ConsequenceProviderControl {
    pub(super) fn supply(
        &self,
        consequence: worth_ui_query_binding::WorthUiCollectionChangeConsequence,
    ) {
        let previous = self
            .state
            .next
            .lock()
            .expect("consequence provider slot")
            .replace(ConsequenceOutcome::query(consequence));
        assert!(
            previous.is_none(),
            "provider slot accepts one exact consequence"
        );
    }

    pub(super) fn supply_none(&self) {
        let previous = self
            .state
            .next
            .lock()
            .expect("consequence provider slot")
            .replace(ConsequenceOutcome::none());
        assert!(
            previous.is_none(),
            "provider slot accepts one exact outcome"
        );
    }

    pub(super) fn calls(&self) -> [usize; 2] {
        [
            self.state.begin_calls.load(Ordering::Relaxed),
            self.state.poll_calls.load(Ordering::Relaxed),
        ]
    }
}
