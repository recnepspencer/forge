pub(crate) struct CanonicalProposalPayload {
    bytes: Vec<u8>,
}

impl CanonicalProposalPayload {
    pub(crate) fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub(crate) fn u64(mut self, value: u64) -> Self {
        self.part(&value.to_be_bytes());
        self
    }

    pub(crate) fn i64(mut self, value: i64) -> Self {
        self.part(&value.to_be_bytes());
        self
    }

    pub(crate) fn text(mut self, value: &str) -> Self {
        self.part(value.as_bytes());
        self
    }

    pub(crate) fn byte(mut self, value: u8) -> Self {
        self.part(&[value]);
        self
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn part(&mut self, value: &[u8]) {
        self.bytes
            .extend_from_slice(&(value.len() as u64).to_be_bytes());
        self.bytes.extend_from_slice(value);
    }
}
