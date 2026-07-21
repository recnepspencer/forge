use std::io::{self, Write};

pub(super) enum WorthServerBoundedJsonSerializationError {
    Serialization(serde_json::Error),
    InlineBudgetExceeded,
}

pub(super) fn serialize_with_inline_budget<T>(
    value: &T,
    max_inline_bytes: usize,
) -> Result<Vec<u8>, WorthServerBoundedJsonSerializationError>
where
    T: serde::Serialize,
{
    let mut writer = InlineBudgetWriter::new(max_inline_bytes);
    match serde_json::to_writer(&mut writer, value) {
        Ok(()) => Ok(writer.into_bytes()),
        Err(_) if writer.budget_exceeded() => {
            Err(WorthServerBoundedJsonSerializationError::InlineBudgetExceeded)
        }
        Err(error) => Err(WorthServerBoundedJsonSerializationError::Serialization(
            error,
        )),
    }
}

struct InlineBudgetWriter {
    bytes: Vec<u8>,
    max_inline_bytes: usize,
    budget_exceeded: bool,
}

impl InlineBudgetWriter {
    fn new(max_inline_bytes: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(max_inline_bytes.min(4 * 1024)),
            max_inline_bytes,
            budget_exceeded: false,
        }
    }

    fn budget_exceeded(&self) -> bool {
        self.budget_exceeded
    }

    fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl Write for InlineBudgetWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let remaining = self.max_inline_bytes.saturating_sub(self.bytes.len());
        if buffer.len() > remaining {
            self.bytes.extend_from_slice(&buffer[..remaining]);
            self.budget_exceeded = true;
            return Err(io::Error::other(
                "product result exceeded its declared inline budget",
            ));
        }
        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
