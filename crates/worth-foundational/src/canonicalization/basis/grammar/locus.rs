use crate::aspects::{AspectKey, CanonicalFieldPath};
use crate::values::InternedString;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalBasisLocus {
    Root,
    EntryOrdinal(u32),
    Aspect(AspectKey),
    AspectField {
        aspect: AspectKey,
        path: CanonicalFieldPath,
    },
    Named(InternedString),
}
