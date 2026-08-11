use super::super::{
    classification_error, CapsuleSupportManifest, RawSupportProgramAction,
    SubscriptionSupportOperationalBasis, SubscriptionSupportPortabilityDecision,
    SubscriptionSupportPortabilityDecisionKind, SubscriptionSupportPortabilityOutcome,
    SubscriptionSupportPortabilityReport, SupportActionBreadthBudget, SupportActionId,
    SupportAllocationScope, SupportPathClass, SupportPortabilityAffectedSet,
    SupportPortabilityBatchPlan, SupportPortabilityManifestBudget,
    SupportPortabilityScopeFootprint, SupportProgramDensityClass,
};
use super::SubscriptionSupportPublicationPipeline;
use crate::failure::StoreError;

impl SubscriptionSupportPublicationPipeline {
    pub fn admit_support_portability_batch(
        &mut self,
        action_id: SupportActionId,
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
        included_support_count: u64,
        omitted_support_count: u64,
        manifest_budget: SupportPortabilityManifestBudget,
        decision: SubscriptionSupportPortabilityDecision,
        path_class: SupportPathClass,
        density_class: SupportProgramDensityClass,
        allocation_scope: SupportAllocationScope,
        budget: SupportActionBreadthBudget,
        manifest_header_bytes: u64,
    ) -> Result<SupportPortabilityBatchPlan, StoreError> {
        let affected_set = SupportPortabilityAffectedSet::from_portability_bases(affected_bases)?;
        let affected_entries = affected_set.affected_count();
        ensure_manifest_budget(
            &mut self.counters,
            &manifest_budget,
            included_support_count,
            manifest_header_bytes,
        )?;
        let path_plan = self.admit_support_program_path(
            path_class,
            density_class,
            allocation_scope,
            budget,
            affected_entries,
            manifest_header_bytes,
        )?;
        let (footprint, manifest) = materialize_portability_manifest(
            &mut self.counters,
            &affected_set,
            included_support_count,
            omitted_support_count,
            manifest_budget,
            manifest_header_bytes,
            &decision,
        )?;
        let plan = SupportPortabilityBatchPlan::new(
            action_id,
            affected_set,
            path_plan,
            footprint,
            manifest,
            decision,
        )?;
        self.counters.record_support_portability_plan(
            plan.manifest().manifest_entry_count(),
            plan.manifest().required_basis_count(),
            plan.manifest().omitted_support_count(),
        );
        Ok(plan)
    }

    pub fn publish_support_portability_consequence(
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
        let completed = self
            .publish_support_consequence(raw_action.plan().verify().execute(), path_plan.budget())?
            .complete();
        let report = SubscriptionSupportPortabilityReport::new(
            completed,
            affected_set,
            manifest,
            &decision,
            &path_plan,
        )?;
        match report.outcome() {
            SubscriptionSupportPortabilityOutcome::FullScopeReplicated(bundle) => {
                self.counters
                    .record_support_replication_inclusion(bundle.preserved_count());
            }
            SubscriptionSupportPortabilityOutcome::PartialScopeOmitted(omission) => {
                self.counters
                    .record_support_replication_omission(omission.omitted_count());
            }
            SubscriptionSupportPortabilityOutcome::Imported(_) => {
                self.counters.record_support_import_admission();
            }
            SubscriptionSupportPortabilityOutcome::ImportedNotResumable(_) => {
                self.counters.record_support_import_admission();
            }
            SubscriptionSupportPortabilityOutcome::Rejected(_) => {
                if decision.kind()
                    == SubscriptionSupportPortabilityDecisionKind::UnsupportedFamilyRejected
                {
                    self.counters.record_support_import_rejection();
                }
            }
        }
        Ok(report)
    }
}

fn ensure_manifest_budget(
    counters: &mut super::super::SubscriptionSupportCounterSnapshot,
    manifest_budget: &SupportPortabilityManifestBudget,
    included_support_count: u64,
    manifest_header_bytes: u64,
) -> Result<(), StoreError> {
    if !manifest_budget.admits(included_support_count, manifest_header_bytes) {
        counters.record_support_capsule_manifest_budget_denial();
        return Err(classification_error(
            "subscription-support capsule manifest exceeds portability manifest budget before footprint materialization",
        ));
    }
    Ok(())
}

fn materialize_portability_manifest(
    counters: &mut super::super::SubscriptionSupportCounterSnapshot,
    affected_set: &SupportPortabilityAffectedSet,
    included_support_count: u64,
    omitted_support_count: u64,
    manifest_budget: SupportPortabilityManifestBudget,
    manifest_header_bytes: u64,
    decision: &SubscriptionSupportPortabilityDecision,
) -> Result<(SupportPortabilityScopeFootprint, CapsuleSupportManifest), StoreError> {
    let omitted_artifact_ids = decision.omitted_artifact_ids_for_scope(affected_set);
    let basis_artifact_ids = decision.basis_artifact_ids_for_scope(affected_set);
    let footprint = SupportPortabilityScopeFootprint::new(
        affected_set,
        included_support_count,
        omitted_support_count,
        &omitted_artifact_ids,
        &basis_artifact_ids,
    )?;
    let manifest = match CapsuleSupportManifest::new(
        affected_set,
        footprint.clone(),
        manifest_budget,
        manifest_header_bytes,
        &basis_artifact_ids,
    ) {
        Ok(manifest) => manifest,
        Err(error) => {
            counters.record_support_capsule_manifest_budget_denial();
            return Err(error);
        }
    };
    Ok((footprint, manifest))
}
