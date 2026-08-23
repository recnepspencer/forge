#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationLineageContract {
    NotRequired,
    Preserve,
    Evolve,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationPromotionContract {
    NotRequired,
    OnDurableReference,
}
