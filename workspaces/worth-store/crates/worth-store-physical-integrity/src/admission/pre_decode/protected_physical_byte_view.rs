use worth_store_buffer_pool::{PinnedFrameView, ZeroCopyRecordView};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectedPhysicalByteView<'lease> {
    bytes: &'lease [u8],
}

impl<'lease> ProtectedPhysicalByteView<'lease> {
    pub fn from_pinned_frame(view: &PinnedFrameView<'lease>) -> Self {
        Self {
            bytes: view.as_bytes(),
        }
    }

    pub fn from_zero_copy_record_view(view: &ZeroCopyRecordView<'lease>) -> Self {
        Self {
            bytes: view.physical_record_bytes(),
        }
    }

    pub const fn as_bytes(self) -> &'lease [u8] {
        self.bytes
    }

    pub const fn len_bytes(self) -> usize {
        self.bytes.len()
    }

    pub const fn is_empty(self) -> bool {
        self.bytes.is_empty()
    }
}
