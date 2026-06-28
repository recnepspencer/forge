use crate::StoreTerminalProjectionDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreTerminalProjectionDocumentBytes {
    terminal_projection_bytes: Vec<u8>,
}

impl StoreTerminalProjectionDocumentBytes {
    pub fn from_terminal_projection_bytes(
        terminal_projection_bytes: Vec<u8>,
    ) -> Result<Self, StoreTerminalProjectionDenial> {
        if terminal_projection_bytes.is_empty() {
            return Err(StoreTerminalProjectionDenial::EmptyTerminalProjectionDocument);
        }

        Ok(Self {
            terminal_projection_bytes,
        })
    }

    pub fn terminal_projection_bytes(&self) -> &[u8] {
        &self.terminal_projection_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreTerminalChecksumAlgorithm {
    StoreTerminalFnv1a64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreTerminalChecksumScope {
    TerminalJsonProjectionDocument,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreTerminalDocumentChecksum {
    scope: StoreTerminalChecksumScope,
    algorithm: StoreTerminalChecksumAlgorithm,
    checksum_bytes: [u8; 8],
}

impl StoreTerminalDocumentChecksum {
    pub fn for_terminal_projection_document_bytes(
        document: &StoreTerminalProjectionDocumentBytes,
    ) -> Self {
        Self {
            scope: StoreTerminalChecksumScope::TerminalJsonProjectionDocument,
            algorithm: StoreTerminalChecksumAlgorithm::StoreTerminalFnv1a64,
            checksum_bytes: terminal_projection_fnv1a64(document.terminal_projection_bytes()),
        }
    }

    pub const fn scope(&self) -> StoreTerminalChecksumScope {
        self.scope
    }

    pub const fn algorithm(&self) -> StoreTerminalChecksumAlgorithm {
        self.algorithm
    }

    pub const fn terminal_checksum_bytes(&self) -> &[u8; 8] {
        &self.checksum_bytes
    }
}

fn terminal_projection_fnv1a64(bytes: &[u8]) -> [u8; 8] {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash.to_be_bytes()
}
