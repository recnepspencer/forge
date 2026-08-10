use crate::collection::CollectionPlanningMode;
use crate::identity::{
    BindingFulfillmentDigest, CanonicalQueryDigest, CanonicalResultShapeDigest,
    CollectionPlanDigest, PlanDigest, ValidatedQueryDigest, ValidatedResultShapeDigest,
};
use crate::validation::ValidatedQueryBundle;

use super::route::{FallbackDisposition, PlannedExecutionRoute};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlannedQueryArtifact {
    validated_query_digest: ValidatedQueryDigest,
    canonical_query_digest: CanonicalQueryDigest,
    canonical_authority: crate::identity_authority::QueryCanonicalAuthority,
    route: PlannedExecutionRoute,
    fallback: FallbackDisposition,
    projection_count: usize,
    traversal_count: usize,
    predicate_count: usize,
    ordering_count: usize,
    policy_narrowing_digest: Option<String>,
    plan_digest: PlanDigest,
}

impl PlannedQueryArtifact {
    pub fn canonical_authority(&self) -> crate::identity_authority::QueryCanonicalAuthority {
        self.canonical_authority.clone()
    }

    pub fn validated_query_digest(&self) -> &ValidatedQueryDigest {
        &self.validated_query_digest
    }

    pub fn canonical_query_digest(&self) -> &CanonicalQueryDigest {
        &self.canonical_query_digest
    }

    pub fn route(&self) -> &PlannedExecutionRoute {
        &self.route
    }

    pub fn fallback(&self) -> &FallbackDisposition {
        &self.fallback
    }

    pub fn plan_digest(&self) -> &PlanDigest {
        &self.plan_digest
    }

    pub fn projection_count(&self) -> usize {
        self.projection_count
    }

    pub fn traversal_count(&self) -> usize {
        self.traversal_count
    }

    pub fn predicate_count(&self) -> usize {
        self.predicate_count
    }

    pub fn ordering_count(&self) -> usize {
        self.ordering_count
    }

    pub fn policy_narrowing_digest(&self) -> Option<&str> {
        self.policy_narrowing_digest.as_deref()
    }

    pub(crate) fn new(
        validated_query_digest: ValidatedQueryDigest,
        canonical_query_digest: CanonicalQueryDigest,
        canonical_authority: crate::identity_authority::QueryCanonicalAuthority,
        validated_result_shape_digest: &ValidatedResultShapeDigest,
        route: PlannedExecutionRoute,
        fallback: FallbackDisposition,
        projection_count: usize,
        traversal_count: usize,
        predicate_count: usize,
        ordering_count: usize,
        collection_digest: Option<&CollectionPlanDigest>,
        binding_digest: Option<&BindingFulfillmentDigest>,
        policy_narrowing_digest: Option<&str>,
    ) -> Self {
        let mut parts = vec![
            format!("validated_query:{}", validated_query_digest.as_str()),
            format!(
                "validated_result_shape:{}",
                validated_result_shape_digest.as_str()
            ),
            format!("route:{}", route.as_str()),
            format!("fallback:{}", fallback.as_str()),
            format!("projection_count:{projection_count}"),
            format!("traversal_count:{traversal_count}"),
            format!("predicate_count:{predicate_count}"),
            format!("ordering_count:{ordering_count}"),
        ];
        if let Some(collection_digest) = collection_digest {
            parts.push(format!("collection:{}", collection_digest.as_str()));
        }
        if let Some(binding_digest) = binding_digest {
            parts.push(format!("binding:{}", binding_digest.as_str()));
        }
        if let Some(policy_narrowing_digest) = policy_narrowing_digest {
            parts.push(format!("policy_narrowing:{policy_narrowing_digest}"));
        }

        Self {
            validated_query_digest,
            canonical_query_digest,
            canonical_authority,
            route,
            fallback,
            projection_count,
            traversal_count,
            predicate_count,
            ordering_count,
            policy_narrowing_digest: policy_narrowing_digest.map(str::to_string),
            plan_digest: PlanDigest::from_parts(&parts),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlannedResultShapeArtifact {
    validated_result_shape_digest: ValidatedResultShapeDigest,
    canonical_result_shape_digest: CanonicalResultShapeDigest,
    binding_count: usize,
}

impl PlannedResultShapeArtifact {
    pub fn validated_result_shape_digest(&self) -> &ValidatedResultShapeDigest {
        &self.validated_result_shape_digest
    }

    pub fn canonical_result_shape_digest(&self) -> &CanonicalResultShapeDigest {
        &self.canonical_result_shape_digest
    }

    pub fn binding_count(&self) -> usize {
        self.binding_count
    }

    pub(crate) fn new(
        validated_result_shape_digest: ValidatedResultShapeDigest,
        canonical_result_shape_digest: CanonicalResultShapeDigest,
        binding_count: usize,
    ) -> Self {
        Self {
            validated_result_shape_digest,
            canonical_result_shape_digest,
            binding_count,
        }
    }

    pub(in crate::planning) fn from_validated_bundle_for_collection_mode(
        bundle: &ValidatedQueryBundle,
        collection_mode: &CollectionPlanningMode,
    ) -> Self {
        if matches!(collection_mode, CollectionPlanningMode::CountRows) {
            let validated_parts = vec![
                format!(
                    "source_validated_result_shape:{}",
                    bundle.result_shape().digest().as_str()
                ),
                "result_family:count_aggregate".to_string(),
                "aggregate:count_rows".to_string(),
            ];
            let canonical_parts = vec![
                format!(
                    "source_canonical_result_shape:{}",
                    bundle
                        .result_shape()
                        .canonical_result_shape_digest()
                        .as_str()
                ),
                "result_family:count_aggregate".to_string(),
                "aggregate:count_rows".to_string(),
            ];

            return Self::new(
                ValidatedResultShapeDigest::from_parts(&validated_parts),
                CanonicalResultShapeDigest::from_parts(&canonical_parts),
                1,
            );
        }

        Self::new(
            bundle.result_shape().digest().clone(),
            bundle
                .result_shape()
                .canonical_result_shape_digest()
                .clone(),
            bundle.result_shape().bindings().len(),
        )
    }
}
