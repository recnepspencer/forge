use super::{
    BlobCompactionColdReadiness, BlobCompactionCounterSnapshot, BlobCompactionDenial,
    BlobCompactionIntent, BlobCompactionPhysicalInterlock, BlobCompactionReadHold,
    BlobCompactionS6Pacing,
};
use crate::{
    AdmittedBlobPlacement, AuthenticatedFrameDigest, BlobAuthorityClassification,
    BlobChunkReachabilityProofSet, BlobChunkRegisteredDedupeReference, BlobChunkRootCanonicalBasis,
    BlobChunkRootPublication, BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectId,
    ChunkTreeRoot, LifecycleReceipt, LogicalContentDigest, StoredChunkDigest,
};
use forge_store_contracts::StableDigest;
use forge_store_physical_isolation::CompactionReadInterlockPlan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BlobCompactionBasis {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    old_root: ChunkTreeRoot,
    logical_digest: LogicalContentDigest,
    stored_digest: StoredChunkDigest,
    frame_digest: AuthenticatedFrameDigest,
    security: BlobChunkSecurityMetadataWitness,
    authority_class: BlobAuthorityClassification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCompactionRewritePlan {
    basis: BlobCompactionBasis,
    physical: CompactionReadInterlockPlan,
    reachability: BlobChunkReachabilityProofSet,
    placement: AdmittedBlobPlacement,
    old_canonical_basis: BlobChunkRootCanonicalBasis,
    dedupe_reference_identities: Vec<StableDigest>,
    counters: BlobCompactionCounterSnapshot,
}

impl BlobCompactionRewritePlan {
    pub(crate) fn admit(intent: BlobCompactionIntent) -> Result<Self, BlobCompactionDenial> {
        let counters = base_counters(&intent);
        let Some(physical) = intent.physical().admitted() else {
            let source = intent
                .physical()
                .denial()
                .expect("physical interlock denial should carry denial source");
            return Err(BlobCompactionDenial::PhysicalInterlockDenied {
                source,
                counters: counters.record_denial(),
            });
        };
        let Some(reachability) = intent.reachability() else {
            return Err(BlobCompactionDenial::MissingReachabilityProof {
                counters: counters.record_denial(),
            });
        };
        if intent.read_hold().is_active() {
            return Err(BlobCompactionDenial::ActiveReadHold {
                counters: counters.record_denial(),
            });
        }
        require_read_hold_matches_physical(intent.read_hold(), physical, counters)?;
        if !intent.pacing().supports_compaction() {
            return Err(BlobCompactionDenial::UnsupportedS6Pacing {
                counters: counters.record_denial(),
            });
        }
        if !intent.cold().permits_compaction() {
            return Err(BlobCompactionDenial::UnavailableColdChunk {
                state: intent.cold().state(),
                counters: counters.record_denial(),
            });
        }
        require_no_quarantine_holds(&intent, counters)?;
        require_uncompacted_publication(
            intent.lifecycle(),
            intent.uncompacted_publication(),
            counters,
        )?;
        require_lifecycle_reachability(intent.lifecycle(), reachability, counters)?;
        require_lifecycle_placement(intent.lifecycle(), intent.placement(), counters)?;
        require_dedupe_edges(intent.dedupe_references(), reachability, counters)?;

        let physical_counters = physical.counters();
        Ok(Self {
            basis: BlobCompactionBasis::from_lifecycle(intent.lifecycle()),
            physical: physical.clone(),
            reachability: reachability.clone(),
            placement: intent.placement().clone(),
            old_canonical_basis: intent.uncompacted_publication().canonical_basis().clone(),
            dedupe_reference_identities: intent
                .dedupe_references()
                .iter()
                .map(|reference| reference.reference_identity().clone())
                .collect(),
            counters: counters
                .with_physical(physical_counters)
                .preserve_dedupe_edges(intent.dedupe_references().len() as u64)
                .record_foreground_yields(intent.pacing().foreground_yields()),
        })
    }

    pub const fn counters(&self) -> BlobCompactionCounterSnapshot {
        self.counters
    }

    pub const fn old_root(&self) -> &ChunkTreeRoot {
        &self.basis.old_root
    }

    pub(crate) const fn basis(&self) -> &BlobCompactionBasis {
        &self.basis
    }

    pub const fn physical(&self) -> &CompactionReadInterlockPlan {
        &self.physical
    }

    pub const fn reachability(&self) -> &BlobChunkReachabilityProofSet {
        &self.reachability
    }

    pub const fn placement(&self) -> &AdmittedBlobPlacement {
        &self.placement
    }

    pub const fn old_canonical_basis(&self) -> &BlobChunkRootCanonicalBasis {
        &self.old_canonical_basis
    }

    pub fn dedupe_reference_identities(&self) -> &[StableDigest] {
        &self.dedupe_reference_identities
    }
}

impl BlobCompactionBasis {
    fn from_lifecycle(receipt: &LifecycleReceipt) -> Self {
        let declaration = receipt.declaration();
        Self {
            object_id: declaration.object_id().clone(),
            generation: declaration.generation(),
            old_root: declaration.chunk_tree_root().clone(),
            logical_digest: declaration.logical_content_digest().clone(),
            stored_digest: declaration.stored_chunk_digest().clone(),
            frame_digest: declaration.authenticated_frame_digest().clone(),
            security: declaration.security_metadata(),
            authority_class: declaration.authority_classification(),
        }
    }

    pub(crate) const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub(crate) const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub(crate) const fn old_root(&self) -> &ChunkTreeRoot {
        &self.old_root
    }

    pub(crate) const fn logical_digest(&self) -> &LogicalContentDigest {
        &self.logical_digest
    }

    pub(crate) const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub(crate) const fn frame_digest(&self) -> &AuthenticatedFrameDigest {
        &self.frame_digest
    }

    pub(crate) const fn security(&self) -> BlobChunkSecurityMetadataWitness {
        self.security
    }

    pub(crate) const fn authority_class(&self) -> BlobAuthorityClassification {
        self.authority_class
    }
}

fn base_counters(intent: &BlobCompactionIntent) -> BlobCompactionCounterSnapshot {
    let chunks = intent.reachability().map_or(0, |reachability| {
        reachability.reachable_chunks().len() as u64
    });
    let references = intent.reachability().map_or(0, |reachability| {
        reachability.reference_edges().len() as u64
    });
    BlobCompactionCounterSnapshot::start(
        chunks,
        references,
        intent
            .physical()
            .admitted()
            .map_or(0, |physical| physical.counters().copied_pages()),
    )
}

fn require_no_quarantine_holds(
    intent: &BlobCompactionIntent,
    counters: BlobCompactionCounterSnapshot,
) -> Result<(), BlobCompactionDenial> {
    if intent.quarantine_holds().is_empty() {
        Ok(())
    } else {
        Err(BlobCompactionDenial::QuarantineHold {
            counters: counters.record_denial(),
        })
    }
}

fn require_uncompacted_publication(
    lifecycle: &LifecycleReceipt,
    publication: &BlobChunkRootPublication,
    counters: BlobCompactionCounterSnapshot,
) -> Result<(), BlobCompactionDenial> {
    let declaration = lifecycle.declaration();
    if publication.chunk_tree_root() == declaration.chunk_tree_root()
        && publication.logical_content_digest() == declaration.logical_content_digest()
        && publication.canonical_basis().chunk_tree_root() == declaration.chunk_tree_root()
        && publication.canonical_basis().logical_content_digest()
            == declaration.logical_content_digest()
    {
        Ok(())
    } else {
        Err(BlobCompactionDenial::EquivalenceBasisMismatch {
            counters: counters.record_denial(),
        })
    }
}

fn require_read_hold_matches_physical(
    read_hold: BlobCompactionReadHold,
    physical: &CompactionReadInterlockPlan,
    counters: BlobCompactionCounterSnapshot,
) -> Result<(), BlobCompactionDenial> {
    let Some(receipt) = read_hold.released_receipt() else {
        return Err(BlobCompactionDenial::ActiveReadHold {
            counters: counters.record_denial(),
        });
    };
    let release = receipt.read_plan_release();
    if release.root() == physical.protected().root()
        && release.footprint_basis() == physical.protected().footprint_basis()
    {
        Ok(())
    } else {
        Err(BlobCompactionDenial::ReadHoldPlanMismatch {
            counters: counters.record_denial(),
        })
    }
}

fn require_lifecycle_reachability(
    lifecycle: &LifecycleReceipt,
    reachability: &BlobChunkReachabilityProofSet,
    counters: BlobCompactionCounterSnapshot,
) -> Result<(), BlobCompactionDenial> {
    if !reachability.matches_lifecycle_declaration(lifecycle.declaration()) {
        return Err(BlobCompactionDenial::LifecycleReachabilityMismatch {
            counters: counters.record_denial(),
        });
    }
    if reachability.protected_holds().is_empty() {
        return Ok(());
    }
    Err(BlobCompactionDenial::ActiveReadHold {
        counters: counters.record_denial(),
    })
}

fn require_lifecycle_placement(
    lifecycle: &LifecycleReceipt,
    placement: &AdmittedBlobPlacement,
    counters: BlobCompactionCounterSnapshot,
) -> Result<(), BlobCompactionDenial> {
    if placement.matches_reachability(lifecycle.reachability()) {
        Ok(())
    } else {
        Err(BlobCompactionDenial::LifecyclePlacementMismatch {
            counters: counters.record_denial(),
        })
    }
}

fn require_dedupe_edges(
    references: &[BlobChunkRegisteredDedupeReference],
    reachability: &BlobChunkReachabilityProofSet,
    counters: BlobCompactionCounterSnapshot,
) -> Result<(), BlobCompactionDenial> {
    for reference in references {
        if reference.security_metadata() != reachability.security_metadata() {
            return Err(BlobCompactionDenial::DedupeScopeMismatch {
                counters: counters.record_denial(),
            });
        }
        if !reachability
            .reachable_chunks()
            .iter()
            .any(|chunk| reference.contains_chunk_identity(chunk))
        {
            return Err(BlobCompactionDenial::StaleDedupeReference {
                counters: counters.record_denial(),
            });
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn _read_hold_is_part_of_the_boundary(_: BlobCompactionReadHold) {}

#[allow(dead_code)]
fn _pacing_is_part_of_the_boundary(_: BlobCompactionS6Pacing) {}

#[allow(dead_code)]
fn _cold_is_part_of_the_boundary(_: BlobCompactionColdReadiness) {}

#[allow(dead_code)]
fn _physical_interlock_is_part_of_the_boundary(_: BlobCompactionPhysicalInterlock) {}
