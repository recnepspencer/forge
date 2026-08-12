//! The declared effect payload one dispatch carries across the rail wire.
//!
//! A correlation names *which* attempt this is. It says nothing about what the
//! attempt asks the rail to do. The payload is the part the rail can act on.

use serde::{Deserialize, Serialize};
use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};

/// What a caller projected onto the wire for one dispatch attempt.
///
/// The three descriptors travel with the bytes because the rail must not
/// assume an encoding: it is told which effect these bytes came from, which
/// stable protocol family and exact version produced them, and the bound that
/// protocol declared. A rail that had to infer any of those values would be
/// guessing at a boundary it does not own.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RailEffectPayload {
    effect: String,
    protocol_identity: BoundaryProtocolIdentity,
    protocol_version: BoundaryProtocolVersion,
    maximum_bytes: u64,
    bytes: Vec<u8>,
}

impl RailEffectPayload {
    pub fn new(
        effect: impl Into<String>,
        protocol_identity: BoundaryProtocolIdentity,
        protocol_version: BoundaryProtocolVersion,
        maximum_bytes: u64,
        bytes: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            effect: effect.into(),
            protocol_identity,
            protocol_version,
            maximum_bytes,
            bytes: bytes.into(),
        }
    }

    pub fn effect(&self) -> &str {
        &self.effect
    }

    pub const fn protocol_identity(&self) -> &BoundaryProtocolIdentity {
        &self.protocol_identity
    }

    pub const fn protocol_version(&self) -> BoundaryProtocolVersion {
        self.protocol_version
    }

    pub const fn maximum_bytes(&self) -> u64 {
        self.maximum_bytes
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
