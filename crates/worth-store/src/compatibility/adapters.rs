use super::admission::{
    CompatibilityAdapterCostClass, CompatibilityAdapterDigest, CompatibilityAdapterId,
    CompatibilityEdgeRegistry, CompatibilityRejection, CompatibilityRejectionKind,
    CompatibilityRelation, DeclaredCompatibilityAdapter, DeclaredCompatibilityEdge,
};
use super::manifests::{ArtifactFamilyId, ArtifactSemanticVersion};
use crate::authority::AuthoritativeExportBundle;
use crate::failure::{StoreError, StoreErrorKind};
use serde::Serialize;
use sha2::{Digest, Sha256};

const FIRST_SHIP_COMMIT_ADAPTER_ID: &str = "first_ship_commit_envelope_adapter";
const FIRST_SHIP_COMMIT_ADAPTER_DIGEST: &str = "first_ship_commit_envelope_adapter_digest_v1";

pub(crate) fn first_ship_authoritative_adapter_edge_registry() -> CompatibilityEdgeRegistry {
    CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
        crate::CompatibilityFamilyKind::CommitEnvelope.family_id(),
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        CompatibilityRelation::AdapterRequired,
    )
    .with_adapter(first_ship_commit_envelope_adapter())])
}

pub(crate) fn first_ship_commit_envelope_adapter() -> DeclaredCompatibilityAdapter {
    DeclaredCompatibilityAdapter::new(
        CompatibilityAdapterId::new(FIRST_SHIP_COMMIT_ADAPTER_ID),
        CompatibilityAdapterDigest::new(FIRST_SHIP_COMMIT_ADAPTER_DIGEST),
        CompatibilityAdapterCostClass::BoundedBatchLocal,
    )
}

pub(crate) fn execute_declared_adapter_parity(
    counters: &mut crate::CompatibilityAdmissionCounters,
    edge_registry: &CompatibilityEdgeRegistry,
    family_id: &ArtifactFamilyId,
    source_semantic_version: ArtifactSemanticVersion,
    target_semantic_version: ArtifactSemanticVersion,
    requested_adapter_id: &CompatibilityAdapterId,
    requested_adapter_digest: &CompatibilityAdapterDigest,
    control_lane_bytes: &[u8],
    adapted_lane_bytes: &[u8],
    input_record_count: u64,
    output_record_count: u64,
    allocation_scope_count: u64,
) -> Result<crate::CompatibilityAdapterParityWitness, CompatibilityRejection> {
    let Some(edge) = edge_registry.get(family_id, source_semantic_version, target_semantic_version)
    else {
        counters.record_adapter_parity_failure();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::AdapterParityFailure,
            family_id.clone(),
            "adapter parity execution requires a declared compatibility edge",
        ));
    };
    let Some(adapter) = edge.adapter() else {
        counters.record_adapter_parity_failure();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::AdapterParityFailure,
            family_id.clone(),
            "adapter parity execution requires a declared compatibility adapter",
        ));
    };
    if edge.relation() != CompatibilityRelation::AdapterRequired {
        counters.record_adapter_parity_failure();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::AdapterParityFailure,
            family_id.clone(),
            "adapter parity execution requires an adapter-required compatibility relation",
        ));
    }
    if adapter.adapter_id() != requested_adapter_id
        || adapter.adapter_digest() != requested_adapter_digest
    {
        counters.record_adapter_parity_failure();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::AdapterParityFailure,
            family_id.clone(),
            "requested adapter id or digest does not match the declared compatibility adapter",
        ));
    }

    counters.record_adapter_execution(
        input_record_count,
        output_record_count,
        allocation_scope_count,
    );
    let control_lane_digest = sha256_hex(control_lane_bytes);
    let adapted_lane_digest = sha256_hex(adapted_lane_bytes);
    if control_lane_digest != adapted_lane_digest {
        counters.record_adapter_parity_failure();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::AdapterParityFailure,
            family_id.clone(),
            "adapter execution did not preserve control-lane parity",
        ));
    }

    Ok(crate::CompatibilityAdapterParityWitness::new(
        adapter.adapter_id().clone(),
        adapter.adapter_digest().clone(),
        adapter.cost_class(),
    ))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct AdapterParityLane {
    bytes: Vec<u8>,
    input_record_count: u64,
    output_record_count: u64,
    allocation_scope_count: u64,
}

impl AdapterParityLane {
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(crate) fn input_record_count(&self) -> u64 {
        self.input_record_count
    }

    pub(crate) fn output_record_count(&self) -> u64 {
        self.output_record_count
    }

    pub(crate) fn allocation_scope_count(&self) -> u64 {
        self.allocation_scope_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct CommitEnvelopeParityRow {
    commit_id: u64,
    branch_id: String,
    commit_sequence: u64,
    parent_commit_ids: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct CommitEnvelopeAdaptedRowV2 {
    commit_ref: u64,
    branch_ref: String,
    sequence: u64,
    parent_chain: Vec<u64>,
}

pub(crate) fn first_ship_commit_envelope_control_lane(
    export: &AuthoritativeExportBundle,
) -> Result<AdapterParityLane, StoreError> {
    let rows = commit_envelope_control_rows(export);
    serialize_adapter_lane(
        &rows,
        adapter_input_record_count(export),
        rows.len() as u64,
        1,
        "commit-envelope control lane",
    )
}

pub(crate) fn first_ship_commit_envelope_adapted_lane(
    export: &AuthoritativeExportBundle,
) -> Result<AdapterParityLane, StoreError> {
    let adapted_rows = export
        .commit_envelopes
        .iter()
        .map(|record| CommitEnvelopeAdaptedRowV2 {
            commit_ref: record.envelope.commit.commit_id.0,
            branch_ref: record.envelope.branch_context.0.clone(),
            sequence: record.commit_sequence,
            parent_chain: commit_parent_ids(export, record.envelope.commit.commit_id.0),
        })
        .collect::<Vec<_>>();
    let normalized_rows = adapted_rows
        .into_iter()
        .map(|row| CommitEnvelopeParityRow {
            commit_id: row.commit_ref,
            branch_id: row.branch_ref,
            commit_sequence: row.sequence,
            parent_commit_ids: row.parent_chain,
        })
        .collect::<Vec<_>>();
    serialize_adapter_lane(
        &normalized_rows,
        adapter_input_record_count(export),
        normalized_rows.len() as u64,
        1,
        "commit-envelope adapted lane",
    )
}

fn serialize_adapter_lane<T: Serialize>(
    rows: &T,
    input_record_count: u64,
    output_record_count: u64,
    allocation_scope_count: u64,
    label: &str,
) -> Result<AdapterParityLane, StoreError> {
    let bytes = serde_json::to_vec(rows).map_err(|error| {
        StoreError::new(
            StoreErrorKind::CompatibilityAdapterParityFailure,
            format!("failed to serialize {label}: {error}"),
        )
    })?;
    Ok(AdapterParityLane {
        bytes,
        input_record_count,
        output_record_count,
        allocation_scope_count,
    })
}

fn commit_envelope_control_rows(
    export: &AuthoritativeExportBundle,
) -> Vec<CommitEnvelopeParityRow> {
    export
        .commit_envelopes
        .iter()
        .map(|record| CommitEnvelopeParityRow {
            commit_id: record.envelope.commit.commit_id.0,
            branch_id: record.envelope.branch_context.0.clone(),
            commit_sequence: record.commit_sequence,
            parent_commit_ids: commit_parent_ids(export, record.envelope.commit.commit_id.0),
        })
        .collect()
}

fn commit_parent_ids(export: &AuthoritativeExportBundle, commit_id: u64) -> Vec<u64> {
    let mut parent_ids = export
        .commit_parent_records
        .iter()
        .filter(|record| record.commit_id.0 == commit_id)
        .map(|record| (record.parent_position, record.parent_commit_id.0))
        .collect::<Vec<_>>();
    parent_ids.sort_by_key(|(position, _)| *position);
    parent_ids
        .into_iter()
        .map(|(_, parent_commit_id)| parent_commit_id)
        .collect()
}

fn adapter_input_record_count(export: &AuthoritativeExportBundle) -> u64 {
    (export.commit_envelopes.len() + export.commit_parent_records.len()) as u64
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
