use crate::data::error::SpecError;
use crate::data::journal::MutationJournal;
use crate::data::snapshot::SpecState;
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation};
use crate::logic::validation::validate_spec_graph;

use super::SpecDraft;

impl SpecDraft {
    pub fn execute<M: SpecMutation>(
        &mut self,
        mutation: M,
    ) -> Result<MutationResult<M::Output>, SpecError> {
        self.ensure_open()?;
        let mut recorder = SpecLineageRecorder;
        mutation.execute(self, &mut recorder)
    }

    pub fn journal(&self) -> &MutationJournal {
        &self.journal
    }

    pub fn commit(mut self) -> Result<SpecState, SpecError> {
        self.ensure_open()?;
        let mut graph = self.base.graph().clone();
        for relation_id in &self.deleted_relations {
            if graph.relation(*relation_id).is_some() {
                graph.remove_relation(*relation_id)?;
            }
        }
        for node_id in &self.deleted_nodes {
            if graph.node(*node_id).is_some() {
                graph.remove_node(*node_id)?;
            }
        }
        for node in self.created_nodes.values() {
            if graph.node(node.id).is_some() {
                graph.replace_node(node.clone())?;
            } else {
                graph.insert_node(node.clone())?;
            }
        }
        for relation in self.created_relations.values() {
            graph.insert_relation(relation.clone())?;
        }
        validate_spec_graph(&graph)?;

        let mut payloads = self.base.payloads().clone();
        for record in &self.created_payloads {
            let inserted = payloads.insert(record.bytes.clone());
            debug_assert_eq!(
                inserted, record.key,
                "payload keys must replay deterministically"
            );
        }

        let mut naming_anchors = self.base.naming_anchors().to_vec();
        naming_anchors.extend(self.created_anchors.into_values());
        naming_anchors.sort_by_key(|anchor| anchor.id);

        let mut lineage_records = self.base.lineage_records().to_vec();
        lineage_records.extend(self.lineage_records);
        lineage_records.sort_by_key(|record| (record.node, record.creation_operation));

        let mut replay_records = self.base.replay_records().to_vec();
        replay_records.extend(self.replay_records);
        replay_records.sort_by_key(|record| (record.operation_id, record.operation_name.clone()));

        self.finished = true;
        Ok(SpecState::from_parts(
            self.base.model_namespace(),
            self.base.epoch() + 1,
            self.allocator.next_sequence(),
            self.next_operation_id,
            graph,
            payloads,
            naming_anchors,
            lineage_records,
            replay_records,
        ))
    }

    pub fn rollback(mut self) -> Result<SpecState, SpecError> {
        self.ensure_open()?;
        self.finished = true;
        Ok(self.base)
    }
}
