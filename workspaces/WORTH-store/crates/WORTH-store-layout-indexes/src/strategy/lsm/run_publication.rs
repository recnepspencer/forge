use crate::key_domain::{CanonicalKeyBytes, ComparatorLaw};
use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LsmRunPublicationLaw {
    comparator: ComparatorLaw,
    manifest_sequence_must_advance: bool,
}

impl S8LsmRunPublicationLaw {
    pub(crate) const fn new(comparator: ComparatorLaw) -> Self {
        Self {
            comparator,
            manifest_sequence_must_advance: true,
        }
    }

    pub const fn comparator(self) -> ComparatorLaw {
        self.comparator
    }

    pub const fn verify_manifest_publication(
        self,
        previous_manifest_sequence: u64,
        next_manifest_sequence: u64,
        published_run_count: u16,
    ) -> Result<(), S8StrategyDenial> {
        if (!self.manifest_sequence_must_advance
            || next_manifest_sequence > previous_manifest_sequence)
            && published_run_count > 0
        {
            return Ok(());
        }
        Err(S8StrategyDenial::RootPublicationViolation)
    }

    pub const fn verify_manifest_publication_progress(
        self,
        manifest_sequence_advanced: bool,
        published_run_count: u16,
    ) -> Result<(), S8StrategyDenial> {
        if (!self.manifest_sequence_must_advance || manifest_sequence_advanced)
            && published_run_count > 0
        {
            return Ok(());
        }
        Err(S8StrategyDenial::RootPublicationViolation)
    }

    pub const fn verify_manifest_update_protocol(
        self,
        previous_manifest_sequence: u64,
        next_manifest_sequence: u64,
        stale_runs_removed: bool,
    ) -> Result<(), S8StrategyDenial> {
        if (!self.manifest_sequence_must_advance
            || next_manifest_sequence > previous_manifest_sequence)
            && stale_runs_removed
        {
            return Ok(());
        }
        Err(S8StrategyDenial::ManifestUpdateViolation)
    }

    pub const fn verify_manifest_update_progress(
        self,
        manifest_sequence_advanced: bool,
        stale_runs_removed: bool,
    ) -> Result<(), S8StrategyDenial> {
        if (!self.manifest_sequence_must_advance || manifest_sequence_advanced)
            && stale_runs_removed
        {
            return Ok(());
        }
        Err(S8StrategyDenial::ManifestUpdateViolation)
    }

    pub fn verify_sorted_run_lookup(
        self,
        probe: &CanonicalKeyBytes,
        run_start: &CanonicalKeyBytes,
        run_end: &CanonicalKeyBytes,
    ) -> Result<bool, S8StrategyDenial> {
        if probe.encoding() != self.comparator.encoding()
            || run_start.encoding() != self.comparator.encoding()
            || run_end.encoding() != self.comparator.encoding()
        {
            return Err(S8StrategyDenial::ComparatorOrderViolation);
        }
        Ok(run_start.as_bytes() <= probe.as_bytes() && probe.as_bytes() < run_end.as_bytes())
    }

    pub const fn verify_sorted_run_membership(
        self,
        probe_within_run: bool,
    ) -> Result<bool, S8StrategyDenial> {
        Ok(probe_within_run)
    }

    pub fn verify_merge_order(
        self,
        newer_start: &CanonicalKeyBytes,
        older_start: &CanonicalKeyBytes,
        newer_precedence_preserved: bool,
    ) -> Result<(), S8StrategyDenial> {
        if newer_start.encoding() != self.comparator.encoding()
            || older_start.encoding() != self.comparator.encoding()
        {
            return Err(S8StrategyDenial::MergeOrderingViolation);
        }
        if older_start.as_bytes() <= newer_start.as_bytes() && newer_precedence_preserved {
            return Ok(());
        }
        Err(S8StrategyDenial::MergeOrderingViolation)
    }

    pub const fn verify_merge_order_boundary(
        self,
        older_precedes_newer_start: bool,
        newer_precedence_preserved: bool,
    ) -> Result<(), S8StrategyDenial> {
        if older_precedes_newer_start && newer_precedence_preserved {
            return Ok(());
        }
        Err(S8StrategyDenial::MergeOrderingViolation)
    }
}
