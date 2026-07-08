use super::btree::{declare_btree_invariant_suite, S8BTreeInvariantSuite};
use super::counter_evidence::S8StrategyCounterEvidence;
use super::counter_path::derive_strategy_counter_evidence;
use super::declaration::S8StrategyDeclaration;
use super::lsm::{declare_lsm_invariant_suite, S8LsmInvariantSuite};
use super::{S8LayoutStrategyFamily, S8StrategyDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8StrategyLookupInvariant {
    SeparatorDirectedLookup,
    NewestRunLookup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8StrategyPublicationInvariant {
    RootPublication,
    ManifestPublication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8StrategyMutationInvariant {
    SplitMaintainsOccupancy,
    TombstonesSurviveCompaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8StrategyIntegrityInvariant {
    ChecksumLocalizesCorruption,
    ManifestBindsRunDigests,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8StrategyRecoveryInvariant {
    StableReadReplay,
    WalReplayRecovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8StrategyCounterProfile {
    point_lookups: u16,
    range_lookups: u16,
    wal_replays: u16,
    publications: u16,
    maintenance_reads: u16,
}

impl S8StrategyCounterProfile {
    pub(crate) const fn new(
        point_lookups: u16,
        range_lookups: u16,
        wal_replays: u16,
        publications: u16,
        maintenance_reads: u16,
    ) -> Self {
        Self {
            point_lookups,
            range_lookups,
            wal_replays,
            publications,
            maintenance_reads,
        }
    }

    pub const fn point_lookups(self) -> u16 {
        self.point_lookups
    }

    pub const fn range_lookups(self) -> u16 {
        self.range_lookups
    }

    pub const fn wal_replays(self) -> u16 {
        self.wal_replays
    }

    pub const fn publications(self) -> u16 {
        self.publications
    }

    pub const fn maintenance_reads(self) -> u16 {
        self.maintenance_reads
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StrategySpecificInvariantSuite {
    BTree(S8BTreeInvariantSuite),
    Lsm(S8LsmInvariantSuite),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8StrategyInvariantSuite {
    declaration: S8StrategyDeclaration,
    lookup: S8StrategyLookupInvariant,
    publication: S8StrategyPublicationInvariant,
    mutation: S8StrategyMutationInvariant,
    integrity: S8StrategyIntegrityInvariant,
    recovery: S8StrategyRecoveryInvariant,
    counter_evidence: S8StrategyCounterEvidence,
    specific: StrategySpecificInvariantSuite,
}

impl S8StrategyInvariantSuite {
    pub(crate) fn declare(declaration: S8StrategyDeclaration) -> Result<Self, S8StrategyDenial> {
        match declaration.family() {
            S8LayoutStrategyFamily::BaselineBTreeRange => Ok(Self {
                declaration,
                lookup: S8StrategyLookupInvariant::SeparatorDirectedLookup,
                publication: S8StrategyPublicationInvariant::RootPublication,
                mutation: S8StrategyMutationInvariant::SplitMaintainsOccupancy,
                integrity: S8StrategyIntegrityInvariant::ChecksumLocalizesCorruption,
                recovery: S8StrategyRecoveryInvariant::StableReadReplay,
                counter_evidence: derive_strategy_counter_evidence(declaration),
                specific: StrategySpecificInvariantSuite::BTree(declare_btree_invariant_suite(
                    declaration,
                )?),
            }),
            S8LayoutStrategyFamily::BaselineLsmWriteOptimized => Ok(Self {
                declaration,
                lookup: S8StrategyLookupInvariant::NewestRunLookup,
                publication: S8StrategyPublicationInvariant::ManifestPublication,
                mutation: S8StrategyMutationInvariant::TombstonesSurviveCompaction,
                integrity: S8StrategyIntegrityInvariant::ManifestBindsRunDigests,
                recovery: S8StrategyRecoveryInvariant::WalReplayRecovery,
                counter_evidence: derive_strategy_counter_evidence(declaration),
                specific: StrategySpecificInvariantSuite::Lsm(declare_lsm_invariant_suite(
                    declaration,
                )?),
            }),
            _ => Err(S8StrategyDenial::InvariantSuiteNotAvailableForFamily),
        }
    }

    pub const fn family(self) -> S8LayoutStrategyFamily {
        self.declaration.family()
    }

    pub const fn lookup_invariant(self) -> S8StrategyLookupInvariant {
        self.lookup
    }

    pub const fn publication_invariant(self) -> S8StrategyPublicationInvariant {
        self.publication
    }

    pub const fn mutation_invariant(self) -> S8StrategyMutationInvariant {
        self.mutation
    }

    pub const fn integrity_invariant(self) -> S8StrategyIntegrityInvariant {
        self.integrity
    }

    pub const fn recovery_invariant(self) -> S8StrategyRecoveryInvariant {
        self.recovery
    }

    pub const fn counter_evidence(self) -> S8StrategyCounterEvidence {
        self.counter_evidence
    }

    pub const fn require_btree_suite(self) -> Result<S8BTreeInvariantSuite, S8StrategyDenial> {
        match self.specific {
            StrategySpecificInvariantSuite::BTree(suite) => Ok(suite),
            StrategySpecificInvariantSuite::Lsm(_) => {
                Err(S8StrategyDenial::InvariantSuiteNotAvailableForFamily)
            }
        }
    }

    pub const fn require_lsm_suite(self) -> Result<S8LsmInvariantSuite, S8StrategyDenial> {
        match self.specific {
            StrategySpecificInvariantSuite::Lsm(suite) => Ok(suite),
            StrategySpecificInvariantSuite::BTree(_) => {
                Err(S8StrategyDenial::InvariantSuiteNotAvailableForFamily)
            }
        }
    }
}
