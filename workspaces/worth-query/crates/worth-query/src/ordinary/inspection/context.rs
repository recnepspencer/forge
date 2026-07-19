use crate::basis_lifecycle::ScopedInspectionBasis;

pub struct WorthQueryInspectionContext {
    pub(super) basis: ScopedInspectionBasis,
}

pub fn inspection_basis(basis: ScopedInspectionBasis) -> WorthQueryInspectionContext {
    WorthQueryInspectionContext { basis }
}
