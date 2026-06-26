#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiRustCompositionMetrics {
    modules_declared: usize,
    declarations_declared: usize,
}

impl WorthUiRustCompositionMetrics {
    pub(super) fn from_counts(modules_declared: usize, declarations_declared: usize) -> Self {
        Self {
            modules_declared,
            declarations_declared,
        }
    }

    pub(crate) fn modules_declared(self) -> usize {
        self.modules_declared
    }

    pub(crate) fn declarations_declared(self) -> usize {
        self.declarations_declared
    }
}
