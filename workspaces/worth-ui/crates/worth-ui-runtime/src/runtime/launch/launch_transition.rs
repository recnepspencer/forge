use std::cell::RefCell;
use std::rc::Rc;

use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::allocation_frame_dispatch::UiAllocationFrameFrameworkScheduler;
use crate::runtime::allocation_planning::WorthUiRetainedAllocationPlanningEvidenceRegistry;
use crate::runtime::allocation_receipt::UiAllocationReceiptLedger;

use super::build_active_state::build_active_runtime_state;
use super::derive_plan::derive_launch_execution_plan;
use super::launch_request::{WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial};
use super::preservation::WorthUiLastValidRuntimeState;
use super::runtime_instance::WorthUiRuntime;
use super::seal_artifact::seal_launch_artifact;

impl WorthUiRuntime {
    pub(crate) fn launch(
        launch: WorthUiRuntimeLaunch,
        snapshot_digest: CapabilitySnapshotDigest,
        retained_allocation_planning_evidence: Rc<
            WorthUiRetainedAllocationPlanningEvidenceRegistry,
        >,
        query_binding: worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    ) -> Result<Self, WorthUiRuntimeLaunchDenial> {
        let WorthUiRuntimeLaunch {
            artifact,
            frame_epoch,
            diagnostic_policy,
            candidate_snapshot_digest,
            candidate_artifact_digest,
        } = launch;
        if let Some(candidate_snapshot_digest) = candidate_snapshot_digest {
            if candidate_snapshot_digest != snapshot_digest.as_u64() {
                return Err(WorthUiRuntimeLaunchDenial::CandidateSnapshotMismatch {
                    candidate_snapshot_digest,
                    app_snapshot_digest: snapshot_digest.as_u64(),
                });
            }
        }
        let (active_artifact, artifact_digest) = match candidate_artifact_digest {
            Some(artifact_digest) => (
                crate::runtime::active::WorthUiActiveArtifact::new(artifact, artifact_digest),
                artifact_digest,
            ),
            None => seal_launch_artifact(artifact),
        };
        let active_plan = derive_launch_execution_plan(artifact_digest, snapshot_digest);
        let active = build_active_runtime_state(
            active_artifact,
            active_plan,
            snapshot_digest,
            frame_epoch,
            diagnostic_policy,
        );
        let last_valid = WorthUiLastValidRuntimeState::record_from_active(&active);

        let allocation_frame_scheduler = UiAllocationFrameFrameworkScheduler::launch(frame_epoch);
        Ok(Self {
            active,
            last_valid,
            retained_allocation_planning_evidence,
            allocation_receipt_ledger: UiAllocationReceiptLedger::for_runtime_generation(
                artifact_digest.raw(),
            ),
            allocation_invalidation_index: RefCell::new(Default::default()),
            allocation_frame_scheduler,
            pending_allocation_frame_handoff: None,
            pending_narrowed_allocation_frame: None,
            allocation_source_order_ledger: Default::default(),
            query_binding,
            transient_interaction_admission: Default::default(),
            host_measurement_source: Rc::new(RefCell::new(Default::default())),
            durable_resize_source: Default::default(),
            scroll_offset_projection: Default::default(),
        })
    }
}
