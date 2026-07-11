use forge_store_operations_vocabulary::{
    BackupExportCustodyReadiness, S10BackupExportCustodyHandoff,
};
use forge_store_physical_isolation::{ReadDuringCheckpointVerdict, StablePhysicalReadPlan};

use crate::reachability::receipt_construction::proof_set::{
    collect_orphan_candidates, collect_unique_reachable_chunks, exact_current_counters_for,
};
use crate::reachability::transitions::admit_edge::transition_admit_edge;
use crate::reachability::transitions::admit_hold::transition_admit_hold;
use crate::reachability::types::{BlobChunkReachabilityProofSet, BlobChunkReachabilityRegistry};
use crate::reachability::verification::authority_match::require_registry_bound_hold_authority;
use crate::{
    BlobChunkProofLeaf, BlobLifecycleDeclaration, BlobReachabilityDenial, BlobReachabilityEdge,
    BlobReachabilityProtectedHold, BlobRetentionHold, ScopedBlobChunk,
};

impl BlobChunkReachabilityRegistry {
    pub fn admit_edge(&mut self, edge: BlobReachabilityEdge) -> Result<(), BlobReachabilityDenial> {
        transition_admit_edge(self, edge)
    }

    pub fn admit_lifecycle_primary_reference(
        &mut self,
        declaration: &BlobLifecycleDeclaration,
        scoped_chunk: ScopedBlobChunk,
    ) -> Result<BlobChunkReachabilityProofSet, BlobReachabilityDenial> {
        let edge = BlobReachabilityEdge::primary_lifecycle_reference(declaration, scoped_chunk)?;
        self.admit_edge(edge)?;
        self.prove_reachable_chunks()
    }

    pub fn admit_lifecycle_multichunk_primary_references(
        &mut self,
        declaration: &BlobLifecycleDeclaration,
        ordered_leaves: &[BlobChunkProofLeaf],
    ) -> Result<BlobChunkReachabilityProofSet, BlobReachabilityDenial> {
        for leaf in ordered_leaves {
            let edge =
                BlobReachabilityEdge::primary_lifecycle_multichunk_reference(declaration, leaf)?;
            self.admit_edge(edge)?;
        }
        let reachable_chunks = collect_unique_reachable_chunks(self);
        let orphan_candidates = collect_orphan_candidates(self);
        let counters = exact_current_counters_for(self, &reachable_chunks);
        Ok(BlobChunkReachabilityProofSet::construct(
            crate::reachability::BlobReachabilityAuthorityKey::from_declaration(declaration),
            reachable_chunks,
            declaration.stored_chunk_digest().clone(),
            declaration.security_metadata(),
            self.edges()
                .iter()
                .map(|edge| edge.identity().clone())
                .collect(),
            self.holds()
                .iter()
                .map(|hold| hold.identity().clone())
                .collect(),
            orphan_candidates,
            counters,
        ))
    }

    pub fn admit_hold(
        &mut self,
        hold: BlobReachabilityProtectedHold,
    ) -> Result<(), BlobReachabilityDenial> {
        transition_admit_hold(self, hold)
    }

    pub fn admit_stable_read_plan_hold(
        &mut self,
        plan: &StablePhysicalReadPlan,
    ) -> Result<(), BlobReachabilityDenial> {
        let authority = require_registry_bound_hold_authority(self)?;
        let hold = BlobReachabilityProtectedHold::from_stable_read_plan(plan, authority);
        self.admit_hold(hold)
    }

    pub fn admit_checkpoint_hold(
        &mut self,
        verdict: &ReadDuringCheckpointVerdict,
    ) -> Result<(), BlobReachabilityDenial> {
        let authority = require_registry_bound_hold_authority(self)?;
        let hold = BlobReachabilityProtectedHold::from_checkpoint_verdict(verdict, authority);
        self.admit_hold(hold)
    }

    pub fn admit_export_hold(
        &mut self,
        readiness: &BackupExportCustodyReadiness,
    ) -> Result<(), BlobReachabilityDenial> {
        let authority = require_registry_bound_hold_authority(self)?;
        let hold = BlobReachabilityProtectedHold::from_export_readiness(readiness, authority)?;
        self.admit_hold(hold)
    }

    pub fn admit_backup_repair_backup_hold(
        &mut self,
        handoff: &S10BackupExportCustodyHandoff,
    ) -> Result<(), BlobReachabilityDenial> {
        let authority = require_registry_bound_hold_authority(self)?;
        let hold = BlobReachabilityProtectedHold::from_backup_repair_backup_handoff(handoff, authority)?;
        self.admit_hold(hold)
    }

    pub fn admit_retention_hold(
        &mut self,
        hold: &BlobRetentionHold,
    ) -> Result<(), BlobReachabilityDenial> {
        let authority = require_registry_bound_hold_authority(self)?;
        let hold = BlobReachabilityProtectedHold::from_retention_hold(hold, authority);
        self.admit_hold(hold)
    }
}
