use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_LAYOUT_COURTROOM_TRANSCRIPT: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutCourtroomTranscriptIdentity(NonZeroU64);

impl LayoutCourtroomTranscriptIdentity {
    pub(super) fn issue() -> Self {
        let raw = NEXT_LAYOUT_COURTROOM_TRANSCRIPT
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .expect("layout courtroom transcript identity space exhausted");
        Self(NonZeroU64::new(raw).expect("transcript sequence starts at one"))
    }

    pub const fn get(self) -> u64 {
        self.0.get()
    }
}
