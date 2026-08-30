use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::limits::WorthQueryPackageArchiveLimits;
use worth_query_installation::facade::{
    WORTH_QUERY_PORTABLE_PACKAGE_MAX_LOGICAL_EXPORT_BYTES,
    WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECONSTRUCTION_ENTRIES,
};

const MAXIMUM_ENVELOPE_REQUIREMENTS: u32 = {
    assert!(WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECONSTRUCTION_ENTRIES <= u32::MAX as u64);
    WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECONSTRUCTION_ENTRIES as u32
};

use super::SIGNATURE_LENGTH_BYTES;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryPackageEnvelopeLimits {
    maximum_envelope_bytes: u64,
    maximum_archive_bytes: u64,
    maximum_descriptive_text_bytes: u64,
    maximum_requirements: u32,
    maximum_signature_bytes: u32,
}

impl WorthQueryPackageEnvelopeLimits {
    pub const DEFAULT: Self = Self {
        maximum_envelope_bytes: WorthQueryPackageArchiveLimits::DEFAULT.maximum_archive_bytes()
            + WORTH_QUERY_PORTABLE_PACKAGE_MAX_LOGICAL_EXPORT_BYTES
            + 1_024 * 1_024,
        maximum_archive_bytes: WorthQueryPackageArchiveLimits::DEFAULT.maximum_archive_bytes(),
        maximum_descriptive_text_bytes: WORTH_QUERY_PORTABLE_PACKAGE_MAX_LOGICAL_EXPORT_BYTES,
        maximum_requirements: MAXIMUM_ENVELOPE_REQUIREMENTS,
        maximum_signature_bytes: 16 * 1_024,
    };

    pub const fn new(
        maximum_envelope_bytes: u64,
        maximum_archive_bytes: u64,
        maximum_descriptive_text_bytes: u64,
        maximum_requirements: u32,
        maximum_signature_bytes: u32,
    ) -> Self {
        Self {
            maximum_envelope_bytes,
            maximum_archive_bytes,
            maximum_descriptive_text_bytes,
            maximum_requirements,
            maximum_signature_bytes,
        }
    }

    pub const fn maximum_envelope_bytes(self) -> u64 {
        self.maximum_envelope_bytes
    }

    pub const fn maximum_archive_bytes(self) -> u64 {
        self.maximum_archive_bytes
    }

    pub const fn maximum_descriptive_text_bytes(self) -> u64 {
        self.maximum_descriptive_text_bytes
    }

    pub const fn maximum_requirements(self) -> u32 {
        self.maximum_requirements
    }

    pub const fn maximum_signature_bytes(self) -> u32 {
        self.maximum_signature_bytes
    }

    pub(crate) const fn narrowed(self) -> Self {
        Self {
            maximum_envelope_bytes: minimum(
                self.maximum_envelope_bytes,
                Self::DEFAULT.maximum_envelope_bytes,
            ),
            maximum_archive_bytes: minimum(
                self.maximum_archive_bytes,
                Self::DEFAULT.maximum_archive_bytes,
            ),
            maximum_descriptive_text_bytes: minimum(
                self.maximum_descriptive_text_bytes,
                Self::DEFAULT.maximum_descriptive_text_bytes,
            ),
            maximum_requirements: minimum_u32(
                self.maximum_requirements,
                Self::DEFAULT.maximum_requirements,
            ),
            maximum_signature_bytes: minimum_u32(
                self.maximum_signature_bytes,
                Self::DEFAULT.maximum_signature_bytes,
            ),
        }
    }
}

const fn minimum(left: u64, right: u64) -> u64 {
    if left < right {
        left
    } else {
        right
    }
}

const fn minimum_u32(left: u32, right: u32) -> u32 {
    if left < right {
        left
    } else {
        right
    }
}

pub(crate) fn require_signature_budget(
    observed: usize,
    limits: WorthQueryPackageEnvelopeLimits,
) -> Result<(), Denial> {
    let limits = limits.narrowed();
    if u64::try_from(observed).unwrap_or(u64::MAX) > u64::from(limits.maximum_signature_bytes()) {
        return Err(Denial::new(Kind::EnvelopeSignatureByteBudgetExceeded));
    }
    Ok(())
}

pub(crate) fn require_complete_envelope_budget(
    signing_payload_bytes: usize,
    signature_bytes: usize,
    limits: WorthQueryPackageEnvelopeLimits,
) -> Result<(), Denial> {
    let observed = u64::try_from(signing_payload_bytes)
        .ok()
        .and_then(|bytes| bytes.checked_add(SIGNATURE_LENGTH_BYTES))
        .and_then(|bytes| bytes.checked_add(u64::try_from(signature_bytes).ok()?))
        .ok_or_else(|| Denial::new(Kind::EnvelopeByteBudgetExceeded))?;
    if observed > limits.narrowed().maximum_envelope_bytes() {
        return Err(Denial::new(Kind::EnvelopeByteBudgetExceeded));
    }
    Ok(())
}
