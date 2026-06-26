#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAccessibilityInvalidation {
    affected_reference_count: usize,
}

impl WorthUiAccessibilityInvalidation {
    pub(crate) fn unchanged() -> Self {
        Self {
            affected_reference_count: 0,
        }
    }

    pub fn affected_reference_count(&self) -> usize {
        self.affected_reference_count
    }
}
