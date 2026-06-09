use crate::ForgeServerCompatibilityRead;

#[derive(Debug)]
pub(super) struct ForgeServerStreamCursor {
    row_count: usize,
    row_index: usize,
    pending_fragment: ForgeServerPendingFragment,
    emitted_open_bracket: bool,
    emitted_close_bracket: bool,
    need_row_separator: bool,
}

#[derive(Debug)]
enum ForgeServerPendingFragment {
    None,
    Static { bytes: &'static [u8], offset: usize },
    Row { bytes: Vec<u8>, offset: usize },
}

impl ForgeServerStreamCursor {
    pub(super) fn from_read(read: &ForgeServerCompatibilityRead) -> Self {
        Self {
            row_count: read.read_result().rows().len(),
            row_index: 0,
            pending_fragment: ForgeServerPendingFragment::None,
            emitted_open_bracket: false,
            emitted_close_bracket: false,
            need_row_separator: false,
        }
    }

    pub(super) fn next_chunk(
        &mut self,
        read: &ForgeServerCompatibilityRead,
        chunk_bytes: usize,
    ) -> Result<Option<Vec<u8>>, serde_json::Error> {
        if self.is_done() {
            return Ok(None);
        }
        let mut chunk = Vec::new();
        while chunk.len() < chunk_bytes && !self.is_done() {
            self.ensure_pending_fragment(read)?;
            let piece = self.take_from_pending(chunk_bytes - chunk.len());
            chunk.extend_from_slice(&piece);
        }
        Ok(Some(chunk))
    }

    pub(super) fn is_done(&self) -> bool {
        self.emitted_open_bracket
            && self.emitted_close_bracket
            && matches!(self.pending_fragment, ForgeServerPendingFragment::None)
    }

    fn ensure_pending_fragment(
        &mut self,
        read: &ForgeServerCompatibilityRead,
    ) -> Result<(), serde_json::Error> {
        if !matches!(self.pending_fragment, ForgeServerPendingFragment::None) {
            return Ok(());
        }
        if !self.emitted_open_bracket {
            self.pending_fragment = ForgeServerPendingFragment::Static {
                bytes: b"[",
                offset: 0,
            };
            self.emitted_open_bracket = true;
            return Ok(());
        }
        if self.row_index < self.row_count {
            if self.need_row_separator {
                self.pending_fragment = ForgeServerPendingFragment::Static {
                    bytes: b",",
                    offset: 0,
                };
                self.need_row_separator = false;
                return Ok(());
            }
            let row = &read.read_result().rows()[self.row_index];
            self.pending_fragment = ForgeServerPendingFragment::Row {
                bytes: serde_json::to_vec(row.external_row())?,
                offset: 0,
            };
            self.row_index += 1;
            return Ok(());
        }
        if !self.emitted_close_bracket {
            self.pending_fragment = ForgeServerPendingFragment::Static {
                bytes: b"]",
                offset: 0,
            };
            self.emitted_close_bracket = true;
        }
        Ok(())
    }

    fn take_from_pending(&mut self, max_bytes: usize) -> Vec<u8> {
        match &mut self.pending_fragment {
            ForgeServerPendingFragment::Static { bytes, offset } => {
                let take = (bytes.len() - *offset).min(max_bytes);
                let piece = bytes[*offset..*offset + take].to_vec();
                *offset += take;
                if *offset == bytes.len() {
                    self.pending_fragment = ForgeServerPendingFragment::None;
                }
                piece
            }
            ForgeServerPendingFragment::Row { bytes, offset } => {
                let take = (bytes.len() - *offset).min(max_bytes);
                let piece = bytes[*offset..*offset + take].to_vec();
                *offset += take;
                if *offset == bytes.len() {
                    self.pending_fragment = ForgeServerPendingFragment::None;
                    self.need_row_separator = self.row_index < self.row_count;
                }
                piece
            }
            ForgeServerPendingFragment::None => Vec::new(),
        }
    }
}

pub(super) fn estimate_payload_bytes(
    read: &ForgeServerCompatibilityRead,
) -> Result<usize, serde_json::Error> {
    let mut bytes = 2usize;
    for (index, row) in read.read_result().rows().iter().enumerate() {
        if index > 0 {
            bytes += 1;
        }
        bytes += serde_json::to_vec(row.external_row())?.len();
    }
    Ok(bytes)
}

pub(super) fn materialize_payload_bytes(
    read: &ForgeServerCompatibilityRead,
) -> Result<Vec<u8>, serde_json::Error> {
    let mut payload = Vec::with_capacity(estimate_payload_bytes(read)?);
    payload.push(b'[');
    for (index, row) in read.read_result().rows().iter().enumerate() {
        if index > 0 {
            payload.push(b',');
        }
        payload.extend_from_slice(&serde_json::to_vec(row.external_row())?);
    }
    payload.push(b']');
    Ok(payload)
}
