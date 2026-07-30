use serde_json::{json, Value};

use super::super::protocol::{
    BoundedResidencyAllocationBoundaryObservation, BoundedResidencyAllocationDimensionObservation,
    BoundedResidencyAllocationObservation,
};

pub(super) fn value(allocation: &BoundedResidencyAllocationObservation) -> Value {
    json!({
        "scopes": {
            "admitted_scopes": allocation.scopes.admitted_scopes,
            "exact_scope_denials": allocation.scopes.exact_scope_denials,
            "global_envelope_denied": allocation.scopes.global_envelope_denied,
            "global_denial_requested": allocation.scopes.global_denial_requested,
            "global_denial_current": allocation.scopes.global_denial_current,
            "global_denial_limit": allocation.scopes.global_denial_limit,
            "peak_operation_bytes": allocation.scopes.peak_operation_bytes,
            "terminal_operation_bytes": allocation.scopes.terminal_operation_bytes,
            "all_effect_free": allocation.scopes.all_effect_free,
        },
        "dimensions": allocation
            .dimensions
            .iter()
            .copied()
            .map(dimension)
            .collect::<Vec<_>>(),
        "trace": {
            "store": crate::physical_work_evidence::hex(&allocation.trace.store),
            "pool_incarnation": allocation.trace.pool_incarnation,
            "event_count": allocation.trace.event_count,
            "process": allocation.trace.process,
            "attributed_actualizations": allocation.trace.attributed_actualizations,
            "events": allocation.trace.events.iter().map(boundary).collect::<Vec<_>>(),
        },
    })
}

fn dimension(dimension: BoundedResidencyAllocationDimensionObservation) -> Value {
    json!({
        "name": dimension.name,
        "attempts": dimension.attempts,
        "admissions": dimension.admissions,
        "releases": dimension.releases,
        "denials": dimension.denials,
        "allocator_failures": dimension.allocator_failures,
        "admitted_units": dimension.admitted_units,
        "released_units": dimension.released_units,
        "denied_units": dimension.denied_units,
        "active_units": dimension.active_units,
        "current_units": dimension.current_units,
        "peak_units": dimension.peak_units,
        "limit_units": dimension.limit_units,
    })
}

fn boundary(event: &BoundedResidencyAllocationBoundaryObservation) -> Value {
    json!({
        "sequence": event.sequence,
        "kind": event.kind,
        "dimension": event.dimension,
        "scope": event.scope,
        "requested_units": event.requested_units,
        "actual_units": event.actual_units,
        "process": event.process,
        "physical_operation": event.physical_operation,
    })
}

#[cfg(test)]
mod tests {
    use super::value;
    use crate::courtroom_campaign::bounded_residency_siege::protocol::{
        BoundedResidencyAllocationBoundaryObservation,
        BoundedResidencyAllocationDimensionObservation, BoundedResidencyAllocationObservation,
        BoundedResidencyAllocationTraceObservation, BoundedResidencyScopeObservation,
    };

    #[test]
    fn projection_retains_scope_and_every_allocation_dimension() {
        let allocation = BoundedResidencyAllocationObservation {
            scopes: BoundedResidencyScopeObservation {
                admitted_scopes: 7,
                exact_scope_denials: 7,
                global_envelope_denied: true,
                global_denial_requested: 1,
                global_denial_current: 4_194_304,
                global_denial_limit: 4_194_304,
                peak_operation_bytes: 4_194_304,
                terminal_operation_bytes: 0,
                all_effect_free: true,
            },
            dimensions: std::array::from_fn(dimension),
            trace: BoundedResidencyAllocationTraceObservation {
                store: [0xa5; 16],
                pool_incarnation: 9,
                event_count: 1,
                process: 41,
                attributed_actualizations: 1,
                events: vec![BoundedResidencyAllocationBoundaryObservation {
                    sequence: 1,
                    kind: "actualization",
                    dimension: "resident-bytes",
                    scope: Some("foreground-read"),
                    requested_units: 8,
                    actual_units: 7,
                    process: 41,
                    physical_operation: Some(3),
                }]
                .into_boxed_slice(),
            },
        };
        let projected = value(&allocation);
        let scopes = projected["scopes"]
            .as_object()
            .expect("scope projection must be structured");
        let dimensions = projected["dimensions"]
            .as_array()
            .expect("dimension projection must be structured");
        assert_eq!(scopes["admitted_scopes"], 7);
        assert_eq!(scopes["exact_scope_denials"], 7);
        assert_eq!(dimensions.len(), 19);
        assert_eq!(dimensions[0]["name"], "dimension-0");
        assert_eq!(dimensions[18]["name"], "dimension-18");
        assert!(dimensions
            .iter()
            .all(|dimension| dimension.get("limit_units").is_some()));
        assert_eq!(
            projected["trace"]["store"],
            "a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5"
        );
        assert_eq!(projected["trace"]["pool_incarnation"], 9);
        assert_eq!(projected["trace"]["event_count"], 1);
        assert_eq!(projected["trace"]["process"], 41);
        assert_eq!(projected["trace"]["attributed_actualizations"], 1);
        let event = &projected["trace"]["events"][0];
        assert_eq!(event["sequence"], 1);
        assert_eq!(event["kind"], "actualization");
        assert_eq!(event["dimension"], "resident-bytes");
        assert_eq!(event["scope"], "foreground-read");
        assert_eq!(event["requested_units"], 8);
        assert_eq!(event["actual_units"], 7);
        assert_eq!(event["process"], 41);
        assert_eq!(event["physical_operation"], 3);
    }

    fn dimension(index: usize) -> BoundedResidencyAllocationDimensionObservation {
        const NAMES: [&str; 19] = [
            "dimension-0",
            "dimension-1",
            "dimension-2",
            "dimension-3",
            "dimension-4",
            "dimension-5",
            "dimension-6",
            "dimension-7",
            "dimension-8",
            "dimension-9",
            "dimension-10",
            "dimension-11",
            "dimension-12",
            "dimension-13",
            "dimension-14",
            "dimension-15",
            "dimension-16",
            "dimension-17",
            "dimension-18",
        ];
        BoundedResidencyAllocationDimensionObservation {
            name: NAMES[index],
            attempts: 1,
            admissions: 1,
            releases: 1,
            denials: 0,
            allocator_failures: 0,
            admitted_units: 1,
            released_units: 1,
            denied_units: 0,
            active_units: 0,
            current_units: 0,
            peak_units: 1,
            limit_units: 1,
        }
    }
}
