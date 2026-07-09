#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeliveryWindowWidth(u64);

impl DeliveryWindowWidth {
    pub fn measured(width: u64) -> Self {
        Self(width)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PatchGroupWidth(u64);

impl PatchGroupWidth {
    pub fn measured(width: u64) -> Self {
        Self(width)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MaintenanceDeltaWidth(u64);

impl MaintenanceDeltaWidth {
    pub fn measured(width: u64) -> Self {
        Self(width)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveDeliveryAffectedLaneWidth(u64);

impl ActiveDeliveryAffectedLaneWidth {
    pub fn measured(width: u64) -> Self {
        Self(width)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveDeliveryAffectedAttachmentWidth(u64);

impl ActiveDeliveryAffectedAttachmentWidth {
    pub fn measured(width: u64) -> Self {
        Self(width)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveDeliveryContinuationWidth(u64);

impl ActiveDeliveryContinuationWidth {
    pub fn measured(width: u64) -> Self {
        Self(width)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContinuationRemapWidth(u64);

impl ContinuationRemapWidth {
    pub fn measured(width: u64) -> Self {
        Self(width)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveDeliveryPreviewResidueWidth(u64);

impl ActiveDeliveryPreviewResidueWidth {
    pub fn measured(width: u64) -> Self {
        Self(width)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PreviewResidueWidth(u64);

impl PreviewResidueWidth {
    pub fn measured(width: u64) -> Self {
        Self(width)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}
