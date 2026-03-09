use serde::{Deserialize, Serialize};

use crate::data::graph::SpecGraph;
use crate::data::identity::DeterministicIdAllocator;
use crate::data::lineage::LineageRecord;
use crate::data::naming::NamingAnchor;
use crate::data::payload::{PayloadStore, ShellPayload, SpecShellKind};
use crate::data::replay::SpecReplayRecord;
use crate::data::{error::SpecError, identity::SpecNodeId, schema::SpecNodeKind};
use crate::logic::transaction::SpecDraft;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecState {
    model_namespace: u128,
    epoch: u64,
    next_sequence: u64,
    next_operation_id: u64,
    spec_hash: u128,
    graph: SpecGraph,
    payloads: PayloadStore,
    naming_anchors: Vec<NamingAnchor>,
    lineage_records: Vec<LineageRecord>,
    replay_records: Vec<SpecReplayRecord>,
}

impl SpecState {
    pub fn empty() -> Self {
        let mut state = Self {
            model_namespace: 1,
            epoch: 0,
            next_sequence: 0,
            next_operation_id: 0,
            spec_hash: 0,
            graph: SpecGraph::new(),
            payloads: PayloadStore::default(),
            naming_anchors: Vec::new(),
            lineage_records: Vec::new(),
            replay_records: Vec::new(),
        };
        state.spec_hash = state.compute_hash();
        state
    }

    pub fn with_namespace(model_namespace: u128) -> Self {
        let mut state = Self {
            model_namespace,
            ..Self::empty()
        };
        state.spec_hash = state.compute_hash();
        state
    }

    pub fn into_draft(self) -> SpecDraft {
        let allocator = DeterministicIdAllocator::new(self.model_namespace, self.next_sequence);
        SpecDraft::new(self, allocator)
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn spec_hash(&self) -> u128 {
        self.spec_hash
    }

    pub fn next_operation_id(&self) -> u64 {
        self.next_operation_id
    }

    pub fn graph(&self) -> &SpecGraph {
        &self.graph
    }

    pub fn payloads(&self) -> &PayloadStore {
        &self.payloads
    }

    pub fn shell_kind(&self, id: SpecNodeId) -> Result<SpecShellKind, SpecError> {
        let node = self
            .graph
            .node(id)
            .ok_or_else(|| SpecError::not_found(format!("node {} not found", id)))?;
        if node.kind != SpecNodeKind::Shell {
            return Err(SpecError::invalid(format!(
                "node {} is not a shell; found {:?}",
                id, node.kind
            )));
        }
        let payload_key = node
            .payload
            .ok_or_else(|| SpecError::not_found(format!("shell {} has no payload", id)))?;
        let payload = self
            .payloads
            .get(payload_key)
            .ok_or_else(|| SpecError::not_found(format!("payload {} not found", payload_key)))?;
        Ok(ShellPayload::decode(&payload.bytes)?.kind())
    }

    pub fn naming_anchors(&self) -> &[NamingAnchor] {
        &self.naming_anchors
    }

    pub fn lineage_records(&self) -> &[LineageRecord] {
        &self.lineage_records
    }

    pub fn replay_records(&self) -> &[SpecReplayRecord] {
        &self.replay_records
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("SpecState serialization must succeed")
    }

    pub(crate) fn from_parts(
        model_namespace: u128,
        epoch: u64,
        next_sequence: u64,
        next_operation_id: u64,
        mut graph: SpecGraph,
        payloads: PayloadStore,
        naming_anchors: Vec<NamingAnchor>,
        lineage_records: Vec<LineageRecord>,
        replay_records: Vec<SpecReplayRecord>,
    ) -> Self {
        graph.rebuild_indexes();
        let mut state = Self {
            model_namespace,
            epoch,
            next_sequence,
            next_operation_id,
            spec_hash: 0,
            graph,
            payloads,
            naming_anchors,
            lineage_records,
            replay_records,
        };
        state.spec_hash = state.compute_hash();
        state
    }

    fn compute_hash(&self) -> u128 {
        let mut hi = 0xcbf29ce484222325u64;
        let mut lo = 0x9e3779b97f4a7c15u64;

        fn feed_bytes(hi: &mut u64, lo: &mut u64, bytes: &[u8]) {
            for &byte in bytes {
                *hi ^= byte as u64;
                *hi = hi.wrapping_mul(0x100000001b3);
                *lo ^= (byte as u64).rotate_left(1);
                *lo = lo.wrapping_mul(0x100000001b3);
            }
        }

        feed_bytes(&mut hi, &mut lo, &self.model_namespace.to_be_bytes());
        feed_bytes(&mut hi, &mut lo, &self.epoch.to_be_bytes());
        for node in self.graph.iter_nodes() {
            feed_bytes(&mut hi, &mut lo, &node.id.raw().to_be_bytes());
            feed_bytes(&mut hi, &mut lo, &(node.kind as u16).to_be_bytes());
            feed_bytes(
                &mut hi,
                &mut lo,
                &node
                    .payload
                    .map(|payload| payload.raw())
                    .unwrap_or(0)
                    .to_be_bytes(),
            );
        }
        for relation in self.graph.iter_relations() {
            feed_bytes(&mut hi, &mut lo, &relation.id.raw().to_be_bytes());
            feed_bytes(&mut hi, &mut lo, &(relation.kind as u16).to_be_bytes());
            feed_bytes(&mut hi, &mut lo, &relation.source.raw().to_be_bytes());
            feed_bytes(&mut hi, &mut lo, &relation.target.raw().to_be_bytes());
            feed_bytes(&mut hi, &mut lo, &relation.ordinal.to_be_bytes());
        }
        for payload in self.payloads.records() {
            feed_bytes(&mut hi, &mut lo, &payload.key.raw().to_be_bytes());
            feed_bytes(&mut hi, &mut lo, &payload.bytes);
        }
        for anchor in &self.naming_anchors {
            feed_bytes(&mut hi, &mut lo, &anchor.id.raw().to_be_bytes());
            feed_bytes(&mut hi, &mut lo, &anchor.target.raw().to_be_bytes());
            feed_bytes(&mut hi, &mut lo, &(anchor.target_kind as u16).to_be_bytes());
            feed_bytes(&mut hi, &mut lo, anchor.semantic_role.as_bytes());
            feed_bytes(&mut hi, &mut lo, &anchor.ordinal.to_be_bytes());
        }

        ((hi as u128) << 64) | lo as u128
    }

    pub(crate) fn model_namespace(&self) -> u128 {
        self.model_namespace
    }
}
