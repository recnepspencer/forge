use worth_foundational::facade::{AspectMask, ProjectionMask};
use worth_query::facade::domain;
use worth_query_decl::facade::{
    authoring::{
        AspectFieldSelector, AuthoredQueryBundleRequest, AuthoredResultShapeField,
        CollectionQueryBuilder, CollectionResultShapeBuilder, OrderingSelector, RootEntityKey,
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
    let identity = native_projection("identity");
    let measurement = native_projection("measurement");
    domain::WorthQueryDomainOperationSemanticClosure {
        parameters: domain::WorthQueryOperationParameterContract::NotRequired,
        native_projection: measurement.clone(),
        canonical_query,
        collection: domain::WorthQueryOperationCollectionContract::Collection {
            row_identity_field: domain::WorthQueryOperationCollectionField::from_dotted(
                "identity.id",
            )
            .expect("the static identity collection field must admit"),
            ordering_fields: vec![domain::WorthQueryOperationCollectionField::from_dotted(
                "identity.id",
            )
            .expect("the static identity ordering field must admit")],
            grouping: domain::WorthQueryOperationGroupingContract::Ungrouped,
            window: domain::WorthQueryOperationWindowPolicy::ContinuationBounded,
            continuation: domain::WorthQueryOperationContinuationPosture::LiveCursor,
        },
        required_capabilities: vec![
            domain::WorthQueryOperationCapabilityRequirement::QueryRead,
            domain::WorthQueryOperationCapabilityRequirement::QueryComposition,
        ],
        required_domains: Vec::new(),
        workflow: domain::WorthQueryOperationWorkflowContract::NotRequired,
        evidence: domain::WorthQueryDomainEvidenceContract::not_required(),
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
        resources:
            crate::installed_domain::execution_resources::operation_execution_resource_contract(),
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
    .order_by(
        OrderingSelector::ascending("identity", "id")
            .expect("static measurement ordering must admit"),
    )
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

fn native_projection(key: &'static str) -> domain::WorthQueryOperationNativeProjectionContract {
    domain::WorthQueryOperationNativeProjectionContract::new(
        native_aspect_contracts::worth_ui_native_aspect_contract(key)
            .expect("the requested installed Worth UI aspect must exist"),
        AspectMask::<ProjectionMask>::whole_aspect(),
    )
    .expect("a whole installed Worth UI aspect mask must be admissible")
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
