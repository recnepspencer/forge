#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobLayoutCloseout;

impl BlobLayoutCloseout {
    pub const fn placeholder() -> Self {
        Self
    }
}
