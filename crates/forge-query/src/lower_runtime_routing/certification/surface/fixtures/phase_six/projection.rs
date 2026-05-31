use crate::identity::hash_parts;
use crate::intent_admission::certification_runtime;
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeReadmissionReceipt,
    ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeSeamKey,
};
use crate::projection_consumption::ProjectionConsumptionSource;
use crate::runtime::{ForgeQueryAspectMutationBuilder, ForgeQueryWriteReceipt};
use forge_foundational::facade::{AspectKey, AspectValue};
use forge_relational::facade::grouped_truth::{
    encode_snapshot_aspect_read_value, materialize_relational_authoritative_row_set,
    project_relational_grouped_truth, GroupedProjectionContract,
};
use forge_runtime_bridge::facade::{
    materialize_bridge_grouped_truth_view_from_projection, materialize_bridge_row_set,
    AdmittedSourceRegistry, BridgeSourceCapability, BridgeSourceCapabilitySet,
    BridgeTruthViewSelector, GroupedProjectionMemberSource, GroupedProjectionSource,
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest,
    SourceDeclaration, SourceDeclarationIdentity, TruthBranchIdentity, TruthSnapshotIdentity,
};

use super::super::{ForgeQueryLowerRuntimeRepresentativeEvidenceSource, RepresentativeArtifacts};
use super::projection_bridge_runtime::projection_bridge_runtime;

pub(crate) fn representative_projection_query_receipts_row() -> RepresentativeArtifacts {
    let receipt = certification_query_write_receipt();
    let source = ProjectionConsumptionSource::from_write_receipt(&receipt);

    readmission_projection_row(
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromQueryReceipts,
        ForgeQueryLowerRuntimeAuthorityOwner::Query,
        "Projection source intake from Query receipts",
        &[
            "projection_query_receipt_route_subject_v1".to_string(),
            format!("family:{}", source.family().as_str()),
            format!("basis:{}", source.basis_digest().unwrap_or("none")),
            format!("source:{}", source.source_identity()),
            format!(
                "references:{}",
                source
                    .source_reference_identities()
                    .iter()
                    .map(|identity| format!("{}:{}", identity.label(), identity.identity()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        ],
        source.source_identity().to_string(),
    )
}

pub(crate) fn representative_projection_relational_row() -> RepresentativeArtifacts {
    let packet = SnapshotReadPacket::new(vec![
        SnapshotReadRequest::for_coarse(
            "entity-1",
            forge_foundational::facade::AspectKey::new("identity.id")
                .expect("valid snapshot aspect key"),
        ),
        SnapshotReadRequest::for_coarse(
            "entity-1",
            forge_foundational::facade::AspectKey::new("status.lane")
                .expect("valid snapshot aspect key"),
        ),
        SnapshotReadRequest::for_coarse(
            "entity-2",
            forge_foundational::facade::AspectKey::new("identity.id")
                .expect("valid snapshot aspect key"),
        ),
        SnapshotReadRequest::for_coarse(
            "entity-2",
            forge_foundational::facade::AspectKey::new("status.lane")
                .expect("valid snapshot aspect key"),
        ),
    ]);
    let result = SnapshotReadPacketResult::new(
        TruthSnapshotIdentity::new("relational-snapshot-a"),
        vec![
            SnapshotReadRecord::new(
                "entity-1:identity.id",
                aspect_bytes(AspectValue::String("task-1".into())),
            ),
            SnapshotReadRecord::new(
                "entity-1:status.lane",
                aspect_bytes(AspectValue::String("todo".into())),
            ),
            SnapshotReadRecord::new(
                "entity-2:identity.id",
                aspect_bytes(AspectValue::String("task-2".into())),
            ),
            SnapshotReadRecord::new(
                "entity-2:status.lane",
                aspect_bytes(AspectValue::String("doing".into())),
            ),
        ],
    );
    let row_set = materialize_relational_authoritative_row_set(&packet, &result)
        .expect("relational projection fixture should materialize row set");
    let grouped = project_relational_grouped_truth(
        &row_set,
        grouped_projection_contract("status", "identity.id", "status.lane"),
    )
    .expect("relational projection fixture should group row set");
    let source = ProjectionConsumptionSource::from_relational_grouped_projection(&grouped);

    readmission_projection_row(
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromRelationalArtifacts,
        ForgeQueryLowerRuntimeAuthorityOwner::Relational,
        "Projection source intake from relational artifacts",
        &[
            "projection_relational_route_subject_v1".to_string(),
            format!("family:{}", source.family().as_str()),
            format!("basis:{}", source.basis_digest().unwrap_or("none")),
            format!("source:{}", source.source_identity()),
        ],
        grouped.digest().as_str().to_string(),
    )
}

pub(crate) fn representative_projection_bridge_row() -> RepresentativeArtifacts {
    let bridge = projection_bridge_runtime();
    let declaration = SourceDeclaration::new(
        SourceDeclarationIdentity::new("source:lower-runtime-certification"),
        BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("main"),
            TruthSnapshotIdentity::new("snapshot-a"),
        ),
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::BranchRead,
        ]),
    );
    let registry = AdmittedSourceRegistry::freeze(vec![declaration.clone()])
        .expect("bridge source declaration should admit");
    let contract = registry
        .contract_for_declaration(&declaration)
        .expect("bridge source contract should exist");
    let packet = SnapshotReadPacket::new(vec![
        SnapshotReadRequest::for_coarse(
            "entity-1",
            forge_foundational::facade::AspectKey::new("identity.id")
                .expect("valid snapshot aspect key"),
        ),
        SnapshotReadRequest::for_coarse(
            "entity-1",
            forge_foundational::facade::AspectKey::new("status")
                .expect("valid snapshot aspect key"),
        ),
        SnapshotReadRequest::for_coarse(
            "entity-2",
            forge_foundational::facade::AspectKey::new("identity.id")
                .expect("valid snapshot aspect key"),
        ),
        SnapshotReadRequest::for_coarse(
            "entity-2",
            forge_foundational::facade::AspectKey::new("status")
                .expect("valid snapshot aspect key"),
        ),
    ]);
    let materialized = bridge
        .materialize_source_packet_batch(contract, vec![packet])
        .expect("bridge projection fixture should materialize source packets");
    let row_set = materialize_bridge_row_set(materialized.first())
        .expect("bridge projection fixture should materialize row set");
    let grouped = materialize_bridge_grouped_truth_view_from_projection(
        &row_set,
        &BridgeProjection {
            snapshot_identity: TruthSnapshotIdentity::new("snapshot-a"),
            grouping_aspect: "status".to_string(),
            identity_binding_aspect_key: "identity.id".to_string(),
            grouping_binding_aspect_key: "status".to_string(),
            members: vec![
                BridgeProjectionMember::new("entity-1", "task-1", "todo"),
                BridgeProjectionMember::new("entity-2", "task-2", "doing"),
            ],
        },
    )
    .expect("bridge projection fixture should group truth view");
    let source = ProjectionConsumptionSource::from_bridge_grouped_truth_view(&grouped);

    readmission_projection_row(
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromBridgeArtifacts,
        ForgeQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        "Projection source intake from bridge artifacts",
        &[
            "projection_bridge_route_subject_v1".to_string(),
            format!("family:{}", source.family().as_str()),
            format!("basis:{}", source.basis_digest().unwrap_or("none")),
            format!("source:{}", source.source_identity()),
        ],
        grouped.digest().as_str().to_string(),
    )
}

fn readmission_projection_row(
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    owner: ForgeQueryLowerRuntimeAuthorityOwner,
    capability_label: &str,
    subject_parts: &[String],
    retained_evidence_digest: String,
) -> RepresentativeArtifacts {
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        seam_key,
        ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff,
        owner,
        capability_label,
        hash_parts(subject_parts),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
        request.clone(),
        retained_evidence_digest.clone(),
    );
    let handoff = ForgeQueryLowerRuntimeReadmissionReceipt::new(
        eligibility.clone(),
        retained_evidence_digest,
    );
    let boundary_receipt =
        ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_readmission_receipt(&handoff);
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(
        seam_key,
        &handoff,
        &boundary_receipt,
    );
    RepresentativeArtifacts {
        seam_key,
        request,
        eligibility,
        route_plan: None,
        boundary_receipt,
        envelope,
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

fn certification_query_write_receipt() -> ForgeQueryWriteReceipt {
    let mut workspace = certification_runtime()
        .workspace("lower-runtime-projection-query-receipts")
        .expect("projection query receipt workspace should build");
    workspace
        .insert("Task", |task: ForgeQueryAspectMutationBuilder| {
            task.aspect("title.value", "Projection fixture")
                .aspect("status.value", "todo")
        })
        .expect("projection query receipt write should execute")
}

#[derive(Debug)]
struct BridgeProjection {
    snapshot_identity: TruthSnapshotIdentity,
    grouping_aspect: String,
    identity_binding_aspect_key: String,
    grouping_binding_aspect_key: String,
    members: Vec<BridgeProjectionMember>,
}

#[derive(Debug)]
struct BridgeProjectionMember {
    row_identity: String,
    identity_value: AspectValue,
    grouping_value: AspectValue,
}

impl BridgeProjectionMember {
    fn new(row_identity: &str, identity_value: &str, grouping_value: &str) -> Self {
        Self {
            row_identity: row_identity.to_string(),
            identity_value: AspectValue::String(identity_value.into()),
            grouping_value: AspectValue::String(grouping_value.into()),
        }
    }
}

impl GroupedProjectionMemberSource for BridgeProjectionMember {
    fn row_identity(&self) -> &str {
        &self.row_identity
    }

    fn identity_value(&self) -> &AspectValue {
        &self.identity_value
    }

    fn grouping_value(&self) -> &AspectValue {
        &self.grouping_value
    }
}

impl GroupedProjectionSource for BridgeProjection {
    type Member = BridgeProjectionMember;

    fn basis_snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.snapshot_identity
    }

    fn grouping_aspect(&self) -> &str {
        &self.grouping_aspect
    }

    fn identity_binding_aspect_key(&self) -> &str {
        &self.identity_binding_aspect_key
    }

    fn grouping_binding_aspect_key(&self) -> &str {
        &self.grouping_binding_aspect_key
    }

    fn members(&self) -> &[Self::Member] {
        &self.members
    }
}

fn aspect_bytes(value: AspectValue) -> Vec<u8> {
    encode_snapshot_aspect_read_value(&value)
}

fn grouped_projection_contract(
    grouping_aspect: &str,
    identity_binding_aspect: &str,
    grouping_binding_aspect: &str,
) -> GroupedProjectionContract {
    GroupedProjectionContract::new(
        aspect_key(grouping_aspect),
        aspect_key(identity_binding_aspect),
        aspect_key(grouping_binding_aspect),
    )
}

fn aspect_key(label: &str) -> AspectKey {
    AspectKey::new(label).expect("fixture aspect key must be foundational")
}
