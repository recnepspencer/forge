use crate::{
    BoundedMemoryResidencySuiteDenial, BufferPoolCertificationBundle,
    BufferPoolCertificationBundleDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedMemoryCloseoutReport {
    bundle: BufferPoolCertificationBundle,
}

impl BoundedMemoryCloseoutReport {
    pub fn close(
        bundle: BufferPoolCertificationBundle,
    ) -> Result<Self, BoundedMemoryCloseoutDenial> {
        Ok(Self { bundle })
    }

    pub const fn bundle(&self) -> &BufferPoolCertificationBundle {
        &self.bundle
    }

    pub fn into_bundle(self) -> BufferPoolCertificationBundle {
        self.bundle
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundedMemoryCloseoutDenial {
    Suite(BoundedMemoryResidencySuiteDenial),
    Bundle(BufferPoolCertificationBundleDenial),
}

impl From<BoundedMemoryResidencySuiteDenial> for BoundedMemoryCloseoutDenial {
    fn from(value: BoundedMemoryResidencySuiteDenial) -> Self {
        Self::Suite(value)
    }
}

impl From<BufferPoolCertificationBundleDenial> for BoundedMemoryCloseoutDenial {
    fn from(value: BufferPoolCertificationBundleDenial) -> Self {
        Self::Bundle(value)
    }
}
