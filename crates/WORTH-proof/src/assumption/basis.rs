#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoAssumptionBasis;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssumptionBasis<B> {
    value: B,
}

impl<B> AssumptionBasis<B> {
    pub fn new(value: B) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &B {
        &self.value
    }
}
