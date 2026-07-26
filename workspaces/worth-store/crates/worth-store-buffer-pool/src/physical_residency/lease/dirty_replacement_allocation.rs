pub(crate) trait DirtyReplacementAllocator {
    fn allocate(&self, length: usize) -> Result<Vec<u8>, ()>;
}

pub(crate) struct ProcessDirtyReplacementAllocator;

impl DirtyReplacementAllocator for ProcessDirtyReplacementAllocator {
    fn allocate(&self, length: usize) -> Result<Vec<u8>, ()> {
        let mut replacement = Vec::new();
        replacement.try_reserve_exact(length).map_err(|_| ())?;
        replacement.resize(length, 0);
        Ok(replacement)
    }
}
