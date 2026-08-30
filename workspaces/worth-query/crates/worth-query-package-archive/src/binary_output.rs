pub(crate) struct BinaryOutput {
    bytes: Vec<u8>,
}

impl BinaryOutput {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }

    pub(crate) fn u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub(crate) fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub(crate) fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub(crate) fn raw_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    pub(crate) fn text(&mut self, value: &str) {
        self.u32(u32::try_from(value.len()).expect("payload length was prevalidated"));
        self.raw_bytes(value.as_bytes());
    }

    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}
