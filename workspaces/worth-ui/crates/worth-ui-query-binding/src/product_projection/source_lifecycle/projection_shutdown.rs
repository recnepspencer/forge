use crate::{UiScalarProjectionBinding, UiScalarProjectionFactReceipt};

pub(super) struct WorthUiScalarProjectionShutdownOwners {
    retained_projection: Option<UiScalarProjectionBinding>,
    projection_receipt: Option<UiScalarProjectionFactReceipt>,
}

pub(super) struct WorthUiScalarProjectionShutdownResidue {
    retained_projection_count: usize,
    projection_receipt_count: usize,
}

impl WorthUiScalarProjectionShutdownOwners {
    pub(super) fn new(
        retained_projection: UiScalarProjectionBinding,
        projection_receipt: UiScalarProjectionFactReceipt,
    ) -> Self {
        Self {
            retained_projection: Some(retained_projection),
            projection_receipt: Some(projection_receipt),
        }
    }

    pub(super) fn release(mut self) -> WorthUiScalarProjectionShutdownResidue {
        drop(self.retained_projection.take());
        drop(self.projection_receipt.take());
        WorthUiScalarProjectionShutdownResidue {
            retained_projection_count: self.retained_projection.iter().count(),
            projection_receipt_count: self.projection_receipt.iter().count(),
        }
    }
}

impl WorthUiScalarProjectionShutdownResidue {
    pub(super) fn retained_projection_count(&self) -> usize {
        self.retained_projection_count
    }

    pub(super) fn projection_receipt_count(&self) -> usize {
        self.projection_receipt_count
    }
}
