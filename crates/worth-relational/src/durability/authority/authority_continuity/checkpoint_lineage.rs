use crate::durability::data::{DurabilityError, DurableCheckpoint, RecoveryFailureClass};
use crate::lineage::data::LineageCheckpointDigestBasis;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn validate_checkpoint_lineage_artifact(
    checkpoint: &DurableCheckpoint,
) -> Result<(), DurabilityError> {
    validate_published_lineage_artifacts(checkpoint)?;
    validate_checkpoint_lineage_nodes(checkpoint)?;
    let published_lineage_commit_count = checkpoint
        .envelopes
        .iter()
        .filter(|envelope| envelope.has_lineage_authority())
        .count();
    let canonical_published_event_ids = checkpoint
        .envelopes
        .iter()
        .flat_map(|envelope| {
            envelope
                .lineage_digest_basis()
                .canonical_event_ids()
                .iter()
                .copied()
        })
        .collect();
    let published_lineage_event_count = checkpoint
        .envelopes
        .iter()
        .map(|envelope| envelope.lineage_digest_basis().lineage_event_count())
        .sum();
    let published_lineage_decision_count = checkpoint
        .envelopes
        .iter()
        .map(|envelope| envelope.lineage_digest_basis().lineage_decision_count())
        .sum();
    let observed_basis = LineageCheckpointDigestBasis::new(
        published_lineage_commit_count,
        canonical_published_event_ids,
        published_lineage_event_count,
        published_lineage_decision_count,
    );
    if checkpoint.lineage.digest_basis() != &observed_basis {
        return Err(DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            "durable checkpoint lineage artifact basis drifted from canonical published lineage",
        ));
    }
    Ok(())
}

fn validate_published_lineage_artifacts(
    checkpoint: &DurableCheckpoint,
) -> Result<(), DurabilityError> {
    for envelope in &checkpoint.envelopes {
        let expected = crate::lineage::data::LineageFinalizationArtifact::new(
            envelope.branch_context.clone(),
            crate::lineage::data::FinalizedLineageEventBatch::new(
                envelope.lineage_events().to_vec(),
            ),
            crate::lineage::data::LineageDecisionLog::new(envelope.lineage_decision_log().to_vec()),
        )
        .publish();
        if &expected != envelope.published_lineage() {
            return Err(corrupt_lineage(format!(
                "checkpoint commit {} carries lineage events or decisions that disagree with their observed canonical basis",
                envelope.commit.commit_id.0
            )));
        }
    }
    Ok(())
}

fn validate_checkpoint_lineage_nodes(
    checkpoint: &DurableCheckpoint,
) -> Result<(), DurabilityError> {
    let counters = checkpoint.lineage.counters();
    if counters.node_count != checkpoint.lineage.nodes().len() {
        return Err(corrupt_lineage(
            "checkpoint lineage counters disagree with their observed artifacts".to_owned(),
        ));
    }
    let nodes = checkpoint
        .lineage
        .nodes()
        .iter()
        .map(|node| (node.lineage_id(), node.entity_id()))
        .collect::<BTreeMap<_, _>>();
    if nodes.len() != checkpoint.lineage.nodes().len() {
        return Err(corrupt_lineage(
            "checkpoint lineage nodes reuse a lineage identity".to_owned(),
        ));
    }

    let mut created_lineages = BTreeSet::new();
    for envelope in &checkpoint.envelopes {
        let created_entities = envelope
            .patch
            .authoritative_record_patches
            .iter()
            .filter_map(|patch| {
                (patch.structural_change
                    == crate::publication::patch::data::RecordStructuralChange::Created)
                    .then_some(&patch.target)
            })
            .filter_map(|target| match target {
                crate::transactions::data::RecordRef::Entity(entity_id) => Some(*entity_id),
                crate::transactions::data::RecordRef::Relation(_) => None,
            })
            .collect::<Vec<_>>();
        let create_events = envelope
            .lineage_events()
            .iter()
            .filter(|event| event.kind() == crate::lineage::data::LineageEventKind::Create)
            .collect::<Vec<_>>();
        if create_events.len() != created_entities.len() {
            return Err(corrupt_lineage(format!(
                "checkpoint commit {} does not bind one create lineage event to each created entity",
                envelope.commit.commit_id.0
            )));
        }
        for (event, expected_entity_id) in create_events.into_iter().zip(created_entities) {
            let [lineage_id] = event.targets() else {
                return Err(corrupt_lineage(format!(
                    "checkpoint create event {} must name exactly one lineage target",
                    event.event_id()
                )));
            };
            if !created_lineages.insert(*lineage_id) {
                return Err(corrupt_lineage(format!(
                    "checkpoint create lineage id {} is reused",
                    lineage_id.0
                )));
            }
            let Some(entity_id) = nodes.get(lineage_id) else {
                return Err(corrupt_lineage(format!(
                    "checkpoint create lineage id {} has no canonical node",
                    lineage_id.0
                )));
            };
            if entity_id != &expected_entity_id {
                return Err(corrupt_lineage(format!(
                    "checkpoint lineage node {} does not name its exact entity in commit {}",
                    lineage_id.0, envelope.commit.commit_id.0,
                )));
            }
        }
    }
    if nodes.keys().copied().collect::<BTreeSet<_>>() != created_lineages {
        return Err(corrupt_lineage(
            "checkpoint lineage nodes are not bijective with canonical create events".to_owned(),
        ));
    }
    Ok(())
}

fn corrupt_lineage(detail: String) -> DurabilityError {
    DurabilityError::new(RecoveryFailureClass::CorruptCheckpoint, detail)
}
