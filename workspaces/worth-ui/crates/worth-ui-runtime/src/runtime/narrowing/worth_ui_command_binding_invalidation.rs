#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCommandBindingInvalidation {
    affected_binding_count: usize,
}

impl WorthUiCommandBindingInvalidation {
    pub(crate) fn binding_only(affected_binding_count: usize) -> Self {
        Self {
            affected_binding_count,
        }
    }

    pub fn affected_binding_count(&self) -> usize {
        self.affected_binding_count
    }
}
