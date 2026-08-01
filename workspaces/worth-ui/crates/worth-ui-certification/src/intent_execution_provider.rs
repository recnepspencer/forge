use core::marker::PhantomData;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use worth_ui::facade::intent::{
    UiIntent, UiIntentExecutionProvider, UiIntentExecutionRequest, UiIntentProviderStart,
    UiIntentProviderStop, UiIntentProviderVersion,
};

/// Typed provider used by pre-execution certification worlds.
///
/// Phase 2-3 scenarios never invoke execution. If a regression crosses that
/// boundary, this provider settles explicitly before effect instead of hiding
/// the call behind a no-op callback.
pub struct WorthUiCertificationBeforeEffectProvider<I: UiIntent> {
    intent: PhantomData<fn() -> I>,
    begin_calls: Option<Arc<AtomicUsize>>,
}

#[derive(Clone)]
pub struct WorthUiCertificationProviderObservation {
    begin_calls: Arc<AtomicUsize>,
}

impl<I: UiIntent> WorthUiCertificationBeforeEffectProvider<I> {
    pub const fn new() -> Self {
        Self {
            intent: PhantomData,
            begin_calls: None,
        }
    }

    pub fn with_observation() -> (Self, WorthUiCertificationProviderObservation) {
        let begin_calls = Arc::new(AtomicUsize::new(0));
        (
            Self {
                intent: PhantomData,
                begin_calls: Some(Arc::clone(&begin_calls)),
            },
            WorthUiCertificationProviderObservation { begin_calls },
        )
    }
}

impl<I: UiIntent> Default for WorthUiCertificationBeforeEffectProvider<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: UiIntent> UiIntentExecutionProvider<I> for WorthUiCertificationBeforeEffectProvider<I> {
    const VERSION: UiIntentProviderVersion = UiIntentProviderVersion::stable(1);

    fn begin(&self, request: UiIntentExecutionRequest<I>) -> UiIntentProviderStart<I> {
        if let Some(begin_calls) = &self.begin_calls {
            begin_calls.fetch_add(1, Ordering::Relaxed);
        }
        drop(request);
        UiIntentProviderStart::RejectedBeforeEffect(UiIntentProviderStop::stable(
            "certification.before_effect",
        ))
    }
}

impl WorthUiCertificationProviderObservation {
    pub fn begin_calls(&self) -> usize {
        self.begin_calls.load(Ordering::Relaxed)
    }
}
