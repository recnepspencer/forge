use crate::obligations::catalog::{UiObligationCheckKind, UiObligationFamily};

use super::{UiObligationSelectionMatrix, UiObligationSupportBasis};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiObligationStarterMatrixRowTopology {
    family: UiObligationFamily,
    check_kind: UiObligationCheckKind,
    support_basis: UiObligationSupportBasis,
}

impl UiObligationStarterMatrixRowTopology {
    pub fn family(self) -> UiObligationFamily {
        self.family
    }

    pub fn check_kind(self) -> UiObligationCheckKind {
        self.check_kind
    }

    pub fn support_basis(self) -> UiObligationSupportBasis {
        self.support_basis
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiObligationStarterMatrixTopology {
    rows: Box<[UiObligationStarterMatrixRowTopology]>,
}

impl UiObligationStarterMatrixTopology {
    pub fn starter() -> Self {
        let rows = UiObligationSelectionMatrix::starter()
            .rows()
            .iter()
            .map(|row| UiObligationStarterMatrixRowTopology {
                family: row.family(),
                check_kind: row.check_kind(),
                support_basis: row.support_basis(),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self { rows }
    }

    pub fn rows(&self) -> &[UiObligationStarterMatrixRowTopology] {
        &self.rows
    }
}
