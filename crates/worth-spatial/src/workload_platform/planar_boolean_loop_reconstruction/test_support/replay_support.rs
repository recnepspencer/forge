use topology::facade::TopologyWorkload;

pub(crate) fn retained_replay_receipt_chain(
    declaration: &str,
) -> crate::workload_platform::retained_replay_workload::ReplayReceiptSet {
    let topology_receipt = TopologyWorkload::declared(format!("{declaration}:topology"))
        .from_query_declaration(format!("{declaration}:query"))
        .expect("test replay support should admit topology workload receipt");
    let geometry_binding =
        crate::workload_platform::vocabulary::GeometryBindingWorkload::for_topology_receipt(
            &topology_receipt,
        )
        .declared(format!("{declaration}:geometry-binding"))
        .admit()
        .expect("test replay support should admit geometry binding workload");
    let surface_support =
        crate::workload_platform::vocabulary::SurfaceSupportWorkload::for_geometry_binding(
            &geometry_binding,
        )
        .declared(format!("{declaration}:surface-support"))
        .admit()
        .expect("test replay support should admit surface support workload");
    let projection = crate::workload_platform::vocabulary::ProjectionWorkload::for_surface_support(
        &surface_support,
    )
    .declared(format!("{declaration}:projection"))
    .admit()
    .expect("test replay support should admit projection workload");
    let transform =
        crate::workload_platform::vocabulary::TransformWorkload::for_projection(&projection)
            .declared(format!("{declaration}:transform"))
            .admit()
            .expect("test replay support should admit transform workload");
    let retained_replay =
        crate::workload_platform::vocabulary::RetainedReplayWorkload::for_transform(&transform)
            .declared(format!("{declaration}:retained-replay"))
            .admit()
            .expect("test replay support should admit retained replay workload");
    crate::workload_platform::retained_replay_workload::ReplayReceiptSet::new(
        retained_replay,
        format!("{declaration}:transformed-workload"),
        format!("{declaration}:retained-artifact"),
        format!("{declaration}:retained-capture"),
        format!("{declaration}:retained-basis"),
        format!("{declaration}:replay-checkpoint"),
        format!("{declaration}:replay-evidence"),
        crate::workload_platform::retained_replay_workload::ReplayWorkloadCounters::new(1, 1, 1, 1),
    )
}
