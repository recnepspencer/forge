//! A recovery handle held inside an in-flight preparation (Q8.22-C5).

use super::handle::WorthQueryRecoveryHandle;

/// A recovery handle owned by a value that is *preparing* a correction.
///
/// A bare [`WorthQueryRecoveryHandle`] records `Disposed` when it is dropped,
/// and that is right for a bare handle: its holder had every transition
/// available and used none, which is a decision to end recovery (Q8.21-L12).
///
/// It is wrong for a preparation. An undo admission, a redo continuation, and a
/// redo admission are all values that mean "a correction is being prepared and
/// has not committed." Reaching the end of one consumes nothing, so it must
/// leave the commit exactly as recoverable as it was — the same non-event the
/// `RelinquishOnDenial` combinators record for a denial *inside* Query
/// (Q8.21-L11).
///
/// This matters most where Query cannot reach. Once a preparation crosses into
/// the application, every `?` in the host's progression drops it, and the host
/// has no way to relinquish: [`super::RelinquishOnDenial`] is `pub(crate)`.
/// Putting the rule in `Drop` is what makes it hold for a holder that has never
/// heard of it — there is no API left to get wrong.
///
/// The one exercising exit is [`Self::into_handle`], which hands the handle to
/// whoever will decide its real fate.
pub(crate) struct WorthQueryHeldRecoveryHandle {
    handle: Option<WorthQueryRecoveryHandle>,
}

impl WorthQueryHeldRecoveryHandle {
    pub(crate) const fn new(handle: WorthQueryRecoveryHandle) -> Self {
        Self {
            handle: Some(handle),
        }
    }

    pub(crate) fn get(&self) -> &WorthQueryRecoveryHandle {
        self.handle
            .as_ref()
            .expect("an in-flight preparation owns one recovery handle")
    }

    /// Give the handle up to a caller that will decide its fate — another
    /// preparation carrier, or a real transition. `Drop` is a no-op afterwards.
    pub(crate) fn into_handle(mut self) -> WorthQueryRecoveryHandle {
        self.handle
            .take()
            .expect("an in-flight preparation owns one recovery handle")
    }
}

impl Drop for WorthQueryHeldRecoveryHandle {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.as_mut() {
            handle.relinquish_in_place();
        }
    }
}

impl std::fmt::Debug for WorthQueryHeldRecoveryHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.handle, formatter)
    }
}
