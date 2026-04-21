use crate::{
    failure::{StoreError, StoreErrorKind},
    layout::{
        AspectLayoutReadRequest, Milestone6LayoutSupportLane, Milestone6LayoutSupportPolicy,
        Milestone6ResolvedLayoutSupportLane,
    },
};

use super::ForgeStore;

impl ForgeStore {
    pub fn structural_block_lookup(
        &self,
        lookup: crate::StructuralBlockLookup,
    ) -> Result<crate::StructuralBlockLookupResult, StoreError> {
        self.backend.structural_block_lookup(lookup)
    }

    pub fn execute_aspect_layout_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<crate::AspectLayoutReadExecutionDecision, StoreError> {
        self.backend.execute_aspect_layout_read(request)
    }

    pub fn execute_aspect_layout_read_in_lane(
        &mut self,
        request: AspectLayoutReadRequest,
        lane: Milestone6LayoutSupportLane,
    ) -> Result<crate::AspectLayoutReadExecutionDecision, StoreError> {
        self.execute_aspect_layout_read_in_lane_with_policy(
            request,
            lane,
            Milestone6LayoutSupportPolicy::new(false, false, 0),
        )
    }

    pub fn execute_aspect_layout_read_in_lane_with_policy(
        &mut self,
        request: AspectLayoutReadRequest,
        lane: Milestone6LayoutSupportLane,
        policy: Milestone6LayoutSupportPolicy,
    ) -> Result<crate::AspectLayoutReadExecutionDecision, StoreError> {
        let prepared =
            self.prepare_milestone_6_layout_support_with_policy(request.clone(), lane, policy)?;
        match prepared.resolved_lane() {
            Milestone6ResolvedLayoutSupportLane::ProofOnly => {
                let plan = self.require_admitted_aspect_layout_plan(
                    request.clone(),
                    "milestone 6 proof-only aspect layout execution",
                )?;
                let control = self.read_aspect_layout_control_truth(request)?;
                let foreground_isolation = self
                    .backend
                    .assess_read_foreground_isolation(plan.request().target().branch_id(), false);
                Ok(crate::AspectLayoutReadExecutionDecision::Admitted(
                    crate::AspectLayoutReadExecutionResult::new(
                        plan.clone(),
                        prepared.requested_lane(),
                        prepared.resolved_lane(),
                        prepared.publication_disposition(),
                        None,
                        crate::layout::structural_block_artifact_id(plan.structural_block_id()),
                        None,
                        None,
                        control.authoritative_truth_digest().to_string(),
                        control.authoritative_commit_count(),
                    )
                    .with_foreground_isolation(foreground_isolation),
                ))
            }
            Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized
            | Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedPublished
            | Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedReuseExisting => {
                let mut decision = self.backend.execute_aspect_layout_read(request)?;
                if let crate::AspectLayoutReadExecutionDecision::Admitted(read) = &mut decision {
                    *read = crate::AspectLayoutReadExecutionResult::new(
                        read.plan().clone(),
                        prepared.requested_lane(),
                        prepared.resolved_lane(),
                        prepared.publication_disposition(),
                        read.scope_membership_artifact_id().map(ToOwned::to_owned),
                        read.structural_block_artifact_id().to_string(),
                        read.chunk_membership_artifact_id().map(ToOwned::to_owned),
                        read.layout_materialization_artifact_id()
                            .map(ToOwned::to_owned),
                        read.semantic_truth_digest().to_string(),
                        read.authoritative_commit_count(),
                    )
                    .with_foreground_isolation(read.foreground_isolation().clone());
                }
                Ok(decision)
            }
        }
    }

    pub fn read_aspect_layout_control_truth(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<crate::AspectLayoutControlTruth, StoreError> {
        let plan =
            self.require_admitted_aspect_layout_plan(request, "aspect layout control truth")?;
        let milestone_7 = self.admit_milestone_7_independent_layout_reference(plan.clone())?;
        let control = self.read_branch_delta_control_from_milestone_7_reference(
            crate::Milestone7IndependentReference::new(
                milestone_7.branch_id().clone(),
                milestone_7.frontier_commit_id(),
            ),
        )?;
        Ok(crate::AspectLayoutControlTruth::new(
            milestone_7.branch_id().clone(),
            milestone_7.frontier_commit_id(),
            milestone_7.scope_class().to_string(),
            milestone_7.projection_digest().to_string(),
            crate::layout::stable_layout_truth_digest(control.authoritative_export()),
            control.authoritative_export().commit_envelopes.len(),
        ))
    }

    pub fn execute_dedup_backed_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<crate::DedupBackedReadResult, StoreError> {
        self.backend.execute_dedup_backed_read(request)
    }

    pub fn execute_dedup_backed_read_in_lane(
        &mut self,
        request: AspectLayoutReadRequest,
        lane: Milestone6LayoutSupportLane,
    ) -> Result<crate::DedupBackedReadResult, StoreError> {
        self.execute_dedup_backed_read_in_lane_with_policy(
            request,
            lane,
            Milestone6LayoutSupportPolicy::new(false, false, 0),
        )
    }

    pub fn execute_dedup_backed_read_in_lane_with_policy(
        &mut self,
        request: AspectLayoutReadRequest,
        lane: Milestone6LayoutSupportLane,
        policy: Milestone6LayoutSupportPolicy,
    ) -> Result<crate::DedupBackedReadResult, StoreError> {
        let read =
            match self.execute_aspect_layout_read_in_lane_with_policy(request, lane, policy)? {
                crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
                crate::AspectLayoutReadExecutionDecision::Fallback(plan) => {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectLayoutFallbackRequired,
                        plan.reason().to_string(),
                    ))
                }
                crate::AspectLayoutReadExecutionDecision::Rejected(plan) => {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectScopeUnsupported,
                        plan.reason().to_string(),
                    ))
                }
            };
        let lookup = if read
            .resolved_layout_support_lane()
            .uses_materialized_support()
        {
            self.structural_block_lookup(crate::StructuralBlockLookup::new(
                read.plan().structural_block_id().clone(),
            ))?
        } else {
            crate::StructuralBlockLookupResult::new(
                read.plan().structural_block_id().clone(),
                read.plan().request().scope_class().label().to_string(),
                crate::EQUIVALENCE_CONTRACT_VERSION,
                read.plan().slice_ids().to_vec(),
                Vec::new(),
            )
        };
        if lookup.slice_ids() != read.plan().slice_ids() {
            return Err(StoreError::backend_integrity(
                "dedup-backed lane-aware read structural block lookup drifted from admitted plan slice ids",
            ));
        }
        Ok(crate::DedupBackedReadResult::new(read, lookup))
    }

    pub fn export_milestone_6_chunk_model(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<crate::Milestone6ChunkModelExport, StoreError> {
        let read = match self.execute_aspect_layout_read(request)? {
            crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
            crate::AspectLayoutReadExecutionDecision::Fallback(plan) => {
                return Err(StoreError::new(
                    StoreErrorKind::AspectLayoutFallbackRequired,
                    plan.reason().to_string(),
                ))
            }
            crate::AspectLayoutReadExecutionDecision::Rejected(plan) => {
                return Err(StoreError::new(
                    StoreErrorKind::AspectScopeUnsupported,
                    plan.reason().to_string(),
                ))
            }
        };
        let materialization_artifact_id = read.layout_materialization_artifact_id().ok_or_else(|| {
            StoreError::backend_integrity(
                "default milestone 6 chunk export requires a materialized layout-support artifact id",
            )
        })?;
        let materialization = self
            .backend
            .fetch_existing_milestone_6_layout_support(materialization_artifact_id)?;
        self.backend.record_physical_chunk_export(
            materialization.milestone_9_reference().chunk_member_count() as u64,
        );
        Ok(crate::Milestone6ChunkModelExport::new(
            crate::Milestone6LayoutSupportLane::OnDemandMaterialized,
            crate::Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized,
            crate::Milestone6LayoutSupportPublicationDisposition::ReusedExisting,
            materialization
                .milestone_9_reference()
                .physical_chunk_id()
                .clone(),
            read.chunk_membership_artifact_id().map(ToOwned::to_owned),
            materialization
                .milestone_9_reference()
                .determinism_digest()
                .to_string(),
            materialization.milestone_9_reference().chunk_member_count(),
            Some(materialization_artifact_id.to_string()),
        ))
    }

    pub fn export_milestone_6_chunk_model_in_lane(
        &mut self,
        request: AspectLayoutReadRequest,
        lane: Milestone6LayoutSupportLane,
    ) -> Result<crate::Milestone6ChunkModelExport, StoreError> {
        self.export_milestone_6_chunk_model_in_lane_with_policy(
            request,
            lane,
            Milestone6LayoutSupportPolicy::new(false, false, 0),
        )
    }

    pub fn export_milestone_6_chunk_model_in_lane_with_policy(
        &mut self,
        request: AspectLayoutReadRequest,
        lane: Milestone6LayoutSupportLane,
        policy: Milestone6LayoutSupportPolicy,
    ) -> Result<crate::Milestone6ChunkModelExport, StoreError> {
        let read =
            match self.execute_aspect_layout_read_in_lane_with_policy(request, lane, policy)? {
                crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
                crate::AspectLayoutReadExecutionDecision::Fallback(plan) => {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectLayoutFallbackRequired,
                        plan.reason().to_string(),
                    ))
                }
                crate::AspectLayoutReadExecutionDecision::Rejected(plan) => {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectScopeUnsupported,
                        plan.reason().to_string(),
                    ))
                }
            };
        if !read
            .resolved_layout_support_lane()
            .uses_materialized_support()
        {
            return Err(StoreError::new(
                StoreErrorKind::AspectLayoutArtifactMissing,
                "proof-only milestone 6 chunk export is unsupported because chunk export requires materialized chunk membership",
            ));
        }
        let materialization_artifact_id = read
            .layout_materialization_artifact_id()
            .ok_or_else(|| {
                StoreError::backend_integrity(
                    "materialized milestone 6 chunk export resolved without a layout materialization artifact id",
                )
            })?;
        let materialization = self
            .backend
            .fetch_existing_milestone_6_layout_support(materialization_artifact_id)?;
        self.backend.record_physical_chunk_export(
            materialization.milestone_9_reference().chunk_member_count() as u64,
        );
        Ok(crate::Milestone6ChunkModelExport::new(
            read.requested_layout_support_lane(),
            read.resolved_layout_support_lane(),
            read.layout_support_publication_disposition(),
            materialization
                .milestone_9_reference()
                .physical_chunk_id()
                .clone(),
            read.chunk_membership_artifact_id().map(ToOwned::to_owned),
            materialization
                .milestone_9_reference()
                .determinism_digest()
                .to_string(),
            materialization.milestone_9_reference().chunk_member_count(),
            Some(materialization_artifact_id.to_string()),
        ))
    }
}
