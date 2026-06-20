use forge_query::facade::{
    ForgeQueryGraphReadAccessAuthorityCounters, ForgeQueryGraphReadAccessComplexityCounters,
    ForgeQueryGraphReadAccessReceiptSummary, ForgeQueryReadFallbackClass, ForgeQueryReadReceipt,
};

use crate::projection::read_views::domain::error::TopologyReadError;

pub(crate) fn require_graph_access_receipt<'a>(
    receipt: &'a ForgeQueryReadReceipt,
    read_surface: &str,
) -> Result<
    (
        &'a ForgeQueryGraphReadAccessReceiptSummary,
        &'a ForgeQueryGraphReadAccessComplexityCounters,
    ),
    TopologyReadError,
> {
    require_no_query_fallback(receipt.fallback_class(), read_surface)?;
    let summary = receipt.graph_read_access_summary().ok_or_else(|| {
        TopologyReadError::read_family_execution_denied(format!(
            "{read_surface} read family executed without a graph-read access receipt"
        ))
    })?;
    if !summary.has_admitted_access_plan() {
        return Err(TopologyReadError::read_family_execution_denied(format!(
            "{read_surface} read family receipt did not prove an admitted access plan"
        )));
    }
    let counters = receipt
        .graph_read_access_complexity_counters()
        .ok_or_else(|| {
            TopologyReadError::read_family_execution_denied(format!(
                "{read_surface} read family executed without graph-read access counters"
            ))
        })?;
    Ok((summary, counters))
}

pub(crate) fn require_no_caller_owned_graph_access(
    receipt: &ForgeQueryReadReceipt,
    counters: &ForgeQueryGraphReadAccessComplexityCounters,
    read_surface: &str,
) -> Result<(), TopologyReadError> {
    let authority_counters = receipt
        .graph_read_access_admission()
        .ok_or_else(|| {
            TopologyReadError::read_family_execution_denied(format!(
                "{read_surface} read family executed without graph-read authority admission"
            ))
        })?
        .authority_receipt()
        .counters();
    require_no_authority_owned_buffer_builds(authority_counters, read_surface)?;
    if counters.executor_strategy_rediscovery_count() != 0 {
        return Err(TopologyReadError::read_family_execution_denied(format!(
            "{read_surface} read family rediscovered graph access strategy during execution"
        )));
    }
    if counters.per_result_neighbor_lookup_count() != 0 {
        return Err(TopologyReadError::read_family_execution_denied(format!(
            "{read_surface} read family performed caller-owned per-result neighbor lookup"
        )));
    }
    if counters.persistent_artifact_bypass_count() != 0 {
        return Err(TopologyReadError::read_family_execution_denied(format!(
            "{read_surface} read family bypassed graph-read persistent artifact posture"
        )));
    }
    Ok(())
}

fn require_no_authority_owned_buffer_builds(
    counters: &ForgeQueryGraphReadAccessAuthorityCounters,
    read_surface: &str,
) -> Result<(), TopologyReadError> {
    if counters.adjacency_buffer_build_count() != 0 {
        return Err(TopologyReadError::read_family_execution_denied(format!(
            "{read_surface} read family built caller-owned adjacency buffers"
        )));
    }
    if counters.frontier_buffer_build_count() != 0 {
        return Err(TopologyReadError::read_family_execution_denied(format!(
            "{read_surface} read family built caller-owned frontier buffers"
        )));
    }
    if counters.visited_buffer_build_count() != 0 {
        return Err(TopologyReadError::read_family_execution_denied(format!(
            "{read_surface} read family built caller-owned visited buffers"
        )));
    }
    if counters.result_buffer_build_count() != 0 {
        return Err(TopologyReadError::read_family_execution_denied(format!(
            "{read_surface} read family built caller-owned result buffers"
        )));
    }
    Ok(())
}

fn require_no_query_fallback(
    fallback_class: &ForgeQueryReadFallbackClass,
    read_surface: &str,
) -> Result<(), TopologyReadError> {
    if fallback_class != &ForgeQueryReadFallbackClass::None {
        return Err(TopologyReadError::read_family_execution_denied(format!(
            "{read_surface} read family unexpectedly executed with fallback `{:?}`",
            fallback_class
        )));
    }
    Ok(())
}
