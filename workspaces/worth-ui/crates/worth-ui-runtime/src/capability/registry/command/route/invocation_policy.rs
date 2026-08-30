#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiCommandRepeatPolicy {
    Suppress,
    Allow,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiCommandTextInputPolicy {
    SuppressDuringCompositionAndTextInput,
    SuppressDuringComposition,
    Allow,
}
