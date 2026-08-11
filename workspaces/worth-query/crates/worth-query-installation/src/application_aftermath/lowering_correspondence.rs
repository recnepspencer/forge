//! Install-time Bridge lowering correspondence catalog (R8.9 / G8).
//!
//! Declaration carries a diagnostic slot string. Installation resolves that slot
//! against a typed catalog of installed correspondences and stores the resolved
//! value. The slot alone is never binding identity.

use worth_foundational::facade::CanonicalDigestId;

/// Typed installed Bridge correspondence resolved at aftermath installation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InstalledLoweringCorrespondence {
    /// Diagnostic label only — never the binding identity (R8.9).
    correspondence_slot: String,
    correspondence_identity: CanonicalDigestId,
    compatibility_generation: u64,
    graph_participation_identity: CanonicalDigestId,
}

impl InstalledLoweringCorrespondence {
    pub fn new(
        correspondence_slot: impl Into<String>,
        correspondence_identity: CanonicalDigestId,
        compatibility_generation: u64,
        graph_participation_identity: CanonicalDigestId,
    ) -> Result<Self, &'static str> {
        let correspondence_slot = correspondence_slot.into();
        if correspondence_slot.trim().is_empty() {
            return Err("empty-lowering-correspondence-slot");
        }
        Ok(Self {
            correspondence_slot,
            correspondence_identity,
            compatibility_generation,
            graph_participation_identity,
        })
    }

    pub fn correspondence_slot(&self) -> &str {
        &self.correspondence_slot
    }

    pub const fn correspondence_identity(&self) -> &CanonicalDigestId {
        &self.correspondence_identity
    }

    pub const fn compatibility_generation(&self) -> u64 {
        self.compatibility_generation
    }

    pub const fn graph_participation_identity(&self) -> &CanonicalDigestId {
        &self.graph_participation_identity
    }
}

/// Host-populated catalog of installed Bridge correspondences available at
/// aftermath installation. Query installation resolves declared slots here
/// without importing Runtime Bridge.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AftermathLoweringCorrespondenceCatalog {
    entries: Vec<InstalledLoweringCorrespondence>,
}

impl AftermathLoweringCorrespondenceCatalog {
    pub const fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn new(entries: impl IntoIterator<Item = InstalledLoweringCorrespondence>) -> Self {
        Self {
            entries: entries.into_iter().collect(),
        }
    }

    pub fn entries(&self) -> &[InstalledLoweringCorrespondence] {
        &self.entries
    }

    /// Resolve a declared slot against this catalog for one installation.
    pub fn resolve(
        &self,
        correspondence_slot: &str,
        expected_generation: u64,
        expected_graph_participation: &CanonicalDigestId,
    ) -> Result<InstalledLoweringCorrespondence, LoweringCorrespondenceResolutionDenial> {
        let matches: Vec<_> = self
            .entries
            .iter()
            .filter(|entry| entry.correspondence_slot() == correspondence_slot)
            .cloned()
            .collect();
        match matches.as_slice() {
            [] => Err(LoweringCorrespondenceResolutionDenial::Unresolved),
            [candidate] => {
                if candidate.compatibility_generation() != expected_generation {
                    return Err(LoweringCorrespondenceResolutionDenial::WrongGeneration);
                }
                if candidate.graph_participation_identity() != expected_graph_participation {
                    return Err(
                        LoweringCorrespondenceResolutionDenial::MismatchedGraphParticipation,
                    );
                }
                Ok(candidate.clone())
            }
            _ => Err(LoweringCorrespondenceResolutionDenial::Ambiguous),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoweringCorrespondenceResolutionDenial {
    Unresolved,
    WrongGeneration,
    MismatchedGraphParticipation,
    Ambiguous,
}
