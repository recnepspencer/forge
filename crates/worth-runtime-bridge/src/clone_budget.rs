pub(crate) trait CheapClone: Clone {}

pub(crate) fn clone_cheap<T: CheapClone>(value: &T) -> T {
    value.clone()
}
