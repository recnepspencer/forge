use crate::PinnedPageLease;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq)]
pub struct PinnedFrameView<'lease> {
    bytes: &'lease [u8],
    _lease: PhantomData<&'lease PinnedPageLease<'lease>>,
}

impl<'lease> PinnedFrameView<'lease> {
    pub(crate) const fn new(bytes: &'lease [u8]) -> Self {
        Self {
            bytes,
            _lease: PhantomData,
        }
    }

    pub const fn as_bytes(&self) -> &'lease [u8] {
        self.bytes
    }
}
