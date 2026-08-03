use super::{PausedScrubExecution, ScrubExecutionReceipt};

#[derive(Debug)]
pub enum ScrubExecutionOutcome<'runtime, 'lease> {
    Completed(ScrubExecutionReceipt),
    Yielded(PausedScrubExecution<'runtime, 'lease>),
}

impl ScrubExecutionOutcome<'_, '_> {
    pub const fn completed(&self) -> Option<&ScrubExecutionReceipt> {
        match self {
            Self::Completed(receipt) => Some(receipt),
            Self::Yielded(_) => None,
        }
    }
}
