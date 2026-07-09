#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerStreamingChunk {
    ordinal: usize,
    bytes: Vec<u8>,
    terminal: bool,
}

impl WorthServerStreamingChunk {
    pub(crate) fn new(ordinal: usize, bytes: Vec<u8>, terminal: bool) -> Self {
        Self {
            ordinal,
            bytes,
            terminal,
        }
    }

    pub fn ordinal(&self) -> usize {
        self.ordinal
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn is_terminal(&self) -> bool {
        self.terminal
    }
}
