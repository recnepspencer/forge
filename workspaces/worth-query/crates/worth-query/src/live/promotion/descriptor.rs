use super::super::refresh::RefreshAdmissionMatrix;
use super::super::relevance::QueryRelevanceContract;
use super::LiveQueryFamily;
use crate::collection::CollectionPlanBundle;
use crate::identity::{CollectionPlanDigest, PlanDigest, ValidatedQueryDigest};
use crate::live_performance::{IncrementalPatchEligibility, LivePerformanceReport};
use crate::validation::ValidatedQueryBundle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LivePromotionDescriptor {
    family: LiveQueryFamily,
    query_digest: ValidatedQueryDigest,
    plan_digest: PlanDigest,
    collection_digest: Option<CollectionPlanDigest>,
    relevance_contract: QueryRelevanceContract,
    refresh_admission_matrix: RefreshAdmissionMatrix,
    incremental_eligibility: IncrementalPatchEligibility,
    performance_report: LivePerformanceReport,
}

impl LivePromotionDescriptor {
    pub fn family(&self) -> &LiveQueryFamily {
        &self.family
    }

    pub fn query_digest(&self) -> &ValidatedQueryDigest {
        &self.query_digest
    }

    pub fn plan_digest(&self) -> &PlanDigest {
        &self.plan_digest
    }

    pub fn collection_digest(&self) -> Option<&CollectionPlanDigest> {
        self.collection_digest.as_ref()
    }

    pub fn relevance_contract(&self) -> &QueryRelevanceContract {
        &self.relevance_contract
    }

    pub fn refresh_admission_matrix(&self) -> &RefreshAdmissionMatrix {
        &self.refresh_admission_matrix
    }

    pub fn incremental_eligibility(&self) -> &IncrementalPatchEligibility {
        &self.incremental_eligibility
    }

    pub fn performance_report(&self) -> &LivePerformanceReport {
        &self.performance_report
    }

    pub(crate) fn for_plan(
        bundle: &ValidatedQueryBundle,
        plan_digest: PlanDigest,
        collection: Option<&CollectionPlanBundle>,
    ) -> Self {
        match collection {
            None => Self {
                family: LiveQueryFamily::Detail,
                query_digest: bundle.query().digest().clone(),
                plan_digest,
                collection_digest: None,
                relevance_contract: QueryRelevanceContract::for_detail(bundle),
                refresh_admission_matrix: RefreshAdmissionMatrix::detail_family(),
                incremental_eligibility: IncrementalPatchEligibility::incremental(
                    "detail live family is admitted for milestone 5",
                ),
                performance_report: LivePerformanceReport::verified_detail_family(),
            },
            Some(collection) if collection.traversal_bound().edge_classes().is_empty() => Self {
                family: LiveQueryFamily::OrderedCollection,
                query_digest: bundle.query().digest().clone(),
                plan_digest,
                collection_digest: Some(collection.digest().clone()),
                relevance_contract: QueryRelevanceContract::for_ordered_collection(
                    bundle, collection,
                ),
                refresh_admission_matrix: RefreshAdmissionMatrix::ordered_collection_family(),
                incremental_eligibility: IncrementalPatchEligibility::incremental(
                    "ordered collection live family is admitted for milestone 5",
                ),
                performance_report: LivePerformanceReport::verified_ordered_collection_family(),
            },
            Some(collection) => Self {
                family: LiveQueryFamily::BoundedMaterialization,
                query_digest: bundle.query().digest().clone(),
                plan_digest,
                collection_digest: Some(collection.digest().clone()),
                relevance_contract: QueryRelevanceContract::for_bounded_materialization(
                    bundle, collection,
                ),
                refresh_admission_matrix: RefreshAdmissionMatrix::bounded_materialization_family(),
                incremental_eligibility: IncrementalPatchEligibility::incremental(
                    "bounded materialization live family is admitted for milestone 5",
                ),
                performance_report: LivePerformanceReport::debt_bounded_materialization_family(),
            },
        }
    }
}
