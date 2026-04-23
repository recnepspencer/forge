#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConsumerDeliveryPacingWidth(u64);

impl ConsumerDeliveryPacingWidth {
    pub fn measured(width: u64) -> Self {
        Self(width)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}
