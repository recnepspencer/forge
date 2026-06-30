#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetainedReplaySampling {
    checkpoint_stride: usize,
}

impl RetainedReplaySampling {
    pub fn every_fourth_checkpoint_plus_trigger_steps() -> Self {
        Self {
            checkpoint_stride: 4,
        }
    }

    pub fn checkpoint_stride(self) -> usize {
        self.checkpoint_stride
    }
}
