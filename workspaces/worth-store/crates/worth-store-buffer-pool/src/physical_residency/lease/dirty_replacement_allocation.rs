use std::sync::Arc;

pub(crate) trait DirtyReplacementAllocator {
    fn allocate(&self, length: usize) -> Result<DirtyReplacementBuffer, ()>;
}

pub(crate) struct ProcessDirtyReplacementAllocator;

impl DirtyReplacementAllocator for ProcessDirtyReplacementAllocator {
    fn allocate(&self, length: usize) -> Result<DirtyReplacementBuffer, ()> {
        let mut replacement = Vec::new();
        replacement.try_reserve_exact(length).map_err(|_| ())?;
        replacement.resize(length, 0);
        Ok(DirtyReplacementBuffer { replacement })
    }
}

pub(crate) struct DirtyReplacementBuffer {
    replacement: Vec<u8>,
}

impl DirtyReplacementBuffer {
    #[cfg(test)]
    pub(crate) fn with_capacity(length: usize, capacity: usize) -> Self {
        let mut replacement = Vec::with_capacity(capacity);
        replacement.resize(length, 0);
        Self { replacement }
    }

    pub(crate) fn capacity(&self) -> usize {
        self.replacement.capacity()
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [u8] {
        self.replacement.as_mut_slice()
    }

    pub(crate) fn into_resident(self) -> Arc<Vec<u8>> {
        Arc::new(self.replacement)
    }
}
