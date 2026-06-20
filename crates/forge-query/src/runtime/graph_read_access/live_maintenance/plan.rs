use super::{
    ForgeQueryLiveGraphReadAccessDenial, ForgeQueryLiveGraphReadAccessPosture,
    ForgeQueryLiveGraphReadMaintenanceBudget, ForgeQueryLiveGraphReadMutationDeltaScope,
};
use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryAdmittedGraphReadAccessPlan, ForgeQueryGraphReadAccessAdmissionPosture,
    ForgeQueryRuntimeLiveSubscriptionInstallation,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLiveGraphReadAccessPlan {
    digest: String,
    one_shot_access_plan_digest: String,
    one_shot_access_shape_digest: String,
    required_index_digest: String,
    posture: ForgeQueryLiveGraphReadAccessPosture,
    maintenance_budget: ForgeQueryLiveGraphReadMaintenanceBudget,
    mutation_delta_scope: ForgeQueryLiveGraphReadMutationDeltaScope,
    maintenance_equivalence_digest: String,
}

impl ForgeQueryLiveGraphReadAccessPlan {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn one_shot_access_plan_digest(&self) -> &str {
        &self.one_shot_access_plan_digest
    }

    pub fn one_shot_access_shape_digest(&self) -> &str {
        &self.one_shot_access_shape_digest
    }

    pub fn required_index_digest(&self) -> &str {
        &self.required_index_digest
    }

    pub fn posture(&self) -> &ForgeQueryLiveGraphReadAccessPosture {
        &self.posture
    }

    pub fn maintenance_budget(&self) -> &ForgeQueryLiveGraphReadMaintenanceBudget {
        &self.maintenance_budget
    }

    pub fn mutation_delta_scope(&self) -> &ForgeQueryLiveGraphReadMutationDeltaScope {
        &self.mutation_delta_scope
    }

    pub fn maintenance_equivalence_digest(&self) -> &str {
        &self.maintenance_equivalence_digest
    }

    pub(crate) fn from_one_shot_access_plan(
        one_shot: &ForgeQueryAdmittedGraphReadAccessPlan,
        maintenance_budget: ForgeQueryLiveGraphReadMaintenanceBudget,
    ) -> Result<Self, ForgeQueryLiveGraphReadAccessDenial> {
        let mutation_delta_scope =
            ForgeQueryLiveGraphReadMutationDeltaScope::from_one_shot_access_plan(one_shot);
        let posture = live_posture_for_one_shot(
            one_shot.posture(),
            &maintenance_budget,
            mutation_delta_scope.affected_requirement_row_count(),
        );
        if !posture.is_admitted() {
            return Err(ForgeQueryLiveGraphReadAccessDenial::new(
                posture,
                one_shot.digest(),
                &maintenance_budget,
                "one-shot graph read access plan is not safely maintainable as a live read",
            ));
        }
        Ok(Self::new(
            one_shot.digest(),
            one_shot.admission().requirement_set().access_shape_digest(),
            one_shot.graph_index_support().requirement_set_digest(),
            posture,
            maintenance_budget,
            mutation_delta_scope,
        ))
    }

    pub(crate) fn from_live_installation(
        installation: &ForgeQueryRuntimeLiveSubscriptionInstallation,
        maintenance_budget: ForgeQueryLiveGraphReadMaintenanceBudget,
    ) -> Result<Self, ForgeQueryLiveGraphReadAccessDenial> {
        let one_shot_digest = installation.query_projection().label().to_string();
        let access_shape_digest = installation
            .canonical_result_shape_digest()
            .as_str()
            .to_string();
        let required_index_digest = installation
            .subscription_family_projection()
            .label()
            .to_string();
        let mutation_delta_scope =
            ForgeQueryLiveGraphReadMutationDeltaScope::from_subscription_family(
                installation.subscription_family_kind(),
            );
        let posture = live_posture_for_subscription(
            installation.subscription_family_kind(),
            &maintenance_budget,
            mutation_delta_scope.affected_requirement_row_count(),
        );
        if !posture.is_admitted() {
            return Err(ForgeQueryLiveGraphReadAccessDenial::new(
                posture,
                one_shot_digest,
                &maintenance_budget,
                "live subscription family requires stronger maintenance support than the admitted budget provides",
            ));
        }
        Ok(Self::new(
            installation.query_projection().label(),
            &access_shape_digest,
            &required_index_digest,
            posture,
            maintenance_budget,
            mutation_delta_scope,
        ))
    }

    fn new(
        one_shot_access_plan_digest: impl Into<String>,
        one_shot_access_shape_digest: impl Into<String>,
        required_index_digest: impl Into<String>,
        posture: ForgeQueryLiveGraphReadAccessPosture,
        maintenance_budget: ForgeQueryLiveGraphReadMaintenanceBudget,
        mutation_delta_scope: ForgeQueryLiveGraphReadMutationDeltaScope,
    ) -> Self {
        let one_shot_access_plan_digest = one_shot_access_plan_digest.into();
        let one_shot_access_shape_digest = one_shot_access_shape_digest.into();
        let required_index_digest = required_index_digest.into();
        let maintenance_equivalence_digest = hash_parts(&[
            "forge_query_live_graph_read_access_equivalence_v1".to_string(),
            format!("shape:{one_shot_access_shape_digest}"),
            format!("indexes:{required_index_digest}"),
            format!("posture:{}", posture.as_str()),
            format!("delta_scope:{}", mutation_delta_scope.delta_scope_digest()),
        ]);
        let digest = hash_parts(&[
            "forge_query_live_graph_read_access_plan_v1".to_string(),
            format!("one_shot_plan:{one_shot_access_plan_digest}"),
            format!("shape:{one_shot_access_shape_digest}"),
            format!("indexes:{required_index_digest}"),
            format!("posture:{}", posture.as_str()),
            format!("budget:{}", maintenance_budget.digest()),
            format!("delta_scope:{}", mutation_delta_scope.delta_scope_digest()),
            format!("equivalence:{maintenance_equivalence_digest}"),
        ]);
        Self {
            digest,
            one_shot_access_plan_digest,
            one_shot_access_shape_digest,
            required_index_digest,
            posture,
            maintenance_budget,
            mutation_delta_scope,
            maintenance_equivalence_digest,
        }
    }
}

fn live_posture_for_one_shot(
    posture: &ForgeQueryGraphReadAccessAdmissionPosture,
    budget: &ForgeQueryLiveGraphReadMaintenanceBudget,
    affected_requirement_rows: usize,
) -> ForgeQueryLiveGraphReadAccessPosture {
    if affected_requirement_rows > budget.max_requirement_rows() {
        return ForgeQueryLiveGraphReadAccessPosture::DeniedLiveMaintenanceBudget;
    }
    match posture {
        ForgeQueryGraphReadAccessAdmissionPosture::InlineIndexed
        | ForgeQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex => {
            ForgeQueryLiveGraphReadAccessPosture::AdmittedLiveIncrementalMaintenance
        }
        ForgeQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming
            if budget.admits_snapshot_refresh() =>
        {
            ForgeQueryLiveGraphReadAccessPosture::AdmittedLiveSnapshotRefresh
        }
        ForgeQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming => {
            ForgeQueryLiveGraphReadAccessPosture::DeniedLiveMaintenanceBudget
        }
        ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired => {
            ForgeQueryLiveGraphReadAccessPosture::LivePersistentIndexRequired
        }
        ForgeQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired
        | ForgeQueryGraphReadAccessAdmissionPosture::PagedStreamingRequired => {
            ForgeQueryLiveGraphReadAccessPosture::LiveAsyncMaterializationRequired
        }
        ForgeQueryGraphReadAccessAdmissionPosture::StoreBackedCapabilityRequired => {
            ForgeQueryLiveGraphReadAccessPosture::LiveStoreBackedCapabilityRequired
        }
        ForgeQueryGraphReadAccessAdmissionPosture::AccessCapabilityRegistrationRequired => {
            ForgeQueryLiveGraphReadAccessPosture::LiveAccessCapabilityRegistrationRequired
        }
        ForgeQueryGraphReadAccessAdmissionPosture::Denied => {
            ForgeQueryLiveGraphReadAccessPosture::DeniedLiveMaintenanceSupport
        }
    }
}

fn live_posture_for_subscription(
    family: &crate::subscription::QuerySubscriptionFamily,
    budget: &ForgeQueryLiveGraphReadMaintenanceBudget,
    affected_requirement_rows: usize,
) -> ForgeQueryLiveGraphReadAccessPosture {
    if affected_requirement_rows > budget.max_requirement_rows() {
        return ForgeQueryLiveGraphReadAccessPosture::DeniedLiveMaintenanceBudget;
    }
    match family {
        crate::subscription::QuerySubscriptionFamily::DetailExact
        | crate::subscription::QuerySubscriptionFamily::CollectionMembership
        | crate::subscription::QuerySubscriptionFamily::InspectorDetailExact => {
            ForgeQueryLiveGraphReadAccessPosture::AdmittedLiveIncrementalMaintenance
        }
        crate::subscription::QuerySubscriptionFamily::GroupedCollectionMembership
        | crate::subscription::QuerySubscriptionFamily::BoundedMaterialization
            if budget.admits_snapshot_refresh() =>
        {
            ForgeQueryLiveGraphReadAccessPosture::AdmittedLiveSnapshotRefresh
        }
        crate::subscription::QuerySubscriptionFamily::GroupedCollectionMembership
        | crate::subscription::QuerySubscriptionFamily::BoundedMaterialization => {
            ForgeQueryLiveGraphReadAccessPosture::DeniedLiveMaintenanceBudget
        }
    }
}
