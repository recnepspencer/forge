use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use super::super::protocol::{
    BoundedResidencyAllocationBoundaryObservation, BoundedResidencyAllocationDimensionObservation,
    BoundedResidencyAllocationObservation,
};

pub(super) struct AllocationProjection<'evidence>(
    pub(super) &'evidence BoundedResidencyAllocationObservation,
);

impl Serialize for AllocationProjection<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let allocation = self.0;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("scopes", &Scopes(allocation))?;
        map.serialize_entry("dimensions", &Dimensions(allocation))?;
        map.serialize_entry("trace", &Trace(allocation))?;
        map.end()
    }
}

struct Scopes<'evidence>(&'evidence BoundedResidencyAllocationObservation);

impl Serialize for Scopes<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let scopes = self.0.scopes;
        let mut map = serializer.serialize_map(Some(9))?;
        map.serialize_entry("admitted_scopes", &scopes.admitted_scopes)?;
        map.serialize_entry("exact_scope_denials", &scopes.exact_scope_denials)?;
        map.serialize_entry("global_envelope_denied", &scopes.global_envelope_denied)?;
        map.serialize_entry("global_denial_requested", &scopes.global_denial_requested)?;
        map.serialize_entry("global_denial_current", &scopes.global_denial_current)?;
        map.serialize_entry("global_denial_limit", &scopes.global_denial_limit)?;
        map.serialize_entry("peak_operation_bytes", &scopes.peak_operation_bytes)?;
        map.serialize_entry("terminal_operation_bytes", &scopes.terminal_operation_bytes)?;
        map.serialize_entry("all_effect_free", &scopes.all_effect_free)?;
        map.end()
    }
}

struct Dimensions<'evidence>(&'evidence BoundedResidencyAllocationObservation);

impl Serialize for Dimensions<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(self.0.dimensions.len()))?;
        for dimension in &self.0.dimensions {
            sequence.serialize_element(&Dimension(*dimension))?;
        }
        sequence.end()
    }
}

struct Dimension(BoundedResidencyAllocationDimensionObservation);

impl Serialize for Dimension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dimension = self.0;
        let mut map = serializer.serialize_map(Some(13))?;
        map.serialize_entry("name", dimension.name)?;
        map.serialize_entry("attempts", &dimension.attempts)?;
        map.serialize_entry("admissions", &dimension.admissions)?;
        map.serialize_entry("releases", &dimension.releases)?;
        map.serialize_entry("denials", &dimension.denials)?;
        map.serialize_entry("allocator_failures", &dimension.allocator_failures)?;
        map.serialize_entry("admitted_units", &dimension.admitted_units)?;
        map.serialize_entry("released_units", &dimension.released_units)?;
        map.serialize_entry("denied_units", &dimension.denied_units)?;
        map.serialize_entry("active_units", &dimension.active_units)?;
        map.serialize_entry("current_units", &dimension.current_units)?;
        map.serialize_entry("peak_units", &dimension.peak_units)?;
        map.serialize_entry("limit_units", &dimension.limit_units)?;
        map.end()
    }
}

struct Trace<'evidence>(&'evidence BoundedResidencyAllocationObservation);

impl Serialize for Trace<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let trace = &self.0.trace;
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("store", &crate::physical_work_evidence::hex(&trace.store))?;
        map.serialize_entry("pool_incarnation", &trace.pool_incarnation)?;
        map.serialize_entry("event_count", &trace.event_count)?;
        map.serialize_entry("process", &trace.process)?;
        map.serialize_entry(
            "attributed_actualizations",
            &trace.attributed_actualizations,
        )?;
        map.serialize_entry("events", &Events(&trace.events))?;
        map.end()
    }
}

struct Events<'evidence>(&'evidence [BoundedResidencyAllocationBoundaryObservation]);

impl Serialize for Events<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for event in self.0 {
            sequence.serialize_element(&Boundary(event))?;
        }
        sequence.end()
    }
}

struct Boundary<'evidence>(&'evidence BoundedResidencyAllocationBoundaryObservation);

impl Serialize for Boundary<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let event = self.0;
        let mut map = serializer.serialize_map(Some(8))?;
        map.serialize_entry("sequence", &event.sequence)?;
        map.serialize_entry("kind", event.kind)?;
        map.serialize_entry("dimension", event.dimension)?;
        map.serialize_entry("scope", &event.scope)?;
        map.serialize_entry("requested_units", &event.requested_units)?;
        map.serialize_entry("actual_units", &event.actual_units)?;
        map.serialize_entry("process", &event.process)?;
        map.serialize_entry("physical_operation", &event.physical_operation)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::AllocationProjection;
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
        let projected = serde_json::to_value(AllocationProjection(&allocation)).unwrap();
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
