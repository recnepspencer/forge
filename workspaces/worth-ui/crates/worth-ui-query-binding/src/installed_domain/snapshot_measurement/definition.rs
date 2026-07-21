use worth_foundational::facade::{
    AspectContractRevision, AspectIdentity, AspectKey, AspectMask, ProjectionMask,
};
use worth_query::facade::domain;
use worth_query_decl::facade::{
    authoring::{
        AspectFieldSelector, AuthoredQueryBundleRequest, AuthoredResultShapeField,
        CollectionQueryBuilder, CollectionResultShapeBuilder, RootEntityKey,
    },
    binding::QueryBindingDescriptor,
    canonicalization::canonicalize_request,
};

use crate::{native_aspect_contracts, WorthUiDomainEntry};

use super::{
    WorthUiSnapshotMeasurement, WorthUiSnapshotMeasurementFamily, LOWERING_FAMILY, MEASUREMENT_ROOT,
};

pub(crate) fn snapshot_measurement_definition() -> domain::WorthQueryDomainOperationDefinition<
    WorthUiDomainEntry,
    WorthUiSnapshotMeasurement,
    WorthUiSnapshotMeasurementFamily,
> {
    domain::WorthQueryDomainOperationDefinition::new(
        domain::WorthQueryDomainOperationIdentity::new("snapshot-measurement", 1),
        semantic_closure(canonical_bundle("measurement.value")),
    )
}

fn semantic_closure(
    canonical_query: worth_query_decl::facade::canonicalization::CanonicalQueryBundle,
) -> domain::WorthQueryDomainOperationSemanticClosure {
    let identity = native_projection(
        "identity",
        native_aspect_contracts::IDENTITY_ASPECT_IDENTITY,
    );
    let measurement = native_projection(
        "measurement",
        native_aspect_contracts::MEASUREMENT_ASPECT_IDENTITY,
    );
    domain::WorthQueryDomainOperationSemanticClosure {
        parameters: domain::WorthQueryOperationParameterContract::NotRequired,
        native_projection: measurement.clone(),
        canonical_query,
        collection: domain::WorthQueryOperationCollectionContract::Collection {
            row_identity_field: "identity.id".into(),
            ordering_fields: vec!["identity.id".into()],
            continuation: domain::WorthQueryOperationContinuationPosture::NotRequired,
        },
        required_capabilities: vec![
            domain::WorthQueryOperationCapabilityRequirement::QueryRead,
            domain::WorthQueryOperationCapabilityRequirement::QueryComposition,
        ],
        required_domains: Vec::new(),
        workflow: domain::WorthQueryOperationWorkflowContract::NotRequired,
        conditional_nodes: Vec::new(),
        graph_reads: domain::WorthQueryOperationGraphReadContract::Declared {
            roles: vec![domain::WorthQueryOperationGraphReadRole {
                role: "measurements".into(),
                participation: domain::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
                access: domain::WorthQueryOperationGraphAccess::Project,
                semantic_reads: vec![identity, measurement],
            }],
        },
        touches: domain::WorthQueryOperationTouchContract::NotRequired,
        effects: domain::WorthQueryOperationEffectContract::NotRequired,
        invariants: domain::WorthQueryOperationInvariantContract::NotRequired,
        replay: domain::WorthQueryOperationReplayContract::ReExecutable,
        reversal: domain::WorthQueryOperationReversalContract::Irreversible,
        lineage: domain::WorthQueryOperationLineageContract::NotRequired,
        promotion: domain::WorthQueryOperationPromotionContract::NotRequired,
        publication: domain::WorthQueryOperationPublicationContract::DerivedProjection {
            projection_role: domain::WorthQueryOperationProjectionRole::new("measurements")
                .expect("static projection role must admit"),
        },
        projection_consumption:
            domain::WorthQueryOperationProjectionConsumptionContract::QueryReadAuthority,
        terminal: domain::WorthQueryOperationTerminalContract {
            result_states: vec![
                domain::WorthQueryOperationResultState::Ready,
                domain::WorthQueryOperationResultState::Partial,
            ],
            failure_classes: vec![domain::WorthQueryOperationFailureClass::Dependency],
        },
        cost: domain::WorthQueryOperationCostContract {
            lookup: domain::WorthQueryOperationCostClass::Constant,
            execution: domain::WorthQueryOperationCostClass::DeclaredWidth,
            result_width: domain::WorthQueryOperationCostClass::DeclaredWidth,
        },
        support: support_requirements(domain::WorthQuerySupportRequirement::Required),
        lowering: domain::WorthQueryOperationLoweringContract {
            family: LOWERING_FAMILY.into(),
            deterministic: true,
        },
    }
}

#[cfg(test)]
pub(crate) fn snapshot_measurement_definition_with_value_alias(
    alias: &str,
) -> domain::WorthQueryDomainOperationDefinition<
    WorthUiDomainEntry,
    WorthUiSnapshotMeasurement,
    WorthUiSnapshotMeasurementFamily,
> {
    domain::WorthQueryDomainOperationDefinition::new(
        domain::WorthQueryDomainOperationIdentity::new("snapshot-measurement", 1),
        semantic_closure(canonical_bundle(alias)),
    )
}

fn canonical_bundle(
    measurement_alias: &str,
) -> worth_query_decl::facade::canonicalization::CanonicalQueryBundle {
    let query = CollectionQueryBuilder::new(
        RootEntityKey::new(MEASUREMENT_ROOT).expect("static measurement root must admit"),
    )
    .project(selector("identity", "id"))
    .project(selector("measurement", "value"))
    .build()
    .expect("static measurement query must admit")
    .into_raw();
    let shape = CollectionResultShapeBuilder::new()
        .field(result_field("identity", "id", "identity.id"))
        .field(result_field("measurement", "value", measurement_alias))
        .build()
        .expect("static measurement result shape must admit")
        .into_raw();
    canonicalize_request(
        AuthoredQueryBundleRequest::for_ordinary_read(query, shape, QueryBindingDescriptor::new())
            .expect("static measurement bundle must admit"),
    )
    .expect("static measurement bundle must canonicalize")
}

fn selector(aspect: &str, field: &str) -> AspectFieldSelector {
    AspectFieldSelector::new(aspect, field).expect("static measurement selector must admit")
}

fn result_field(aspect: &str, field: &str, alias: &str) -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(aspect, field, alias)
        .expect("static measurement result field must admit")
}

fn native_projection(
    key: &'static str,
    identity: AspectIdentity,
) -> domain::WorthQueryOperationNativeProjectionContract {
    domain::WorthQueryOperationNativeProjectionContract {
        aspect_key: AspectKey::new(key).expect("static aspect key must admit"),
        aspect_identity: identity,
        contract_revision: AspectContractRevision(1),
        mask: AspectMask::<ProjectionMask>::whole_aspect(),
    }
}

fn support_requirements(
    projection_consumption: domain::WorthQuerySupportRequirement,
) -> domain::WorthQueryOperationSupportRequirements {
    domain::WorthQueryOperationSupportRequirements {
        live: domain::WorthQuerySupportRequirement::NotRequired,
        continuation: domain::WorthQuerySupportRequirement::NotRequired,
        async_result_state: domain::WorthQuerySupportRequirement::NotRequired,
        recovery: domain::WorthQuerySupportRequirement::NotRequired,
        inspection: domain::WorthQuerySupportRequirement::NotRequired,
        projection_consumption,
        dependency_impact: domain::WorthQuerySupportRequirement::NotRequired,
        sharing: domain::WorthQuerySupportRequirement::NotRequired,
        invalidation: domain::WorthQuerySupportRequirement::NotRequired,
        collection_delivery: domain::WorthQuerySupportRequirement::NotRequired,
        conditional_evaluation: domain::WorthQuerySupportRequirement::NotRequired,
        conditional_comparator: domain::WorthQuerySupportRequirement::NotRequired,
        conditional_trigger: domain::WorthQuerySupportRequirement::NotRequired,
        conditional_temporal_or_on_demand: domain::WorthQuerySupportRequirement::NotRequired,
    }
}
