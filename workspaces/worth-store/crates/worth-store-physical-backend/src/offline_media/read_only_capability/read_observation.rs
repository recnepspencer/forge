#[derive(Debug)]
pub struct OfflineMediaReadObservation {
    file_index: usize,
    offset: u64,
    bytes_read: usize,
}

impl OfflineMediaReadObservation {
    pub(super) const fn new(file_index: usize, offset: u64, bytes_read: usize) -> Self {
        Self {
            file_index,
            offset,
            bytes_read,
        }
    }

    pub const fn file_index(&self) -> usize {
        self.file_index
    }
    pub const fn offset(&self) -> u64 {
        self.offset
    }
    pub const fn bytes_read(&self) -> usize {
        self.bytes_read
    }
}
