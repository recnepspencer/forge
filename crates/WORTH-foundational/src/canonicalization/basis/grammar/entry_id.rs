use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct CanonicalBasisEntryId<D> {
    ordinal: u32,
    domain: PhantomData<fn() -> D>,
}

impl<D> CanonicalBasisEntryId<D> {
    pub const fn new(ordinal: u32) -> Self {
        Self {
            ordinal,
            domain: PhantomData,
        }
    }

    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }
}

impl<D> Clone for CanonicalBasisEntryId<D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<D> Copy for CanonicalBasisEntryId<D> {}

impl<D> PartialEq for CanonicalBasisEntryId<D> {
    fn eq(&self, other: &Self) -> bool {
        self.ordinal == other.ordinal
    }
}

impl<D> Eq for CanonicalBasisEntryId<D> {}

impl<D> Hash for CanonicalBasisEntryId<D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ordinal.hash(state);
    }
}

impl<D> PartialOrd for CanonicalBasisEntryId<D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<D> Ord for CanonicalBasisEntryId<D> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ordinal.cmp(&other.ordinal)
    }
}
