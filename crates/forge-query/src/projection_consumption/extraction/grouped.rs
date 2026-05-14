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
        projection.contract().grouping_aspect(),
        projection.members().iter().map(|member| {
            (
                member.row_identity().as_str(),
                member.identity_value().clone(),
                member.grouping_value().value().clone(),
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
                member.identity_value().clone(),
                member.lane().value().clone(),
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
