use crate::identity::hash_parts;
use crate::live::{
    AdmittedStreamConsumerContract, DeliveryContractLowering, DeliveryLocalityOutcome,
    LivePatchPayload, LivePolicyCounters, LiveQueryFamily, RegionScopedLiveError,
    RegionScopedLiveExecutionEnvelope, RegionScopedLivePlan, StreamConsumerShape,
    StreamContractDigest, StreamContractRequest, StreamLoweredDeliveryContract,
    StreamMemberProjection, StreamWindowCompatibility,
};

pub fn lower_region_scoped_execution_to_stream_contract(
    plan: &RegionScopedLivePlan,
    execution: &RegionScopedLiveExecutionEnvelope,
    consumer_shape: StreamConsumerShape,
) -> Result<StreamLoweredDeliveryContract, RegionScopedLiveError> {
    match (plan.live().descriptor().family(), &consumer_shape) {
        (LiveQueryFamily::Detail, StreamConsumerShape::DetailCurrentState)
        | (LiveQueryFamily::OrderedCollection, StreamConsumerShape::CdcCollectionPatch) => {}
        _ => return Err(RegionScopedLiveError::UnsupportedStreamConsumerShape),
    }

    let query_delivery_contract = crate::live::QueryDeliveryContract {
        digest: hash_parts(&[
            format!("query:{}", execution.report().query_digest()),
            format!("locality:{}", plan.locality().digest().as_str()),
            format!("delivery:{}", execution.report().delivery_digest()),
            format!("family:{}", execution.patch_envelope().family().as_str()),
            format!(
                "locality_outcome:{}",
                DeliveryLocalityOutcome::from_region_scoped_report(execution.report()).as_str()
            ),
        ]),
        query_digest: execution.report().query_digest().to_string(),
        locality_digest: plan.locality().digest().as_str().to_string(),
        delivery_digest: execution.report().delivery_digest().to_string(),
        family: execution.patch_envelope().family().clone(),
        locality_outcome: DeliveryLocalityOutcome::from_region_scoped_report(execution.report()),
    };
    let request = StreamContractRequest {
        digest: hash_parts(&[
            format!("query:{}", execution.report().query_digest()),
            format!("delivery:{}", execution.report().delivery_digest()),
            format!("consumer_shape:{}", consumer_shape.as_str()),
        ]),
        query_digest: execution.report().query_digest().to_string(),
        delivery_digest: execution.report().delivery_digest().to_string(),
        consumer_shape: consumer_shape.clone(),
    };
    let admitted_consumer_contract = AdmittedStreamConsumerContract {
        digest: hash_parts(&[
            format!("query:{}", request.query_digest()),
            format!("delivery:{}", request.delivery_digest()),
            format!("consumer_shape:{}", request.consumer_shape().as_str()),
        ]),
        consumer_shape: consumer_shape.clone(),
    };
    let (member_count, window_width, delivery_width) = stream_contract_widths(
        execution.patch_envelope().payload(),
        execution.report().locality_outcome(),
        &consumer_shape,
    );
    if window_width > plan.stream_window_width_budget().limit() {
        return Err(RegionScopedLiveError::StreamWindowWidthBudgetExceeded {
            limit: plan.stream_window_width_budget().limit(),
            actual: window_width,
        });
    }
    if delivery_width > plan.stream_member_width_budget().limit() {
        return Err(RegionScopedLiveError::StreamMemberWidthBudgetExceeded {
            limit: plan.stream_member_width_budget().limit(),
            actual: delivery_width,
        });
    }
    let member_projection = StreamMemberProjection {
        digest: hash_parts(&[
            format!("consumer_shape:{}", consumer_shape.as_str()),
            format!("member_count:{member_count}"),
            format!("delivery_width:{delivery_width}"),
        ]),
        consumer_shape: consumer_shape.clone(),
        member_count,
        delivery_width,
    };
    let window_compatibility = StreamWindowCompatibility {
        digest: hash_parts(&[
            format!("consumer_shape:{}", consumer_shape.as_str()),
            format!("window_width:{window_width}"),
            format!("budget_limit:{}", plan.stream_window_width_budget().limit()),
        ]),
        consumer_shape: consumer_shape.clone(),
        window_width,
        budget_limit: plan.stream_window_width_budget().limit(),
    };
    let stream_contract_digest = StreamContractDigest(hash_parts(&[
        format!("query_delivery:{}", query_delivery_contract.digest()),
        format!("request:{}", request.digest()),
        format!("admitted_consumer:{}", admitted_consumer_contract.digest()),
        format!("members:{member_count}"),
        format!("window_width:{window_width}"),
        format!("width:{delivery_width}"),
        format!(
            "cost_posture:{}",
            plan.stream_lowering_cost_posture().as_str()
        ),
    ]));
    let delivery_contract_lowering = DeliveryContractLowering {
        digest: hash_parts(&[
            format!("query_delivery:{}", query_delivery_contract.digest()),
            format!("request:{}", request.digest()),
            format!("admitted_consumer:{}", admitted_consumer_contract.digest()),
            format!("stream_contract:{}", stream_contract_digest.as_str()),
        ]),
        query_delivery_digest: query_delivery_contract.digest().to_string(),
        request_digest: request.digest().to_string(),
        admitted_consumer_contract_digest: admitted_consumer_contract.digest().to_string(),
        stream_contract_digest: stream_contract_digest.as_str().to_string(),
    };
    let replay_record = execution
        .region_scoped_replay_bundle()
        .replay_record()
        .with_stream_contract_digest(stream_contract_digest.as_str());
    let mut counter_snapshot = execution.counters().clone();
    counter_snapshot.absorb(&LivePolicyCounters::from_stream_lowered_delivery(
        &StreamLoweredDeliveryContract {
            query_digest: execution.report().query_digest().to_string(),
            locality_digest: plan.locality().digest().as_str().to_string(),
            delivery_digest: execution.report().delivery_digest().to_string(),
            query_delivery_contract: query_delivery_contract.clone(),
            stream_contract_digest: stream_contract_digest.clone(),
            delivery_contract_lowering: delivery_contract_lowering.clone(),
            request: request.clone(),
            admitted_consumer_contract: admitted_consumer_contract.clone(),
            member_projection: member_projection.clone(),
            window_compatibility: window_compatibility.clone(),
            replay_record: replay_record.clone(),
            counter_snapshot: LivePolicyCounters::default(),
            member_count,
            window_width,
            delivery_width,
            cost_posture: plan.stream_lowering_cost_posture().clone(),
        },
    ));

    Ok(StreamLoweredDeliveryContract {
        query_digest: execution.report().query_digest().to_string(),
        locality_digest: plan.locality().digest().as_str().to_string(),
        delivery_digest: execution.report().delivery_digest().to_string(),
        query_delivery_contract,
        stream_contract_digest,
        delivery_contract_lowering,
        request,
        admitted_consumer_contract,
        member_projection,
        window_compatibility,
        replay_record,
        counter_snapshot,
        member_count,
        window_width,
        delivery_width,
        cost_posture: plan.stream_lowering_cost_posture().clone(),
    })
}

fn stream_contract_widths(
    payload: &LivePatchPayload,
    locality_outcome: &DeliveryLocalityOutcome,
    consumer_shape: &StreamConsumerShape,
) -> (usize, usize, usize) {
    match (payload, consumer_shape) {
        (LivePatchPayload::Detail(_), StreamConsumerShape::DetailCurrentState) => {
            let window_width = match locality_outcome {
                DeliveryLocalityOutcome::InRegionRegionWithPeerWidening { peer_scopes }
                | DeliveryLocalityOutcome::InRegionPartitionWithPeerWidening { peer_scopes } => {
                    1 + peer_scopes.len()
                }
                _ => 1,
            };
            (1, window_width, 1)
        }
        (LivePatchPayload::OrderedCollection(patch), StreamConsumerShape::CdcCollectionPatch) => {
            (1, 1, 1 + patch.projected_field_deltas().len())
        }
        (LivePatchPayload::Suppressed(_), _) => (1, 1, 1),
        _ => (1, 1, 1),
    }
}
