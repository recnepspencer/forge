#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPlacementMovementFreshness {
    Current,
    Stale,
}