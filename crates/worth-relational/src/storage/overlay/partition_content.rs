use serde::Serialize;
use sha2::{Digest, Sha256};

use super::PartitionState;
use crate::symbols::data::{StringInterner, Symbol};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PartitionContentDigestError {
    UnresolvedContentSymbol(Symbol),
}

impl PartitionState {
    /// Canonical commitment to authoritative partition content.
    ///
    /// Capacity, cache buckets, diagnostics, and retention pins are excluded:
    /// they have independent lifecycles and cannot decide branch truth.
    pub(crate) fn authoritative_content_digest(
        &self,
        symbols: &StringInterner,
    ) -> Result<[u8; 32], PartitionContentDigestError> {
        let entity_aspect_versions =
            resolve_aspect_versions(&self.entity_arena.aspect_versions, symbols)?;
        let entity_extra = self
            .entity_arena
            .extra
            .iter()
            .map(|extra| {
                let structural_fingerprint = extra
                    .structural_fingerprint
                    .map(|fingerprint| {
                        symbols
                            .resolve(fingerprint.family)
                            .map(|family| (family, fingerprint.value))
                            .ok_or(PartitionContentDigestError::UnresolvedContentSymbol(
                                fingerprint.family,
                            ))
                    })
                    .transpose()?;
                Ok((
                    structural_fingerprint,
                    &extra.lineage_id,
                    &extra.authoritative_aspect_state,
                ))
            })
            .collect::<Result<Vec<_>, PartitionContentDigestError>>()?;
        let entity = encode(&(
            self.entity_arena.slots.slots(),
            &self.entity_arena.partition_ids,
            &self.entity_arena.generations,
            &self.entity_arena.lifecycle,
            &self.entity_arena.kind_ids,
            &self.entity_arena.metadata_history,
            &self.entity_arena.created_at,
            &self.entity_arena.retired_at,
            entity_extra,
            entity_aspect_versions,
            self.entity_arena.live_bitset.sparse_words(),
            self.entity_arena.reclaimable_bitset.sparse_words(),
        ));
        let relation_aspect_versions =
            resolve_aspect_versions(&self.relation_arena.aspect_versions, symbols)?;
        let relation = encode(&(
            self.relation_arena.slots.slots(),
            &self.relation_arena.partition_ids,
            &self.relation_arena.generations,
            &self.relation_arena.lifecycle,
            &self.relation_arena.kind_ids,
            &self.relation_arena.metadata_history,
            &self.relation_arena.created_at,
            &self.relation_arena.retired_at,
            &self.relation_arena.extra,
            relation_aspect_versions,
            self.relation_arena.live_bitset.sparse_words(),
            self.relation_arena.reclaimable_bitset.sparse_words(),
        ));
        let adjacency = self
            .adjacency
            .iter()
            .map(|(slot, set)| (*slot, set.as_slice()))
            .collect::<Vec<_>>();
        let reverse_adjacency = self
            .reverse_adjacency
            .iter()
            .map(|(slot, set)| (*slot, set.as_slice()))
            .collect::<Vec<_>>();
        let graph = encode(&(adjacency, reverse_adjacency));
        let mut digest = Sha256::new();
        digest.update(b"worth.relational.partition-content.v1\0");
        digest.update(self.partition_id.as_u32().to_be_bytes());
        digest.update(Sha256::digest(entity));
        digest.update(Sha256::digest(relation));
        digest.update(Sha256::digest(graph));
        Ok(digest.finalize().into())
    }
}

fn resolve_aspect_versions<'symbols>(
    versions: &[std::collections::BTreeMap<Symbol, u64>],
    symbols: &'symbols StringInterner,
) -> Result<Vec<Vec<(&'symbols str, u64)>>, PartitionContentDigestError> {
    versions
        .iter()
        .map(|slot| {
            let mut resolved = slot
                .iter()
                .map(|(symbol, version)| {
                    symbols.resolve(*symbol).map(|key| (key, *version)).ok_or(
                        PartitionContentDigestError::UnresolvedContentSymbol(*symbol),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            resolved.sort_unstable_by(|left, right| left.0.cmp(right.0));
            Ok(resolved)
        })
        .collect()
}

fn encode(value: &impl Serialize) -> Vec<u8> {
    rmp_serde::to_vec(value).expect("authoritative in-memory partition values are serializable")
}

#[cfg(test)]
mod tests {
    use crate::config::data::{AdjacencyBackend, AdjacencyPolicy};
    use crate::identity::data::{KindId, PartitionId, RelationId};
    use crate::storage::overlay::PartitionState;
    use crate::storage::partition::AdjacencySet;
    use crate::storage::substrate::{EntityArena, RelationArena};
    use crate::symbols::data::StringInterner;

    #[test]
    fn derived_adjacency_caches_cannot_change_truth_digest() {
        let policy = AdjacencyPolicy {
            backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
            small_degree_inline_capacity: 0,
        };
        let mut partition = PartitionState {
            partition_id: PartitionId(5),
            adjacency_policy: policy.clone(),
            relation_overlay_is_sparse: false,
            entity_arena: EntityArena::with_capacity(0),
            relation_arena: RelationArena::with_capacity(0),
            adjacency: vec![AdjacencySet::new(&policy)].into(),
            reverse_adjacency: Default::default(),
        };
        let symbols = StringInterner::default();
        let baseline_digest = partition.authoritative_content_digest(&symbols).unwrap();
        let baseline_inventory = partition.allocation_inventory();

        partition.adjacency[0]
            .index_historical_kind(KindId(9), RelationId::new(PartitionId(5), 4, 1));
        let perturbed_inventory = partition.allocation_inventory();

        assert_eq!(
            partition.authoritative_content_digest(&symbols).unwrap(),
            baseline_digest
        );
        assert_eq!(
            perturbed_inventory.authoritative_bytes,
            baseline_inventory.authoritative_bytes
        );
        assert_eq!(perturbed_inventory.allocator_bookkeeping_bytes, 0);
        assert!(perturbed_inventory.optional_cache_bytes > baseline_inventory.optional_cache_bytes);
    }
}
