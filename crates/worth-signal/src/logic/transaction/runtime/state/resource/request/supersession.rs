use super::super::observation::output_continuity::ResourceTerminalVisibilityCause;
use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime::state::resource::request) fn supersede_active_request_for_node(
        &mut self,
        node: ResourceNodeId,
        replacing: ResourceRequestHandle,
        replacing_descriptor_id: ResourceDescriptorId,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<ResourceSupersessionRecord> {
        let request_id = self.active_request_by_node.get(&node).copied()?;
        let (supersession_digest, permits_overlap_admission, requests_old_host_work_cancel) = {
            let plan = self
                .descriptors
                .get(&replacing_descriptor_id)?
                .supersession_decision_plan();
            (
                plan.decision_digest().clone(),
                plan.permits_overlapping_generation_admission(),
                plan.requests_old_host_work_advisory_cancel(),
            )
        };
        let ordinal = self.issue_lifecycle_ordinal();
        let supersession_ordinal = self.issue_supersession_ordinal();
        let (output_continuity, _) = self.classify_terminal_output_continuity_for_node(
            node,
            replacing_descriptor_id,
            ResourceTerminalVisibilityCause::Supersession,
            telemetry,
        );
        let in_flight = self.in_flight_by_request.get_mut(&request_id)?;
        let previous = in_flight.handle();
        in_flight.supersede(ordinal, replacing);
        self.mark_terminal_in_flight(request_id);
        telemetry.resource_supersession_policy_decision_count += 1;
        telemetry.resource_superseded_in_flight_count += 1;
        telemetry.resource_supersession_record_count += 1;
        telemetry.resource_supersession_lineage_width =
            telemetry.resource_supersession_lineage_width.max(2);
        let overlap_admission = if permits_overlap_admission {
            telemetry.resource_overlapping_generation_admission_count += 1;
            if requests_old_host_work_cancel {
                telemetry.resource_old_host_work_advisory_cancelled_count += 1;
            } else {
                telemetry.resource_old_host_work_retained_count += 1;
            }
            Some(ResourceOverlappingGenerationAdmission::new(
                previous,
                replacing,
                supersession_digest.clone(),
                requests_old_host_work_cancel.then(|| {
                    ResourceOldHostWorkCancellationAdvisory::requested(supersession_digest.clone())
                }),
            ))
        } else {
            None
        };
        Some(ResourceSupersessionRecord::new(
            supersession_ordinal,
            previous,
            replacing,
            supersession_digest,
            overlap_admission,
            ResourceLifecycleTransition::new(
                node,
                ResourceLifecycleClass::Pending,
                ResourceLifecycleClass::Superseded,
                ResourceLifecycleTransitionKind::RequestSuperseded,
                ordinal,
                output_continuity,
            ),
        ))
    }
}
