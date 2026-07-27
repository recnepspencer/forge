use worth_query::facade::domain;

const SCALE_CEILING: u64 = 1_000_000;
const RESOURCE_CEILING: u64 = 1_000_000;
const CONCURRENT_ATTEMPT_LIMIT: usize = 1_024;

pub(crate) fn operation_execution_resource_contract() -> domain::WorthQueryExecutionResourceContract
{
    domain::WorthQueryExecutionResourceContract::declared([
        domain::WorthQueryExecutionStrategyContract::new(
            domain::WorthQueryExecutionStrategyName::new("worth-ui-bounded")
                .expect("the static Worth UI execution strategy must admit"),
            resource_envelope(),
            domain::WorthQueryExecutionProviderRequirements::new(
                provider_family(),
                access_product_family(),
                allocator_family(),
            ),
        ),
    ])
    .expect("the static Worth UI execution resource contract must admit")
}

pub(crate) fn operation_execution_resource_support() -> domain::WorthQueryExecutionResourceSupport {
    static SUPPORT: std::sync::OnceLock<domain::WorthQueryExecutionResourceSupport> =
        std::sync::OnceLock::new();
    SUPPORT
        .get_or_init(|| {
            domain::WorthQueryExecutionResourceSupport::new(
                provider_family(),
                access_product_family(),
                allocator_family(),
                resource_envelope(),
                std::sync::Arc::new(
                    domain::WorthQueryFixedExecutionCapacity::mint(
                        "worth-ui-installed-operation-capacity",
                        CONCURRENT_ATTEMPT_LIMIT,
                    )
                    .expect("the static Worth UI execution capacity must admit"),
                ),
            )
        })
        .clone()
}

pub(crate) fn operation_execution_resource_request() -> domain::WorthQueryExecutionResourceRequest {
    domain::WorthQueryExecutionResourceRequest::bounded(
        SCALE_CEILING,
        RESOURCE_CEILING,
        cancellation_safe_point(),
    )
}

fn resource_envelope() -> domain::WorthQueryExecutionResourceEnvelope {
    domain::WorthQueryExecutionResourceEnvelope::bounded(
        SCALE_CEILING,
        RESOURCE_CEILING,
        domain::WorthQueryExecutionMode::Synchronous,
        cancellation_safe_point(),
    )
}

fn provider_family() -> domain::WorthQueryExecutionProviderFamily {
    domain::WorthQueryExecutionProviderFamily::new("worth-ui-installed")
        .expect("the static Worth UI provider family must admit")
}

fn access_product_family() -> domain::WorthQueryExecutionAccessProductFamily {
    domain::WorthQueryExecutionAccessProductFamily::new("worth-ui-operation")
        .expect("the static Worth UI access product must admit")
}

fn allocator_family() -> domain::WorthQueryExecutionAllocatorFamily {
    domain::WorthQueryExecutionAllocatorFamily::new("worth-ui-workspace")
        .expect("the static Worth UI allocator family must admit")
}

fn cancellation_safe_point() -> domain::WorthQueryCancellationSafePointFamily {
    domain::WorthQueryCancellationSafePointFamily::new("worth-ui-operation-boundary")
        .expect("the static Worth UI cancellation safe point must admit")
}

#[cfg(test)]
mod tests {
    #[test]
    fn installed_operations_share_one_capacity_authority() {
        assert_eq!(
            super::operation_execution_resource_support(),
            super::operation_execution_resource_support()
        );
    }
}
