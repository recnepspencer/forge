use serde::{Deserialize, Serialize};

use crate::data::error::SpecError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SpecShellOrientation {
    Outer,
    Inner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SpecShellKind {
    Solid(SpecShellOrientation),
    Sheet,
    Wire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellPayload {
    kind: SpecShellKind,
}

impl ShellPayload {
    pub const fn new(kind: SpecShellKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> SpecShellKind {
        self.kind
    }

    pub fn encode(self) -> Vec<u8> {
        let tag = match self.kind {
            SpecShellKind::Solid(SpecShellOrientation::Outer) => 0_u8,
            SpecShellKind::Solid(SpecShellOrientation::Inner) => 1_u8,
            SpecShellKind::Sheet => 2_u8,
            SpecShellKind::Wire => 3_u8,
        };
        vec![tag]
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, SpecError> {
        if bytes.len() != 1 {
            return Err(SpecError::invalid(format!(
                "shell payload must be exactly 1 byte, got {} bytes",
                bytes.len()
            )));
        }
        let kind = match bytes[0] {
            0 => SpecShellKind::Solid(SpecShellOrientation::Outer),
            1 => SpecShellKind::Solid(SpecShellOrientation::Inner),
            2 => SpecShellKind::Sheet,
            3 => SpecShellKind::Wire,
            tag => {
                return Err(SpecError::invalid(format!(
                    "unknown shell payload tag {}",
                    tag
                )))
            }
        };
        Ok(Self { kind })
    }
}
