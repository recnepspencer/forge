use std::sync::Arc;

pub(crate) trait CandidateFrameAllocator {
    fn allocate(&self, length: usize) -> Result<CandidateFrameBuffer, ()>;
}

pub(crate) struct ProcessCandidateFrameAllocator;

impl CandidateFrameAllocator for ProcessCandidateFrameAllocator {
    fn allocate(&self, length: usize) -> Result<CandidateFrameBuffer, ()> {
        let mut frame = Vec::new();
        frame.try_reserve_exact(length).map_err(|_| ())?;
        frame.resize(length, 0);
        Ok(CandidateFrameBuffer { frame })
    }
}

pub(crate) struct CandidateFrameBuffer {
    frame: Vec<u8>,
}

impl CandidateFrameBuffer {
    #[cfg(test)]
    pub(crate) fn with_capacity(length: usize, capacity: usize) -> Self {
        let mut frame = Vec::with_capacity(capacity);
        frame.resize(length, 0);
        Self { frame }
    }

    pub(crate) fn capacity(&self) -> usize {
        self.frame.capacity()
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [u8] {
        self.frame.as_mut_slice()
    }

    pub(crate) fn into_resident(self) -> Arc<Vec<u8>> {
        Arc::new(self.frame)
    }
}
