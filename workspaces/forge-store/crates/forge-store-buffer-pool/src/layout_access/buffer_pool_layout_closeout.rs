#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferPoolLayoutCloseout;

impl BufferPoolLayoutCloseout {
    pub const fn placeholder() -> Self {
        Self
    }
}
