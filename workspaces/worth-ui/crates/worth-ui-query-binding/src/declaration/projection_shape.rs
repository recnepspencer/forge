#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum UiProjectionShape {
    Scalar,
    Collection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum UiProjectionNativeFamily {
    Text,
    Boolean,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum UiProjectionLifecycleRequirement {
    Snapshot,
    Live,
}
