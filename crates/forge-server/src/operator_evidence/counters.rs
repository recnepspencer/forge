use forge_foundational::facade::{
    counter_backed_performance_receipt, performance, performance_bundle,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBundleConstructionDenial,
    FoundationalPerformanceClaimConstructionDenial, FoundationalPerformanceContractName,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

use super::{
    classification::ForgeServerOperatorEvidenceClass,
    counter_receipt::ForgeServerOperatorCounterReceipt,
};

const REQUEST_CONTEXT_DENIAL_COUNT: &str = "response.request_context_denial.count";
const MIDDLEWARE_DENIAL_COUNT: &str = "response.middleware_denial.count";
const QUERY_HANDOFF_DENIAL_COUNT: &str = "response.query_handoff_denial.count";
const RESPONSE_SUCCESS_COUNT: &str = "response.success.count";
const RESPONSE_DENIAL_COUNT: &str = "response.denial.count";
const QUERY_READ_SUCCESS_COUNT: &str = "response.query_read_success.count";
const QUERY_MUTATION_SUCCESS_COUNT: &str = "response.query_mutation_success.count";
const DOWNSTREAM_DELIVERY_SUCCESS_COUNT: &str = "response.downstream_delivery_success.count";
const UNSUPPORTED_CAPABILITY_COUNT: &str = "response.unsupported_capability.count";

pub(crate) fn build_counter_receipt(
    class: &ForgeServerOperatorEvidenceClass,
) -> Result<ForgeServerOperatorCounterReceipt, ForgeServerOperatorEvidenceCounterError> {
    let claim = performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::WarmPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
        .include_work(FoundationalPerformanceWorkClass::PublicationDelivery)
        .exclude_work(FoundationalPerformanceWorkClass::ReplayReconstruction)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .exclude_work(FoundationalPerformanceWorkClass::ForensicParity)
        .finish()
        .map_err(ForgeServerOperatorEvidenceCounterError::PerformanceClaim)?;

    let bundle = performance_bundle(claim)
        .attach_contract_name(
            FoundationalPerformanceContractName::new(class.contract_name())
                .expect("static counter contract names should be valid"),
        )
        .attach_counter_spec(counter_spec(
            REQUEST_CONTEXT_DENIAL_COUNT,
            FoundationalPerformanceWorkClass::ValidationPlanning,
            expected_count(class, REQUEST_CONTEXT_DENIAL_COUNT),
        ))
        .attach_counter_spec(counter_spec(
            MIDDLEWARE_DENIAL_COUNT,
            FoundationalPerformanceWorkClass::ValidationPlanning,
            expected_count(class, MIDDLEWARE_DENIAL_COUNT),
        ))
        .attach_counter_spec(counter_spec(
            QUERY_HANDOFF_DENIAL_COUNT,
            FoundationalPerformanceWorkClass::ValidationPlanning,
            expected_count(class, QUERY_HANDOFF_DENIAL_COUNT),
        ))
        .attach_counter_spec(counter_spec(
            RESPONSE_SUCCESS_COUNT,
            FoundationalPerformanceWorkClass::PublicationDelivery,
            expected_count(class, RESPONSE_SUCCESS_COUNT),
        ))
        .attach_counter_spec(counter_spec(
            RESPONSE_DENIAL_COUNT,
            FoundationalPerformanceWorkClass::PublicationDelivery,
            expected_count(class, RESPONSE_DENIAL_COUNT),
        ))
        .attach_counter_spec(counter_spec(
            QUERY_READ_SUCCESS_COUNT,
            FoundationalPerformanceWorkClass::PublicationDelivery,
            expected_count(class, QUERY_READ_SUCCESS_COUNT),
        ))
        .attach_counter_spec(counter_spec(
            QUERY_MUTATION_SUCCESS_COUNT,
            FoundationalPerformanceWorkClass::PublicationDelivery,
            expected_count(class, QUERY_MUTATION_SUCCESS_COUNT),
        ))
        .attach_counter_spec(counter_spec(
            DOWNSTREAM_DELIVERY_SUCCESS_COUNT,
            FoundationalPerformanceWorkClass::PublicationDelivery,
            expected_count(class, DOWNSTREAM_DELIVERY_SUCCESS_COUNT),
        ))
        .attach_counter_spec(counter_spec(
            UNSUPPORTED_CAPABILITY_COUNT,
            FoundationalPerformanceWorkClass::ValidationPlanning,
            expected_count(class, UNSUPPORTED_CAPABILITY_COUNT),
        ))
        .finish()
        .map_err(ForgeServerOperatorEvidenceCounterError::PerformanceBundle)?;

    let receipt = counter_backed_performance_receipt(bundle)
        .attach_counter_row(counter_row(
            REQUEST_CONTEXT_DENIAL_COUNT,
            expected_count(class, REQUEST_CONTEXT_DENIAL_COUNT),
        ))
        .attach_counter_row(counter_row(
            MIDDLEWARE_DENIAL_COUNT,
            expected_count(class, MIDDLEWARE_DENIAL_COUNT),
        ))
        .attach_counter_row(counter_row(
            QUERY_HANDOFF_DENIAL_COUNT,
            expected_count(class, QUERY_HANDOFF_DENIAL_COUNT),
        ))
        .attach_counter_row(counter_row(
            RESPONSE_SUCCESS_COUNT,
            expected_count(class, RESPONSE_SUCCESS_COUNT),
        ))
        .attach_counter_row(counter_row(
            RESPONSE_DENIAL_COUNT,
            expected_count(class, RESPONSE_DENIAL_COUNT),
        ))
        .attach_counter_row(counter_row(
            QUERY_READ_SUCCESS_COUNT,
            expected_count(class, QUERY_READ_SUCCESS_COUNT),
        ))
        .attach_counter_row(counter_row(
            QUERY_MUTATION_SUCCESS_COUNT,
            expected_count(class, QUERY_MUTATION_SUCCESS_COUNT),
        ))
        .attach_counter_row(counter_row(
            DOWNSTREAM_DELIVERY_SUCCESS_COUNT,
            expected_count(class, DOWNSTREAM_DELIVERY_SUCCESS_COUNT),
        ))
        .attach_counter_row(counter_row(
            UNSUPPORTED_CAPABILITY_COUNT,
            expected_count(class, UNSUPPORTED_CAPABILITY_COUNT),
        ))
        .finish()
        .map_err(ForgeServerOperatorEvidenceCounterError::CounterReceipt)?;

    Ok(ForgeServerOperatorCounterReceipt::new(receipt))
}

fn counter_spec(
    name: &'static str,
    work_class: FoundationalPerformanceWorkClass,
    expected_exact_count: u64,
) -> FoundationalPerformanceCounterSpec {
    FoundationalPerformanceCounterSpec::new(
        FoundationalPerformanceCounterName::new(name)
            .expect("static counter names should be valid"),
        work_class,
        expected_exact_count,
    )
}

fn counter_row(name: &'static str, observed_count: u64) -> FoundationalPerformanceCounterRow {
    FoundationalPerformanceCounterRow::new(
        FoundationalPerformanceCounterName::new(name)
            .expect("static counter names should be valid"),
        observed_count,
    )
}

fn expected_count(class: &ForgeServerOperatorEvidenceClass, counter_name: &str) -> u64 {
    match counter_name {
        REQUEST_CONTEXT_DENIAL_COUNT
            if matches!(
                class,
                ForgeServerOperatorEvidenceClass::RequestContextDenied(_)
            ) =>
        {
            1
        }
        MIDDLEWARE_DENIAL_COUNT
            if matches!(class, ForgeServerOperatorEvidenceClass::MiddlewareDenied(_)) =>
        {
            1
        }
        QUERY_HANDOFF_DENIAL_COUNT
            if matches!(
                class,
                ForgeServerOperatorEvidenceClass::QueryHandoffDenied(_)
            ) =>
        {
            1
        }
        RESPONSE_SUCCESS_COUNT
            if matches!(
                class,
                ForgeServerOperatorEvidenceClass::QueryReadSucceeded
                    | ForgeServerOperatorEvidenceClass::QueryMutationSucceeded
                    | ForgeServerOperatorEvidenceClass::DownstreamDeliverySucceeded
            ) =>
        {
            1
        }
        RESPONSE_DENIAL_COUNT
            if matches!(
                class,
                ForgeServerOperatorEvidenceClass::RequestContextDenied(_)
                    | ForgeServerOperatorEvidenceClass::MiddlewareDenied(_)
                    | ForgeServerOperatorEvidenceClass::QueryHandoffDenied(_)
            ) =>
        {
            1
        }
        QUERY_READ_SUCCESS_COUNT
            if matches!(class, ForgeServerOperatorEvidenceClass::QueryReadSucceeded) =>
        {
            1
        }
        QUERY_MUTATION_SUCCESS_COUNT
            if matches!(
                class,
                ForgeServerOperatorEvidenceClass::QueryMutationSucceeded
            ) =>
        {
            1
        }
        DOWNSTREAM_DELIVERY_SUCCESS_COUNT
            if matches!(
                class,
                ForgeServerOperatorEvidenceClass::DownstreamDeliverySucceeded
            ) =>
        {
            1
        }
        UNSUPPORTED_CAPABILITY_COUNT if class.unsupported_capability() => 1,
        _ => 0,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerOperatorEvidenceCounterError {
    PerformanceClaim(FoundationalPerformanceClaimConstructionDenial),
    PerformanceBundle(FoundationalPerformanceBundleConstructionDenial),
    CounterReceipt(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
}
