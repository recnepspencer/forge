use crate::{
    authority::AuthoritativeExportBundle,
    delta::BranchDeltaReadRequest,
    evidence::{Milestone1CertificationBundle, Milestone35CertificationBundle, Milestone4CertificationBundle, Milestone6CertificationBundle, Milestone7CertificationBundle, ObservedPublicationFailure},
    failure::StoreError,
    layout::{AspectLayoutReadRequest, Milestone6LayoutSupportLane, Milestone6LayoutSupportPolicy, Milestone6ResolvedLayoutSupportLane},
    live_query::{acknowledgment::ContinuationAcknowledgmentEffect, compatibility::ContinuationPlanningEffect},
    PublicationWriteOutcome,
    snapshot::SnapshotImageBundle,
};

use super::ForgeStore;

impl ForgeStore {
    pub fn milestone_1_certification_bundle(&self) -> Milestone1CertificationBundle {
        let export = self.export_authoritative_records();
        Milestone1CertificationBundle::from_export(&export, self.counters())
    }

    pub fn milestone_3_5_certification_bundle(
        &self,
        ack_boundary_report: PublicationWriteOutcome,
        failures: &[ObservedPublicationFailure],
    ) -> Milestone35CertificationBundle {
        Milestone35CertificationBundle::new(
            self.durable_media_report(),
            ack_boundary_report,
            self.counters(),
            failures,
        )
    }

    pub fn milestone_4_certification_bundle(
        &self,
        truth_image: &SnapshotImageBundle,
        restored_image: &SnapshotImageBundle,
        rebuilt_image: &SnapshotImageBundle,
    ) -> Milestone4CertificationBundle {
        Milestone4CertificationBundle::new(
            truth_image,
            restored_image,
            rebuilt_image,
            self.counters(),
        )
    }

    pub fn milestone_5_certification_bundle(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<crate::Milestone5CertificationBundle, StoreError> {
        let witness = self.admit_same_branch_descendant(request.clone())?;
        let direct = self.read_branch_delta(witness)?;
        let reference = self.admit_milestone_7_independent_reference(request.clone())?;
        let control = self.read_branch_delta_control_from_milestone_7_reference(reference)?;
        let delta_storage_report = self.backend.milestone_5_delta_storage_report(
            request.branch_id,
            request.target_commit_id,
            &direct.plan,
            &control.plan,
        )?;
        Ok(crate::Milestone5CertificationBundle::new(
            direct.authoritative_export(),
            control.authoritative_export(),
            delta_storage_report,
            self.counters(),
        ))
    }

    pub(crate) fn require_admitted_aspect_layout_plan(
        &self,
        request: AspectLayoutReadRequest,
        operation_name: &str,
    ) -> Result<crate::AdmittedAspectLayoutReadPlan, StoreError> {
        match self.plan_aspect_layout_read(request)? {
            crate::AspectLayoutReadPlanDecision::Admitted(plan) => Ok(plan),
            crate::AspectLayoutReadPlanDecision::Fallback(plan) => Err(StoreError::new(
                crate::StoreErrorKind::AspectLayoutFallbackRequired,
                format!(
                    "{operation_name} requires an admitted layout read, but request fell back: {}",
                    plan.reason()
                ),
            )),
            crate::AspectLayoutReadPlanDecision::Rejected(plan) => Err(StoreError::new(
                crate::StoreErrorKind::AspectScopeUnsupported,
                format!(
                    "{operation_name} requires an admitted layout read, but request was rejected: {}",
                    plan.reason()
                ),
            )),
        }
    }

    pub fn milestone_6_certification_bundle(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<Milestone6CertificationBundle, StoreError> {
        let plan =
            self.require_admitted_aspect_layout_plan(request.clone(), "milestone 6 certification")?;
        let artifact_id = crate::layout::layout_materialization_artifact_id(&plan);
        match self
            .backend
            .fetch_existing_milestone_6_layout_support(&artifact_id)
        {
            Ok(materialization) => {
                return Ok(Milestone6CertificationBundle::from_materialization(
                    &materialization,
                    self.milestone_6_access_structure_verification(),
                    self.counters(),
                ));
            }
            Err(error)
                if matches!(
                    error.kind(),
                    crate::StoreErrorKind::AspectLayoutArtifactMissing
                ) => {}
            Err(error) => return Err(error),
        }
        let reuse = self.admit_structural_block_reuse(plan.clone())?;
        let frozen = self.freeze_chunk_model(plan.clone())?;
        let milestone_7 = self.admit_milestone_7_independent_layout_reference(plan.clone())?;
        let milestone_9 = self.admit_milestone_9_physical_chunk_reference(frozen.clone())?;
        Ok(Milestone6CertificationBundle::new(
            &plan,
            &reuse,
            &frozen,
            &milestone_7,
            &milestone_9,
            self.milestone_6_access_structure_verification(),
            self.counters(),
        ))
    }

    pub fn milestone_6_certification_bundle_in_lane(
        &mut self,
        request: AspectLayoutReadRequest,
        lane: Milestone6LayoutSupportLane,
    ) -> Result<Milestone6CertificationBundle, StoreError> {
        self.milestone_6_certification_bundle_in_lane_with_policy(
            request,
            lane,
            Milestone6LayoutSupportPolicy::new(false, false, 0),
        )
    }

    pub fn milestone_6_certification_bundle_in_lane_with_policy(
        &mut self,
        request: AspectLayoutReadRequest,
        lane: Milestone6LayoutSupportLane,
        policy: Milestone6LayoutSupportPolicy,
    ) -> Result<Milestone6CertificationBundle, StoreError> {
        let prepared =
            self.prepare_milestone_6_layout_support_with_policy(request.clone(), lane, policy)?;
        match prepared.resolved_lane() {
            Milestone6ResolvedLayoutSupportLane::ProofOnly => {
                let plan = self.require_admitted_aspect_layout_plan(
                    request,
                    "milestone 6 proof-only certification",
                )?;
                let reuse = self.admit_structural_block_reuse(plan.clone())?;
                let frozen = self.freeze_chunk_model(plan.clone())?;
                let milestone_7 =
                    self.admit_milestone_7_independent_layout_reference(plan.clone())?;
                let milestone_9 =
                    self.admit_milestone_9_physical_chunk_reference(frozen.clone())?;
                Ok(Milestone6CertificationBundle::for_lane(
                    &plan,
                    &reuse,
                    &frozen,
                    &milestone_7,
                    &milestone_9,
                    self.milestone_6_access_structure_verification(),
                    self.counters(),
                    prepared.requested_lane(),
                    prepared.resolved_lane(),
                    prepared.publication_disposition(),
                ))
            }
            Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized
            | Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedPublished
            | Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedReuseExisting => {
                let artifact_id = prepared
                    .layout_materialization_artifact_id()
                    .expect("materialized certification lane should always return an artifact id")
                    .to_string();
                let materialization = self
                    .backend
                    .fetch_existing_milestone_6_layout_support(&artifact_id)?;
                Ok(Milestone6CertificationBundle::from_materialization_in_lane(
                    &materialization,
                    self.milestone_6_access_structure_verification(),
                    self.counters(),
                    prepared.requested_lane(),
                    prepared.resolved_lane(),
                    prepared.publication_disposition(),
                ))
            }
        }
    }

    pub fn milestone_7_certification_bundle(
        &self,
        control_export: &AuthoritativeExportBundle,
    ) -> Milestone7CertificationBundle {
        Milestone7CertificationBundle::new(
            &self.export_authoritative_records(),
            control_export,
            self.durable_media_report(),
            self.support_artifact_recovery_report(),
            self.milestone_7_access_structure_verification(),
            self.counters(),
        )
    }

    pub(crate) fn record_continuation_planning_effects(
        &self,
        effects: Vec<ContinuationPlanningEffect>,
    ) {
        for effect in effects {
            match effect {
                ContinuationPlanningEffect::SchemaMismatch => {
                    self.backend.record_continuation_schema_mismatch()
                }
                ContinuationPlanningEffect::ScopeMismatch => {
                    self.backend.record_continuation_scope_mismatch()
                }
                ContinuationPlanningEffect::DegradedBasis => {
                    self.backend.record_continuation_degraded_basis()
                }
                ContinuationPlanningEffect::StableBasisBroadening => {
                    self.backend.record_stable_basis_broadening()
                }
                ContinuationPlanningEffect::ContinuationBroadening => {
                    self.backend.record_continuation_broadening()
                }
                ContinuationPlanningEffect::RejectedBasis => {
                    self.backend.record_continuation_rejected_basis()
                }
                ContinuationPlanningEffect::ContinuationPlan => {
                    self.backend.record_continuation_plan()
                }
            }
        }
    }

    pub(crate) fn record_continuation_ack_effects(
        &self,
        effects: Vec<ContinuationAcknowledgmentEffect>,
    ) {
        for effect in effects {
            match effect {
                ContinuationAcknowledgmentEffect::Parity => {
                    self.backend.record_continuation_parity()
                }
                ContinuationAcknowledgmentEffect::IllegalAcknowledgment => {
                    self.backend.record_continuation_illegal_acknowledgment()
                }
                ContinuationAcknowledgmentEffect::BatchDuplicate => {
                    self.backend.record_continuation_batch_duplicate()
                }
                ContinuationAcknowledgmentEffect::BatchGap => {
                    self.backend.record_continuation_batch_gap()
                }
            }
        }
    }
}
