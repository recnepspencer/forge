#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorthServerStreamDisposition {
    Buffered,
    Incremental,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerStreamSelection {
    disposition: WorthServerStreamDisposition,
    chunk_bytes: usize,
    background_export_threshold_bytes: Option<usize>,
}

impl WorthServerStreamSelection {
    pub fn buffered() -> Self {
        Self {
            disposition: WorthServerStreamDisposition::Buffered,
            chunk_bytes: 16 * 1024,
            background_export_threshold_bytes: None,
        }
    }

    pub fn incremental() -> Self {
        Self {
            disposition: WorthServerStreamDisposition::Incremental,
            chunk_bytes: 16 * 1024,
            background_export_threshold_bytes: None,
        }
    }

    pub fn with_chunk_bytes(mut self, chunk_bytes: usize) -> Self {
        self.chunk_bytes = chunk_bytes.max(1);
        self
    }

    pub fn with_background_export_threshold_bytes(mut self, threshold_bytes: usize) -> Self {
        self.background_export_threshold_bytes = Some(threshold_bytes.max(1));
        self
    }

    pub fn chunk_bytes(&self) -> usize {
        self.chunk_bytes
    }

    pub(crate) fn is_buffered(&self) -> bool {
        matches!(self.disposition, WorthServerStreamDisposition::Buffered)
    }

    pub(crate) fn background_export_threshold_bytes(&self) -> Option<usize> {
        self.background_export_threshold_bytes
    }

    pub(crate) fn canonical_digest(&self) -> String {
        format!(
            "compat-http-stream-selection-v1|mode:{}|chunk_bytes:{}|background_threshold:{}",
            match self.disposition {
                WorthServerStreamDisposition::Buffered => "buffered",
                WorthServerStreamDisposition::Incremental => "incremental",
            },
            self.chunk_bytes,
            self.background_export_threshold_bytes
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_string()),
        )
    }
}
