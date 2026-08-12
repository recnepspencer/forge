//! The death notice this rail exists to serve, and the rail's own decoder for
//! it.
//!
//! The decoder is deliberately independent. This crate shares no type, no
//! constant, and no encoder with the bank that dispatches to it — it agrees
//! with the bank only about the wire, which is the only thing two separate
//! processes can honestly agree about. If both sides called the same function
//! the "decode" would prove nothing about the boundary.

use serde::{Deserialize, Serialize};
use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolUnsupportedVersion};

use super::payload::RailEffectPayload;
use super::support_profile::RailProtocolSupportProfile;

/// The effect name this rail serves. Anything else is refused.
const SERVED_EFFECT: &str = "EstateDeathNotificationEffect";
/// The stable payload protocol this rail independently knows how to decode.
const SERVED_PROTOCOL_IDENTITY: BoundaryProtocolIdentity =
    BoundaryProtocolIdentity::new("bank.estate.death-notification");
const V1_MAXIMUM_BYTES: u64 = 24;
const V2_MAXIMUM_BYTES: u64 = 32;
/// Three big-endian `u64`s: estate, notice, subject.
const NOTICE_BYTES: usize = 24;

/// One death notice as the rail itself understands it.
///
/// This is domain meaning, not a token: the rail can tell you which estate was
/// notified, under which notice, about whom. A correlation-only protocol could
/// not produce this value at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EstateDeathNotice {
    estate: u64,
    notice: u64,
    subject: u64,
}

impl EstateDeathNotice {
    pub const fn estate(&self) -> u64 {
        self.estate
    }

    pub const fn notice(&self) -> u64 {
        self.notice
    }

    pub const fn subject(&self) -> u64 {
        self.subject
    }
}

/// Why the rail refused a dispatch outright.
///
/// A rejection is a determinate answer: the rail read the payload, understood
/// that it could not serve it, and did not admit it to the ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RailRejection {
    /// The payload names an effect this rail does not serve.
    UnknownEffect,
    /// The payload names a protocol family this rail does not serve.
    UnknownProtocolIdentity,
    /// The family is known but the exact produced version is unsupported.
    UnsupportedProtocolVersion(BoundaryProtocolUnsupportedVersion),
    /// The caller's claimed bound differs from the served protocol contract.
    DeclaredBoundMismatch,
    /// The bytes are not a decodable notice.
    MalformedNotice,
    /// This correlation was already reserved for different immutable meaning.
    CorrelationPayloadMismatch,
}

pub(crate) fn decode_notice_for_profile(
    payload: &RailEffectPayload,
    profile: RailProtocolSupportProfile,
) -> Result<EstateDeathNotice, RailRejection> {
    if payload.effect() != SERVED_EFFECT {
        return Err(RailRejection::UnknownEffect);
    }
    if payload.protocol_identity() != &SERVED_PROTOCOL_IDENTITY {
        return Err(RailRejection::UnknownProtocolIdentity);
    }
    let version = profile
        .compatibility_window()
        .admit(payload.protocol_version())
        .map_err(RailRejection::UnsupportedProtocolVersion)?;
    let expected_maximum = match version.get() {
        1 => V1_MAXIMUM_BYTES,
        2 => V2_MAXIMUM_BYTES,
        _ => return Err(RailRejection::MalformedNotice),
    };
    if payload.maximum_bytes() != expected_maximum {
        return Err(RailRejection::DeclaredBoundMismatch);
    }
    let bytes = payload.bytes();
    let bytes = match version.get() {
        1 if bytes.len() == NOTICE_BYTES => bytes,
        2 if bytes.len() == NOTICE_BYTES + 8 && &bytes[..8] == b"DEATHV2!" => &bytes[8..],
        _ => return Err(RailRejection::MalformedNotice),
    };
    Ok(EstateDeathNotice {
        estate: field(bytes, 0),
        notice: field(bytes, 1),
        subject: field(bytes, 2),
    })
}

fn field(bytes: &[u8], index: usize) -> u64 {
    let start = index * 8;
    let mut word = [0u8; 8];
    word.copy_from_slice(&bytes[start..start + 8]);
    u64::from_be_bytes(word)
}
