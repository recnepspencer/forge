use super::ValidationObservedAuthoredBatch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationCapturedAuthoredBatch {
    observed_batch: ValidationObservedAuthoredBatch,
}

impl ValidationCapturedAuthoredBatch {
    pub fn new(observed_batch: ValidationObservedAuthoredBatch) -> Self {
        Self { observed_batch }
    }

    pub fn observed_batch(&self) -> &ValidationObservedAuthoredBatch {
        &self.observed_batch
    }
}
