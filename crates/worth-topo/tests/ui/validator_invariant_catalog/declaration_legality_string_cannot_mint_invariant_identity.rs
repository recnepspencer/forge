use topology::facade::WorthTopologyInvariantFamilyIdentity;

fn main() {
    let declaration_legality_string = "authoritative_hot_artifact";
    let _ = WorthTopologyInvariantFamilyIdentity::registered(
        declaration_legality_string,
        "v1",
    );
}
