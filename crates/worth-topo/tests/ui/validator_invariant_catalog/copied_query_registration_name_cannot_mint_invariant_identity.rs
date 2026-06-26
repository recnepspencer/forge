use topology::facade::WorthTopologyInvariantFamilyIdentity;

fn main() {
    let copied_query_registration_name = "topology.loop_wiring.commit_boundary";
    let _ = WorthTopologyInvariantFamilyIdentity::registered(
        copied_query_registration_name,
        "v1",
    );
}
