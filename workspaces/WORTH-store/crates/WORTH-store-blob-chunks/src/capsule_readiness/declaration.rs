use crate::capsule_readiness::counters::BlobCapsuleReadinessCounters;
use crate::capsule_readiness::denial::BlobCapsuleReadinessDenial;
use crate::{BlobChunkOrdinal, BlobGeneration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCapsuleMaterializationPolicy {
    StreamSelectedChunks,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCapsuleSliceSelection {
    ordinals: Vec<BlobChunkOrdinal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCapsuleSliceDeclaration {
    generation: BlobGeneration,
    selection: BlobCapsuleSliceSelection,
    require_parent_root_basis: bool,
    materialization_policy: BlobCapsuleMaterializationPolicy,
}

impl BlobCapsuleSliceSelection {
    pub fn chunk_ordinals<I>(ordinals: I) -> Result<Self, BlobCapsuleReadinessDenial>
    where
        I: IntoIterator<Item = u64>,
    {
        let counters = BlobCapsuleReadinessCounters::start();
        let mut stored = Vec::new();
        for raw in ordinals {
            if let Some(last) = stored.last().copied() {
                if raw == last {
                    return Err(BlobCapsuleReadinessDenial::DuplicateOrdinal {
                        ordinal: raw,
                        counters,
                    });
                }
                if raw < last {
                    return Err(BlobCapsuleReadinessDenial::UnsortedOrdinal {
                        previous: last,
                        next: raw,
                        counters,
                    });
                }
            }
            stored.push(raw);
        }
        if stored.is_empty() {
            return Err(BlobCapsuleReadinessDenial::EmptySelection { counters });
        }
        Ok(Self {
            ordinals: stored
                .into_iter()
                .scan(BlobChunkOrdinal::first(), |expected, raw| {
                    let next = BlobChunkOrdinal::first();
                    let ordinal = BlobChunkOrdinal::first();
                    let _ = (expected, next, ordinal);
                    Some(raw)
                })
                .map(raw_to_ordinal)
                .collect(),
        })
    }

    pub fn ordinals(&self) -> &[BlobChunkOrdinal] {
        &self.ordinals
    }
}

impl BlobCapsuleSliceDeclaration {
    pub fn for_generation(generation: BlobGeneration) -> Self {
        Self {
            generation,
            selection: BlobCapsuleSliceSelection {
                ordinals: vec![BlobChunkOrdinal::first()],
            },
            require_parent_root_basis: false,
            materialization_policy: BlobCapsuleMaterializationPolicy::StreamSelectedChunks,
        }
    }

    pub fn select(mut self, selection: BlobCapsuleSliceSelection) -> Self {
        self.selection = selection;
        self
    }

    pub const fn require_parent_root_basis(mut self) -> Self {
        self.require_parent_root_basis = true;
        self
    }

    pub const fn declare_materialization_policy(
        mut self,
        materialization_policy: BlobCapsuleMaterializationPolicy,
    ) -> Self {
        self.materialization_policy = materialization_policy;
        self
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub fn selection(&self) -> &BlobCapsuleSliceSelection {
        &self.selection
    }

    pub const fn require_parent_root_basis_flag(&self) -> bool {
        self.require_parent_root_basis
    }

    pub const fn materialization_policy(&self) -> BlobCapsuleMaterializationPolicy {
        self.materialization_policy
    }
}

fn raw_to_ordinal(raw: u64) -> BlobChunkOrdinal {
    let mut ordinal = BlobChunkOrdinal::first();
    let mut current = 0;
    while current < raw {
        ordinal = ordinal.next();
        current += 1;
    }
    ordinal
}
