use std::marker::PhantomData;

use crate::facade::*;

pub(crate) const DEFAULT_ASPECT: Aspect = Aspect::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Signal<T> {
    pub(crate) node: NodeId,
    marker: PhantomData<fn() -> T>,
}

impl<T> Signal<T> {
    pub(crate) fn new(node: NodeId) -> Self {
        Self {
            node,
            marker: PhantomData,
        }
    }
}

pub type InputSignal<T> = Signal<T>;
pub type ComputedSignal<T> = Signal<T>;