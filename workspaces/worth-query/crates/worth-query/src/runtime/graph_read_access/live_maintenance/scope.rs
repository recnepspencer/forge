use crate::identity::hash_parts;
#[cfg(test)]
use crate::runtime::WorthQueryAdmittedGraphReadAccessPlan;
use crate::subscription::QuerySubscriptionFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLiveGraphReadMutationDeltaScope {
    affected_requirement_row_count: usize,
    delta_scope_digest: String,
}

impl WorthQueryLiveGraphReadMutationDeltaScope {
    pub fn affected_requirement_row_count(&self) -> usize {
        self.affected_requirement_row_count
    }

    pub fn delta_scope_digest(&self) -> &str {
        &self.delta_scope_digest
    }

    #[cfg(test)]
    pub(crate) fn from_one_shot_access_plan(
        one_shot: &WorthQueryAdmittedGraphReadAccessPlan,
    ) -> Self {
        let affected_requirement_row_count = one_shot.admission().requirement_set().rows().len();
        Self::new(
            "one_shot_graph_read_access_plan",
            one_shot.digest(),
            affected_requirement_row_count,
        )
    }

    pub(crate) fn from_subscription_family(family: &QuerySubscriptionFamily) -> Self {
        let affected_requirement_row_count = match family {
            QuerySubscriptionFamily::DetailExact
            | QuerySubscriptionFamily::InspectorDetailExact => 1,
            QuerySubscriptionFamily::CollectionMembership => 2,
            QuerySubscriptionFamily::GroupedCollectionMembership => 3,
            QuerySubscriptionFamily::BoundedMaterialization => 4,
        };
        Self::new(
            "subscription_family_live_access",
            family.as_str(),
            affected_requirement_row_count,
        )
    }

    fn new(authority: &str, source: &str, affected_requirement_row_count: usize) -> Self {
        let delta_scope_digest = hash_parts(&[
            "worth_query_live_graph_read_mutation_delta_scope_v1".to_string(),
            format!("authority:{authority}"),
            format!("source:{source}"),
            format!("affected_requirement_rows:{affected_requirement_row_count}"),
        ]);
        Self {
            affected_requirement_row_count,
            delta_scope_digest,
        }
    }
}
