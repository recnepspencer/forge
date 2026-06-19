use std::sync::Arc;

use crate::{
    ForgeServerAdmission, ForgeServerOperationAdmissionFacade, ForgeServerOperationFamily,
    ForgeServerOperationRegistry, ForgeServerOperationRequestFacade,
    ForgeServerOperationSchedulerCounters, ForgeServerProductOperationSurfaceDenial,
    ForgeServerProductOperationSurfaceDenialCode, ForgeServerQueryHandoffConfig,
};

use super::{
    runtime_support::{
        build_envelope, build_request_input, close_product_operation_readiness,
        declaration_metadata, validate_payload_schema,
    },
    ForgeServerCompletedProductOperation, ForgeServerExecutedProductReadBatch,
    ForgeServerLoweredProductOperationPlan, ForgeServerProductAdapterRegistry,
    ForgeServerProductApplicationAdapter, ForgeServerProductOperationInput,
    ForgeServerProductOperationOutcome, ForgeServerScheduledProductOperation,
};

pub(super) fn execute_shared_read_batch_from_forge_native(
    operation_registry: &ForgeServerOperationRegistry,
    adapter_registry: &ForgeServerProductAdapterRegistry,
    query_handoff_config: &ForgeServerQueryHandoffConfig,
    admission: &ForgeServerAdmission,
    inputs: Vec<ForgeServerProductOperationInput>,
) -> Result<ForgeServerExecutedProductReadBatch, ForgeServerProductOperationSurfaceDenial> {
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
    let mut counters = ForgeServerOperationSchedulerCounters::default();
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
    Ok(ForgeServerExecutedProductReadBatch::new(
        operations, counters,
    ))
}

struct ForgeServerPreparedProductReadSlot {
    adapter: Arc<dyn ForgeServerProductApplicationAdapter>,
    declaration: super::ForgeServerProductOperationDeclaration,
    scheduled: ForgeServerScheduledProductOperation,
}

fn prepare_shared_read_slot(
    operation_registry: &ForgeServerOperationRegistry,
    adapter_registry: &ForgeServerProductAdapterRegistry,
    query_handoff_config: &ForgeServerQueryHandoffConfig,
    admission: &ForgeServerAdmission,
    input: ForgeServerProductOperationInput,
) -> Result<ForgeServerPreparedProductReadSlot, ForgeServerProductOperationSurfaceDenial> {
    let (adapter, declaration) = adapter_registry
        .resolve(input.operation_name())
        .ok_or_else(|| {
            ForgeServerProductOperationSurfaceDenial::new(
                ForgeServerProductOperationSurfaceDenialCode::UnknownOperationName,
                format!(
                    "no registered product adapter owns `{}`",
                    input.operation_name()
                ),
            )
        })?;
    if declaration.operation_family() != ForgeServerOperationFamily::ProductApplicationRead {
        return Err(ForgeServerProductOperationSurfaceDenial::new(
            ForgeServerProductOperationSurfaceDenialCode::InvalidDeclaration,
            "shared-read batch only admits product read operations".to_string(),
        ));
    }
    let request_input = build_request_input(declaration, &input);
    let payload = input.into_payload();
    validate_payload_schema(declaration, &payload).map_err(|denial| {
        ForgeServerProductOperationSurfaceDenial::new(
            ForgeServerProductOperationSurfaceDenialCode::InvalidDeclaration,
            format!(
                "shared-read batch requires scheduler-backed read proofs, but payload schema validation denied early: {}",
                denial.reason_key()
            ),
        )
    })?;
    let request = ForgeServerOperationRequestFacade::new(operation_registry.clone())
        .admit_from_forge_native_admission(admission, request_input)
        .map_err(ForgeServerProductOperationSurfaceDenial::from_request_denial)?;
    let operation_admission =
        ForgeServerOperationAdmissionFacade::with_operation_registry(operation_registry.clone())
            .admit(
                admission,
                &request,
                declaration_metadata(declaration, &request)?,
            )
            .map_err(ForgeServerProductOperationSurfaceDenial::from_admission_denial)?;
    let readiness = close_product_operation_readiness(
        operation_registry,
        query_handoff_config,
        &operation_admission,
        declaration,
        request.resolved_request_context(),
    )?;
    let scheduled =
        ForgeServerScheduledProductOperation::admit(ForgeServerLoweredProductOperationPlan::new(
            operation_admission,
            declaration.clone(),
            payload,
            readiness.support_posture().clone(),
            readiness.precondition_posture().clone(),
            readiness.concurrency_class(),
        ))?;
    if scheduled.scheduler_admission().scheduler_lane() != "shared-read" {
        return Err(ForgeServerProductOperationSurfaceDenial::new(
            ForgeServerProductOperationSurfaceDenialCode::InvalidDeclaration,
            "shared-read batch requires shared-read scheduler admission".to_string(),
        ));
    }
    Ok(ForgeServerPreparedProductReadSlot {
        adapter: adapter.clone(),
        declaration: declaration.clone(),
        scheduled,
    })
}

fn execute_shared_read_slot(
    slot: ForgeServerPreparedProductReadSlot,
) -> ForgeServerCompletedProductOperation {
    let outcome = match slot.adapter.execute(&slot.scheduled) {
        Ok(success) => ForgeServerProductOperationOutcome::Success(success),
        Err(error) => slot.declaration.error_map().map_error(error),
    };
    let envelope = build_envelope(&slot.scheduled, &outcome);
    ForgeServerCompletedProductOperation::new(outcome, envelope)
        .with_scheduled_operation(&slot.scheduled)
}
