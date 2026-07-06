use crate::{
    BlobChunkDedupeReferenceRelease, BlobChunkIdentity, BlobChunkReachabilityRegistry,
    BlobReachabilityDenial, BlobReachabilityEdge, BlobReachabilityEdgeKind,
    BlobReachabilityEdgeRelease, BlobReachabilityReclaimDecision, BlobReachabilityReclaimRelease,
    BlobRetentionHold,
};

impl BlobChunkReachabilityRegistry {
    pub fn release_edge(
        &mut self,
        edge: &BlobReachabilityEdge,
    ) -> Result<BlobReachabilityEdgeRelease, BlobReachabilityDenial> {
        let Some(position) = self
            .edges
            .iter()
            .position(|candidate| candidate.identity() == edge.identity())
        else {
            return Err(BlobReachabilityDenial::MissingReclaimReleaseEvidence {
                counters: self.counters.record_reclaim_denial(),
            });
        };
        let removed = self.edges.remove(position);
        let release = BlobReachabilityEdgeRelease::from_edge(&removed);
        self.released_edges.push(release.clone());
        self.sort_released_edges();
        Ok(release)
    }

    pub(crate) fn apply_registry_owned_dedupe_reference_release(
        &mut self,
        release: &BlobChunkDedupeReferenceRelease,
    ) {
        let mut removed = Vec::new();
        self.edges.retain(|edge| {
            let remove = edge.kind() == BlobReachabilityEdgeKind::DedupeSharedReference
                && edge.security_metadata() == release.security_metadata()
                && edge
                    .dedupe_reference_identity()
                    .is_some_and(|identity| release.contains_reference_identity(identity));
            if remove {
                removed.push(BlobReachabilityEdgeRelease::from_edge(edge));
            }
            !remove
        });
        self.released_edges.extend(removed);
        self.sort_released_edges();
    }

    pub fn reclaim_decision_for(
        &self,
        identity: &BlobChunkIdentity,
    ) -> BlobReachabilityReclaimDecision {
        if self
            .edges
            .iter()
            .any(|edge| edge.chunk_identity() == identity)
            || !self.holds.is_empty()
        {
            return BlobReachabilityReclaimDecision::ReclaimDenied(
                BlobReachabilityDenial::ReclaimBlockedByReferenceEdge {
                    counters: self.counters.record_reclaim_denial(),
                },
            );
        }
        let released_edges: Vec<_> = self
            .released_edges
            .iter()
            .filter(|release| release.chunk_identity() == identity)
            .cloned()
            .collect();
        if released_edges.is_empty() {
            return BlobReachabilityReclaimDecision::ReclaimDenied(
                BlobReachabilityDenial::MissingReclaimReleaseEvidence {
                    counters: self.counters.record_reclaim_denial(),
                },
            );
        }
        BlobReachabilityReclaimDecision::ReclaimPermitted(
            BlobReachabilityReclaimRelease::from_released_edges(
                identity.clone(),
                released_edges,
                self.counters,
            ),
        )
    }

    pub(crate) fn first_retention_hold_for_reclaim(&self) -> Option<BlobRetentionHold> {
        self.holds.first().map(|hold| {
            BlobRetentionHold::from_reachability_hold_kind(hold.kind(), hold.identity().as_str())
        })
    }

    fn sort_released_edges(&mut self) {
        self.released_edges.sort_by(|left, right| {
            left.edge_identity()
                .as_str()
                .cmp(right.edge_identity().as_str())
        });
    }
}
