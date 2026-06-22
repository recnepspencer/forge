use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::intent_admission::certification_runtime;
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeReadmissionReceipt,
    ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeSeamKey,
};
use crate::projection_consumption::ProjectionConsumptionSource;
use crate::runtime::{
    ForgeQueryAspectMutationBuilder, ForgeQueryAspectTouch, ForgeQueryWriteReceipt,
};
use forge_foundational::facade::{AspectKey, AspectValue, ScalarAspectType};
use forge_relational::facade::grouped_truth::{
    encode_snapshot_aspect_read_value, materialize_relational_authoritative_row_set,
    project_relational_grouped_truth, GroupedProjectionContract,
};
use forge_runtime_bridge::facade::{
    materialize_bridge_grouped_truth_view_from_projection, materialize_bridge_row_set,
    AdmittedSourceRegistry, BridgeIdentityEvidence, BridgeSourceCapability,
    BridgeSourceCapabilitySet, BridgeTruthViewSelector, GroupedProjectionMemberSource,
    GroupedProjectionSource, RelationalBridgeRecordIdentityParts,
    RelationalBridgeSnapshotIdentityParts, SnapshotReadContract, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest, SourceDeclaration,
    SourceDeclarationIdentity, TruthBranchIdentity, TruthSnapshotIdentity,
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
        projection_source_evidence_identity(&source, "query-receipt-source"),
        projection_source_evidence_identity(&source, "query-receipt-retained"),
    )
}

pub(crate) fn representative_projection_relational_row() -> RepresentativeArtifacts {
    let entity_one = RelationalBridgeRecordIdentityParts::entity(1, 1, 1);
    let entity_two = RelationalBridgeRecordIdentityParts::entity(1, 2, 1);
    let packet = SnapshotReadPacket::new(vec![
        string_read(entity_one, "identity.id"),
        string_read(entity_one, "status.lane"),
        string_read(entity_two, "identity.id"),
        string_read(entity_two, "status.lane"),
    ]);
    let result = SnapshotReadPacketResult::new(
        TruthSnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(6, 1),
        ),
        vec![
            read_record(&packet, 0, AspectValue::String("task-1".into())),
            read_record(&packet, 1, AspectValue::String("todo".into())),
            read_record(&packet, 2, AspectValue::String("task-2".into())),
            read_record(&packet, 3, AspectValue::String("doing".into())),
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
        relational_grouped_projection_evidence(&source, grouped.digest().as_str(), "source"),
        relational_grouped_projection_evidence(&source, grouped.digest().as_str(), "retained"),
    )
}

pub(crate) fn representative_projection_bridge_row() -> RepresentativeArtifacts {
    let bridge = projection_bridge_runtime();
    let declaration = SourceDeclaration::new(
        SourceDeclarationIdentity::from_stable_name("source:lower-runtime-certification"),
        BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::from_relational_branch_id("main"),
            projection_snapshot_identity(),
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
    let entity_one = RelationalBridgeRecordIdentityParts::entity(1, 1, 1);
    let entity_two = RelationalBridgeRecordIdentityParts::entity(1, 2, 1);
    let packet = SnapshotReadPacket::new(vec![
        string_read(entity_one, "identity.id"),
        string_read(entity_one, "status"),
        string_read(entity_two, "identity.id"),
        string_read(entity_two, "status"),
    ]);
    let materialized = bridge
        .materialize_source_packet_batch(contract, vec![packet])
        .expect("bridge projection fixture should materialize source packets");
    let row_set = materialize_bridge_row_set(materialized.first())
        .expect("bridge projection fixture should materialize row set");
    let grouped = materialize_bridge_grouped_truth_view_from_projection(
        &row_set,
        &BridgeProjection {
            snapshot_identity: projection_snapshot_identity(),
            grouping_aspect: aspect_key("status"),
            identity_binding_aspect_key: aspect_key("identity.id"),
            grouping_binding_aspect_key: aspect_key("status"),
            members: vec![
                BridgeProjectionMember::new(entity_one, "task-1", "todo"),
                BridgeProjectionMember::new(entity_two, "task-2", "doing"),
            ],
        },
    )
    .expect("bridge projection fixture should group truth view");
    let source = ProjectionConsumptionSource::from_bridge_grouped_truth_view(&grouped);

    readmission_projection_row(
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromBridgeArtifacts,
        ForgeQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        "Projection source intake from bridge artifacts",
        bridge_grouped_projection_evidence(
            &source,
            &grouped.digest().bridge_admission_evidence(),
            "source",
        ),
        bridge_grouped_projection_evidence(
            &source,
            &grouped.digest().bridge_admission_evidence(),
            "retained",
        ),
    )
}

fn readmission_projection_row(
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    owner: ForgeQueryLowerRuntimeAuthorityOwner,
    capability_label: &str,
    subject_identity: ForgeQueryEvidenceIdentity,
    retained_evidence_source: ForgeQueryEvidenceIdentity,
) -> RepresentativeArtifacts {
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        seam_key,
        ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff,
        owner,
        capability_label,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSubjectIdentity::compose(
            "phase-six-projection-route-subject",
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("source"), &subject_identity)
        .seal(),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request.clone(),
        &retained_evidence_source,
    );
    let retained_evidence_identity =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "phase-six-projection-readmission",
            &retained_evidence_source,
        );
    let handoff = ForgeQueryLowerRuntimeReadmissionReceipt::new(
        eligibility.clone(),
        &retained_evidence_identity,
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
            task.aspect(
                ForgeQueryAspectTouch::from_authoring_path("title.value")
                    .expect("projection query title aspect should admit"),
                AspectValue::String("Projection fixture".into()),
            )
            .aspect(
                ForgeQueryAspectTouch::from_authoring_path("status.value")
                    .expect("projection query status aspect should admit"),
                AspectValue::String("todo".into()),
            )
        })
        .expect("projection query receipt write should execute")
}

#[derive(Debug)]
struct BridgeProjection {
    snapshot_identity: TruthSnapshotIdentity,
    grouping_aspect: AspectKey,
    identity_binding_aspect_key: AspectKey,
    grouping_binding_aspect_key: AspectKey,
    members: Vec<BridgeProjectionMember>,
}

#[derive(Debug)]
struct BridgeProjectionMember {
    row_identity: String,
    identity_value: AspectValue,
    grouping_value: AspectValue,
}

impl BridgeProjectionMember {
    fn new(
        row_identity: RelationalBridgeRecordIdentityParts,
        identity_value: &str,
        grouping_value: &str,
    ) -> Self {
        Self {
            row_identity: relational_row_identity(row_identity),
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

    fn grouping_aspect_key(&self) -> &AspectKey {
        &self.grouping_aspect
    }

    fn identity_binding_aspect_key(&self) -> &AspectKey {
        &self.identity_binding_aspect_key
    }

    fn grouping_binding_aspect_key(&self) -> &AspectKey {
        &self.grouping_binding_aspect_key
    }

    fn members(&self) -> &[Self::Member] {
        &self.members
    }
}

fn aspect_value(value: AspectValue) -> AspectValue {
    encode_snapshot_aspect_read_value(&value)
}

fn read_record(
    packet: &SnapshotReadPacket,
    index: usize,
    value: AspectValue,
) -> SnapshotReadRecord {
    SnapshotReadRecord::for_request(&packet.reads()[index], aspect_value(value))
}

fn string_read(
    entity_identity: RelationalBridgeRecordIdentityParts,
    aspect: &str,
) -> SnapshotReadRequest {
    SnapshotReadRequest::for_relational_record(
        entity_identity,
        SnapshotReadContract::scalar(aspect_key(aspect), ScalarAspectType::String),
    )
}

fn projection_source_evidence_identity(
    source: &ProjectionConsumptionSource,
    role: &'static str,
) -> ForgeQueryEvidenceIdentity {
    let mut builder =
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(ForgeQueryEvidenceTag::new("role"), role)
            .field_shape(
                ForgeQueryEvidenceTag::new("family"),
                source.family().as_str(),
            );
    if let Some(basis) = source.basis_digest() {
        builder = builder.field_value(ForgeQueryEvidenceTag::new("basis"), basis);
    }
    if let Some(identity) = source.source_identity_handle().evidence_identity() {
        builder = builder.field_evidence_identity(ForgeQueryEvidenceTag::new("source"), identity);
    } else {
        builder = builder.field_value(
            ForgeQueryEvidenceTag::new("source"),
            source.source_identity(),
        );
    }
    for reference in source.source_reference_identities() {
        builder = builder.field_value(
            ForgeQueryEvidenceTag::new(reference.label()),
            reference.identity(),
        );
    }
    builder.seal()
}

fn relational_grouped_projection_evidence(
    source: &ProjectionConsumptionSource,
    grouped_digest: &str,
    role: &'static str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("source"),
            &projection_source_evidence_identity(source, "relational-grouped"),
        )
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_value(ForgeQueryEvidenceTag::new("grouped"), grouped_digest)
        .seal()
}

fn bridge_grouped_projection_evidence(
    source: &ProjectionConsumptionSource,
    grouped_identity: &BridgeIdentityEvidence,
    role: &'static str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("source"),
            &projection_source_evidence_identity(source, "bridge-grouped"),
        )
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_bridge_retained_evidence_identity(
            ForgeQueryEvidenceTag::new("grouped"),
            grouped_identity,
        )
        .seal()
}

fn projection_snapshot_identity() -> TruthSnapshotIdentity {
    TruthSnapshotIdentity::from_relational_snapshot(RelationalBridgeSnapshotIdentityParts::new(
        6, 2,
    ))
}

fn relational_row_identity(entity_identity: RelationalBridgeRecordIdentityParts) -> String {
    format!(
        "entity:{}:{}:{}",
        entity_identity.partition_id(),
        entity_identity.local_slot(),
        entity_identity.generation()
    )
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
