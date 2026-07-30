use std::sync::Arc;

pub(crate) trait PhysicalFrameAllocator {
    fn allocate(&self, length: usize) -> Result<PhysicalFrameBuffer, ()>;
}

pub(crate) struct ProcessPhysicalFrameAllocator;

impl PhysicalFrameAllocator for ProcessPhysicalFrameAllocator {
    fn allocate(&self, length: usize) -> Result<PhysicalFrameBuffer, ()> {
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(length).map_err(|_| ())?;
        bytes.resize(length, 0);
        Ok(PhysicalFrameBuffer { bytes })
    }
}

pub(crate) struct PhysicalFrameBuffer {
    bytes: Vec<u8>,
}

impl PhysicalFrameBuffer {
    #[cfg(test)]
    pub(crate) fn with_capacity(length: usize, capacity: usize) -> Self {
        let mut bytes = Vec::with_capacity(capacity);
        bytes.resize(length, 0);
        Self { bytes }
    }

    pub(crate) fn capacity(&self) -> usize {
        self.bytes.capacity()
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [u8] {
        self.bytes.as_mut_slice()
    }

    pub(crate) fn into_resident(self) -> Arc<Vec<u8>> {
        Arc::new(self.bytes)
    }
}
