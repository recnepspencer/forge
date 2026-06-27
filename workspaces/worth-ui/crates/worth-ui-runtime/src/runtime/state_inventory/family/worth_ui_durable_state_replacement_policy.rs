#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiDurableStateReplacementPolicy {
    PreserveWhenNodeCarriesState,
    DropOnReplacement,
    ReplaceOnReplacement,
    ReconcileOnLaneChange,
}
