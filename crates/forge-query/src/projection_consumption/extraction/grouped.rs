use forge_foundational::facade::{AspectValue, InternedString};
use forge_relational::facade::grouped_truth::RelationalGroupedProjectionArtifact;
use forge_runtime_bridge::facade::BridgeGroupedTruthViewArtifact;

use super::super::consumed::{
    ConsumedMembershipFact, ConsumedProjectionFactSet, ConsumedRelationEndpointFact,
    ConsumedViewLocalIdentityFact, ProjectionFactExtractionCounters,
};
use super::super::contracts::MaterializedProjectionContract;
use super::super::facts::ProjectionFactKind;
use super::super::source::ProjectionSourceFamily;
use crate::projection_consumption::ProjectionFactExtractionError;

pub(super) fn extract_relational_grouped_facts(
    contract: &MaterializedProjectionContract,
    projection: &RelationalGroupedProjectionArtifact,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
    extract_grouped_facts(
        contract,
        ProjectionSourceFamily::RelationalGroupedProjection,
        projection.digest().as_str(),
        projection.contract().grouping_aspect().as_str(),
        projection.members().iter().map(|member| {
            (
                member.row_identity().as_str(),
                aspect_value_to_json(member.identity_value()),
                aspect_value_to_json(member.grouping_value().value()),
            )
        }),
    )
}

pub(super) fn extract_bridge_grouped_facts(
    contract: &MaterializedProjectionContract,
    grouped_truth_view: &BridgeGroupedTruthViewArtifact,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
    extract_grouped_facts(
        contract,
        ProjectionSourceFamily::BridgeGroupedTruthView,
        grouped_truth_view.digest().as_str(),
        grouped_truth_view.contract().grouping_aspect(),
        grouped_truth_view.members().iter().map(|member| {
            (
                member.row_identity().as_str(),
                aspect_value_to_json(member.identity_value()),
                aspect_value_to_json(member.lane().value()),
            )
        }),
    )
}

fn extract_grouped_facts<'a, Members>(
    contract: &MaterializedProjectionContract,
    expected_family: ProjectionSourceFamily,
    source_identity: &str,
    grouping_aspect: &str,
    members: Members,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError>
where
    Members: Iterator<Item = (&'a str, serde_json::Value, serde_json::Value)>,
{
    super::ensure_contract_family(contract, expected_family)?;
    super::ensure_source_identity(contract.source_identity(), source_identity)?;

    let materialized_members = members.collect::<Vec<_>>();
    let extracts_view_local_identity = contract
        .fact_families()
        .iter()
        .any(|fact| fact.kind() == ProjectionFactKind::ViewLocalIdentity);
    let extracts_membership = contract
        .fact_families()
        .iter()
        .any(|fact| fact.kind() == ProjectionFactKind::Membership);
    let extracts_relation_endpoint = contract
        .fact_families()
        .iter()
        .any(|fact| fact.kind() == ProjectionFactKind::RelationEndpoint);
    let mut view_local_identities = Vec::new();
    let mut memberships = Vec::new();
    let mut relation_endpoints = Vec::new();

    for (row_identity, member_identity, grouping_value) in &materialized_members {
        for fact_family in contract.fact_families() {
            match fact_family.kind() {
                ProjectionFactKind::ViewLocalIdentity => {
                    view_local_identities.push(ConsumedViewLocalIdentityFact::new(
                        *row_identity,
                        *row_identity,
                    ));
                }
                ProjectionFactKind::Membership => {
                    memberships.push(ConsumedMembershipFact::new(
                        *row_identity,
                        member_identity.clone(),
                        grouping_aspect,
                        grouping_value.clone(),
                    ));
                }
                ProjectionFactKind::RelationEndpoint => {
                    relation_endpoints.push(ConsumedRelationEndpointFact::grouped(
                        *row_identity,
                        member_identity.clone(),
                        grouping_aspect,
                        grouping_value.clone(),
                    ));
                }
                ProjectionFactKind::EntityIdentity
                | ProjectionFactKind::TargetIdentity
                | ProjectionFactKind::SourceReference
                | ProjectionFactKind::EffectContinuity
                | ProjectionFactKind::DisplayField
                | ProjectionFactKind::DerivedScalarField => {}
            }
        }
    }

    let row_count = materialized_members.len();
    let row_identity_surface_count =
        usize::from(extracts_view_local_identity || extracts_relation_endpoint);
    let member_identity_surface_count =
        usize::from(extracts_membership || extracts_relation_endpoint);
    let grouping_value_surface_count =
        usize::from(extracts_membership || extracts_relation_endpoint);
    let row_width_per_row =
        row_identity_surface_count + member_identity_surface_count + grouping_value_surface_count;
    let source_row_width_consumed = row_count * row_width_per_row;
    let extracted_fact_count =
        view_local_identities.len() + memberships.len() + relation_endpoints.len();

    Ok(ConsumedProjectionFactSet::new(
        contract.declaration_digest(),
        contract.contract_digest(),
        contract.source_family(),
        contract.source_identity(),
        contract.support_posture().clone(),
        ProjectionFactExtractionCounters::new(
            contract.fact_families().len(),
            contract.fact_families().len(),
            extracted_fact_count,
            source_row_width_consumed,
            0,
        ),
        Vec::new(),
        view_local_identities,
        memberships,
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        relation_endpoints,
    ))
}

fn aspect_value_to_json(value: &AspectValue) -> serde_json::Value {
    match value {
        AspectValue::Null => serde_json::Value::Null,
        AspectValue::Bool(value) => serde_json::Value::Bool(*value),
        AspectValue::Int8(value) => serde_json::Value::from(*value),
        AspectValue::Int16(value) => serde_json::Value::from(*value),
        AspectValue::Int32(value) => serde_json::Value::from(*value),
        AspectValue::Int64(value) => serde_json::Value::from(*value),
        AspectValue::UInt8(value) => serde_json::Value::from(*value),
        AspectValue::UInt16(value) => serde_json::Value::from(*value),
        AspectValue::UInt32(value) => serde_json::Value::from(*value),
        AspectValue::UInt64(value) => serde_json::Value::from(*value),
        AspectValue::Float32(value) => float_value_to_json(
            f32::from_bits(value.bits()) as f64,
            "f32-bits",
            value.bits() as u64,
        ),
        AspectValue::Float64(value) => {
            float_value_to_json(f64::from_bits(value.bits()), "f64-bits", value.bits())
        }
        AspectValue::Decimal(value) => serde_json::Value::String(value.as_str().to_string()),
        AspectValue::BigInt(value) => serde_json::Value::String(value.as_str().to_string()),
        AspectValue::Rational(value) => serde_json::Value::String(format!(
            "{}/{}",
            value.numerator.as_str(),
            value.denominator.as_str()
        )),
        AspectValue::String(value) => serde_json::Value::String(interned_string_text(value)),
        AspectValue::Bytes(value) => serde_json::Value::String(format!("bytes-ref:{}", value.0)),
        AspectValue::Uuid(value) => serde_json::Value::String(format!("uuid:{}", hex_bytes(value))),
        AspectValue::Date(value) => {
            serde_json::Value::String(format!("date-days:{}", value.days_from_unix_epoch))
        }
        AspectValue::Time(value) => {
            serde_json::Value::String(format!("time-nanos:{}", value.nanos_since_midnight))
        }
        AspectValue::Timestamp(value) => serde_json::Value::String(format!(
            "timestamp-micros:{}",
            value.micros_since_unix_epoch
        )),
        AspectValue::TimestampTz(value) => serde_json::Value::String(format!(
            "timestamp-tz:{}:{}",
            value.utc_micros_since_unix_epoch, value.offset_minutes
        )),
        AspectValue::EntityRef(value) => serde_json::Value::String(format!(
            "entity-ref:{}:{}:{}",
            value.partition_id.0, value.local_slot.0, value.generation.0
        )),
        AspectValue::ContentRef(value) => {
            serde_json::Value::String(format!("content-ref:{}", value.0))
        }
    }
}

fn float_value_to_json(value: f64, label: &str, bits: u64) -> serde_json::Value {
    serde_json::Number::from_f64(value)
        .map(serde_json::Value::Number)
        .unwrap_or_else(|| serde_json::Value::String(format!("{label}:{bits}")))
}

fn hex_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn interned_string_text(value: &InternedString) -> String {
    match value {
        InternedString::Raw(text) => text.clone(),
        InternedString::Symbol(symbol) => format!("symbol:{}", symbol.0),
    }
}
