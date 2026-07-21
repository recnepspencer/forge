use std::rc::Rc;

/// Exact, non-reconstructible identity of one prepared application instance.
#[derive(Clone)]
pub(crate) struct WorthUiPreparedApplicationGenerationWitness(Rc<()>);

impl WorthUiPreparedApplicationGenerationWitness {
    pub(super) fn issue() -> Self {
        Self(Rc::new(()))
    }
}

impl std::fmt::Debug for WorthUiPreparedApplicationGenerationWitness {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("WorthUiPreparedApplicationGenerationWitness(sealed)")
    }
}

impl PartialEq for WorthUiPreparedApplicationGenerationWitness {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for WorthUiPreparedApplicationGenerationWitness {}
