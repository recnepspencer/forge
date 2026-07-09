use std::sync::Arc;

use crate::{
    WorthServerAdmission, WorthServerOperationAdmissionFacade, WorthServerOperationFamily,
    WorthServerOperationRegistry, WorthServerOperationRequestFacade,
    WorthServerOperationSchedulerCounters, WorthServerProductOperationSurfaceDenial,
    WorthServerProductOperationSurfaceDenialCode, WorthServerQueryHandoffConfig,
};

use super::{
    runtime_support::{
        build_envelope, build_request_input, close_product_operation_readiness,
        declaration_metadata, validate_payload_schema,
    },
    WorthServerCompletedProductOperation, WorthServerExecutedProductReadBatch,
    WorthServerLoweredProductOperationPlan, WorthServerProductAdapterRegistry,
    WorthServerProductApplicationAdapter, WorthServerProductOperationInput,
    WorthServerProductOperationOutcome, WorthServerScheduledProductOperation,
};

pub(super) fn execute_shared_read_batch_from_worth_native(
    operation_registry: &WorthServerOperationRegistry,
    adapter_registry: &WorthServerProductAdapterRegistry,
    query_handoff_config: &WorthServerQueryHandoffConfig,
    admission: &WorthServerAdmission,
    inputs: Vec<WorthServerProductOperationInput>,
) -> Result<WorthServerExecutedProductReadBatch, WorthServerProductOperationSurfaceDenial> {
    let prepared = inputs
        .into_iter()
        .map(|input| {
            prepare_shared_read_slot(
                operation_registry,
                adapter_registry,
                query_handoff_config,
                admission,
                input,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut counters = WorthServerOperationSchedulerCounters::default();
    counters.set_planned_batch_width(prepared.len());
    counters.increment_admitted_read_slot_count_by(prepared.len());
    counters.increment_queued_read_slot_count_by(prepared.len());
    let operations = std::thread::scope(|scope| {
        prepared
            .into_iter()
            .map(|slot| scope.spawn(move || execute_shared_read_slot(slot)))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|handle| handle.join().expect("shared read slot thread should join"))
            .collect::<Vec<_>>()
    });
    counters.increment_completed_read_slot_count_by(operations.len());
    Ok(WorthServerExecutedProductReadBatch::new(
        operations, counters,
    ))
}

struct WorthServerPreparedProductReadSlot {
    adapter: Arc<dyn WorthServerProductApplicationAdapter>,
    declaration: super::WorthServerProductOperationDeclaration,
    scheduled: WorthServerScheduledProductOperation,
}

fn prepare_shared_read_slot(
    operation_registry: &WorthServerOperationRegistry,
    adapter_registry: &WorthServerProductAdapterRegistry,
    query_handoff_config: &WorthServerQueryHandoffConfig,
    admission: &WorthServerAdmission,
    input: WorthServerProductOperationInput,
) -> Result<WorthServerPreparedProductReadSlot, WorthServerProductOperationSurfaceDenial> {
    let (adapter, declaration) = adapter_registry
        .resolve(input.operation_name())
        .ok_or_else(|| {
            WorthServerProductOperationSurfaceDenial::new(
                WorthServerProductOperationSurfaceDenialCode::UnknownOperationName,
                format!(
                    "no registered product adapter owns `{}`",
                    input.operation_name()
                ),
            )
        })?;
    if declaration.operation_family() != WorthServerOperationFamily::ProductApplicationRead {
        return Err(WorthServerProductOperationSurfaceDenial::new(
            WorthServerProductOperationSurfaceDenialCode::InvalidDeclaration,
            "shared-read batch only admits product read operations".to_string(),
        ));
    }
    let request_input = build_request_input(declaration, &input);
    let payload = input.into_payload();
    validate_payload_schema(declaration, &payload).map_err(|denial| {
        WorthServerProductOperationSurfaceDenial::new(
            WorthServerProductOperationSurfaceDenialCode::InvalidDeclaration,
            format!(
                "shared-read batch requires scheduler-backed read proofs, but payload schema validation denied early: {}",
                denial.reason_key()
            ),
        )
    })?;
    let request = WorthServerOperationRequestFacade::new(operation_registry.clone())
        .admit_from_worth_native_admission(admission, request_input)
        .map_err(WorthServerProductOperationSurfaceDenial::from_request_denial)?;
    let operation_admission =
        WorthServerOperationAdmissionFacade::with_operation_registry(operation_registry.clone())
            .admit(
                admission,
                &request,
                declaration_metadata(declaration, &request)?,
            )
            .map_err(WorthServerProductOperationSurfaceDenial::from_admission_denial)?;
    let readiness = close_product_operation_readiness(
        operation_registry,
        query_handoff_config,
        &operation_admission,
        declaration,
        request.resolved_request_context(),
    )?;
    let scheduled =
        WorthServerScheduledProductOperation::admit(WorthServerLoweredProductOperationPlan::new(
            operation_admission,
            declaration.clone(),
            payload,
            readiness.support_posture().clone(),
            readiness.precondition_posture().clone(),
            readiness.concurrency_class(),
        ))?;
    if scheduled.scheduler_admission().scheduler_lane() != "shared-read" {
        return Err(WorthServerProductOperationSurfaceDenial::new(
            WorthServerProductOperationSurfaceDenialCode::InvalidDeclaration,
            "shared-read batch requires shared-read scheduler admission".to_string(),
        ));
    }
    Ok(WorthServerPreparedProductReadSlot {
        adapter: adapter.clone(),
        declaration: declaration.clone(),
        scheduled,
    })
}

fn execute_shared_read_slot(
    slot: WorthServerPreparedProductReadSlot,
) -> WorthServerCompletedProductOperation {
    let outcome = match slot.adapter.execute(&slot.scheduled) {
        Ok(success) => WorthServerProductOperationOutcome::Success(success),
        Err(error) => slot.declaration.error_map().map_error(error),
    };
    let envelope = build_envelope(&slot.scheduled, &outcome);
    WorthServerCompletedProductOperation::new(outcome, envelope)
        .with_scheduled_operation(&slot.scheduled)
}
