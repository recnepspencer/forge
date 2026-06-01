use crate::authorized_projection::{
    AuthorizedProjectionArtifact, AuthorizedProjectionCounters, MaskedProjectionArtifact,
    PolicyFieldInfluenceSet,
};
use crate::identity::hash_parts;
use crate::projection_consumption::{
    declare_projection_consumption, evaluate_projection_consumption_eligibility,
    ProjectMaterializedFacts, ProjectionConsumptionAuthoringSurface,
    ProjectionConsumptionBindingContext, ProjectionConsumptionDeclaration,
    ProjectionConsumptionEligibility, ProjectionConsumptionSource,
};
use forge_foundational::facade::{AspectKey, AspectValue};
use forge_relational::facade::grouped_truth::{
    encode_snapshot_aspect_read_value, materialize_relational_authoritative_row_set,
    project_relational_grouped_truth, RelationalAuthoritativeRowSetArtifact,
    RelationalGroupedProjectionArtifact,
};
use forge_runtime_bridge::facade::{
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest,
    TruthSnapshotIdentity,
};

use super::super::consumed::ConsumedProjectionFactSet;
use super::super::contracts::MaterializedProjectionContract;
use super::super::envelope::SelfDescribingProjectionConsumptionEnvelope;
use super::super::receipt::ProjectionConsumptionReceipt;
use super::super::source::{
    ProjectionSourceCapabilityProfile, ProjectionSourceExecutionPosture, ProjectionSourceFamily,
};
use super::grouped_projection_contract::grouped_projection_contract;

const QUERY_DIGEST: &str = "query:projection_consumption_certification";
const RESULT_SHAPE_DIGEST: &str = "result-shape:projection_consumption_certification";
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectionConsumptionCertifiedLifecycle {
    declaration: ProjectionConsumptionDeclaration,
    contract: MaterializedProjectionContract,
    facts: ConsumedProjectionFactSet,
    receipt: ProjectionConsumptionReceipt,
    envelope: SelfDescribingProjectionConsumptionEnvelope,
}

impl ProjectionConsumptionCertifiedLifecycle {
    pub fn declaration(&self) -> &ProjectionConsumptionDeclaration {
        &self.declaration
    }

    pub fn contract(&self) -> &MaterializedProjectionContract {
        &self.contract
    }

    pub fn facts(&self) -> &ConsumedProjectionFactSet {
        &self.facts
    }

    pub fn receipt(&self) -> &ProjectionConsumptionReceipt {
        &self.receipt
    }

    pub fn envelope(&self) -> &SelfDescribingProjectionConsumptionEnvelope {
        &self.envelope
    }
}

pub fn control_row_set_lifecycle(row_count: usize) -> ProjectionConsumptionCertifiedLifecycle {
    let row_set = certification_row_set(row_count);
    let source = ProjectionConsumptionSource::from_relational_row_set(&row_set);
    let declaration = declare_projection_consumption(
        source,
        control_binding(&[
            "identity.id",
            "profile.display_name",
            "status.lane",
            "metrics.priority",
        ]),
        ProjectMaterializedFacts::declare()
            .entity_identities()
            .display_field("profile.display_name"),
    )
    .expect("control declaration should author cleanly");
    let contract = admitted_contract(&declaration);
    let facts = contract
        .extract_from_relational_row_set(&row_set)
        .expect("control extraction should succeed");
    let receipt = facts.issue_receipt();
    let envelope = receipt.projection_consumption_envelope();
    ProjectionConsumptionCertifiedLifecycle {
        declaration,
        contract,
        facts,
        receipt,
        envelope,
    }
}

pub fn grouped_worth_lifecycle(row_count: usize) -> ProjectionConsumptionCertifiedLifecycle {
    let grouped = certification_grouped_projection(row_count);
    let source = ProjectionConsumptionSource::from_relational_grouped_projection(&grouped);
    let declaration = declare_projection_consumption(
        source,
        control_binding(&["identity.id", "status.lane"]),
        ProjectMaterializedFacts::declare()
            .memberships()
            .relation_endpoints()
            .view_local_identities(),
    )
    .expect("grouped declaration should author cleanly");
    let contract = admitted_contract(&declaration);
    let facts = contract
        .extract_from_relational_grouped_projection(&grouped)
        .expect("grouped extraction should succeed");
    let receipt = facts.issue_receipt();
    let envelope = receipt.projection_consumption_envelope();
    ProjectionConsumptionCertifiedLifecycle {
        declaration,
        contract,
        facts,
        receipt,
        envelope,
    }
}

pub fn parity_row_set_lifecycle(row_count: usize) -> ProjectionConsumptionCertifiedLifecycle {
    let row_set = certification_row_set(row_count);
    let declaration = ProjectMaterializedFacts::declare()
        .entity_identities()
        .display_field("profile.display_name")
        .source(
            ProjectionConsumptionAuthoringSurface::from_relational_row_set(
                &row_set,
                RESULT_SHAPE_DIGEST,
                &control_authorized_projection(&[
                    "identity.id",
                    "profile.display_name",
                    "status.lane",
                    "metrics.priority",
                ]),
            ),
        )
        .build()
        .expect("parity declaration should author cleanly");
    let contract = admitted_contract(&declaration);
    let facts = contract
        .extract_from_relational_row_set(&row_set)
        .expect("parity extraction should succeed");
    let receipt = facts.issue_receipt();
    let envelope = receipt.projection_consumption_envelope();
    ProjectionConsumptionCertifiedLifecycle {
        declaration,
        contract,
        facts,
        receipt,
        envelope,
    }
}

pub fn denied_masked_field_failure_digest() -> String {
    let row_set = certification_row_set(2);
    let source = ProjectionConsumptionSource::from_relational_row_set(&row_set);
    let declaration = declare_projection_consumption(
        source,
        control_binding(&["identity.id"]),
        ProjectMaterializedFacts::declare()
            .entity_identities()
            .display_field("profile.display_name"),
    )
    .expect("denial declaration should author cleanly");
    match evaluate_projection_consumption_eligibility(&declaration) {
        ProjectionConsumptionEligibility::Denied(denied) => denied.failure_digest().to_string(),
        other => panic!("expected denied certification failure, got {other:?}"),
    }
}

pub fn source_mismatch_failure_digest() -> String {
    let row_set = certification_row_set(2);
    let source = ProjectionConsumptionSource::from_relational_row_set(&row_set);
    let declaration = declare_projection_consumption(
        source,
        control_binding(&["identity.id", "status.lane"]),
        ProjectMaterializedFacts::declare().source_references(),
    )
    .expect("source mismatch declaration should author cleanly");
    match evaluate_projection_consumption_eligibility(&declaration) {
        ProjectionConsumptionEligibility::SourceMismatch(mismatch) => {
            mismatch.failure_digest().to_string()
        }
        other => panic!("expected source mismatch certification failure, got {other:?}"),
    }
}

pub fn source_digest(contract: &MaterializedProjectionContract) -> String {
    hash_parts(&[
        "projection_consumption_certified_source_v1".to_string(),
        format!("family:{}", contract.source_family().as_str()),
        format!("identity:{}", contract.source_identity()),
        format!(
            "source_references:{}",
            contract
                .source_reference_identities()
                .iter()
                .map(|identity| format!("{}:{}", identity.label(), identity.identity()))
                .collect::<Vec<_>>()
                .join(",")
        ),
    ])
}

pub fn source_receipt_digest(contract: &MaterializedProjectionContract) -> String {
    hash_parts(&[
        "projection_consumption_certified_source_receipt_v1".to_string(),
        source_digest(contract),
        format!(
            "query:{}",
            contract
                .query_digest()
                .unwrap_or("no-query-owned-source-receipt")
        ),
        format!(
            "basis:{}",
            contract
                .basis_digest()
                .unwrap_or("no-query-owned-source-basis-receipt")
        ),
        format!(
            "result:{}",
            contract
                .result_digest()
                .unwrap_or("no-query-owned-source-result-receipt")
        ),
    ])
}

fn admitted_contract(
    declaration: &ProjectionConsumptionDeclaration,
) -> MaterializedProjectionContract {
    match evaluate_projection_consumption_eligibility(declaration) {
        ProjectionConsumptionEligibility::Admitted(admitted) => admitted.bind_contract(),
        ProjectionConsumptionEligibility::AdmittedWithWarnings(admitted, _) => {
            admitted.bind_contract()
        }
        other => panic!("expected admitted certification lane, got {other:?}"),
    }
}

fn control_binding(visible_fields: &[&str]) -> ProjectionConsumptionBindingContext {
    ProjectionConsumptionBindingContext::from_result_shape_digest(
        RESULT_SHAPE_DIGEST,
        &control_authorized_projection(visible_fields),
    )
}

fn control_authorized_projection(visible_fields: &[&str]) -> AuthorizedProjectionArtifact {
    AuthorizedProjectionArtifact::new(
        QUERY_DIGEST,
        RESULT_SHAPE_DIGEST,
        "policy:projection-consumption-certification",
        "tenant-schema:projection-consumption-certification",
        visible_fields
            .iter()
            .map(|field| field.to_string())
            .collect(),
        MaskedProjectionArtifact::new(
            vec!["masked-field:test".to_string()],
            vec!["masked-family:test".to_string()],
        ),
        "narrowed-result-shape:projection-consumption-certification".to_string(),
        PolicyFieldInfluenceSet::new(&["policy-influence:test".to_string()], 1),
        AuthorizedProjectionCounters::default(),
    )
}

pub fn certification_grouped_projection(row_count: usize) -> RelationalGroupedProjectionArtifact {
    project_relational_grouped_truth(
        &certification_row_set(row_count),
        grouped_projection_contract("status", "identity.id", "status.lane"),
    )
    .expect("grouped projection certification fixture")
}

pub fn certification_row_set(row_count: usize) -> RelationalAuthoritativeRowSetArtifact {
    let mut reads = Vec::new();
    let mut records = Vec::new();
    for index in 0..row_count {
        let entity = format!("entity-{}", index + 1);
        let task = format!("task-{}", index + 1);
        let lane = if index % 2 == 0 { "todo" } else { "doing" };
        let name = format!("Task {}", index + 1);
        reads.push(SnapshotReadRequest::for_coarse(
            entity.as_str(),
            snapshot_aspect_key("identity.id"),
        ));
        reads.push(SnapshotReadRequest::for_coarse(
            entity.as_str(),
            snapshot_aspect_key("status.lane"),
        ));
        reads.push(SnapshotReadRequest::for_coarse(
            entity.as_str(),
            snapshot_aspect_key("profile.display_name"),
        ));
        reads.push(SnapshotReadRequest::for_coarse(
            entity.as_str(),
            snapshot_aspect_key("metrics.priority"),
        ));
        records.push(SnapshotReadRecord::new(
            format!("{entity}:identity.id"),
            aspect_bytes(AspectValue::String(task.into())),
        ));
        records.push(SnapshotReadRecord::new(
            format!("{entity}:status.lane"),
            aspect_bytes(AspectValue::String(lane.into())),
        ));
        records.push(SnapshotReadRecord::new(
            format!("{entity}:profile.display_name"),
            aspect_bytes(AspectValue::String(name.into())),
        ));
        records.push(SnapshotReadRecord::new(
            format!("{entity}:metrics.priority"),
            aspect_bytes(AspectValue::UInt64((index + 1) as u64)),
        ));
    }
    materialize_relational_authoritative_row_set(
        &SnapshotReadPacket::new(reads),
        &SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new(format!("snapshot-certification-{row_count}")),
            records,
        ),
    )
    .expect("row set certification fixture")
}

fn snapshot_aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("valid certification snapshot aspect key")
}

fn aspect_bytes(value: AspectValue) -> Vec<u8> {
    encode_snapshot_aspect_read_value(&value)
}

pub(crate) fn intent_admission_admitted_projection_declaration() -> ProjectionConsumptionDeclaration
{
    declare_projection_consumption(
        ProjectionConsumptionSource::intent_admission_certification(
            ProjectionSourceFamily::QueryReadReceipt,
            ProjectionSourceCapabilityProfile::QueryReadReceipt {
                execution_posture: ProjectionSourceExecutionPosture::Current,
            },
            Some("query-digest".to_string()),
            Some("basis-digest".to_string()),
            Some("result-digest".to_string()),
            Some("shape-digest".to_string()),
            "query-read:certification-admitted",
            Vec::new(),
        ),
        intent_admission_projection_binding("query-read:certification-admitted"),
        ProjectMaterializedFacts::declare().display_field("field.visible"),
    )
    .expect("intent-admission admitted projection declaration should build")
}

pub(crate) fn intent_admission_warning_projection_declaration() -> ProjectionConsumptionDeclaration
{
    declare_projection_consumption(
        ProjectionConsumptionSource::intent_admission_certification(
            ProjectionSourceFamily::QueryContextExecution,
            ProjectionSourceCapabilityProfile::QueryContextExecution {
                execution_posture: ProjectionSourceExecutionPosture::Current,
            },
            Some("query-digest".to_string()),
            Some("basis-digest".to_string()),
            Some("result-digest".to_string()),
            Some("shape-digest".to_string()),
            "query-context:certification-warning",
            Vec::new(),
        ),
        intent_admission_projection_binding("query-context:certification-warning"),
        ProjectMaterializedFacts::declare().display_field("field.visible"),
    )
    .expect("intent-admission warning projection declaration should build")
}

fn intent_admission_projection_binding(
    source_identity: &str,
) -> ProjectionConsumptionBindingContext {
    ProjectionConsumptionBindingContext::intent_admission_certification_binding(
        "shape-digest",
        "query-digest",
        "shape-digest",
        source_identity,
        "narrowed-shape-digest",
        "policy-digest",
        "tenant-schema-digest",
        vec!["field.visible".to_string()],
    )
}
