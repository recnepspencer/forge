#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobPlacementMovementAuthority {
    _private: (),
}

impl BlobPlacementMovementAuthority {
    pub const fn store_owned() -> Self {
        Self { _private: () }
    }
}
