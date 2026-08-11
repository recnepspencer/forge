use crate::{
    failure::{StoreError, StoreErrorKind},
    CapsuleSupportManifest, RawSupportProgramAction, SubscriptionSupportPortabilityBatchRequest,
    SubscriptionSupportPortabilityDecision, SubscriptionSupportPortabilityDecisionKind,
    SubscriptionSupportPortabilityOutcome, SubscriptionSupportPortabilityReport, SupportActionId,
    SupportPortabilityAffectedSet, SupportPortabilityBatchPlan, SupportPortabilityManifestBudget,
    SupportPortabilityScopeFootprint, SupportProgramPathPlan,
};

use super::super::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn admit_subscription_support_portability_batch(
        &mut self,
        request: SubscriptionSupportPortabilityBatchRequest,
    ) -> Result<SupportPortabilityBatchPlan, StoreError> {
        let SubscriptionSupportPortabilityBatchRequest {
            action_id,
            affected_bases,
            included_support_count,
            omitted_support_count,
            manifest_budget,
            decision,
            path,
        } = request;
        let manifest_header_bytes = path.payload_header_bytes;
        let affected_set = SupportPortabilityAffectedSet::from_portability_bases(affected_bases)?;
        admit_portability_manifest_budget(
            self,
            manifest_budget,
            included_support_count,
            manifest_header_bytes,
        )?;
        let path_plan = self.admit_subscription_support_program_path(
            path.admission_request(affected_set.affected_count()),
        )?;
        let materialization = materialize_portability_scope(
            self,
            PortabilityScopeMaterializationInput {
                affected_set,
                included_support_count,
                omitted_support_count,
                manifest_budget,
                manifest_header_bytes,
                decision: &decision,
            },
        )?;
        publish_portability_plan_and_counters(
            self,
            PortabilityPlanPublication {
                action_id,
                path_plan,
                materialization,
                decision,
            },
        )
    }

    pub fn publish_subscription_support_portability_consequence(
        &mut self,
        plan: SupportPortabilityBatchPlan,
    ) -> Result<SubscriptionSupportPortabilityReport, StoreError> {
        let (action_id, affected_set, path_plan, _footprint, manifest, decision) =
            plan.into_parts();
        let raw_action = RawSupportProgramAction::new(
            action_id,
            affected_set.primary_basis().clone(),
            decision.verdict(),
        )?;
        let completed =
            self.publish_support_action_with_durable_recovery(raw_action, path_plan.budget())?;
        let report = SubscriptionSupportPortabilityReport::new(
            completed,
            affected_set,
            manifest,
            &decision,
            &path_plan,
        )?;
        match report.outcome() {
            SubscriptionSupportPortabilityOutcome::FullScopeReplicated(bundle) => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_replication_inclusion(bundle.preserved_count());
            }
            SubscriptionSupportPortabilityOutcome::PartialScopeOmitted(omission) => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_replication_omission(omission.omitted_count());
            }
            SubscriptionSupportPortabilityOutcome::Imported(_) => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_import_admission();
            }
            SubscriptionSupportPortabilityOutcome::ImportedNotResumable(_) => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_import_admission();
            }
            SubscriptionSupportPortabilityOutcome::Rejected(_) => {
                if decision.kind()
                    == SubscriptionSupportPortabilityDecisionKind::UnsupportedFamilyRejected
                {
                    self.state
                        .subscription_support_counter_snapshot
                        .record_support_import_rejection();
                }
            }
        }
        Ok(report)
    }
}

struct PortabilityScopeMaterializationInput<'a> {
    affected_set: SupportPortabilityAffectedSet,
    included_support_count: u64,
    omitted_support_count: u64,
    manifest_budget: SupportPortabilityManifestBudget,
    manifest_header_bytes: u64,
    decision: &'a SubscriptionSupportPortabilityDecision,
}

struct PortabilityScopeMaterialization {
    affected_set: SupportPortabilityAffectedSet,
    footprint: SupportPortabilityScopeFootprint,
    manifest: CapsuleSupportManifest,
}

struct PortabilityPlanPublication {
    action_id: SupportActionId,
    path_plan: SupportProgramPathPlan,
    materialization: PortabilityScopeMaterialization,
    decision: SubscriptionSupportPortabilityDecision,
}

fn admit_portability_manifest_budget<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    manifest_budget: SupportPortabilityManifestBudget,
    included_support_count: u64,
    manifest_header_bytes: u64,
) -> Result<(), StoreError> {
    if manifest_budget.admits(included_support_count, manifest_header_bytes) {
        return Ok(());
    }
    backend
        .state
        .subscription_support_counter_snapshot
        .record_support_capsule_manifest_budget_denial();
    Err(StoreError::new(
        StoreErrorKind::SubscriptionSupportClassificationViolation,
        "subscription-support capsule manifest exceeds portability manifest budget before footprint materialization",
    ))
}

fn materialize_portability_scope<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    input: PortabilityScopeMaterializationInput<'_>,
) -> Result<PortabilityScopeMaterialization, StoreError> {
    let omitted_artifact_ids = input
        .decision
        .omitted_artifact_ids_for_scope(&input.affected_set);
    let basis_artifact_ids = input
        .decision
        .basis_artifact_ids_for_scope(&input.affected_set);
    let footprint = SupportPortabilityScopeFootprint::new(
        &input.affected_set,
        input.included_support_count,
        input.omitted_support_count,
        &omitted_artifact_ids,
        &basis_artifact_ids,
    )?;
    let manifest = match CapsuleSupportManifest::new(
        &input.affected_set,
        footprint.clone(),
        input.manifest_budget,
        input.manifest_header_bytes,
        &basis_artifact_ids,
    ) {
        Ok(manifest) => manifest,
        Err(error) => {
            backend
                .state
                .subscription_support_counter_snapshot
                .record_support_capsule_manifest_budget_denial();
            return Err(error);
        }
    };
    Ok(PortabilityScopeMaterialization {
        affected_set: input.affected_set,
        footprint,
        manifest,
    })
}

fn publish_portability_plan_and_counters<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    publication: PortabilityPlanPublication,
) -> Result<SupportPortabilityBatchPlan, StoreError> {
    let PortabilityPlanPublication {
        action_id,
        path_plan,
        materialization,
        decision,
    } = publication;
    let plan = SupportPortabilityBatchPlan::new(
        action_id,
        materialization.affected_set,
        path_plan,
        materialization.footprint,
        materialization.manifest,
        decision,
    )?;
    backend
        .state
        .subscription_support_counter_snapshot
        .record_support_portability_plan(
            plan.manifest().manifest_entry_count(),
            plan.manifest().required_basis_count(),
            plan.manifest().omitted_support_count(),
        );
    Ok(plan)
}
