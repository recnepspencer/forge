use forge_proof::TransitionOutcome;

use crate::{
    StoreCustodyPosture, StoreKeyVersionPosture, StoreLegacySecurityPosture, StoreSecurityMetadata,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityScopePropagationSite {
    StableReadProtection,
    StableReadRootObservation,
    LogicalDecodeEntry,
    RecoveryAdmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StoreSecurityScopePropagationCounters {
    preserved: u64,
    missing: u64,
    stale: u64,
    drifted: u64,
    unsupported: u64,
    unavailable: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityScopePropagationWitness {
    metadata: StoreSecurityMetadata,
    counters: StoreSecurityScopePropagationCounters,
    site: StoreSecurityScopePropagationSite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityScopePropagationDenial {
    kind: StoreSecurityScopePropagationDenialKind,
    counters: StoreSecurityScopePropagationCounters,
    site: StoreSecurityScopePropagationSite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityScopePropagationDenialKind {
    MissingPropagatedSecurityScope,
    StalePropagatedSecurityScope,
    ScopeDriftBeforeLogicalDecode,
    UnsupportedPropagatedSecurityScope,
    UnavailablePropagatedSecurityScope,
}

pub type StoreSecurityScopePropagationOutcome =
    TransitionOutcome<StoreSecurityScopePropagationWitness, StoreSecurityScopePropagationDenial>;

pub fn propagate_store_security_scope(
    expected: StoreSecurityMetadata,
    observed: StoreSecurityMetadata,
    site: StoreSecurityScopePropagationSite,
) -> StoreSecurityScopePropagationOutcome {
    if let Some(denial) = classify_candidate_denial(observed) {
        return TransitionOutcome::denied(StoreSecurityScopePropagationDenial::new(
            denial,
            StoreSecurityScopePropagationCounters::default().with_denial(denial),
            site,
        ));
    }

    if expected == observed {
        return TransitionOutcome::success(StoreSecurityScopePropagationWitness {
            metadata: observed,
            counters: StoreSecurityScopePropagationCounters::default().with_preserved(),
            site,
        });
    }

    TransitionOutcome::denied(StoreSecurityScopePropagationDenial::new(
        StoreSecurityScopePropagationDenialKind::ScopeDriftBeforeLogicalDecode,
        StoreSecurityScopePropagationCounters::default().with_drifted(),
        site,
    ))
}

pub fn deny_missing_store_security_scope(
    site: StoreSecurityScopePropagationSite,
) -> StoreSecurityScopePropagationDenial {
    StoreSecurityScopePropagationDenial::new(
        StoreSecurityScopePropagationDenialKind::MissingPropagatedSecurityScope,
        StoreSecurityScopePropagationCounters::default().with_missing(),
        site,
    )
}

pub fn deny_stale_store_security_scope(
    site: StoreSecurityScopePropagationSite,
) -> StoreSecurityScopePropagationDenial {
    StoreSecurityScopePropagationDenial::new(
        StoreSecurityScopePropagationDenialKind::StalePropagatedSecurityScope,
        StoreSecurityScopePropagationCounters::default().with_stale(),
        site,
    )
}

pub fn deny_drifted_store_security_scope(
    site: StoreSecurityScopePropagationSite,
) -> StoreSecurityScopePropagationDenial {
    StoreSecurityScopePropagationDenial::new(
        StoreSecurityScopePropagationDenialKind::ScopeDriftBeforeLogicalDecode,
        StoreSecurityScopePropagationCounters::default().with_drifted(),
        site,
    )
}

fn classify_candidate_denial(
    metadata: StoreSecurityMetadata,
) -> Option<StoreSecurityScopePropagationDenialKind> {
    if metadata
        .legacy_posture()
        .requires_readmission_when_unscoped()
        || matches!(
            metadata.key_version_posture(),
            StoreKeyVersionPosture::Stale | StoreKeyVersionPosture::RebindRequired
        )
    {
        return Some(StoreSecurityScopePropagationDenialKind::StalePropagatedSecurityScope);
    }

    if matches!(
        metadata.key_version_posture(),
        StoreKeyVersionPosture::Unsupported | StoreKeyVersionPosture::Denied
    ) || matches!(
        metadata.custody_posture(),
        StoreCustodyPosture::CustodyUnsupported | StoreCustodyPosture::CustodyDenied
    ) || matches!(
        metadata.legacy_posture(),
        StoreLegacySecurityPosture::UnsupportedLegacyArtifact
    ) {
        return Some(StoreSecurityScopePropagationDenialKind::UnsupportedPropagatedSecurityScope);
    }

    if matches!(
        metadata.key_version_posture(),
        StoreKeyVersionPosture::Unavailable
    ) || matches!(
        metadata.custody_posture(),
        StoreCustodyPosture::CustodyUnavailable | StoreCustodyPosture::ImportedUnreadmitted
    ) || matches!(
        metadata.legacy_posture(),
        StoreLegacySecurityPosture::SecurityMetadataUnavailable
    ) {
        return Some(StoreSecurityScopePropagationDenialKind::UnavailablePropagatedSecurityScope);
    }

    None
}

impl StoreSecurityScopePropagationCounters {
    pub const fn empty() -> Self {
        Self {
            preserved: 0,
            missing: 0,
            stale: 0,
            drifted: 0,
            unsupported: 0,
            unavailable: 0,
        }
    }

    pub const fn with_preserved(self) -> Self {
        Self {
            preserved: self.preserved + 1,
            ..self
        }
    }

    pub const fn with_missing(self) -> Self {
        Self {
            missing: self.missing + 1,
            ..self
        }
    }

    pub const fn with_stale(self) -> Self {
        Self {
            stale: self.stale + 1,
            ..self
        }
    }

    pub const fn with_drifted(self) -> Self {
        Self {
            drifted: self.drifted + 1,
            ..self
        }
    }

    pub const fn with_unsupported(self) -> Self {
        Self {
            unsupported: self.unsupported + 1,
            ..self
        }
    }

    pub const fn with_unavailable(self) -> Self {
        Self {
            unavailable: self.unavailable + 1,
            ..self
        }
    }

    const fn with_denial(self, kind: StoreSecurityScopePropagationDenialKind) -> Self {
        match kind {
            StoreSecurityScopePropagationDenialKind::MissingPropagatedSecurityScope => {
                self.with_missing()
            }
            StoreSecurityScopePropagationDenialKind::StalePropagatedSecurityScope => {
                self.with_stale()
            }
            StoreSecurityScopePropagationDenialKind::ScopeDriftBeforeLogicalDecode => {
                self.with_drifted()
            }
            StoreSecurityScopePropagationDenialKind::UnsupportedPropagatedSecurityScope => {
                self.with_unsupported()
            }
            StoreSecurityScopePropagationDenialKind::UnavailablePropagatedSecurityScope => {
                self.with_unavailable()
            }
        }
    }

    pub const fn preserved(self) -> u64 {
        self.preserved
    }

    pub const fn missing(self) -> u64 {
        self.missing
    }

    pub const fn stale(self) -> u64 {
        self.stale
    }

    pub const fn drifted(self) -> u64 {
        self.drifted
    }

    pub const fn unsupported(self) -> u64 {
        self.unsupported
    }

    pub const fn unavailable(self) -> u64 {
        self.unavailable
    }
}

impl StoreSecurityScopePropagationWitness {
    pub const fn metadata(self) -> StoreSecurityMetadata {
        self.metadata
    }

    pub const fn counters(self) -> StoreSecurityScopePropagationCounters {
        self.counters
    }

    pub const fn site(self) -> StoreSecurityScopePropagationSite {
        self.site
    }
}

impl StoreSecurityScopePropagationDenial {
    const fn new(
        kind: StoreSecurityScopePropagationDenialKind,
        counters: StoreSecurityScopePropagationCounters,
        site: StoreSecurityScopePropagationSite,
    ) -> Self {
        Self {
            kind,
            counters,
            site,
        }
    }

    pub const fn kind(self) -> StoreSecurityScopePropagationDenialKind {
        self.kind
    }

    pub const fn counters(self) -> StoreSecurityScopePropagationCounters {
        self.counters
    }

    pub const fn site(self) -> StoreSecurityScopePropagationSite {
        self.site
    }
}
