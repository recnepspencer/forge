use super::UiGraphFactIndexBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiGraphFactLookupDenial {
    BasisMismatch {
        index_basis: UiGraphFactIndexBasis,
        requested_basis: UiGraphFactIndexBasis,
    },
    UnknownAuthoredDeclaration {
        authored_identity: Box<str>,
    },
}
